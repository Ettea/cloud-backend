use axum::{Json, response::IntoResponse, extract::State};
use crate::models::{TelemetryPayload, TelemetryResponse, ErrorResponse, AppState, Telemetry};
pub enum TelemetryResult {
    Success(Json<TelemetryResponse>),
    Error(Json<ErrorResponse>),
}

impl IntoResponse for TelemetryResult {
    fn into_response(self) -> axum::response::Response {
        match self {
            TelemetryResult::Success(json) => json.into_response(),
            TelemetryResult::Error(json) => json.into_response(),
        }
    }
}

pub async fn post_telemetry(State(state): State<AppState>, Json(payload): Json<TelemetryPayload>) -> TelemetryResult {
    if payload.device_id.is_empty() {
        return TelemetryResult::Error(Json(ErrorResponse {
            error: "device_id cannot be empty".to_string(),
        }));
    }
    if payload.timestamp <= 0 {
        return TelemetryResult::Error(Json(ErrorResponse {
            error: "timestamp must be greater than 0".to_string(),
        }));
    }

    let pool = match state.pool {
        Some(ref p) => p,
        None => {
            return TelemetryResult::Error(Json(ErrorResponse {
                error: "Database not available".to_string(),
            }));
        }
    };

    // Save to database
    match sqlx::query("INSERT INTO telemetry (device_id, timestamp, payload) VALUES ($1, $2, $3)")
        .bind(&payload.device_id)
        .bind(&payload.timestamp)
        .bind(&payload.payload)
        .execute(pool)
        .await
    {
        Ok(_) => {
            tracing::info!("Received telemetry: {:?}", payload);
            TelemetryResult::Success(Json(TelemetryResponse {
                status: "received".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to save telemetry: {:?}", e);
            TelemetryResult::Error(Json(ErrorResponse {
                error: "Failed to save telemetry".to_string(),
            }))
        }
    }
}
pub async fn get_telemetry(
    State(state): State<AppState>,
) -> Json<Vec<Telemetry>> {
    let pool = state.pool.as_ref().expect("Database pool not initialized");

    let rows = sqlx::query_as::<_, Telemetry>(
        "SELECT id, device_id, timestamp, payload FROM telemetry ORDER BY id DESC"
    )
        .fetch_all(pool)
        .await
        .unwrap();

    Json(rows)
}