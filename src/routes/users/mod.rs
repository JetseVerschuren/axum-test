mod index;

use axum::{routing::get, Router};

pub fn create_users_router() -> Router {
    Router::new().route("/", get(index::get_index))
}
