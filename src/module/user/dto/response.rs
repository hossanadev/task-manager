use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserDTO {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String
}