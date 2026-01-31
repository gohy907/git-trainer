use crate::task::TaskStatus;
use chrono::Utc;
use rusqlite::{Connection, Result, params};
use std::fs;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct TaskModel {
    pub id: Option<i64>,
    pub name: String,
    pub work_name: String,
    pub description: String,
    pub extended_description: String,
    pub status: u8,
}

pub trait Task {
    fn id(&self) -> i64;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn extended_description(&self) -> String;
    fn status(&self) -> TaskStatus;
    fn container_name(&self) -> String;
    fn image_name(&self) -> String;
    fn work_name(&self) -> String;
}

impl Task for TaskModel {
    fn id(&self) -> i64 {
        self.id.expect("While working with db: ")
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn description(&self) -> String {
        self.description.clone()
    }
    fn extended_description(&self) -> String {
        self.extended_description.clone()
    }
    fn status(&self) -> TaskStatus {
        match self.status {
            0 => TaskStatus::NotInProgress,
            1 => TaskStatus::InProgress,
            _ => TaskStatus::Done,
        }
    }
    fn container_name(&self) -> String {
        let username = whoami::username().unwrap_or("UNKNOWN".to_string());
        format!("git-trainer_{}_{}", self.work_name, username)
    }
    fn image_name(&self) -> String {
        format!("git-trainer:{}", self.work_name)
    }
    fn work_name(&self) -> String {
        self.work_name.clone()
    }
}

impl TaskModel {
    pub fn container_name(&self) -> String {
        let user = whoami::username().expect("Can't find username");
        format!("git-trainer_{}_{}", self.work_name, user)
    }
}

#[derive(Debug, Clone)]
pub struct Attempt {
    pub id: Option<i64>,
    pub user_id: i64,
    pub task_id: i64,
    pub passed: bool,
    pub timestamp: String,
    pub tests: Vec<Test>,
}

#[derive(Debug, Clone)]
pub struct Test {
    pub id: Option<i64>,
    pub attempt_id: i64,
    pub description: String,
    pub passed: bool,
}

pub struct Repo {
    connection: Connection,
}

impl Repo {
    pub fn init_database() -> Self {
        let db_path = "db.sqlite";
        let schema_path = "schema.sql"; // ← ИСПРАВЛЕНО

        let conn = Connection::open(db_path).expect("Failed to connect to db.sqlite");

        // Читаем SQL-скрипт из schema.sql
        let schema_sql = fs::read_to_string(schema_path).expect("Failed to read schema.sql");

        // Выполняем SQL в БД
        conn.execute_batch(&schema_sql)
            .expect("Failed to execute schema.sql");

        Repo { connection: conn }
    }
    pub fn create_user(&self, username: &str) -> Result<i64> {
        let conn = &self.connection;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO users (username, created_at) VALUES (?1, ?2)",
            params![username, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    // ============ READ ============
    pub fn get_user_by_id(&self, user_id: i64) -> Result<User> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, username, created_at FROM users WHERE id = ?1",
            [user_id],
            |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    username: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
    }

    pub fn get_tasks_count(&self) -> Result<usize> {
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))?;

        Ok(count as usize)
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<User> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, username, created_at FROM users WHERE username = ?1",
            [username],
            |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    username: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
    }

