use actix_web::{web, App, HttpServer};
use std::env;
use module::user::{controller as user_controller};
use module::task::{controller as task_controller, data};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod constant;
mod module;
mod documentation;
mod configuration;
mod common_lib;

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

    let pool = configuration::database::init_pool(&database_url)
        .await
        .expect(constant::error_message::DATABASE_POOL_CREATION_ERROR_MESSAGE);

    let mut user_docs = documentation::user_docs::UserApiDoc::openapi();
    let task_docs = documentation::task_docs::TaskApiDoc::openapi();

    user_docs.merge(task_docs);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_controller::init_user_routes)
            .configure(task_controller::init_task_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", user_docs.clone())
            )
    })
        .bind(&bind_address)?
        .run()
        .await
}