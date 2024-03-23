use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::data::AppState;
use crate::handlers::healthcheck_handler;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(healthcheck_handler))
        .with_state(app_state)
}