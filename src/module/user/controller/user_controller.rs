use actix_web::{get, post, web, HttpResponse, Responder};
use crate::configuration::database::DbPool;
use crate::constant::{error_message, success_message};
use crate::common::response::CustomResponse;
use crate::module::user::data::user_repository;
use crate::module::user::dto::request::CreateUserRequest;
use crate::module::user::dto::response::UserDTO;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_user)
        .service(get_user)
        .service(get_users);
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User is created successfully", body = CustomResponse<UserDTO>)
    ),
    tag = "Users Module"
)]
#[post("")]
async fn create_user(pool: web::Data<DbPool>, user: web::Json<CreateUserRequest>) -> impl Responder {
    match user_repository::create_user(&pool, user.into_inner()).await {
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
    tag = "Users Module"
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
    tag = "Users Module"
)]
#[get("{id}")]
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> impl Responder {
    match user_repository::get_user(&pool, user_id.to_string()).await {
        Ok(user) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(user))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}