    pub fn get_all_users(&self) -> Result<Vec<User>> {
        let conn = &self.connection;
        let mut stmt =
            conn.prepare("SELECT id, username, created_at FROM users ORDER BY created_at DESC")?;

        let users = stmt.query_map([], |row| {
            Ok(User {
                id: Some(row.get(0)?),
                username: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

        users.collect()
    }

    pub fn get_all_tasks<T: Task>(&self) -> Result<Vec<T>>
    where
        Vec<T>: FromIterator<TaskModel>,
    {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, name, work_name, description, extended_description, status FROM tasks",
        )?;
        let tasks = stmt.query_map([], |row| {
            Ok(TaskModel {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                work_name: row.get(2)?,
                description: row.get(3)?,
                extended_description: row.get(4)?,
                status: row.get(5)?,
            })
        })?;

        tasks.collect()
    }

    // ============ UPDATE ============
    pub fn update_user(&self, user_id: i64, new_username: &str) -> Result<()> {
        let conn = &self.connection;
        conn.execute(
            "UPDATE users SET username = ?1 WHERE id = ?2",
            params![new_username, user_id],
        )?;
        Ok(())
    }

    // ============ DELETE ============
    pub fn delete_user(&self, user_id: i64) -> Result<()> {
        let conn = &self.connection;
        conn.execute("DELETE FROM users WHERE id = ?1", [user_id])?;
        Ok(())
    }

    // ============ CREATE ============
    pub fn create_attempt(
        &mut self,
        user_id: i64,
        task_id: i64,
        test_results: Vec<(String, bool)>, // (description, passed)
    ) -> Result<i64> {
        let conn = &mut self.connection;
        let tx = conn.transaction()?;

        // Проверяем, все ли тесты пройдены
        let all_passed = test_results.iter().all(|(_, passed)| *passed);
        let now = Utc::now().to_rfc3339();

        // Создаём попытку
        tx.execute(
            "INSERT INTO attempts (user_id, task_id, passed, timestamp)
         VALUES (?1, ?2, ?3, ?4)",
            params![user_id, task_id, if all_passed { 1 } else { 0 }, now],
        )?;

        let attempt_id = tx.last_insert_rowid();

        // Добавляем тесты
        for (description, passed) in test_results {
            tx.execute(
                "INSERT INTO attempt_tests (attempt_id, description, passed)
             VALUES (?1, ?2, ?3)",
                params![attempt_id, description, if passed { 1 } else { 0 }],
            )?;
        }

        tx.commit()?;
        Ok(attempt_id)
    }

    // ============ READ ============
    pub fn get_attempt_by_id(&self, attempt_id: i64) -> Result<Attempt> {
        let conn = &self.connection;
        let attempt = conn.query_row(
            "SELECT id, user_id, task_id, passed, timestamp 
         FROM attempts WHERE id = ?1",
            [attempt_id],
            |row| {
                Ok(Attempt {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    task_id: row.get(2)?,
                    passed: row.get::<_, i32>(3)? != 0,
                    timestamp: row.get(4)?,
                    tests: vec![],
                })
            },
        )?;

        let tests = self.get_attempt_tests(attempt_id)?;

        Ok(Attempt { tests, ..attempt })
    }

    pub fn get_user_attempts(&self, user_id: i64) -> Result<Vec<Attempt>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, passed, timestamp 
         FROM attempts WHERE user_id = ?1 
         ORDER BY timestamp DESC",
        )?;

        let attempt_rows = stmt.query_map([user_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i32>(3)? != 0,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut attempts = Vec::new();
        for attempt_row in attempt_rows {
            let (id, user_id, task_id, passed, timestamp) = attempt_row?;
            let tests = self.get_attempt_tests(id)?;

            attempts.push(Attempt {
                id: Some(id),
                user_id,
                task_id,
                passed,
                timestamp,
                tests,
            });
        }

        Ok(attempts)
    }

    pub fn get_task_attempts(&self, task_id: i64) -> Result<Vec<Attempt>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, passed, timestamp 
         FROM attempts WHERE task_id = ?1 
         ORDER BY timestamp DESC",
        )?;

        let attempt_rows = stmt.query_map([task_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i32>(3)? != 0,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut attempts = Vec::new();
        for attempt_row in attempt_rows {
            let (id, user_id, task_id, passed, timestamp) = attempt_row?;
            let tests = self.get_attempt_tests(id)?;

            attempts.push(Attempt {
                id: Some(id),
                user_id,
                task_id,
                passed,
                timestamp,
                tests,
            });
        }

        Ok(attempts)
    }

    pub fn get_user_task_attempts(&self, user_id: i64, task_id: i64) -> Result<Vec<Attempt>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, task_id, passed, timestamp 
         FROM attempts WHERE user_id = ?1 AND task_id = ?2 
         ORDER BY timestamp DESC",
        )?;

        let attempt_rows = stmt.query_map(params![user_id, task_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i32>(3)? != 0,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut attempts = Vec::new();
        for attempt_row in attempt_rows {
            let (id, user_id, task_id, passed, timestamp) = attempt_row?;
            let tests = self.get_attempt_tests(id)?;

            attempts.push(Attempt {
                id: Some(id),
                user_id,
                task_id,
                passed,
                timestamp,
                tests,
            });
        }

        Ok(attempts)
    }

    // ============ UPDATE ============
    pub fn update_attempt_status(&self, attempt_id: i64, passed: bool) -> Result<()> {
        let conn = &self.connection;
        conn.execute(
            "UPDATE attempts SET passed = ?1 WHERE id = ?2",
            params![if passed { 1 } else { 0 }, attempt_id],
        )?;
        Ok(())
    }

    // ============ DELETE ============
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

    pub fn delete_task_attempts(&self, task_id: i64) -> Result<()> {
        let conn = &self.connection;
        conn.execute("DELETE FROM attempts WHERE task_id = ?1", [task_id])?;
        Ok(())
    }

    // ============ CREATE ============
    pub fn create_attempt_test(
        &self,

        attempt_id: i64,
        description: &str,
        passed: bool,
    ) -> Result<i64> {
        let conn = &self.connection;
        conn.execute(
            "INSERT INTO attempt_tests (attempt_id, description, passed)
         VALUES (?1, ?2, ?3)",
            params![attempt_id, description, if passed { 1 } else { 0 }],
        )?;

        Ok(conn.last_insert_rowid())
    }

    // ============ READ ============
    pub fn get_attempt_tests(&self, attempt_id: i64) -> Result<Vec<Test>> {
        let conn = &self.connection;
        let mut stmt = conn.prepare(
            "SELECT id, attempt_id, description, passed 
         FROM attempt_tests WHERE attempt_id = ?1 
         ORDER BY id",
        )?;

        let tests = stmt.query_map([attempt_id], |row| {
            Ok(Test {
                id: Some(row.get(0)?),
                attempt_id: row.get(1)?,
                description: row.get(2)?,
                passed: row.get::<_, i32>(3)? != 0,
            })
        })?;

        tests.collect()
    }

    pub fn get_test_by_id(&self, test_id: i64) -> Result<Test> {
        let conn = &self.connection;
        conn.query_row(
            "SELECT id, attempt_id, description, passed 
         FROM attempt_tests WHERE id = ?1",
            [test_id],
            |row| {
                Ok(Test {
                    id: Some(row.get(0)?),
                    attempt_id: row.get(1)?,
                    description: row.get(2)?,
                    passed: row.get::<_, i32>(3)? != 0,
                })
            },
        )
    }

    // ============ UPDATE ============
    pub fn update_attempt_test(&self, test_id: i64, description: &str, passed: bool) -> Result<()> {
        let conn = &self.connection;
        conn.execute(
            "UPDATE attempt_tests SET description = ?1, passed = ?2 WHERE id = ?3",
            params![description, if passed { 1 } else { 0 }, test_id],
        )?;
        Ok(())
    }

    // ============ DELETE ============
    pub fn delete_attempt_test(&self, test_id: i64) -> Result<()> {
        let conn = &self.connection;
        conn.execute("DELETE FROM attempt_tests WHERE id = ?1", [test_id])?;
        Ok(())
    }

    pub fn delete_all_attempt_tests(&self, attempt_id: i64) -> Result<()> {
        let conn = &self.connection;
        conn.execute(
            "DELETE FROM attempt_tests WHERE attempt_id = ?1",
            [attempt_id],
        )?;
        Ok(())
    }

    // Проверить, есть ли зачтённая попытка
    pub fn has_passed_attempt(&self, user_id: i64, task_id: i64) -> Result<bool> {
        let conn = &self.connection;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM attempts 
         WHERE user_id = ?1 AND task_id = ?2 AND passed = 1",
            params![user_id, task_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // Получить статистику по задаче
    pub fn get_task_stats(&self, user_id: i64, task_id: i64) -> Result<(usize, usize)> {
        let conn = &self.connection;
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM attempts 
         WHERE user_id = ?1 AND task_id = ?2",
            params![user_id, task_id],
            |row| row.get(0),
        )?;

        let passed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM attempts 
         WHERE user_id = ?1 AND task_id = ?2 AND passed = 1",
            params![user_id, task_id],
            |row| row.get(0),
        )?;

        Ok((total as usize, passed as usize))
    }

    // Получить последнюю попытку
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

    // Подсчёт тестов в попытке
    pub fn count_passed_tests(&self, attempt_id: i64) -> Result<(usize, usize)> {
        let conn = &self.connection;
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM attempt_tests WHERE attempt_id = ?1",
            [attempt_id],
            |row| row.get(0),
        )?;

        let passed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM attempt_tests 
         WHERE attempt_id = ?1 AND passed = 1",
            [attempt_id],
            |row| row.get(0),
        )?;

        Ok((total as usize, passed as usize))
    }
}
