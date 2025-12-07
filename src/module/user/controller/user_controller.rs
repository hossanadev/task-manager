use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use crate::configuration::database::DbPool;
use crate::util::custom_response::CustomResponse;
use crate::module::user::data::user_model::{UserStatus};
use crate::module::user::dto::request::{CreateUserRequest, UpdateUserRequest, UpdateUserStatusRequest};
use crate::module::user::dto::response::UserDTO;
use crate::module::user::service::user_service::UserService;
use crate::util::auth_middleware::AuthMiddleware;

pub const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal server error";
pub const REQUEST_SUCCESSFUL_MESSAGE: &str = "Request successful";
const API_VERSION: &str = "/api/v1/";
const USER_API: &str = "users";

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", API_VERSION, USER_API);
    cfg.service(
        web::scope(path.as_str())
            .wrap(AuthMiddleware)
            .service(create_user)
            .service(get_user)
            .service(get_users)
            .service(update_user)
            .service(update_user_status)
            .service(delete_user)
    );
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
async fn create_user(pool: web::Data<DbPool>, request: web::Json<CreateUserRequest>) -> impl Responder {
    match UserService::create_user(&pool, request.into_inner()).await {
        Ok(user) =>
            HttpResponse::Created()
                .json(CustomResponse::new(201, REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) =>
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
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
    match UserService::get_users(pool).await {
        Ok(users) =>
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(users))),
        Err(_) =>
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
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
    match UserService::get_user(pool, user_id).await {
        Ok(user) =>
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) =>
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
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
    match UserService::update_user(pool, user_id, data).await {
        Ok(user) =>
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) =>
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
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
    match UserService::update_user_status(pool, user_id, data).await {
        Ok(user) =>
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) =>
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
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
    match UserService::delete_user(pool, user_id).await {
        Ok(u64) =>
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(u64))),
        Err(_) =>
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}