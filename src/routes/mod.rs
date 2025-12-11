use axum::{Router, routing::get};

use crate::handlers::health_handler::health_handler;
use crate::handlers::serveurs_handler::servers_handler;
use crate::state::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/serveurs", get(servers_handler))
}
