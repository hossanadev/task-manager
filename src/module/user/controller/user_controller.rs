use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use crate::configuration::database::DbPool;
use crate::constant::{error_message, success_message};
use crate::common::response::CustomResponse;
use crate::module::user::data::user_model::{UserStatus};
use crate::module::user::data::user_repository;
use crate::module::user::dto::request::{CreateUserRequest, UpdateUserRequest, UpdateUserStatusRequest};
use crate::module::user::dto::response::UserDTO;
use crate::util::password_hasher::hash_password;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_user)
        .service(get_user)
        .service(get_users)
        .service(update_user)
        .service(update_user_status)
        .service(delete_user);
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User is created successfully", body = CustomResponse<UserDTO>)
    ),
    tag = "User Module"
)]
#[post("")]
async fn create_user(pool: web::Data<DbPool>, user: web::Json<CreateUserRequest>) -> impl Responder {
    let mut request = user.into_inner();
    request.password = hash_password(&request.password);
    match user_repository::create_user(&pool, request).await {
        Ok(user) => HttpResponse::Created().json(CustomResponse::new(201, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    get,
    path =  "/api/v1/users",
    responses(
        (status = 200, description = "Users retrieved successfully", body = CustomResponse<Vec<UserDTO>>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User Module"
)]
#[get("")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    match user_repository::get_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(users))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    get,
    path =  "/api/v1/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = CustomResponse<UserDTO>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User Module"
)]
#[get("{id}")]
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> impl Responder {
    match user_repository::get_user(&pool, user_id.to_string()).await {
        Ok(user) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    put,
    path =  "/api/v1/users/{id}",
    request_body = UpdateUserRequest,
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User updated successfully", body = CustomResponse<UserDTO>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User Module"
)]
#[put("{id}")]
pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<String>, data: web::Json<UpdateUserRequest>) -> impl Responder {
    match user_repository::update_user(&pool, user_id.to_string(), data.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}",
    params(
        ("id" = String, Path, description = "User ID"),
        ("status" = UserStatus, Query, description = "User Status")
    ),
    responses(
        (status = 200, description = "User status updated successfully", body = CustomResponse<UserDTO>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User Module"
)]
#[patch("{id}")]
pub async fn update_user_status(pool: web::Data<DbPool>, user_id: web::Path<String>, data: web::Query<UpdateUserStatusRequest>) -> impl Responder {
    match user_repository::update_status(&pool, user_id.to_string(), data.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    delete,
    path =  "/api/v1/users/{id}",
    params(
        ("id" = String, Path, description = "User ID"),
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User Module"
)]
#[delete("{id}")]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> impl Responder {
    match user_repository::delete_user(&pool, user_id.to_string()).await {
        Ok(u64) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(u64))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}