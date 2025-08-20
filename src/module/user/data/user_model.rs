use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub status: Option<UserStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
pub enum UserStatus {
    Active,
    Inactive
}