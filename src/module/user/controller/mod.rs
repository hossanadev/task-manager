use actix_web::web;
use crate::constant::module_api;

pub mod user_controller;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", module_api::API_VERSION, module_api::USER_API);
    cfg.service(web::scope(
        path.as_str())
        .configure(user_controller::init_user_routes));
}