use utoipa::{OpenApi};
use crate::user_controller::user_controller::{__path_create_user, __path_get_users};
#[derive(OpenApi)]
#[openapi(
    paths(
       create_user, get_users),
    components(
        schemas()
    ),
    tags(
        (name = "Users Module", description = "User Module API")
    )
)]
pub struct UserApiDoc;