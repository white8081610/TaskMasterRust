use axum::{
    extract::{Path, Query, Extension, Json},
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::database::operations::Database;
use crate::models::task::Task;
use crate::models::engineer::Engineer;
use chrono::NaiveDate;

// Query parameters for tasks
#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub date: Option<String>,
    pub engineer: Option<String>,
}

// Response types
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// Helper functions to create response
pub fn success<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    })
}

pub fn error<T>(message: &str) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }),
    )
}

pub fn not_found<T>() -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Resource not found".to_string()),
        }),
    )
}

pub fn server_error<T>(message: &str) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }),
    )
}
