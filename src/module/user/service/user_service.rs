use actix_web::web;
use tracing::error;
use crate::configuration::database::DbPool;
use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;
use crate::module::auth::dto::request::LoginRequest;
use crate::module::auth::dto::response::LoginResponse;
use crate::module::user::data::user_model::User;
use crate::module::user::data::user_repository::UserRepository;
use crate::module::user::dto::request::{CreateUserRequest, UpdateUserRequest, UpdateUserStatusRequest};
use crate::module::user::dto::response::UserDTO;
use crate::util::password_hasher::hash_password;
use crate::util::auth_token::generate_token;

pub const NOT_FOUND_ERROR_MESSAGE: &str = "User Not Found";

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: &DbPool, mut req: CreateUserRequest) -> Result<UserDTO> {
        let hashed = match hash_password(&req.password) {
            Ok(h) => h,
            Err(e) => {
                error!("Password hashing failed: {:?}", e);
                return Err(anyhow::anyhow!("Password hashing failed"));
            }
        };
        req.password = hashed;
        let user = UserRepository::create_user(pool, req).await?;
        Ok(user)
    }

    pub async fn login(request: LoginRequest, pool: &PgPool) -> Result<LoginResponse> {

        let user: User = UserRepository::get_raw_user(pool, request.email).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let parsed_hash = PasswordHash::new(&user.password)
            .map_err(|_| anyhow::anyhow!("Stored password hash is invalid"))?;

        Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow::anyhow!("Invalid credentials"))?;

        Ok(
            LoginResponse {
                token: generate_token(user.id.clone()),
                user: UserDTO {
                    id: user.id,
                    email: user.email,
                    username: user.username,
                    status: user.status,
                }
            }
        )
    }

    pub async fn get_users(pool: web::Data<DbPool>) -> Result<Vec<UserDTO>> {
        UserRepository::get_users(&pool).await
    }

    pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> Result<UserDTO> {
        let user = UserRepository::get_user(&pool, user_id.to_string()).await?
            .ok_or_else(|| anyhow::anyhow!(NOT_FOUND_ERROR_MESSAGE))?;
        Ok(user)
    }

    pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<String>, data: web::Json<UpdateUserRequest>) -> Result<UserDTO> {
        let user = UserRepository::update_user(&pool, user_id.to_string(), data.into_inner()).await?
            .ok_or_else(|| anyhow::anyhow!(NOT_FOUND_ERROR_MESSAGE))?;
        Ok(user)
    }

    pub async fn update_user_status(pool: web::Data<DbPool>, user_id: web::Path<String>, data: web::Query<UpdateUserStatusRequest>) -> Result<UserDTO> {
        let user = UserRepository::update_status(&pool, user_id.to_string(), data.into_inner()).await?
            .ok_or_else(|| anyhow::anyhow!(NOT_FOUND_ERROR_MESSAGE))?;
        Ok(user)
    }

    pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> Result<u64> {
        UserRepository::delete_user(&pool, user_id.to_string()).await
    }
}