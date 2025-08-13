use actix_web::{delete, get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::database::DbPool;
use crate::data::task_model::Task;
use crate::data::task_repository;
use crate::dto::response::CustomResponse;
use crate::constant::{success_message, error_message};

pub fn init_task_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_task)
        .service(get_tasks)
        .service(delete_task);
}

#[post("")]
async fn create_task(pool: web::Data<DbPool>, data: web::Json<Task>) -> impl Responder {
    match task_repository::create_task(&pool, data.into_inner()).await {
        Ok(task) => HttpResponse::Created().json(CustomResponse::new(201, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[get("")]
async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    match task_repository::get_tasks(&pool).await {
        Ok(tasks) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(tasks))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[delete("{id}")]
async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<Uuid>) -> impl Responder {
    match task_repository::delete_task(&pool, task_id.to_string()).await {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json(CustomResponse::<()>::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, None)),
        Ok(_) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}