use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use crate::configuration::database::DbPool;
use crate::module::task::data::task_model::{Task, TaskStatus};
use crate::util::custom_response::CustomResponse;
use crate::module::task::dto::request::{CreateTaskRequest, UpdateTaskRequest, UpdateTaskStatusRequest};
use crate::module::task::service::task_service::TaskService;
use crate::util::auth_middleware::AuthMiddleware;

const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal server error";
const REQUEST_SUCCESSFUL_MESSAGE: &str = "Request successful";
const API_VERSION: &str = "/api/v1/";
const TASK_API: &str = "tasks";

pub fn init_task_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", API_VERSION, TASK_API);
    cfg.service(
        web::scope(
            path.as_str())
            .wrap(AuthMiddleware)
            .service(create_task)
            .service(get_tasks)
            .service(get_task)
            .service(update_task)
            .service(update_task_status)
            .service(delete_task)
    );
}

#[utoipa::path(
    post,
    path =  "/api/v1/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created successfully", body = CustomResponse<Task>),
        (status = 409, description = "Task with this title already exists"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Task Module"
)]
#[post("")]
pub async fn create_task(pool: web::Data<DbPool>, task: web::Json<CreateTaskRequest>) -> impl Responder {
    match TaskService::create_task(pool, task).await {
        Ok(task) => 
            HttpResponse::Created()
                .json(CustomResponse::new(201, REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Err(_) => 
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    get,
    path =  "/api/v1/tasks",
    responses(
        (status = 200, description = "Tasks retrieved successfully", body = CustomResponse<Vec<Task>>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Task Module"
)]
#[get("")]
pub async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    match TaskService::get_tasks(pool).await {
        Ok(tasks) => 
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(tasks))),
        Err(_) => 
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task retrieved successfully", body = CustomResponse<Task>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Task Module"
)]
#[get("{id}")]
pub async fn get_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> impl Responder {
    match TaskService::get_task(pool, task_id).await {
        Ok(task) => 
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Err(_) => 
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/tasks/{id}",
    request_body = UpdateTaskRequest,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task updated successfully", body = CustomResponse<Task>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Task Module"
)]
#[put("{id}")]
pub async fn update_task(pool: web::Data<DbPool>, task: web::Json<UpdateTaskRequest>, task_id: web::Path<String>) -> impl Responder {
    match TaskService::update_task(pool, task, task_id).await {
        Ok(task) => 
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Err(_) => 
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID"),
        ("status" = TaskStatus, Query, description = "Task status")
    ),
    responses(
        (status = 200, description = "Task status updated successfully", body = CustomResponse<Task>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Task Module"
)]
#[patch("{id}")]
pub async fn update_task_status(pool: web::Data<DbPool>, task_id: web::Path<String>, request: web::Query<UpdateTaskStatusRequest>) -> impl Responder {
    match TaskService::update_task_status(pool, task_id, request).await {
        Ok(task) => 
            HttpResponse::Ok()
                .json(CustomResponse::new(200, REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Err(_) => 
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task deleted successfully"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Task Module"
)]
#[delete("{id}")]
pub async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> impl Responder {
    match TaskService::delete_task(pool, task_id).await {
        Ok(_u64) => 
            HttpResponse::Ok()
                .json(CustomResponse::<()>::new(200, REQUEST_SUCCESSFUL_MESSAGE, None)),
        Err(_) => 
            HttpResponse::InternalServerError()
                .json(CustomResponse::<()>::new(500, INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}