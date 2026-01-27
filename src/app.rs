use crate::app::event::Event;
use crate::docker;
use crate::io;
use crate::task::Task;
use crate::ui;
use crate::ui::Popup;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use std::fs;
use thiserror::Error;

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

// TODO: Rewrite tasks in struct
pub struct App {
    pub table_state: TableState,
    pub config: Config,
    pub task_under_cursor: usize,
    pub status: AppStatus,
    pub active_popup: Option<Popup>,
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
            config: { Config::load_config(INFO_PATH).expect("failed to load config") },
            task_under_cursor: 0,
            status: AppStatus::Idling,
            active_popup: None,
        }
    }

    pub async fn run_app(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.status != AppStatus::Exiting {
            terminal.draw(|f| ui(f, self))?;
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
                    let task = self.config.tasks[self.task_under_cursor].clone();

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

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.previous_row(),
            KeyCode::Down => self.next_row(),
            KeyCode::Enter => {
                if let Some(popup) = self.active_popup.take() {
                    match popup {
                        Popup::RunConifrmation => self.status = AppStatus::RunningTask,
                        Popup::ResetConfirmation => {
                            self.status = AppStatus::RestartingTask;
                            self.active_popup = Some(Popup::ResetDone);
                        }
                        _ => self.active_popup = None,
                    }
                } else {
                    self.active_popup = Some(Popup::RunConifrmation);
                }
            }
            KeyCode::Char('r') => {
                self.active_popup = Some(Popup::ResetConfirmation);
            }

            KeyCode::Esc => {
                self.active_popup = None;
            }
            _ => {}
        }
    }

    pub fn next_row(&mut self) {
        if self.is_popup_active() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) if i + 1 < self.config.tasks.len() => i + 1,
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.task_under_cursor = i;
    }

    pub fn previous_row(&mut self) {
        if self.is_popup_active() {
            return;
        }
        match &self.active_popup {
            Some(popup) => match popup {
                Popup::Error(_) => return,
                _ => {}
            },
            _ => {}
        }
        let len = self.config.tasks.len();
        let i = match self.table_state.selected() {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };
        self.table_state.select(Some(i));
        self.task_under_cursor = i;
    }

    fn exit(&mut self) {
        self.status = AppStatus::Exiting;
    }

    fn is_popup_active(&self) -> bool {
        match self.active_popup {
            Some(_) => true,
            _ => false,
        }
    }
}
