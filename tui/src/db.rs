use chrono::{DateTime, Local, ParseError, Utc};
use core::fmt;
use rusqlite::{Connection, Result, params};
use std::fs;
use std::path::Path;
use thiserror::Error;

pub struct TaskModel {
    pub id: i64,
    pub name: String,
    pub work_name: String,
    pub description: String,
    pub extended_description: String,
}

struct TaskEntity {
    id: i64,
    name: String,
    work_name: String,
    description: String,
    extended_description: String,
    status: i64,
}

impl From<&TaskModel> for TaskEntity {
    fn from(value: &TaskModel) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            work_name: value.work_name.clone(),
            description: value.description.clone(),
            extended_description: value.extended_description.clone(),
            status: 0,
        }
    }
}

pub struct TaskPayload {
    name: String,
    work_name: String,
    description: String,
    extended_description: String,
    status: i64,
}

impl From<&Task> for TaskPayload {
    fn from(task: &Task) -> Self {
        TaskPayload {
            name: task.name.clone(),
            work_name: task.work_name.clone(),
            description: task.description.clone(),
            extended_description: task.extended_description.clone(),
            status: match task.status {
                TaskStatus::NotInProgress => 0,
                TaskStatus::InProgress => 1,
                TaskStatus::Done => 2,
                TaskStatus::Approved => 3,
                TaskStatus::Pending => 4,
            },
        }
    }
}

#[derive(Clone)]
pub enum TaskStatus {
    NotInProgress,
    InProgress,
    Done,
    Pending,
    Approved,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::NotInProgress => write!(f, "НЕ НАЧАТО"),
            TaskStatus::InProgress => write!(f, "НАЧАТО"),
            TaskStatus::Done => write!(f, "ПРОВЕРЕНО"),
            TaskStatus::Pending => write!(f, "В ПРОВЕРКЕ"),
            TaskStatus::Approved => write!(f, "СДАНО"),
        }
    }
}

pub struct Task {
    pub id: i64,
    pub name: String,
    pub work_name: String,
    pub image_name: String,
    pub container_name: String,
    pub description: String,
    pub extended_description: String,
    pub status: TaskStatus,
    pub attempts: Result<Vec<Attempt>>,
}

impl From<TaskEntity> for Task {
    fn from(task_entity: TaskEntity) -> Self {
        let username = whoami::username()
            .expect("While getting username:")
            .to_string()
            .replace(" ", "-");
        Task {
            id: task_entity.id,
            name: task_entity.name,
            work_name: task_entity.work_name.clone(),
            container_name: format!("git-trainer_{}_{}", task_entity.work_name, username),
            image_name: format!("git-trainer:{}", task_entity.work_name),
            description: task_entity.description,
            extended_description: task_entity.extended_description,
            status: match task_entity.status {
                0 => TaskStatus::NotInProgress,
                1 => TaskStatus::InProgress,
                2 => TaskStatus::Done,
                3 => TaskStatus::Approved,
                _ => TaskStatus::Pending,
            },
            attempts: Repo::get_task_attempts(&Repo::init_database(), task_entity.id),
        }
    }
}

struct AttemptEntity {
    id: i64,
    user_id: i64,
    task_id: i64,
    timestamp: String,
}

pub fn format_timestamp(timestamp_str: &str) -> Result<String, ParseError> {
    let dt = DateTime::parse_from_rfc3339(timestamp_str)?;

    let local_dt: DateTime<Local> = dt.with_timezone(&Local);

    Ok(local_dt.format("%d.%m.%Y %H:%M:%S").to_string())
}

pub struct Attempt {
    pub id: i64,
    pub user_id: i64,
    pub task_id: i64,
    pub timestamp: Result<String, ParseError>,
    pub tests: Result<Vec<Test>>,
}

pub struct NewAttemptEntity {
    pub user_id: i64,
    pub task_id: i64,
    pub tests: Vec<NewTestEntity>,
}

impl From<AttemptEntity> for Attempt {
    fn from(attempt_entity: AttemptEntity) -> Self {
        Attempt {
            id: attempt_entity.id,
            user_id: attempt_entity.user_id,
            task_id: attempt_entity.task_id,
            timestamp: format_timestamp(&attempt_entity.timestamp),
            tests: Repo::get_attempt_tests(&Repo::init_database(), attempt_entity.id),
        }
    }
}

