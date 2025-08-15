use anyhow::Result;
use sqlx::PgPool;
use crate::data::task_model::Task;

pub async fn create_task(pool: &PgPool, new_task: Task) -> Result<Task> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (title)
        VALUES ($1)
        RETURNING id, title, status
        "#
    )
        .bind(new_task.title)
        .fetch_one(pool)
        .await?;

    Ok(task)
}

pub async fn get_tasks(pool: &PgPool) -> Result<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, status
        FROM tasks
        ORDER BY id
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(tasks)
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