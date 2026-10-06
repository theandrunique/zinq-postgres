use std::sync::Arc;

use anyhow::{Context, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{Pool, Postgres, prelude::FromRow};

use crate::{
    domain::messages::{Message, MessageType, data::MessageRepository},
    infra::data::message_ack_repository,
};

#[derive(Debug, FromRow)]
struct MessageDb {
    id: i64,
    chat_id: i64,
    author_id: i64,
    content: String,
    message_type: String,
    edited_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl TryFrom<MessageDb> for Message {
    type Error = anyhow::Error;

    fn try_from(value: MessageDb) -> Result<Self, Self::Error> {
        Ok(Message {
            id: value.id,
            chat_id: value.chat_id,
            author_id: value.author_id,
            content: value.content,
            message_type: serde_json::from_str(&value.message_type)?,
            edited_at: value.edited_at,
            created_at: value.created_at,
        })
    }
}

pub struct PostgresMessageRepository {
    pool: Pool<Postgres>,
}

impl PostgresMessageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for PostgresMessageRepository {
    async fn upsert(&self, message: &Message) -> Result<(), anyhow::Error> {
        let type_json = serde_json::to_string(&message.message_type)?;

        sqlx::query(
            "
            INSERT INTO messages (
                id,
                chat_id,
                author_id,
                content,
                type,
                edited_at,
                created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
        )
        .bind(&message.id)
        .bind(&message.chat_id)
        .bind(&message.author_id)
        .bind(&message.content)
        .bind(&type_json)
        .bind(&message.edited_at)
        .bind(&message.created_at)
        .execute(&self.pool)
        .await
        .context("Failed to upsert a message")?;

        Ok(())
    }

    async fn get_by_id(&self, message_id: i64) -> Result<Option<Message>, anyhow::Error> {
        sqlx::query_as::<_, MessageDb>("SELECT * FROM messages WHERE id = $1")
            .bind(message_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch message by id")?
            .map(Message::try_from)
            .transpose()
    }

    async fn get_by_ids(&self, message_ids: &[i64]) -> Result<Vec<Message>, anyhow::Error> {
        if message_ids.is_empty() {
            return Ok(Vec::new());
        }

        sqlx::query_as::<_, MessageDb>("SELECT * FROM messages WHERE id IN $2")
            .bind(message_ids)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch message by ids")?
            .into_iter()
            .map(Message::try_from)
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_messages(
        &self,
        chat_id: i64,
        before_message_id: i64,
        limit: i32,
    ) -> Result<Vec<Message>, anyhow::Error> {
        sqlx::query_as::<_, MessageDb>(
            "
            SELECT *
            FROM messages
            WHERE chat_id = $1 AND message_id < $2
            ORDER BY message_id DESC
            LIMIT $3
            ",
        )
        .bind(chat_id)
        .bind(before_message_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch chat message")?
        .into_iter()
        .map(Message::try_from)
        .collect::<Result<Vec<_>, _>>()
    }

    async fn delete_by_id(&self, message_id: i64) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM messages WHERE AND id = $1")
            .bind(message_id)
            .execute(&self.pool)
            .await
            .context("Failed to delete a message")?;
        Ok(())
    }

    async fn count_messages(
        &self,
        chat_id: i64,
        from_message_id: i64,
        to_message_id: i64,
    ) -> Result<i64, anyhow::Error> {
        let result: i64 = sqlx::query_scalar(
            "
            SELECT COUNT(*)
            FROM messages
            WHERE chat_id = ?
                AND id >= ?
                AND id <= ?
            ",
        )
        .bind(chat_id)
        .bind(from_message_id)
        .bind(to_message_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to count messages")?;
        Ok(result)
    }

    async fn get_message_ids_in_range(
        &self,
        chat_id: i64,
        from_message_id: i64,
        to_message_id: i64,
    ) -> Result<Vec<i64>, anyhow::Error> {
        let result: Vec<i64> = sqlx::query_scalar(
            "
            SELECT id
            FROM messages
            WHERE chat_id = ?
                AND id >= ?
                AND id <= ?
            ",
        )
        .bind(chat_id)
        .bind(from_message_id)
        .bind(to_message_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to get message ids in range")?;

        Ok(result)
    }
}
