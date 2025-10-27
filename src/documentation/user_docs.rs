use utoipa::{OpenApi};
use crate::module::user::data::user_model::UserStatus;
use crate::common::response::CustomResponse;
use crate::module::user::dto::response::UserDTO;
use crate::user_controller::user_controller::{__path_create_user, __path_get_users, __path_get_user,
__path_update_user, __path_update_user_status,__path_delete_user};
#[derive(OpenApi)]
#[openapi(
    paths(
       create_user, get_users, get_user, update_user, update_user_status, delete_user),
    components(
        schemas(CustomResponse<UserDTO>, UserStatus)
    ),
    tags(
        (name = "Users Module", description = "User Module API")
    )
)]
pub struct UserApiDoc;