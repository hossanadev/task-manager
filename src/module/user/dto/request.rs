use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::module::user::data::user_model::UserStatus;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UpdateUserRequest {
    pub username: String
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UpdateUserStatusRequest {
    pub status: UserStatus
}