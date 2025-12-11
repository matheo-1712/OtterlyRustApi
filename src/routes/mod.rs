use axum::{Router, routing::get};

use crate::handlers::health_handler::health_handler;

pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(health_handler))
}
