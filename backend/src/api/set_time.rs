use axum::{extract::State, http::StatusCode, Json};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::models::{DateTimeApiPayload, SharedState};

#[derive(Serialize)]
pub struct SetTimeApiResponse {
    status: bool,
    message: Option<String>,
}

pub async fn set_time_handler(
    State(state): State<SharedState>,
    Json(payload): Json<DateTimeApiPayload>,
) -> (StatusCode, Json<SetTimeApiResponse>) {
    if let Ok(time) = NaiveDateTime::parse_from_str(&payload.date_time, "%Y-%m-%d %H:%M:%S") {
        state.lock().await.time = time;
        println!("New time set: {}", time.format("%Y-%m-%d %H:%M:%S"));
        (
            StatusCode::OK,
            Json(SetTimeApiResponse {
                status: true,
                message: None,
            }),
        )
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(SetTimeApiResponse {
                status: false,
                message: Some("Parse error".to_string()),
            }),
        )
    }
}
