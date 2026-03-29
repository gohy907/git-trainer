use crate::db::{
    Attempt, AttemptCreate, Repo, Task, TaskStatus, Test, TestCreate, TestResult, User,
};
use crate::docker;
use crate::io;
use crate::popup::Popup;
use crate::pty::ui::PtyExitStatus;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::{ListState, ScrollbarState, TableState};
use rusqlite::Error as SqlError;
use std::fs;

pub const VERSION: &str = "0.1.0";

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
            attempts_table_state,
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
            list_state,
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

impl App {
    pub fn new() -> App {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        let mut repo = Repo::init_database();
        let username = whoami::username()
            .expect("While getting username:")
            .to_string()
            .replace(" ", "-");
        if !repo.user_exists(&username).expect("While working with db:") {
            let _ = repo.create_user(&username);
        }
        let user = repo.get_user_by_username(username);
        App {
            context: Context {
                tasks: repo.get_tasks_user_local(user.as_ref().unwrap().id),
                user,
            },
            repo,
            table_state,
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
            // self.status = AppStatus::ShowingAttempts;
            self.update_context();
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
            match self.status {
                AppStatus::RestartingTask => {
                    if let Err(err) = docker::restart_task(self.task_under_cursor()).await {
                        self.active_popup = Some(Popup::Error(err.to_string()))
                    };
                    self.status = AppStatus::Idling;
                }
                AppStatus::RunningTask => {
                    let task: &mut Task = self.task_under_cursor_mut();

                    if let TaskStatus::NotInProgress = task.status {
                        task.status = TaskStatus::InProgress
                    }
                    match self.prepare_pty_bollard(terminal).await {
                        Err(err) => self.active_popup = Some(Popup::Error(err.to_string())),
                        Ok(PtyExitStatus::RestartTask) => {
                            if let Err(err) = docker::restart_task(self.task_under_cursor()).await {
                                self.active_popup = Some(Popup::Error(err.to_string()))
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

    pub fn task_under_cursor(&self) -> &Task {
        &self.context.tasks.as_ref().expect("while working with db:")[self.task_under_cursor]
    }

    pub fn task_under_cursor_mut(&mut self) -> &mut Task {
        &mut self.context.tasks.as_mut().expect("while working with db:")[self.task_under_cursor]
    }

    pub fn attempts_of_choosed_task(&self) -> &Vec<Attempt> {
        let task = self.task_under_cursor();
        task.attempts.as_ref().expect("While working wih db:")
    }

    pub fn attempt_under_cursor(&self) -> Option<&Attempt> {
        let attempts = &self.attempts_of_choosed_task();
        if attempts.is_empty() {
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

        #[cfg(debug_assertions)]
        let path = format!("tests/{}", task.work_name);

        #[cfg(not(debug_assertions))]
        let path = format!("/var/lib/git-trainer/tests/{}", task.work_name);

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
                    test_results.push(TestCreate {
                        description: res.output,
                        result: 0,
                    });
                } else {
                    test_results.push(TestCreate {
                        description: res.output,
                        result: 1,
                    });
                    failed = true;
                }
            } else {
                let res = format!("{}. Не выполнялся.", i);
                test_results.push(TestCreate {
                    description: res,
                    result: 2,
                });
            }
        }

        let _ = docker::exec_command(task, "sudo rm -rf /etc/git-trainer/tests/*").await;

        let user_id = self
            .context
            .user
            .as_ref()
            .expect("While working with db:")
            .id;

        let bash_history = docker::exec_command(task, "cat /home/student/.bash_history")
            .await
            .unwrap()
            .output;

        let attempt = AttemptCreate {
            tests: test_results,
            task_id: task.id,
            user_id: user_id,
            bash_history: bash_history.clone(),
        };

        self.repo
            .create_attempt(attempt)
            .expect("While working with db:");
    }

    pub fn update_context(&mut self) {
        let user_id = self.context.user.as_ref().unwrap().id;
        let tasks = self.context.tasks.as_mut().expect("While working with db:");
        _ = self.repo.load_new_tasks(user_id, tasks);
        for task in tasks.iter_mut() {
            let attempts = task.attempts.as_ref().expect("While working with db:");
            if attempts.is_empty() {
                continue;
            }

            let all_passed = attempts.iter().any(|attempt| {
                attempt
                    .tests
                    .as_ref()
                    .expect("While working with db:")
                    .iter()
                    .all(|test| test.result == TestResult::Passed)
            });

            task.status = if all_passed {
                TaskStatus::Approved
            } else {
                TaskStatus::Done
            };
        }

        for task in tasks.iter() {
            let _ = self
                .repo
                .update_task_status(task.id, user_id, task.status.clone());
        }

        self.context.tasks = self.repo.get_tasks_user_local(user_id);
    }
}
