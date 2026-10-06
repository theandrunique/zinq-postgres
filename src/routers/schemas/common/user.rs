use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::auth::{SessionLifetime, User};

#[derive(Serialize)]
pub struct UserPrivateSchema {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,

    pub sessions_ttl: SessionLifetime,
    pub email: String,
    pub email_verified: bool,
}

impl From<User> for UserPrivateSchema {
    fn from(value: User) -> Self {
        Self {
            id: value.id.to_string(),
            username: value.username,
            display_name: value.display_name,
            bio: value.bio,
            avatar: value.avatar,
            created_at: value.created_at,
            sessions_ttl: value.sessions_ttl,
            email: value.email,
            email_verified: value.email_verified,
        }
    }
}

#[derive(Serialize)]
pub struct UserPublicSchema {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublicSchema {
    fn from(value: User) -> Self {
        Self {
            id: value.id.to_string(),
            username: value.username,
            display_name: value.display_name,
            bio: value.bio,
            avatar: value.avatar,
            created_at: value.created_at,
        }
    }
}
