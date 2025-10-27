use serde::{Deserialize, Serialize};
use sqlx::{Type};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
pub enum UserStatus {
    Active,
    Inactive
}