use axum::extract::{Json, State};

use crate::models::{SharedState, Time};

pub async fn set_time_handler(State(state): State<SharedState>, Json(payload): Json<Time>) {
    state.lock().await.time = payload;
    let time = &state.lock().await.time;
    println!("New time set: {}", time.format_string());
}
