use crate::docker;
use crate::io;
use crate::popup::Popup;
use crate::task::Task;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::TableState;
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
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub first_time: bool,
    pub first_time_desc: String,
    pub tasks: Vec<Task>,
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
        let text = fs::read_to_string(path).expect("failed to read config");
        Ok(toml::from_str::<Config>(&text)?)
    }

    pub fn save_config(&self) -> Result<(), SavingConfigError> {
        let toml_str = toml::to_string_pretty(&self)?;
        fs::write(INFO_PATH, toml_str)?;
        Ok(())
    }
}

pub struct Attempt {
    pub date: String,
    pub grade: String,
}

// TODO: Rewrite tasks in struct
pub struct App {
    pub table_state: TableState,
    pub config: Config,
    pub task_under_cursor: usize,
    pub status: AppStatus,
    pub active_popup: Option<Popup>,
    pub attempts: Option<Vec<Attempt>>,
    pub attempts_table_state: TableState,
}

#[cfg(debug_assertions)]
const INFO_PATH: &str = "src/info.toml";

#[cfg(not(debug_assertions))]
const INFO_PATH: &str = "/etc/git-trainer/info.toml";

impl App {
    pub fn new() -> App {
        let mut table_state = TableState::default();
        table_state.select(Some(0)); // стартуем с первой строки
        App {
            table_state: table_state,
            attempts_table_state: table_state,
            config: { Config::load_config(INFO_PATH).expect("failed to load config") },
            task_under_cursor: 0,
            status: AppStatus::Idling,
            active_popup: None,
            attempts: Some(vec![
                Attempt {
                    date: "Попытка от 01.01.26".to_string(),
                    grade: "5/100".to_string(),
                },
                Attempt {
                    date: "Попытка от 15.01.26".to_string(),
                    grade: "23/100".to_string(),
                },
                Attempt {
                    date: "Попытка от 20.01.26".to_string(),
                    grade: "45/100".to_string(),
                },
                Attempt {
                    date: "Попытка от 25.01.26".to_string(),
                    grade: "67/100".to_string(),
                },
                Attempt {
                    date: "Попытка от 29.01.26".to_string(),
                    grade: "89/100".to_string(),
                },
            ]),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match self.status {
            AppStatus::Idling => {
                self.render_main_menu(frame);
            }
            _ => {}
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match self.status {
            AppStatus::Idling => {
                self.main_menu_handle_events()?;
            }
            _ => {}
        };
        Ok(())
    }

    pub async fn run_app(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.status != AppStatus::Exiting {
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
}
