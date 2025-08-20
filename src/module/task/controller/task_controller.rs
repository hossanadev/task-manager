use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use crate::configuration::database::DbPool;
use crate::module::task::data::task_model::{Task, TaskStatus};
use crate::module::task::data::task_repository;
use crate::module::task::dto::response::CustomResponse;
use crate::constant::{success_message, error_message};

pub fn init_task_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(health_check)
        .service(create_task)
        .service(get_tasks)
        .service(get_task)
        .service(update_task)
        .service(update_status)
        .service(delete_task);
}

#[utoipa::path(
    get,
    path = "/api/v1/tasks/health",
    responses(
        (status = 200, description = "API is live")
    ),
    tag = "Tasks Module"
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(CustomResponse::<String>::new(200, "Task api is alive", Some(String::from("Task API"))))
}

#[utoipa::path(
    post,
    path =  "/api/v1/tasks",
    request_body = Task,
    responses(
        (status = 201, description = "Task created successfully", body = CustomResponse<Task>),
        (status = 409, description = "Task with this title already exists"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Tasks Module"
)]
#[post("")]
pub async fn create_task(pool: web::Data<DbPool>, data: web::Json<Task>) -> impl Responder {
    match task_repository::exists_by_task_title(&pool, &data).await {
        Ok(true) => {
            return HttpResponse::Conflict().json(CustomResponse::<()>::new(409, "Task with this title already exists", None));
        }
        Ok(false) => {}
        Err(_) => {
            return HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None));
        }
    }
    match task_repository::create_task(&pool, data.into_inner()).await {
        Ok(task) => HttpResponse::Created().json(CustomResponse::new(201, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[utoipa::path(
    get,
    path =  "/api/v1/tasks",
    responses(
        (status = 200, description = "Tasks retrieved successfully", body = CustomResponse<Vec<Task>>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Tasks Module"
)]
#[get("")]
pub async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    match task_repository::get_tasks(&pool).await {
        Ok(tasks) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(tasks))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
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
    tag = "Tasks Module"
)]
#[get("{id}")]
pub async fn get_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> impl Responder {
    match task_repository::get_task_by_id(&pool, task_id.to_string()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/tasks/{id}",
    request_body = Task,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task updated successfully", body = CustomResponse<Task>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Tasks Module"
)]
#[put("{id}")]
pub async fn update_task(pool: web::Data<DbPool>, data: web::Json<Task>, task_id: web::Path<String>) -> impl Responder {
    match task_repository::update_task_by_id(&pool, data.into_inner(), task_id.to_string()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/tasks/{id}",
    params(
        ("id" = String, Path, description = "Task ID"),
        ("status" = String, Query, description = "New task status")
    ),
    responses(
        (status = 200, description = "Task status updated successfully", body = CustomResponse<Task>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Tasks Module"
)]
#[patch("{id}")]
pub async fn update_status(pool: web::Data<DbPool>, task_id: web::Path<String>, status: web::Query<TaskStatus>) -> impl Responder {
    match task_repository::update_status_by_task_id(&pool, status, task_id.to_string()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None))
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
    tag = "Tasks Module"
)]
#[delete("{id}")]
pub async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> impl Responder {
    match task_repository::delete_task(&pool, task_id.to_string()).await {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json(CustomResponse::<()>::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, None)),
        Ok(_) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}