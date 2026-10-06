use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, prelude::FromRow};
use uuid::Uuid;

use crate::domain::auth::{UserSession, data::user_session_repository::UserSessionRepository};

#[derive(Debug, FromRow)]
struct UserSessionDb {
    id: i64,
    user_id: i64,
    client_name: String,
    device_name: String,
    location: String,
    token_id: Uuid,
    last_refresh_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl TryFrom<UserSessionDb> for UserSession {
    type Error = anyhow::Error;

    fn try_from(value: UserSessionDb) -> Result<Self, Self::Error> {
        Ok(UserSession {
            id: value.id,
            user_id: value.user_id,
            device_name: value.device_name,
            client_name: value.client_name,
            location: value.location,
            token_id: value.token_id,
            last_refresh_at: value.last_refresh_at,
            created_at: value.created_at,
        })
    }
}

pub struct PostgresUserSessionRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserSessionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSessionRepository for PostgresUserSessionRepository {
    async fn save(&self, session: &UserSession) -> Result<(), anyhow::Error> {
        sqlx::query(
            "
            INSERT INTO user_sessions (
                id,
                user_id,
                client_name,
                device_name,
                location,
                token_id,
                last_refresh_at,
                created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
        )
        .bind(&session.id)
        .bind(&session.user_id)
        .bind(&session.client_name)
        .bind(&session.device_name)
        .bind(&session.location)
        .bind(&session.token_id)
        .bind(&session.last_refresh_at)
        .bind(&session.created_at)
        .execute(&self.pool)
        .await
        .context("Failed to insert user session")?;

        Ok(())
    }

    async fn get_by_id(&self, session_id: i64) -> Result<Option<UserSession>, anyhow::Error> {
        sqlx::query_as::<_, UserSessionDb>("SELECT * FROM user_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch user session by id")?
            .map(UserSession::try_from)
            .transpose()
    }

    async fn get_user_sessions(&self, user_id: i64) -> Result<Vec<UserSession>, anyhow::Error> {
        sqlx::query_as::<_, UserSessionDb>("SELECT * FROM user_sessions WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch all user session by user_id")?
            .into_iter()
            .map(UserSession::try_from)
            .collect::<Result<Vec<_>, _>>()
    }

    async fn update_token_id(&self, session: &UserSession) -> Result<(), anyhow::Error> {
        sqlx::query(
            "
            UPDATE user_sessions
            SET
                last_refresh_at = ?,
                token_id = ?
            WHERE id = ?",
        )
        .bind(&session.last_refresh_at)
        .bind(&session.token_id)
        .bind(&session.id)
        .execute(&self.pool)
        .await
        .context("Failed to update token_id")?;

        Ok(())
    }

    async fn delete_by_id(&self, session_id: i64) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM user_sessions WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .context("Failed to delete user session by id")?;

        Ok(())
    }
}
