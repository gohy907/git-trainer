use crate::app::App;
use crate::{AppStatus, app::AttemptManagerStatus};
use crossterm::event::{self, Event, KeyCode};
use std::io;

impl App {
    // Прокрутка вниз
    fn next_attempt(&mut self) {
        let attempts = self.attempts_of_choosed_task();
        let i = match self
            .attempt_manager_config
            .attempts_table_config
            .attempts_table_state
            .selected()
        {
            Some(i) => {
                if i >= attempts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.attempt_manager_config
            .attempts_table_config
            .attempts_table_state
            .select(Some(i));
        self.attempt_manager_config
            .attempts_table_config
            .attempt_under_cursor = i;
    }

    // Прокрутка вверх
    fn previous_attempt(&mut self) {
        let attempts = self.attempts_of_choosed_task();
        let i = match self
            .attempt_manager_config
            .attempts_table_config
            .attempts_table_state
            .selected()
        {
            Some(i) => {
                if i == 0 {
                    attempts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.attempt_manager_config
            .attempts_table_config
            .attempts_table_state
            .select(Some(i));
        self.attempt_manager_config
            .attempts_table_config
            .attempt_under_cursor = i;
    }

    pub fn attempt_manager_handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => self.status = AppStatus::Idling,
                KeyCode::Down => match self.attempt_manager_config.status {
                    AttemptManagerStatus::SelectingAttempts => self.next_attempt(),
                    AttemptManagerStatus::SelectingTests => self.next_test(),
                },
                KeyCode::Up => match self.attempt_manager_config.status {
                    AttemptManagerStatus::SelectingAttempts => self.previous_attempt(),
                    AttemptManagerStatus::SelectingTests => self.previous_test(),
                },
                KeyCode::Right => {
                    self.attempt_manager_config.status = AttemptManagerStatus::SelectingTests
                }
                KeyCode::Left => {
                    self.attempt_manager_config.status = AttemptManagerStatus::SelectingAttempts;
                    self.attempt_manager_config
                        .tests_table_config
                        .list_state
                        .select(Some(0));
                    self.attempt_manager_config
                        .tests_table_config
                        .test_under_cursor = 0;
                }

                _ => {}
            }
        }
        Ok(())
    }
    fn next_test(&mut self) {
        let tests = self.tests_of_choosed_attempt();
        if tests.is_empty() {
            return;
        }

        let i = match self
            .attempt_manager_config
            .tests_table_config
            .list_state
            .selected()
        {
            Some(i) => {
                if i >= tests.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.attempt_manager_config
            .tests_table_config
            .list_state
            .select(Some(i));
        self.attempt_manager_config
            .tests_table_config
            .scrollbar_state = self
            .attempt_manager_config
            .tests_table_config
            .scrollbar_state
            .position(i);

        self.attempt_manager_config
            .tests_table_config
            .test_under_cursor = i;
    }

    fn previous_test(&mut self) {
        let tests = self.tests_of_choosed_attempt();
        if tests.is_empty() {
            return;
        }

        let table_config = &mut self.attempt_manager_config.tests_table_config;

        let i = match table_config.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    tests.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        table_config.list_state.select(Some(i));
        table_config.scrollbar_state = table_config.scrollbar_state.position(i);
        table_config.test_under_cursor = i;
    }
}
