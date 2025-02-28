use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod database;
mod models;
mod repository;
mod routes;
mod schema;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = database::init_pool(&database_url).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}