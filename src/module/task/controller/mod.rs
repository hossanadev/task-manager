use actix_web::web;
use crate::constant::module_api;

pub mod task_controller;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", module_api::API_VERSION, module_api::TASK_API);
    cfg.service(web::scope(
             path.as_str())
            .configure(task_controller::init_task_routes));
}