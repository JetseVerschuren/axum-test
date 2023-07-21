use axum::async_trait;
use uuid::Uuid;

use super::{CreateUser, User, UserRepo, UserRepoError};

/// Example implementation of `UserRepo`.
pub struct ExampleUserRepo {
    users: Vec<User>,
}

impl ExampleUserRepo {
    pub fn new() -> Self {
        ExampleUserRepo { users: vec![] }
    }
}

#[async_trait]
impl UserRepo for ExampleUserRepo {
    async fn find_all(&self) -> Result<Vec<User>, UserRepoError> {
        Ok(self.users.clone())
    }

    async fn find(&self, _user_id: Uuid) -> Result<User, UserRepoError> {
        unimplemented!()
    }

    async fn create(&self, _params: CreateUser) -> Result<User, UserRepoError> {
        unimplemented!()
    }
}
