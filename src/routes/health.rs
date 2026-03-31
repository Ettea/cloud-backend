use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
