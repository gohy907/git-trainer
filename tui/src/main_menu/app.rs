use crate::AppStatus;
use crate::app::App;
use crate::popup::Popup;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

impl App {
    pub fn main_menu_handle_events(&mut self) -> io::Result<()> {
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
        let len = self.repo.get_tasks_count().unwrap_or(0);
        let i = if len != 0 {
            if self.task_under_cursor != len - 1 {
                self.task_under_cursor + 1
            } else {
                0
            }
        } else {
            len
        };
        self.table_state.select(Some(i));
        self.task_under_cursor = i;
    }

    pub fn previous_row(&mut self) {
        if self.is_popup_active() {
            return;
        }
        let len = self.repo.get_tasks_count().unwrap_or(0);
        let i = if len != 0 {
            if self.task_under_cursor != 0 {
                self.task_under_cursor - 1
            } else {
                len - 1
            }
        } else {
            len
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
