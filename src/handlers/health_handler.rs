use axum::Json;
use serde_json::{json, Value};
use crate::response::ApiResponse;

pub async fn health_handler() -> Json<ApiResponse<serde_json::Value>> {
    let health_data = json!({ "uptime": "healthy" });
    Json(ApiResponse::success(health_data))
}