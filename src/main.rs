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

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let host = env::var("HOST").unwrap_or_else(|_| {
        if env::var("PORT").is_ok() {
            "0.0.0.0".to_string()
        } else {
            "127.0.0.1".to_string()
        }
    });

    let bind_address = format!("{}:{}", host, port);

    let pool = database::init_pool(&database_url)
        .await
        .expect(constant::error_message::DATABASE_POOL_CREATION_ERROR_MESSAGE);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(controller::init_routes)
    })
        .bind(&bind_address)?
        .run()
        .await
}