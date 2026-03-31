// Placeholder for models
// Add your data models here

use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    pub pool: Option<PgPool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TelemetryPayload {
    pub device_id: String,
    pub timestamp: i64,
    pub payload: Value,
}

#[derive(Serialize)]
pub struct TelemetryResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ---------------------------------------------------------
// NEW: Telemetry model for GET /telemetry
// ---------------------------------------------------------
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Telemetry {
    pub id: i32,
    pub device_id: String,
    pub timestamp: i64,
    pub payload: Value,
}
