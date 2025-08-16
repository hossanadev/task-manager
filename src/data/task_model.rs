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

impl Task {
    pub fn new(title: String, status: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title,
            status: Option::from(TaskStatus::NotStarted),
            created_at: Option::from(now),
            updated_at: Option::from(now),
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Option::from(Utc::now());
    }
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