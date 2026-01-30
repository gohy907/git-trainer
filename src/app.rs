use crate::docker;
use crate::io;
use crate::popup::Popup;
use crate::task::Task;
use crate::test::Test;
use chrono::{DateTime, Utc};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::{ListState, ScrollbarState, TableState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AttemptRepo {
    pub tasks: HashMap<String, TaskAttempts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskAttempts {
    pub attempts: Vec<Attempt>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attempt {
    pub grade: u32,
    pub timestamp: DateTime<Utc>,
    pub tests: Vec<TestResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestResult {
    pub id: usize,
    pub description: String,
    pub passed: bool,
}

impl AttemptRepo {
    pub fn load(path: &str) -> Result<Self, LoadingConfigError> {
        let text = fs::read_to_string(path).unwrap_or_else(|_| String::from("")); // пустой файл если не существует

        if text.is_empty() {
            Ok(AttemptRepo::default())
        } else {
            Ok(toml::from_str::<AttemptRepo>(&text)?)
        }
    }

    pub fn save(&self, path: &str) -> Result<(), SavingConfigError> {
        let toml_str = toml::to_string_pretty(&self)?;
        fs::write(path, toml_str)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub first_time: bool,
    pub first_time_desc: String,
    pub tasks: Vec<Task>,
    #[serde(skip)]
    pub attempt_repo: AttemptRepo,
}

#[derive(PartialEq)]
pub enum AppStatus {
    Idling,
    RunningTask,
    RestartingTask,
    ShowingAttempts,
    Exiting,
}

impl Config {
    pub fn load_config(path: &str) -> Result<Self, LoadingConfigError> {
        // Загружаем основной конфиг
        let info_text = fs::read_to_string(path)?;
        let mut config = toml::from_str::<Config>(&info_text)?;

        // Загружаем attempt_repo из другого файла
        config.attempt_repo = AttemptRepo::load(ATTEMPTS_REPO_PATH)?;

        Ok(config)
    }

    pub fn save_config(&self) -> Result<(), SavingConfigError> {
        // Сохраняем основной конфиг (без attempt_repo)
        let toml_str = toml::to_string_pretty(&self)?;
        fs::write(INFO_PATH, toml_str)?;

        // Сохраняем attempt_repo отдельно
        self.attempt_repo.save(ATTEMPTS_REPO_PATH)?;

        Ok(())
    }
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
    pub table_state: TableState,
    pub config: Config,
    pub task_under_cursor: usize,
    pub status: AppStatus,
    pub active_popup: Option<Popup>,

    pub attempts_table_config: AttemptsTableConfig,

    pub tests: Option<Vec<Test>>,
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
        App {
            table_state: table_state,
            attempts_table_config: AttemptsTableConfig::default(),
            config: { Config::load_config(INFO_PATH).expect("failed to load config") },
            task_under_cursor: 0,
            status: AppStatus::Idling,
            active_popup: None,
            tests: None,
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
                    let task = &mut self.config.tasks[self.task_under_cursor];
                    match docker::restart_task(task).await {
                        Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),

                        _ => {}
                    };
                    self.status = AppStatus::Idling;
                }
                AppStatus::RunningTask => {
                    let task = self.task_choosed();
                    match self.prepare_pty_bollard(terminal, &task).await {
                        Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),
                        _ => {}
                    }
                    self.status = AppStatus::Idling;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn task_choosed(&self) -> Task {
        self.config.tasks[self.task_under_cursor].clone()
    }

    pub fn submitted_attempts(&self) -> &Vec<Attempt> {
        &self.config.attempt_repo.tasks[&self.task_choosed().work_name].attempts
    }
}
