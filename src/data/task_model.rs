use sqlx::{FromRow, Type};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}