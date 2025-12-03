use actix_web::web;

pub mod user_controller;

const API_VERSION: &str = "api/v1/";
const USER_API: &str = "users";

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", API_VERSION, USER_API);
    cfg.service(web::scope(
        path.as_str())
        .configure(user_controller::init_user_routes));
}