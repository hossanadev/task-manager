use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub completed: bool,
}