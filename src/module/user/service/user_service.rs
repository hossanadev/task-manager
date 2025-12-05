use actix_web::web;
use tracing::error;
use crate::configuration::database::DbPool;
use anyhow::Result;
use crate::module::user::data::user_repository::UserRepository;
use crate::module::user::dto::request::{CreateUserRequest, UpdateUserRequest, UpdateUserStatusRequest};
use crate::module::user::dto::response::UserDTO;
use crate::util::password_hasher::hash_password;

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