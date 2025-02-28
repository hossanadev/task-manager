use diesel::prelude::*;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{Todo};
use crate::database::DbPool;
use crate::schema::todos::dsl::*;

pub fn create_todo(pool: &DbPool, new_todo: Todo) -> Result<Todo> {
    let mut conn = pool.get()?;
    let todo = Todo {
        id: Uuid::new_v4(),
        title: new_todo.title,
        completed: new_todo.completed,
    };

    diesel::insert_into(todos).values(&todo).execute(&mut conn)?;
    Ok(todo)
}

pub fn get_todos(pool: &DbPool) -> Result<Vec<Todo>> {
    let mut conn = pool.get()?;
    let results = todos.load::<Todo>(&mut conn)?;
    Ok(results)
}

pub fn delete_todo(pool: &DbPool, todo_id: Uuid) -> Result<usize> {
    let mut conn = pool.get()?;
    let deleted = diesel::delete(todos.filter(id.eq(todo_id))).execute(&mut conn)?;
    Ok(deleted)
}