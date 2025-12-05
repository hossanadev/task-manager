use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::module::user::dto::response::UserDTO;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
struct LoginResponse {
    token: String,
    user: UserDTO,
}