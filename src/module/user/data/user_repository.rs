use chrono::Utc;
use sqlx::PgPool;
use anyhow::Result;
use crate::module::user::data::user_model::User;
use crate::module::user::dto::request::{CreateUserRequest, UpdateUserRequest, UpdateUserStatusRequest};
use crate::module::user::dto::response::UserDTO;

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(pool: &PgPool, user: CreateUserRequest) -> Result<UserDTO> {
        let user = sqlx::query_as::<_, UserDTO>(
            r#"
        INSERT INTO users (email, username, password)
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

    pub async fn get_users(pool: &PgPool) -> Result<Vec<UserDTO>> {
        let users = sqlx::query_as::<_, UserDTO>(
            r#"
        SELECT id, email, username, status
        FROM users
        ORDER BY created_at DESC
        "#
        )
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn get_user(pool: &PgPool, user_id: String) -> Result<Option<UserDTO>> {
        let user = sqlx::query_as::<_, UserDTO>(
            r#"
        SELECT id, email, username, status
        FROM users
        WHERE id = $1
        "#
        )
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_raw_user(pool: &PgPool, email: String) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
        SELECT id, email, username, status, password
        FROM users
        WHERE email = $1
        "#
        )
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn update_user(pool: &PgPool, user_id: String, data: UpdateUserRequest) -> Result<Option<UserDTO>> {
        let user = sqlx::query_as::<_, UserDTO>(
            r#"
        UPDATE users SET username = $2
        WHERE id = $1
        RETURNING id, email, username, status
        "#
        )
            .bind(user_id)
            .bind(data.username)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn update_status(pool: &PgPool, user_id: String, data: UpdateUserStatusRequest) -> Result<Option<UserDTO>> {
        let user = sqlx::query_as::<_, UserDTO>(
            r#"
        UPDATE users SET status = $1, updated_at = $3
        WHERE id = $2
        RETURNING id, email, username, status
        "#
        )
            .bind(data.status)
            .bind(user_id)
            .bind(Utc::now())
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn delete_user(pool: &PgPool, user_id: String) -> Result<u64> {
        let deleted = sqlx::query(
            r#"
        DELETE FROM users
        WHERE id = $1
        "#
        )
            .bind(user_id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(deleted)
    }
}