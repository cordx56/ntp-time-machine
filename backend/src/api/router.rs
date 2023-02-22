use axum::{routing::put, Router};

use super::set_time::set_time_handler;
use crate::models::SharedState;

pub fn router(shared_state: SharedState) -> Router {
    Router::new().route(
        "/api/v1/time",
        put(set_time_handler).with_state(shared_state),
    )
}
