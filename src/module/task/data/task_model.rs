use sqlx::{FromRow, Type};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}