mod models;
mod routes;
mod settings;

use std::{net::SocketAddr, str::FromStr, sync::Arc};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json, Server,
};
use models::UserRepoError;
use serde_json::json;

use crate::models::{DynUserRepo, ExampleUserRepo};

/// Our app's top level error type.
pub enum AppError {
    /// Something went wrong when calling the user repo.
    UserRepo(UserRepoError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserRepo(UserRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            AppError::UserRepo(UserRepoError::InvalidUsername) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let settings = settings::get_settings().expect("Failed to read settings.");

    tracing_subscriber::fmt::init();

    tracing::event!(target: "backend", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", settings.application.port);

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let address = SocketAddr::from_str(&address).unwrap();
    tracing::debug!("listening on {}", address);

    let user_repo = Arc::new(ExampleUserRepo::new()) as DynUserRepo;
    let router = routes::create_router().layer(Extension(user_repo));
    Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();

}
