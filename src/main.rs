use actix_web::{web, App, HttpServer};
use std::env;
use module::user::{controller as user_controller};
use module::task::{controller as task_controller, data};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};
use tracing::{info};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::SecurityRequirement;
use crate::module::auth::controller::auth_controller;
use crate::module::auth::controller::auth_controller::init_auth_routes;
use crate::module::task::controller::task_controller::init_task_routes;
use crate::module::user::controller::user_controller::init_user_routes;

mod module;
mod documentation;
mod configuration;
mod util;

const DATABASE_URL_CONNECTION_ERROR_MESSAGE: &str = "Database URL is required";
const DATABASE_POOL_CREATION_ERROR_MESSAGE: &str = "Database pool connection failed";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_ansi(true).init();

    let database_url = env::var("DATABASE_URL")
        .expect(DATABASE_URL_CONNECTION_ERROR_MESSAGE);

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
        .expect(DATABASE_POOL_CREATION_ERROR_MESSAGE);
    info!("Database Connection Successful");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    info!("Database Migration Completed");

    /* Swagger OpenAPI Docs */
    let mut docs = documentation::user_docs::UserApiDoc::openapi();
    docs.merge(documentation::task_docs::TaskApiDoc::openapi());
    docs.merge(documentation::auth_docs::AuthApiDoc::openapi());

    if docs.components.is_none() {
        docs.components = Some(utoipa::openapi::Components::default());
    }

    docs.components
        .as_mut()
        .unwrap()
        .security_schemes
        .insert(
            "bearer_auth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Enter JWT token here"))
                    .build(),
            ),
        );

    if docs.security.is_none() {
        docs.security = Some(vec![]);
    }

    docs.security
        .as_mut()
        .unwrap()
        .push(SecurityRequirement::new(
            "bearer_auth",
            &[] as &[String],
        ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", docs.clone())
                    .config(
                        Config::default()
                            .validator_url("none")
                            .doc_expansion("none")
                            .display_request_duration(true)
                    )
            )
            .configure(init_auth_routes)
            .configure(init_user_routes)
            .configure(init_task_routes)
    }).bind(&bind_address)?.run().await
}