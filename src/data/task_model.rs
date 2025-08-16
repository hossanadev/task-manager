use sqlx::{FromRow, Type};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub status: Option<TaskStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Deserialize)]
pub struct StatusParam {
    pub status: String,
}