impl From<Attempt> for NewAttemptEntity {
    fn from(attempt: Attempt) -> Self {
        NewAttemptEntity {
            user_id: attempt.user_id,
            task_id: attempt.task_id,
            tests: attempt
                .tests
                .expect("While working with db:")
                .into_iter()
                .map(NewTestEntity::from)
                .collect(),
        }
    }
}

pub struct NewTestEntity {
    description: String,
    result: i64,
}

struct TestEntity {
    id: i64,
    attempt_id: i64,
    description: String,
    result: i64,
}

#[derive(Clone)]
pub struct Test {
    pub id: i64,
    pub attempt_id: i64,
    pub description: String,
    pub result: TestResult,
}

impl From<TestEntity> for Test {
    fn from(test_entity: TestEntity) -> Self {
        Test {
            id: test_entity.id,
            attempt_id: test_entity.attempt_id,
            description: test_entity.description,
            result: match test_entity.result {
                0 => TestResult::Passed,
                2 => TestResult::NotExecuted,
                _ => TestResult::Failed,
            },
        }
    }
}

impl From<Test> for NewTestEntity {
    fn from(test: Test) -> Self {
        NewTestEntity {
            description: test.description,
            result: match test.result {
                TestResult::Passed => 0,
                TestResult::Failed => 1,
                TestResult::NotExecuted => 2,
            },
        }
    }
}

struct UserEntity {
    id: i64,
    username: String,
    created_at: String,
}

pub struct User {
    pub id: i64,
    pub username: String,
}

