use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::auth::User;

pub enum AddUserError {
    UsernameTaken,
    EmailTaken,
    InternalError(anyhow::Error),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), AddUserError>;

    async fn get_by_id(&self, user_id: i64) -> Result<Option<User>, anyhow::Error>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error>;
    async fn get_by_ids(&self, user_ids: &[i64]) -> Result<Vec<User>, anyhow::Error>;

    async fn exists_by_email(&self, email: &str) -> Result<bool, anyhow::Error>;
    async fn exists_by_username(&self, username: &str) -> Result<bool, anyhow::Error>;
}
