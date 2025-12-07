use crate::auth_controller::{__path_login};
use crate::util::custom_response::CustomResponse;
use crate::module::auth::dto::response::LoginResponse;
use utoipa::{OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(login),
    components(
        schemas(CustomResponse<LoginResponse>)
    ),
    tags(
        (name = "Auth Module", description = "")
    )
)]
pub struct AuthApiDoc;