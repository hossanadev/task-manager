use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use crate::data::task_model::Task;

pub async fn create_task(pool: &PgPool, new_task: Task) -> Result<Task> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (title)
        VALUES ($1)
        RETURNING id, title, status, created_at, updated_at
        "#
    )
        .bind(new_task.title)
        .fetch_one(pool)
        .await?;

    Ok(task)
}

pub async fn exists_by_task_title(pool: &PgPool, task: &Task) -> Result<bool> {
    let exists: (bool, ) = sqlx::query_as(
        r#"
        SELECT EXISTS
        (SELECT 1
        FROM tasks
        WHERE title = $1)
        "#
    )
        .bind(&task.title)
        .fetch_one(pool)
        .await?;

    Ok(exists.0)
}

pub async fn get_task_by_id(pool: &PgPool, task_id: String) -> Result<Option<Task>> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, status, created_at, updated_at
        FROM tasks
        WHERE id = $1
        "#
    )
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

    Ok(task)
}

pub async fn get_tasks(pool: &PgPool) -> Result<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, status, created_at, updated_at
        FROM tasks
        ORDER BY created_at DESC
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(tasks)
}

pub async fn update_task_by_id(pool: &PgPool, task: Task, task_id: String) -> Result<Option<Task>> {
    let updated_task = sqlx::query_as::<_, Task>(
        r#"
          UPDATE tasks SET title = $1, updated_at = $2
          WHERE id = $3
          RETURNING id, title, status, created_at, updated_at
        "#
    )
        .bind(task.title)
        .bind(Utc::now())
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

    Ok(updated_task)
}

pub async fn update_status_by_task_id(pool: &PgPool, status: String, task_id: String) -> Result<Option<Task>> {
    let updated_task = sqlx::query_as::<_, Task>(
        r#"
          UPDATE tasks SET status = $1, updated_at = $2
          WHERE id = $3
          RETURNING id, title, status, created_at, updated_at
        "#
    )
        .bind(status)
        .bind(Utc::now())
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

    Ok(updated_task)
}

pub async fn delete_task(pool: &PgPool, task_id: String) -> Result<u64> {
    let deleted = sqlx::query(
        r#"
        DELETE FROM tasks
        WHERE id = $1
        "#
    )
        .bind(task_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(deleted)
}