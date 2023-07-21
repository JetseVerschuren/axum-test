use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use uuid::Uuid;

use crate::{
    models::{CreateUser, DynUserRepo, User},
    AppError,
};

pub fn create_users_router() -> Router {
    Router::new()
        .route("/", get(get_all_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user))
}

pub async fn get_all_users(
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = user_repo.find_all().await?;
    Ok(users.into())
}

pub async fn create_user(
    Extension(user_repo): Extension<DynUserRepo>,
    Json(params): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = user_repo.create(params).await?;
    Ok(user.into())
}

pub async fn get_user(
    Path(user_id): Path<Uuid>,
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<User>, AppError> {
    let user = user_repo.find(user_id).await?;
    Ok(user.into())
}
