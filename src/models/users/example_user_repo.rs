use std::sync::Mutex;

use axum::async_trait;
use uuid::Uuid;

use super::{CreateUser, User, UserRepo, UserRepoError};

/// Example implementation of `UserRepo`.
pub struct ExampleUserRepo {
    users: Mutex<Vec<User>>,
}

impl ExampleUserRepo {
    pub fn new() -> Self {
        ExampleUserRepo {
            users: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl UserRepo for ExampleUserRepo {
    async fn find_all(&self) -> Result<Vec<User>, UserRepoError> {
        let users = self.users.lock().unwrap();
        Ok(users.clone())
    }

    async fn find(&self, _user_id: Uuid) -> Result<User, UserRepoError> {
        let users = self.users.lock().unwrap();
        Ok(users
            .iter()
            .find(|user| user.id == _user_id)
            .ok_or(UserRepoError::NotFound)?
            .clone())
    }

    async fn create(&self, params: CreateUser) -> Result<User, UserRepoError> {
        let mut users = self.users.lock().unwrap();
        let new_user = User {
            id: Uuid::new_v4(),
            username: params.username,
        };
        users.push(new_user.clone());
        Ok(new_user)
    }
}
