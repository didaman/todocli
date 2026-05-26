use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Task {
    pub(crate) id: u32,
    pub(crate) task_name: String,
    pub(crate) is_done: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}", self.id, self.task_name)
    }
}
