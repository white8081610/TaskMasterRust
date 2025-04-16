use axum::{
    extract::{Path, Query, Extension, Json},
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::NaiveDate;
use crate::database::operations::Database;
use crate::models::task::Task;
use crate::models::engineer::Engineer;
use crate::api::routes::{TaskQuery, success, error, not_found, server_error};

// GET /api/tasks
pub async fn get_tasks(
    Query(query): Query<TaskQuery>,
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Result<Json<crate::api::routes::ApiResponse<Vec<Task>>>, (StatusCode, Json<crate::api::routes::ApiResponse<Vec<Task>>>)> {
    let date = match query.date {
        Some(date_str) => {
            match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => return Err(error("Invalid date format. Use YYYY-MM-DD")),
            }
        },
        None => None,
    };
    
    let db = db.lock().await;
    match db.get_tasks(date, query.engineer).await {
        Ok(tasks) => Ok(success(tasks)),
        Err(e) => Err(server_error(&e.to_string())),
    }
}

// GET /api/tasks/:id
pub async fn get_task(
    Path(task_id): Path<i64>,
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Result<Json<crate::api::routes::ApiResponse<Task>>, (StatusCode, Json<crate::api::routes::ApiResponse<Task>>)> {
    let db = db.lock().await;
    match db.get_task(task_id).await {
        Ok(Some(task)) => Ok(success(task)),
        Ok(None) => Err(not_found()),
        Err(e) => Err(server_error(&e.to_string())),
    }
}

// POST /api/tasks
pub async fn create_task(
    Json(task): Json<Task>,
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Result<Json<crate::api::routes::ApiResponse<Task>>, (StatusCode, Json<crate::api::routes::ApiResponse<Task>>)> {
    let mut db = db.lock().await;
    match db.create_task(task).await {
        Ok(created_task) => Ok(success(created_task)),
        Err(e) => Err(server_error(&e.to_string())),
    }
}

// PUT /api/tasks/:id
pub async fn update_task(
    Path(task_id): Path<i64>,
    Json(task): Json<Task>,
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Result<Json<crate::api::routes::ApiResponse<Task>>, (StatusCode, Json<crate::api::routes::ApiResponse<Task>>)> {
    if task_id != task.id {
        return Err(error("Task ID in URL does not match task ID in body"));
    }
    
    let mut db = db.lock().await;
    match db.update_task(task).await {
        Ok(updated_task) => Ok(success(updated_task)),
        Err(e) => Err(server_error(&e.to_string())),
    }
}

// DELETE /api/tasks/:id
pub async fn delete_task(
    Path(task_id): Path<i64>,
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Result<Json<crate::api::routes::ApiResponse<bool>>, (StatusCode, Json<crate::api::routes::ApiResponse<bool>>)> {
    let mut db = db.lock().await;
    match db.delete_task(task_id).await {
        Ok(true) => Ok(success(true)),
        Ok(false) => Err(not_found()),
        Err(e) => Err(server_error(&e.to_string())),
    }
}

// GET /api/engineers
pub async fn get_engineers(
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Result<Json<crate::api::routes::ApiResponse<Vec<Engineer>>>, (StatusCode, Json<crate::api::routes::ApiResponse<Vec<Engineer>>>)> {
    let db = db.lock().await;
    match db.get_engineers().await {
        Ok(engineers) => Ok(success(engineers)),
        Err(e) => Err(server_error(&e.to_string())),
    }
}
