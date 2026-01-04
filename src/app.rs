use crate::Backend;
use crate::Terminal;
use crate::app::event::Event;
use crate::io;
use crate::ui;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use std::fs;
use toml::de::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub first_time: bool,
    pub first_time_desc: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub name: String,
    pub desc: String,
    pub work_name: String,
    pub dir: String,
    pub in_progress: bool,
    pub done: bool,
    pub pending: bool,
    pub approved: bool,
}

fn load_config(path: &str) -> Result<Config, Error> {
    let text = fs::read_to_string(path).expect("failed to read config");
    toml::from_str::<Config>(&text)
}

// TODO: Rewrite tasks in struct
pub struct App {
    pub config: Config,
    pub task_under_cursor: usize,
    pub is_popup_active: bool,
    pub exit: bool,
    pub task_to_run: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            config: {
                #[cfg(debug_assertions)]
                {
                    load_config("info.toml").unwrap()
                }

                #[cfg(not(debug_assertions))]
                {
                    load_config("info.toml").unwrap()
                }
            },
            exit: false,
            is_popup_active: false,
            task_under_cursor: 0,
            task_to_run: None,
        }
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| ui(f, &self))?;
            self.handle_events()?;
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
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Enter => {
                if self.is_popup_active {
                    self.exit();
                    self.is_popup_active = false;
                } else {
                    self.is_popup_active = true;
                }
            }

            KeyCode::Esc => {
                self.is_popup_active = false;
            }
            _ => {}
        }
    }

    fn move_cursor_up(&mut self) {
        if self.task_under_cursor != 0 {
            self.task_under_cursor -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        // if self.task_under_cursor != self.tasks.len() - 1 {
        //     self.task_under_cursor += 1;
        // }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
