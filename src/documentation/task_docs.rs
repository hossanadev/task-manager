use crate::controller::task_controller::{
    __path_health_check, __path_create_task, __path_get_tasks, __path_get_task,
    __path_update_task, __path_update_status, __path_delete_task
};
use crate::data::task_model::{Task, StatusParam};
use crate::module::task::dto::response::CustomResponse;
use utoipa::{OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
       create_task, update_task, delete_task,
        get_task, get_tasks, update_status, health_check),
    components(
        schemas(Task, CustomResponse<Task>, StatusParam)
    ),
    tags(
        (name = "Tasks Module", description = "Task Module API")
    )
)]
pub struct ApiDoc;