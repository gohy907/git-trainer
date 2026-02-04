use crate::db::{Attempt, Repo, Task, Test, TestResult, User};
use crate::docker;
use crate::io;
use crate::popup::Popup;
use crate::pty::ui::PtyExitStatus;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::{ListState, ScrollbarState, TableState};
use rusqlite::Error as SqlError;
use std::fs;
use thiserror::Error;

pub const VERSION: &str = "0.1.0";

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
        let mut attempts_table_state = TableState::default();
        attempts_table_state.select(Some(0));
        AttemptsTableConfig {
            attempts_table_state: attempts_table_state,
            attempts_scrollbar_state: ScrollbarState::default(),
            attempt_under_cursor: 0,
        }
    }
}

pub struct TestsTableConfig {
    pub list_state: ListState,
    pub scrollbar_state: ScrollbarState,
    pub test_under_cursor: usize,
}

impl TestsTableConfig {
    pub fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        TestsTableConfig {
            list_state: list_state,
            scrollbar_state: ScrollbarState::default(),
            test_under_cursor: 0,
        }
    }
}

#[derive(PartialEq)]
pub enum AttemptManagerStatus {
    SelectingAttempts,
    SelectingTests,
}

pub struct AttemptManagerConfig {
    pub status: AttemptManagerStatus,
    pub attempts_table_config: AttemptsTableConfig,
    pub tests_table_config: TestsTableConfig,
}

impl AttemptManagerConfig {
    pub fn default() -> AttemptManagerConfig {
        AttemptManagerConfig {
            status: AttemptManagerStatus::SelectingAttempts,
            attempts_table_config: AttemptsTableConfig::default(),
            tests_table_config: TestsTableConfig::default(),
        }
    }
}

pub struct Context {
    pub user: Result<User, SqlError>,
    pub tasks: Result<Vec<Task>, SqlError>,
}

// TODO: Rewrite tasks in struct
pub struct App {
    pub repo: Repo,
    pub context: Context,
    pub table_state: TableState,
    pub task_under_cursor: usize,
    pub status: AppStatus,
    pub active_popup: Option<Popup>,

    pub attempt_manager_config: AttemptManagerConfig,
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
            context: Context {
                user: repo.get_user_by_username(username),
                tasks: repo.get_all_tasks(),
            },
            repo: repo,
            table_state: table_state,
            task_under_cursor: 0,
            status: AppStatus::Idling,
            active_popup: None,
            attempt_manager_config: AttemptManagerConfig::default(),
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
            self.update_context();
            // self.status = AppStatus::ShowingAttempts;
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
            match self.status {
                AppStatus::RestartingTask => {
                    match docker::restart_task(&self.task_under_cursor()).await {
                        Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),

                        _ => {}
                    };
                    self.status = AppStatus::Idling;
                }
                AppStatus::RunningTask => match self.prepare_pty_bollard(terminal).await {
                    Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),
                    Ok(PtyExitStatus::RestartTask) => {
                        match docker::restart_task(&self.task_under_cursor()).await {
                            Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),

                            _ => {}
                        };
                    }
                    Ok(PtyExitStatus::Exit) => {
                        self.status = AppStatus::Idling;
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }

    pub fn task_under_cursor(&self) -> &Task {
        &self.context.tasks.as_ref().expect("While working wirh db:")[self.task_under_cursor]
    }

    pub fn attempts_of_choosed_task(&self) -> &Vec<Attempt> {
        let task = self.task_under_cursor();
        task.attempts.as_ref().expect("While working wih db:")
    }

    pub fn attempt_under_cursor(&self) -> Option<&Attempt> {
        let attempts = &self.attempts_of_choosed_task();
        if attempts.len() == 0 {
            return None;
        }
        Some(
            &attempts[self
                .attempt_manager_config
                .attempts_table_config
                .attempt_under_cursor],
        )
    }

    pub fn tests_of_choosed_attempt(&self) -> Vec<Test> {
        let attempt = self.attempt_under_cursor();
        match attempt {
            Some(attempt) => attempt
                .tests
                .as_ref()
                .expect("While working with db:")
                .to_vec(),
            None => Vec::new(),
        }
    }

    pub async fn test_submitted_task(&mut self) {
        let task = self.task_under_cursor();
        let path = format!("tests/{}", task.work_name);
        let count = fs::read_dir(&path)
            .expect("No test directory for task")
            .count();

        docker::copy_directory(&task.container_name, &path, "/etc/git-trainer/tests")
            .await
            .unwrap();

        let mut test_results = Vec::new();
        let mut failed = false;
        for i in 1..count + 1 {
            if !failed {
                let cmd = format!("/etc/git-trainer/tests/test{}.sh", i);
                let res = docker::exec_command(task, &cmd).await.unwrap();
                if res.exit_code == 0 {
                    test_results.push(Test {
                        id: 0,
                        attempt_id: 0,
                        description: res.output,
                        result: TestResult::Passed,
                    });
                } else {
                    test_results.push(Test {
                        id: 0,
                        attempt_id: 0,
                        description: res.output,
                        result: TestResult::Failed,
                    });
                    failed = true;
                }
            } else {
                let res = format!("{}. Не выполнялся.", i);
                test_results.push(Test {
                    id: 0,
                    attempt_id: 0,
                    description: res,
                    result: TestResult::NotExecuted,
                });
            }
        }

        let user_id = self
            .context
            .user
            .as_ref()
            .expect("While working with db:")
            .id;

        let attempt = Attempt {
            id: 0,
            task_id: task.id,
            user_id: user_id,
            tests: Ok(test_results),
            timestamp: Ok("0".to_string()),
        };

        self.repo
            .create_attempt(user_id, task.id, attempt.into())
            .expect("While working with db:");
    }

    pub fn update_context(&mut self) {
        self.context.tasks = self.repo.get_all_tasks();
    }
}
