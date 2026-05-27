use crate::repository::TaskRepository;
use crate::task::Task;
use directories::ProjectDirs;
use rusqlite::{Connection, params};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const APP_NAME: &str = "todocli";

pub(crate) struct SqliteTaskRepository {
    database_path: PathBuf,
}

impl SqliteTaskRepository {
    pub(crate) fn new() -> Self {
        Self {
            database_path: default_database_path(),
        }
    }

    #[cfg(test)]
    fn with_database_path(database_path: PathBuf) -> Self {
        Self { database_path }
    }

    fn connect(&self) -> Result<Connection, Box<dyn Error>> {
        if let Some(parent) = self.database_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let connection = Connection::open(&self.database_path)?;
        create_tasks_table(&connection)?;
        Ok(connection)
    }
}

impl TaskRepository for SqliteTaskRepository {
    fn list(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let connection = self.connect()?;
        let mut statement = connection.prepare(
            "SELECT id, task_name, is_done
             FROM tasks
             ORDER BY id",
        )?;

        let task_rows = statement.query_map([], task_from_row)?;
        let mut tasks = Vec::new();

        for task_row in task_rows {
            tasks.push(task_row?);
        }

        Ok(tasks)
    }

    fn add(&self, task_name: &str) -> Result<Task, Box<dyn Error>> {
        let connection = self.connect()?;

        connection.execute(
            "INSERT INTO tasks (task_name, is_done)
             VALUES (?1, ?2)",
            params![task_name, false],
        )?;

        let id = row_id_to_task_id(connection.last_insert_rowid())?;

        Ok(Task {
            id,
            task_name: task_name.to_string(),
            is_done: false,
        })
    }

    fn remove(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let connection = self.connect()?;
        let changed_rows = connection.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;

        match changed_rows {
            0 => Err("没找到 id".into()),
            _ => Ok(()),
        }
    }

    fn done(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let connection = self.connect()?;
        let changed_rows = connection.execute(
            "UPDATE tasks
             SET is_done = ?1
             WHERE id = ?2",
            params![true, id],
        )?;

        match changed_rows {
            0 => Err("没找到 id".into()),
            _ => Ok(()),
        }
    }
}

fn default_database_path() -> PathBuf {
    match ProjectDirs::from("com", "didaman", APP_NAME) {
        Some(project_dirs) => project_dirs.data_dir().join("tasks.sqlite3"),
        None => Path::new("tasks.sqlite3").to_path_buf(),
    }
}

fn create_tasks_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_name TEXT NOT NULL,
            is_done INTEGER NOT NULL DEFAULT 0 CHECK (is_done IN (0, 1))
        )",
        [],
    )?;

    Ok(())
}

fn task_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    let row_id: i64 = row.get("id")?;
    let id = row_id_to_task_id(row_id).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Integer,
            Box::new(SimpleError(error.to_string())),
        )
    })?;

    Ok(Task {
        id,
        task_name: row.get("task_name")?,
        is_done: row.get("is_done")?,
    })
}

fn row_id_to_task_id(row_id: i64) -> Result<u32, Box<dyn Error>> {
    u32::try_from(row_id).map_err(|_| format!("数据库中的 task id 超出范围: {row_id}").into())
}

#[derive(Debug)]
struct SimpleError(String);

impl std::fmt::Display for SimpleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Error for SimpleError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn stores_tasks_in_sqlite() -> Result<(), Box<dyn Error>> {
        let database_path = test_database_path();
        let repository = SqliteTaskRepository::with_database_path(database_path.clone());

        let first_task = repository.add("learn sqlite")?;
        let second_task = repository.add("write a repository test")?;

        repository.done(first_task.id)?;
        repository.remove(second_task.id)?;

        let tasks = repository.list()?;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, first_task.id);
        assert_eq!(tasks[0].task_name, "learn sqlite");
        assert!(tasks[0].is_done);

        fs::remove_file(database_path)?;
        Ok(())
    }

    fn test_database_path() -> PathBuf {
        let unique_number = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "todocli-test-{}-{unique_number}.sqlite3",
            std::process::id()
        ))
    }
}
