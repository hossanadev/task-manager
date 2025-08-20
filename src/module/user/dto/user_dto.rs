use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, sqlx::Type, sqlx::FromRow)]
pub struct UserDTO {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String
}