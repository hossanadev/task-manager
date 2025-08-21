use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::module::task::data::task_model::TaskStatus;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CreateTaskRequest {
    pub title: String
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UpdateTaskRequest {
    pub title: String
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus
}