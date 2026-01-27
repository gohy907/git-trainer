use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub desc: String,
    pub work_name: String,
    pub dir: String,
    pub status: TaskStatus,
    pub grade: Option<usize>,
    pub extended_desc: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")] // "in_progress", "done", ...
pub enum TaskStatus {
    NotInProgress,
    InProgress,
    Done,
    Pending,
    Approved,
}

impl Task {
    pub fn image_name(&self) -> String {
        format!("git-trainer:{}", self.work_name)
    }
    pub fn container_name(&self) -> String {
        format!("git-trainer_{}", self.work_name)
    }
}
