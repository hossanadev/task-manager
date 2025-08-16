use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use crate::database::DbPool;
use crate::data::task_model::{StatusParam, Task};
use crate::data::task_repository;
use crate::dto::response::CustomResponse;
use crate::constant::{success_message, error_message};

pub fn init_task_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_task)
        .service(get_tasks)
        .service(get_task)
        .service(update_task)
        .service(update_status)
        .service(delete_task);
}

#[post("")]
async fn create_task(pool: web::Data<DbPool>, data: web::Json<Task>) -> impl Responder {
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

#[get("")]
async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    match task_repository::get_tasks(&pool).await {
        Ok(tasks) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(tasks))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}

#[get("{id}")]
async fn get_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> impl Responder {
    match task_repository::get_task_by_id(&pool, task_id.to_string()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[put("{id}")]
async fn update_task(pool: web::Data<DbPool>, mut data: web::Json<Task>, task_id: web::Path<String>) -> impl Responder {
    data.touch();
    match task_repository::update_task_by_id(&pool, data.into_inner(), task_id.to_string()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[patch("{id}")]
async fn update_status(pool: web::Data<DbPool>, task_id: web::Path<String>, query: web::Query<StatusParam>) -> impl Responder {
    match task_repository::update_status_by_task_id(&pool, query.into_inner().status, task_id.to_string()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(CustomResponse::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, Some(task))),
        Ok(None) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None))
    }
}

#[delete("{id}")]
async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> impl Responder {
    match task_repository::delete_task(&pool, task_id.to_string()).await {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json(CustomResponse::<()>::new(200, success_message::REQUEST_SUCCESSFUL_MESSAGE, None)),
        Ok(_) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, error_message::NOT_FOUND_ERROR_MESSAGE, None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, error_message::INTERNAL_SERVER_ERROR_MESSAGE, None)),
    }
}