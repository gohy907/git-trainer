use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::io;

impl App {
    // Прокрутка вниз
    fn next_attempt(&mut self) {
        let attempts = self.submitted_attempts();
        let i = match self.attempts_table_config.attempts_table_state.selected() {
            Some(i) => {
                if i >= attempts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.attempts_table_config
            .attempts_table_state
            .select(Some(i));
        self.attempts_table_config.attempt_under_cursor = i;
    }

    // Прокрутка вверх
    fn previous_attempt(&mut self) {
        let attempts = self.submitted_attempts();
        let i = match self.attempts_table_config.attempts_table_state.selected() {
            Some(i) => {
                if i == 0 {
                    attempts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.attempts_table_config
            .attempts_table_state
            .select(Some(i));
        self.attempts_table_config.attempt_under_cursor = i;
    }

    pub fn attempt_manager_handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.next_attempt();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.previous_attempt();
                }
                _ => {}
            }
        }
        Ok(())
    }
    fn next_test(&mut self) {
        if let Some(tests) = &self.tests {
            if tests.is_empty() {
                return;
            }

            let i = match self.tests_list_state.selected() {
                Some(i) => {
                    if i >= tests.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.tests_list_state.select(Some(i));
            self.tests_scrollbar_state = self.tests_scrollbar_state.position(i);
        }
    }

    fn previous_test(&mut self) {
        if let Some(tests) = &self.tests {
            if tests.is_empty() {
                return;
            }

            let i = match self.tests_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        tests.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.tests_list_state.select(Some(i));
            self.tests_scrollbar_state = self.tests_scrollbar_state.position(i);
        }
    }

    pub fn tests_handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.next_test();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.previous_test();
                }
                KeyCode::Home | KeyCode::Char('g') => {
                    self.tests_list_state.select(Some(0));
                    self.tests_scrollbar_state = self.tests_scrollbar_state.position(0);
                }
                KeyCode::End | KeyCode::Char('G') => {
                    if let Some(tests) = &self.tests {
                        let last = tests.len() - 1;
                        self.tests_list_state.select(Some(last));
                        self.tests_scrollbar_state = self.tests_scrollbar_state.position(last);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
