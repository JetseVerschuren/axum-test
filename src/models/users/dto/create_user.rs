use serde::Deserialize;

#[derive(Debug, Deserialize)]
// #[allow(dead_code)]
pub struct CreateUser {
    pub username: String,
}
