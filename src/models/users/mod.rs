use std::sync::Arc;

use axum::async_trait;
use serde::Serialize;
use uuid::Uuid;

mod dto;
pub use dto::*;

mod example_user_repo;
pub use example_user_repo::ExampleUserRepo;

use crate::AppError;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    id: Uuid,
    username: String,
}

/// Type alias that makes it easier to extract `UserRepo` trait objects.
pub type DynUserRepo = Arc<dyn UserRepo + Send + Sync>;

/// A trait that defines things a user repo might support.
#[async_trait]
pub trait UserRepo {
    async fn find_all(&self) -> Result<Vec<User>, UserRepoError>;

    /// Loop up a user by their id.
    async fn find(&self, user_id: Uuid) -> Result<User, UserRepoError>;

    /// Create a new user.
    async fn create(&self, params: CreateUser) -> Result<User, UserRepoError>;
}

/// Errors that can happen when using the user repo.
#[derive(Debug)]
pub enum UserRepoError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidUsername,
}

impl From<UserRepoError> for AppError {
    fn from(inner: UserRepoError) -> Self {
        AppError::UserRepo(inner)
    }
}
