use crate::task::Task;
use std::error::Error;

pub(crate) trait TaskRepository {
    fn list(&self) -> Result<Vec<Task>, Box<dyn Error>>;
    fn add(&self, task_name: &str) -> Result<Task, Box<dyn Error>>;
    fn remove(&self, id: u32) -> Result<(), Box<dyn Error>>;
    fn done(&self, id: u32) -> Result<(), Box<dyn Error>>;
}

pub(crate) mod json;
pub(crate) use json::JsonTaskRepository;

pub(crate) mod sqlite;
pub(crate) use sqlite::SqliteTaskRepository;
