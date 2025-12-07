use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::module::user::dto::response::UserDTO;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDTO,
}