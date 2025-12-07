use actix_web::{post, web, HttpResponse, Responder};
use crate::module::auth::dto::request::LoginRequest;
use crate::module::user::service::user_service::UserService;
use crate::util::custom_response::CustomResponse;
use crate::module::auth::dto::response::LoginResponse;

const REQUEST_SUCCESSFUL_MESSAGE: &str = "Request successful";

const API_VERSION: &str = "api/v1/";
const AUTH_API: &str = "auth";

pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    let path = format!("{}{}", API_VERSION, AUTH_API);
    cfg.service(
        web::scope(path.as_str())
            .service(login)
    );
}

#[utoipa::path(
    post,
    path =  "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 201, description = "Task created successfully", body = CustomResponse<LoginResponse>),
        (status = 409, description = "Task with this title already exists"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Auth Module"
)]
#[post("/login")]
pub async fn login(body: web::Json<LoginRequest>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match UserService::login(body.into_inner(), pool.get_ref()).await {
        Ok(login_response) =>
            HttpResponse::Ok().json(
                CustomResponse::new(201, REQUEST_SUCCESSFUL_MESSAGE, Some(login_response))
            ),
        Err(e) =>
            HttpResponse::Unauthorized().body(format!("{}", e))
    }
}