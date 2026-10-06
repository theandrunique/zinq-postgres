use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionLifetime {
    Week,
    Month,
    Month3,
    Month6,
    Month12,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub username_updated_at: DateTime<Utc>,
    pub display_name: String,
    pub bio: Option<String>,
    pub email: String,
    pub email_updated_at: DateTime<Utc>,
    pub email_verified: bool,
    pub is_active: bool,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,

    pub totp_key: Option<Vec<u8>>,
    pub password_hash: String,
    pub password_updated_at: DateTime<Utc>,
    pub sessions_ttl: SessionLifetime,
}

pub struct UserCreateRequest {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub display_name: String,
    pub email: String,
}

impl User {
    pub fn create(request: UserCreateRequest) -> Self {
        let current_time = Utc::now();

        Self {
            id: request.id,
            username: request.username,
            username_updated_at: current_time,
            display_name: request.display_name,
            bio: None,
            email: request.email,
            email_verified: false,
            email_updated_at: current_time,
            is_active: true,
            avatar: None,
            created_at: current_time,

            totp_key: None,
            password_hash: request.password_hash,
            password_updated_at: current_time,
            sessions_ttl: SessionLifetime::Month3,
        }
    }
}
