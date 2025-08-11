use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}