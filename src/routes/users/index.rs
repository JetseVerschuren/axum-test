use axum::{Extension, Json};

use crate::{
    models::{DynUserRepo, User},
    AppError,
};

// #[tracing::instrument]
pub async fn get_index(
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<Vec<User>>, AppError> {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing health-check endpoint.");
    let users = user_repo.find_all().await?;
    Ok(users.into())
}