impl From<UserEntity> for User {
    fn from(user_entity: UserEntity) -> Self {
        User {
            id: user_entity.id,
            username: user_entity.username,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum TestResult {
    Passed,
    Failed,
    NotExecuted,
}

pub struct Repo {
    connection: Connection,
}

pub struct UserTaskStatus {
    pub id: i64,
    pub user_id: i64,
    pub task_id: i64,
    pub status: i64,
}

impl From<UserTaskStatus> for TaskStatus {
    fn from(value: UserTaskStatus) -> Self {
        match value.status {
            0 => TaskStatus::NotInProgress,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::Done,
            3 => TaskStatus::Approved,
            _ => TaskStatus::Pending,
        }
    }
}

#[derive(Debug, Error)]
pub enum RunMigrationsError {
    #[error("No migrations directory found")]
    NoDirectory(),

    #[error("While working with SQL: {0}")]
    SQLiteError(#[from] rusqlite::Error),
}

impl Repo {
    pub fn init_database() -> Self {
        #[cfg(debug_assertions)]
        let db_path = "db.sqlite";

        #[cfg(not(debug_assertions))]
        let db_path = "/var/lib/git-trainer/db.sqlite";

        #[cfg(debug_assertions)]
        let schema_path = "schema.sql";

        #[cfg(not(debug_assertions))]
        let schema_path = "/var/lib/git-trainer/schema.sql";

        let conn = Connection::open(db_path).expect("Failed to connect to db.sqlite");

        let schema_sql = fs::read_to_string(schema_path).expect("Failed to read schema.sql");

        conn.execute_batch(&schema_sql)
            .expect("Failed to execute schema.sql");
        let mut repo = Repo { connection: conn };
        repo.run_migrations().unwrap();
        repo
    }

    pub fn get_task_attempts_user_local(&self, user_id: i64, task_id: i64) -> Result<Vec<Attempt>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, timestamp 
         FROM attempts WHERE user_id = ?1 AND task_id = ?2 
         ORDER BY timestamp DESC",
        )?;

        let attempt_rows = stmt.query_map([user_id, task_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;

        let mut attempts = Vec::new();
        for attempt_row in attempt_rows {
            let (id, user_id, task_id, timestamp) = attempt_row?;

            attempts.push(
                AttemptEntity {
                    id: id,
                    user_id,
                    task_id,
                    timestamp,
                }
                .into(),
            );
        }

        Ok(attempts)
    }

    pub fn get_tasks_user_local(&self, user_id: i64) -> Result<Vec<Task>> {
        let conn = &self.connection;

        // let mut stmt = conn.prepare(
        //     "SELECT id, attempt_id, description, result
        //  FROM attempt_tests WHERE attempt_id = ?1
        //  ORDER BY id",
        // )?;
        //
        // let tests = stmt.query_map([attempt_id], |row| {
        //     Ok(TestEntity {
        //         id: row.get(0)?,
        //         attempt_id: row.get(1)?,
        //         description: row.get(2)?,
        //         result: row.get(3)?,
        //     }
        //     .into())
        // })?;

        // println!("{}", user_id);

        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, status 
             FROM user_task_statuses 
             WHERE user_id = ?1
             ORDER BY task_id",
        )?;

        let statuses = stmt.query_map([user_id], |row| {
            let user_task_status = UserTaskStatus {
                id: row.get(0)?,
                user_id: row.get(1)?,
                task_id: row.get(2)?,
                status: row.get(3)?,
            };

            let task_model = self.get_task_by_id(user_task_status.task_id)?;

            let attempts = self.get_task_attempts_user_local(user_id, user_task_status.task_id)?;

            let username = self.get_username_by_id(user_id)?;

            Ok(Task {
                id: task_model.id,
                name: task_model.name,
                attempts: Ok(attempts),
                container_name: format!("git-trainer_{}_{}", task_model.work_name, username),
                work_name: task_model.work_name.clone(),
                image_name: format!("git-trainer:{}", task_model.work_name),
                description: task_model.description,
                extended_description: task_model.extended_description,
                status: match user_task_status.status {
                    0 => TaskStatus::NotInProgress,
                    1 => TaskStatus::InProgress,
                    2 => TaskStatus::Done,
                    3 => TaskStatus::Approved,
                    _ => TaskStatus::Pending,
                },
            })
        })?;

        statuses.collect()
    }

    pub fn get_username_by_id(&self, user_id: i64) -> Result<String> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT username FROM users WHERE id = ?1",
            [user_id],
            |row| Ok(row.get(0)?),
        )
    }

    pub fn get_task_by_id(&self, task_id: i64) -> Result<TaskModel> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, name, work_name, description, extended_description FROM tasks WHERE id = ?1",
            [task_id],
            |row| {
                Ok(TaskModel {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    work_name: row.get(2)?,
                    description: row.get(3)?,
                    extended_description: row.get(4)?
                })
            }
        )
    }

    pub fn get_all_tasks(&self) -> Result<Vec<TaskModel>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare("SELECT * FROM tasks")?;
        let task_rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;
        let mut task_models = Vec::new();
        for task_row in task_rows {
            let (id, name, work_name, description, extended_description) = task_row?;
            task_models.push(TaskModel {
                id: id,
                name: name,
                work_name: work_name,
                description: description,
                extended_description: extended_description,
            });
        }

        Ok(task_models)
    }

    // let mut stmt = conn.prepare(
    //     "SELECT id, user_id, task_id, timestamp
    //  FROM attempts WHERE user_id = ?1
    //  ORDER BY timestamp DESC",
    // )?;
    // let attempt_rows = stmt.query_map([user_id], |row| {
    //     Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    // })?;
    //
    // let mut attempts = Vec::new();
    // for attempt_row in attempt_rows {
    //     let (id, user_id, task_id, timestamp) = attempt_row?;
    //
    //     attempts.push(
    //         AttemptEntity {
    //             id: id,
    //             user_id,
    //             task_id,
    //             timestamp,
    //         }
    //         .into(),
    //     );
    // }

    pub fn load_new_tasks(&self, user_id: i64, loaded_tasks: &Vec<Task>) -> Result<()> {
        let conn = &self.connection;
        let task_models = self.get_all_tasks()?;

        for task_model in task_models {
            let mut found = false;
            for task in loaded_tasks {
                if task.id == task_model.id {
                    found = true;
                }
            }
            if !found {
                conn.execute(
                    "INSERT INTO user_task_statuses (user_id, task_id, status)
             VALUES (?1, ?2, ?3)",
                    params![user_id, task_model.id, 0],
                )?;
            }
        }
        Ok(())
    }

    pub fn create_user(&mut self, username: &str) -> Result<i64> {
        let task_models = self.get_all_tasks()?;

        let conn = &mut self.connection;
        let tx = conn.transaction()?;
        let now = Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO users (username, created_at) VALUES (?1, ?2)",
            params![username, now],
        )
        .unwrap();

        let user_id = tx.last_insert_rowid();

        for task_model in task_models {
            tx.execute(
                "INSERT INTO user_task_statuses (user_id, task_id, status)
             VALUES (?1, ?2, ?3)",
                params![user_id, task_model.id, 0],
            )?;
        }

        tx.commit()?;
        Ok(user_id)

        // let conn = &mut self.connection;
        // let tx = conn.transaction()?;
        //
        // let now = Utc::now().to_rfc3339();
        //
        // tx.execute(
        //     "INSERT INTO attempts (user_id, task_id, timestamp)
        //  VALUES (?1, ?2, ?3)",
        //     params![user_id, task_id, now],
        // )?;
        //
        // let attempt_id = tx.last_insert_rowid();
        //
        // for test in attempt.tests {
        //     tx.execute(
        //         "INSERT INTO attempt_tests (attempt_id, description, result)
        //      VALUES (?1, ?2, ?3)",
        //         params![attempt_id, test.description, test.result],
        //     )?;
        // }
        //
        // tx.commit()?;
        // Ok(attempt_id)
    }

    pub fn user_exists(&self, username: &str) -> Result<bool> {
        let count: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM users WHERE username = ?1",
            [username],
            |row| row.get(0),
        )?;

        Ok(if count > 0 { true } else { false })
    }

    pub fn get_tasks_count(&self) -> Result<usize> {
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))?;

        Ok(count as usize)
    }

    pub fn get_user_by_username(&self, username: String) -> Result<User> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, username, created_at FROM users WHERE username = ?1",
            [username],
            |row| {
                Ok(UserEntity {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    created_at: row.get(2)?,
                }
                .into())
            },
        )
    }

    pub fn create_attempt(
        &mut self,
        user_id: i64,
        task_id: i64,
        attempt: NewAttemptEntity,
    ) -> Result<i64> {
        let conn = &mut self.connection;
        let tx = conn.transaction()?;

        let now = Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO attempts (user_id, task_id, timestamp)
         VALUES (?1, ?2, ?3)",
            params![user_id, task_id, now],
        )?;

        let attempt_id = tx.last_insert_rowid();

        for test in attempt.tests {
            tx.execute(
                "INSERT INTO attempt_tests (attempt_id, description, result)
             VALUES (?1, ?2, ?3)",
                params![attempt_id, test.description, test.result],
            )?;
        }

        tx.commit()?;
        Ok(attempt_id)
    }

    pub fn get_attempt_by_id(&self, attempt_id: i64) -> Result<Attempt> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, user_id, task_id, timestamp 
        FROM attempts WHERE id = ?1",
            [attempt_id],
            |row| {
                Ok(AttemptEntity {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    task_id: row.get(2)?,
                    timestamp: row.get(3)?,
                }
                .into())
            },
        )
    }

    pub fn get_user_attempts(&self, user_id: i64) -> Result<Vec<Attempt>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, timestamp 
         FROM attempts WHERE user_id = ?1 
         ORDER BY timestamp DESC",
        )?;

        let attempt_rows = stmt.query_map([user_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;

        let mut attempts = Vec::new();
        for attempt_row in attempt_rows {
            let (id, user_id, task_id, timestamp) = attempt_row?;

            attempts.push(
                AttemptEntity {
                    id: id,
                    user_id,
                    task_id,
                    timestamp,
                }
                .into(),
            );
        }

        Ok(attempts)
    }

    pub fn get_task_attempts(&self, task_id: i64) -> Result<Vec<Attempt>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, timestamp
         FROM attempts WHERE task_id = ?1
         ORDER BY timestamp DESC",
        )?;

        let attempt_rows = stmt.query_map([task_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;

        let mut attempts = Vec::new();
        for attempt_row in attempt_rows {
            let (id, user_id, task_id, timestamp) = attempt_row?;

            attempts.push(
                AttemptEntity {
                    id: id,
                    user_id: user_id,
                    task_id: task_id,
                    timestamp: timestamp,
                }
                .into(),
            );
        }

        Ok(attempts)
    }

    pub fn delete_attempt(&self, attempt_id: i64) -> Result<()> {
        let conn = &self.connection;
        conn.execute("DELETE FROM attempts WHERE id = ?1", [attempt_id])?;
        Ok(())
    }

    pub fn delete_user_attempts(&self, user_id: i64) -> Result<()> {
        let conn = &self.connection;
        conn.execute("DELETE FROM attempts WHERE user_id = ?1", [user_id])?;
        Ok(())
    }

    pub fn get_attempt_tests(&self, attempt_id: i64) -> Result<Vec<Test>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, attempt_id, description, result 
         FROM attempt_tests WHERE attempt_id = ?1 
         ORDER BY id",
        )?;

        let tests = stmt.query_map([attempt_id], |row| {
            Ok(TestEntity {
                id: row.get(0)?,
                attempt_id: row.get(1)?,
                description: row.get(2)?,
                result: row.get(3)?,
            }
            .into())
        })?;

        tests.collect()
    }

    pub fn get_test_by_id(&self, test_id: i64) -> Result<Test> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, attempt_id, description, result
         FROM attempt_tests WHERE id = ?1",
            [test_id],
            |row| {
                Ok(TestEntity {
                    id: row.get(0)?,
                    attempt_id: row.get(1)?,
                    description: row.get(2)?,
                    result: row.get(3)?,
                }
                .into())
            },
        )
    }

    pub fn get_last_attempt(&self, user_id: i64, task_id: i64) -> Result<Attempt> {
        let conn = &self.connection;
        let attempt_id: i64 = conn.query_row(
            "SELECT id FROM attempts 
         WHERE user_id = ?1 AND task_id = ?2 
         ORDER BY timestamp DESC LIMIT 1",
            params![user_id, task_id],
            |row| row.get(0),
        )?;

        self.get_attempt_by_id(attempt_id)
    }

    pub fn get_task_id_by_name(&self, name: String) -> Result<i64> {
        let conn = &self.connection;
        conn.query_row("SELECT id FROM tasks WHERE name = ?1", [name], |row| {
            row.get(0)
        })
    }

    pub fn update_task_status(&self, task_id: i64, user_id: i64, status: TaskStatus) -> Result<()> {
        let conn = &self.connection;
        let status = match status {
            TaskStatus::NotInProgress => 0,
            TaskStatus::InProgress => 1,
            TaskStatus::Done => 2,
            TaskStatus::Approved => 3,
            _ => 4,
        };

        conn.execute(
            "UPDATE user_task_statuses SET status = ?1 WHERE task_id = ?2 AND user_id = ?3",
            params![status, task_id, user_id],
        )?;
        Ok(())
    }

    pub fn run_migrations(&mut self) -> Result<(), RunMigrationsError> {
        #[cfg(debug_assertions)]
        let migrations_dir = "migrations";

        #[cfg(not(debug_assertions))]
        let migrations_dir = "/var/lib/git-trainer/migrations";

        let migrations_dir = Path::new(migrations_dir);
        if !migrations_dir.exists() {
            return Err(RunMigrationsError::NoDirectory());
        }

        self.connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                migration_name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            )",
        )?;

        let read_dir = match std::fs::read_dir(&migrations_dir) {
            Ok(dir) => dir,
            Err(_) => return Ok(()),
        };

        for entry_result in read_dir {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            if !entry.path().is_dir() {
                continue;
            }

            let mig_name = entry.file_name().to_string_lossy().to_string();

            if self.is_migration_applied(&mig_name)? {
                continue;
            }

            let tx = self.connection.transaction()?;
            let up_path = entry.path().join("up.sql");

            if !up_path.exists() {
                let _ = tx.rollback();
                continue;
            }

            let up_sql = match std::fs::read_to_string(&up_path) {
                Ok(sql) => sql,
                Err(_) => {
                    let _ = tx.rollback();
                    continue;
                }
            };

            tx.execute_batch(&up_sql)?;

            let now = Utc::now().to_rfc3339();
            tx.execute(
                "INSERT INTO schema_migrations (migration_name, applied_at) VALUES (?1, ?2)",
                params![&mig_name, &now],
            )?;

            tx.commit()?;
        }
        Ok(())
    }

    fn is_migration_applied(&self, name: &str) -> Result<bool> {
        let mut stmt = self
            .connection
            .prepare("SELECT 1 FROM schema_migrations WHERE migration_name = ?1")?;
        Ok(stmt.exists(params![name])?)
    }
}
