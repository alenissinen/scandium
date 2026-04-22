use async_trait::async_trait;
use uuid::Uuid;

use super::{entity::User, error::UserError};

// Information required to create an user
pub struct CreateUserInput {
    pub email: String,
    pub username: String,
    pub password_hash: Option<String>,
    pub display_name: Option<String>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, input: CreateUserInput) -> Result<User, UserError>;
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<User, UserError>;
    async fn find_by_username(&self, username: &str) -> Result<User, UserError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, UserError>;
    async fn exists_by_username(&self, username: &str) -> Result<bool, UserError>;
}
