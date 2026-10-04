use crate::domain::auth::data::user_repository::{
    AddUserError, UserRepository,
};
use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use sqlx::prelude::FromRow;
use std::str::FromStr;
use std::sync::Arc;

use crate::domain::auth::{SessionLifetime, User};

#[derive(Debug, FromRow)]
struct UserDb {
    id: i64,
    username: String,
    username_updated_at: DateTime<Utc>,
    display_name: String,
    bio: Option<String>,
    email: String,
    email_updated_at: DateTime<Utc>,
    email_verified: bool,
    is_active: bool,
    avatar: Option<String>,
    created_at: DateTime<Utc>,

    totp_key: Option<Vec<u8>>,
    password_hash: String,
    password_updated_at: DateTime<Utc>,
    sessions_ttl: i16,
}

impl TryFrom<UserDb> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserDb) -> Result<Self, Self::Error> {
        let sessions_ttl = SessionLifetime::from_i16(value.sessions_ttl)
            .ok_or_else(|| anyhow::anyhow!(
                "Unknown session lifetime value: {}",
                value.sessions_ttl
            ))?;

        Ok(User {
            id: value.id,
            username: value.username,
            username_updated_at: value.username_updated_at,
            display_name: value.display_name,
            bio: value.bio,
            email: value.email,
            email_updated_at: value.email_updated_at,
            email_verified: value.email_verified,
            is_active: value.is_active,
            avatar: value.avatar,
            created_at: value.created_at,

            totp_key: value.totp_key,
            password_hash: value.password_hash,
            password_updated_at: value.password_updated_at,
            sessions_ttl: sessions_ttl,
        })
    }
}

pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    fn map_insert_error(err: sqlx::Error, constraint: &str) -> AddUserError {
        if let sqlx::Error::Database(ref db_err) = err {
            if db_err.code().as_deref() == Some("23505") {
                match db_err.constraint() {
                    Some(c) if c == "users_username_key" => return AddUserError::UsernameTaken,
                    Some(c) if c == "users_email_key" => return AddUserError::EmailTaken,
                    _ => {}
                }
            }
        }
        AddUserError::InternalError(err.into())
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), AddUserError> {
        sqlx::query(
            "INSERT INTO users (
                id,
                username,
                username_updated_at,
                display_name,
                bio,
                email,
                email_updated_at,
                email_verified,
                is_active,
                avatar,
                created_at,
                totp_key,
                password_hash,
                password_updated_at,
                sessions_ttl
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.username_updated_at)
        .bind(&user.display_name)
        .bind(&user.bio)
        .bind(&user.email)
        .bind(&user.email_updated_at)
        .bind(&user.email_verified)
        .bind(&user.is_active)
        .bind(&user.avatar)
        .bind(&user.created_at)
        .bind(&user.totp_key)
        .bind(&user.password_hash)
        .bind(&user.password_updated_at)
        .bind(&user.sessions_ttl.as_i16())
        .execute(&self.pool)
        .await
        .map_err(|e| Self::map_insert_error(e, ""))?;

        Ok(())
    }


    async fn get_by_id(&self, user_id: i64) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by id")?
        .map(|db| User::try_from(db)?)?;

        Ok(user)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by email")?
        .map(|db| User::try_from(db)?)
        .transpose()?;

        Ok(user)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by username")?
        .map(|db| User::try_from(db)?)
        .transpose()?;

        Ok(user)
    }

    async fn get_by_ids(&self, user_ids: &[i64]) -> Result<Vec<User>, anyhow::Error> {
        if user_ids.is_empty() {
            return Ok(vec![]);
        }

        let users = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE id = ANY($1)",
        )
        .bind(user_ids)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch users by ids")?
        .into_iter()
        .map(|db| User::try_from(db)?)
        .collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, anyhow::Error> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .context("Failed to check email existence")?;

        Ok(exists)
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool, anyhow::Error> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .context("Failed to check username existence")?;

        Ok(exists)
    }
}
