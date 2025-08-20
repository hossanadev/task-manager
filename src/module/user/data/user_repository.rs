use sqlx::PgPool;
use crate::module::user::data::user_model::{User};
use crate::module::user::dto::user_dto::UserDTO;

pub async fn create_user(pool: &PgPool, user: User) -> anyhow::Result<UserDTO> {
    let user = sqlx::query_as::<_, UserDTO>(
        r#"
        INSERT INTO users_tm (email, username, password)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, status
        "#
    )
        .bind(user.email)
        .bind(user.username)
        .bind(user.password)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn get_users(pool: &PgPool) -> anyhow::Result<Vec<UserDTO>> {
    let users = sqlx::query_as::<_, UserDTO>(
        r#"
        SELECT id, email, username, status
        FROM users_tm
        ORDER BY created_at DESC
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(users)
}