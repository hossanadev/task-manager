use anyhow::Result;
use sqlx::PgPool;
use crate::data::models::Todo;

pub async fn create_todo(pool: &PgPool, new_todo: Todo) -> Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO tasks (id, title, completed)
        VALUES ($1, $2, $3)
        RETURNING id, title, completed
        "#
    )
        .bind(new_todo.id)
        .bind(new_todo.title)
        .bind(new_todo.completed)
        .fetch_one(pool)
        .await?;

    Ok(todo)
}

pub async fn get_todos(pool: &PgPool) -> Result<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed
        FROM tasks
        ORDER BY id
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(todos)
}

pub async fn delete_todo(pool: &PgPool, todo_id: String) -> Result<u64> {
    let deleted = sqlx::query(
        r#"
        DELETE FROM tasks
        WHERE id = $1
        "#
    )
        .bind(todo_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(deleted)
}