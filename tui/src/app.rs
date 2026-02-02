use crate::db::Attempt;
use crate::db::Repo;
use crate::db::Task;
use crate::db::Test;
use crate::docker;
use crate::io;
use crate::popup::Popup;
use crate::pty::ui::PtyExitStatus;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::{ListState, ScrollbarState, TableState};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const VERSION: &str = "0.0.3";

#[derive(Debug, Error)]
pub enum Error {
    #[error("While working with config: {0}")]
    ConfigError(#[from] ConfigIOError),

    // TODO: Disambiguate
    #[error("While working with Docker: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("While trying to run task: {0}")]
    RunTaskError(#[from] docker::RunTaskError),
}

#[derive(Debug, Error)]
pub enum ConfigIOError {
    #[error("While saving config: {0}")]
    Saving(#[from] SavingConfigError),

    #[error("While  loading config : {0}")]
    Loading(#[from] LoadingConfigError),
}

#[derive(Debug, Error)]
pub enum SavingConfigError {
    #[error("While serializing to TOML: {0}")]
    SerializingError(#[from] toml::ser::Error),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum LoadingConfigError {
    #[error("While reading from TOML: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestResult {
    pub id: usize,
    pub description: String,
    pub passed: bool,
}

#[derive(PartialEq)]
pub enum AppStatus {
    Idling,
    RunningTask,
    RestartingTask,
    ShowingAttempts,
    Exiting,
}

pub struct AttemptsTableConfig {
    pub attempts_table_state: TableState,
    pub attempts_scrollbar_state: ScrollbarState,
    pub attempt_under_cursor: usize,
}

impl AttemptsTableConfig {
    pub fn default() -> Self {
        AttemptsTableConfig {
            attempts_table_state: TableState::default(),
            attempts_scrollbar_state: ScrollbarState::default(),
            attempt_under_cursor: 0,
        }
    }
}

// TODO: Rewrite tasks in struct
pub struct App {
    pub user: String,
    pub repo: Repo,
    pub table_state: TableState,
    pub task_under_cursor: usize,
    pub status: AppStatus,
    pub active_popup: Option<Popup>,

    pub attempts_table_config: AttemptsTableConfig,

    pub tests_list_state: ListState,
    pub tests_scrollbar_state: ScrollbarState,
}

#[cfg(debug_assertions)]
const INFO_PATH: &str = "src/info.toml";

#[cfg(not(debug_assertions))]
const INFO_PATH: &str = "/etc/git-trainer/info.toml";

#[cfg(debug_assertions)]
const ATTEMPTS_REPO_PATH: &str = "src/attempts.toml";

#[cfg(not(debug_assertions))]
const ATTEMPTS_REPO_PATH: &str = "/etc/git-trainer/attempts.toml";

impl App {
    pub fn new() -> App {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        let repo = Repo::init_database();
        let username = whoami::username()
            .expect("While getting username:")
            .to_string()
            .replace(" ", "-");
        if !repo.user_exists(&username).expect("While working with db:") {
            let _ = repo.create_user(&username);
        }
        App {
            user: username,
            repo: repo,
            table_state: table_state,
            attempts_table_config: AttemptsTableConfig::default(),
            task_under_cursor: 0,
            status: AppStatus::Idling,
            active_popup: None,
            tests_list_state: ListState::default(),
            tests_scrollbar_state: ScrollbarState::default(),
        }
    }

    // На то, чтобы придумать эту функцию ушло 500 миллиардов нейронов
    pub fn render(&mut self, frame: &mut Frame) {
        match self.status {
            AppStatus::Idling => self.render_main_menu(frame),
            AppStatus::ShowingAttempts => self.render_attempt_manager(frame),
            _ => {}
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match self.status {
            AppStatus::Idling => self.main_menu_handle_events()?,
            AppStatus::ShowingAttempts => self.attempt_manager_handle_events()?,
            _ => {}
        };
        Ok(())
    }

    pub async fn run_app(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.status != AppStatus::Exiting {
            // self.status = AppStatus::ShowingAttempts;
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
            match self.status {
                AppStatus::RestartingTask => {
                    match docker::restart_task(&self.task_choosed()).await {
                        Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),

                        _ => {}
                    };
                    self.status = AppStatus::Idling;
                }
                AppStatus::RunningTask => {
                    match self
                        .prepare_pty_bollard(terminal, &self.task_choosed())
                        .await
                    {
                        Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),
                        Ok(PtyExitStatus::RestartTask) => {
                            match docker::restart_task(&self.task_choosed()).await {
                                Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),

                                _ => {}
                            };
                        }
                        Ok(PtyExitStatus::Exit) => {
                            self.status = AppStatus::Idling;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn task_choosed(&self) -> impl Task + use<> {
        self.repo.get_all_tasks().expect("While working with db:")[self.task_under_cursor].clone()
    }

    pub fn get_attempts_of_task(&self) -> Vec<Attempt> {
        let task = self.task_choosed();
        self.repo
            .get_task_attempts(task.id())
            .expect("While working with db:")
    }

    pub fn get_tests_of_attempt(&self) -> Vec<Test> {
        let attempt = self.get_attempts_of_task();
        if attempt.is_empty() {
            return Vec::new();
        }
        self.repo
            .get_attempt_tests(
                attempt[self.attempts_table_config.attempt_under_cursor]
                    .id
                    .expect("While working with db:"),
            )
            .expect("While working with db:")
    }
}
