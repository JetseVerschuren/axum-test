mod health;
mod users;

use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/health-check", get(health::health_check))
        .nest("/users", users::create_users_router())
}
