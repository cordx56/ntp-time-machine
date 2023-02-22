pub mod router;
mod set_time;

use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::models::SharedState;

pub async fn start_server(shared_state: SharedState) {
    let app = router::router(shared_state);
    let app = app.layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_headers([header::CONTENT_TYPE]),
    );
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .ok();
}
