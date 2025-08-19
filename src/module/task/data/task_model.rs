use sqlx::{FromRow, Type};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub status: Option<TaskStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct StatusParam {
    pub status: String,
}