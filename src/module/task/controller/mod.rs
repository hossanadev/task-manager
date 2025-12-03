use actix_web::web;

pub mod task_controller;

const API_VERSION: &str = "api/v1/";
const TASK_API: &str = "tasks";

pub fn init_task_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", API_VERSION, TASK_API);
    cfg.service(web::scope(
             path.as_str())
            .configure(task_controller::init_task_routes));
}