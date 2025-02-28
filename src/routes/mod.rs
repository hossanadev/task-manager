use actix_web::web;

mod todos;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/todos").configure(todos::init_todo_routes));
}