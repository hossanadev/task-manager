use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;

use crate::schema::todos;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = todos)]
pub struct Todo {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}