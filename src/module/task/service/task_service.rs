use actix_web::web;
use crate::configuration::database::DbPool;
use anyhow::Result;
use crate::module::task::data::task_model::Task;
use crate::module::task::data::task_repository::TaskRepository;
use crate::module::task::dto::request::{CreateTaskRequest, UpdateTaskRequest, UpdateTaskStatusRequest};

pub struct TaskService;

const DUPLICATE_REQUEST_ERROR_MESSAGE: &str = "Task with this title already exists";
const NOT_FOUND_ERROR_MESSAGE: &str = "Task Not found";

impl TaskService {
    pub async fn create_task(pool: web::Data<DbPool>, task: web::Json<CreateTaskRequest>) -> Result<Task> {
        if Self::exists_by_task_title(&pool, &task).await? {
          return Err(anyhow::anyhow!(DUPLICATE_REQUEST_ERROR_MESSAGE))
        }
        let task = TaskRepository::create_task(&pool, task.into_inner()).await?;
        Ok(task)
    }

    pub async fn get_tasks(pool: web::Data<DbPool>) -> Result<Vec<Task>>{
       TaskRepository::get_tasks(&pool).await
    }

    pub async fn get_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> Result<Task> {
        let task = TaskRepository::get_task_by_id(&pool, task_id.to_string()).await?
        .ok_or_else(|| anyhow::anyhow!(NOT_FOUND_ERROR_MESSAGE))?;
        Ok(task)
    }

    pub async fn update_task(pool: web::Data<DbPool>, task: web::Json<UpdateTaskRequest>, task_id: web::Path<String>) -> Result<Task> {
        let task = TaskRepository::update_task_by_id(&pool, task.into_inner(), task_id.to_string()).await?
        .ok_or_else(|| anyhow::anyhow!(NOT_FOUND_ERROR_MESSAGE))?;
        Ok(task)
    }

    pub async fn update_task_status(pool: web::Data<DbPool>, task_id: web::Path<String>, request: web::Query<UpdateTaskStatusRequest>) -> Result<Task> {
        let task = TaskRepository::update_status_by_task_id(&pool, request.into_inner(), task_id.to_string()).await?
        .ok_or_else(|| anyhow::anyhow!(NOT_FOUND_ERROR_MESSAGE))?;
        Ok(task)
    }

    pub async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<String>) -> Result<u64> {
        TaskRepository::delete_task(&pool, task_id.to_string()).await
    }

    async fn exists_by_task_title(pool: &web::Data<DbPool>, task: &web::Json<CreateTaskRequest>) -> Result<bool> {
        TaskRepository::exists_by_task_title(&pool, &task).await
    }
}