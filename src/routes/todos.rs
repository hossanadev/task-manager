use actix_web::{delete, get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::database::DbPool;
use crate::models::Todo;
use crate::repository;
use crate::response::CustomResponse;

pub fn init_todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_todo)
        .service(get_todos)
        .service(delete_todo);
}

#[post("")]
async fn create_todo(pool: web::Data<DbPool>, data: web::Json<Todo>) -> impl Responder {
    match repository::create_todo(&pool, data.into_inner()).await {
        Ok(todo) => HttpResponse::Created().json(CustomResponse::new(201, "Todo created successfully", Some(todo))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, "Error creating todo", None)),
    }
}

#[get("")]
async fn get_todos(pool: web::Data<DbPool>) -> impl Responder {
    match repository::get_todos(&pool).await {
        Ok(todos) => HttpResponse::Ok().json(CustomResponse::new(200, "Todos fetched successfully", Some(todos))),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, "Error fetching todos", None)),
    }
}

#[delete("/{id}")]
async fn delete_todo(pool: web::Data<DbPool>, todo_id: web::Path<Uuid>) -> impl Responder {
    match repository::delete_todo(&pool, todo_id.to_string()).await {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json(CustomResponse::<()>::new(200, "Todo deleted successfully", None)),
        Ok(_) => HttpResponse::NotFound().json(CustomResponse::<()>::new(404, "Todo not found", None)),
        Err(_) => HttpResponse::InternalServerError().json(CustomResponse::<()>::new(500, "Error deleting todo", None)),
    }
}