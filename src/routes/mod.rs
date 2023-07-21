mod health;

use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/health-check", get(health::health_check))
}
