use actix_web::{web, App, HttpServer};
use std::env;
use crate::data::database;

mod controller;
mod data;
mod dto;
mod constant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL")
        .expect(constant::error_message::DATABASE_URL_CONNECTION_ERROR_MESSAGE);
    let base_url = env::var("BASE_URL")
        .expect("BASE_URL env var not set");

    let pool = database::init_pool(&database_url)
        .await
        .expect(constant::error_message::DATABASE_POOL_CREATION_ERROR_MESSAGE);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(controller::init_routes)
    })
        .bind(&base_url)?
        .run()
        .await
}