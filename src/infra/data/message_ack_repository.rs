use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, prelude::FromRow};

use crate::domain::message_acks::{MessageAck, data::MessageAckRepository};

#[derive(FromRow)]
struct MessageAckDb {
    chat_id: i64,
    message_id: i64,
    user_id: i64,
    created_at: DateTime<Utc>,
}

impl From<MessageAckDb> for MessageAck {
    fn from(value: MessageAckDb) -> Self {
        MessageAck {
            chat_id: value.chat_id,
            message_id: value.message_id,
            user_id: value.user_id,
            created_at: value.created_at,
        }
    }
}

pub struct PostgresMessageAckRepository {
    pool: Pool<Postgres>,
}

impl PostgresMessageAckRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageAckRepository for PostgresMessageAckRepository {
    async fn bulk_upsert(&self, message_acks: &[MessageAck]) -> Result<(), anyhow::Error> {
        if message_acks.is_empty() {
            return Ok(());
        }

        let chat_ids: Vec<i64> = message_acks.iter().map(|ack| ack.chat_id).collect();
        let message_ids: Vec<i64> = message_acks.iter().map(|ack| ack.message_id).collect();
        let user_ids: Vec<i64> = message_acks.iter().map(|ack| ack.user_id).collect();
        let created_ats: Vec<DateTime<Utc>> =
            message_acks.iter().map(|ack| ack.created_at).collect();

        // Joining against `messages` keeps acks scoped to the chat and drops
        // references to unknown messages instead of failing the whole batch.
        sqlx::query(
            "
            INSERT INTO message_acks (chat_id, message_id, user_id, created_at)
            SELECT ack.chat_id, ack.message_id, ack.user_id, ack.created_at
            FROM UNNEST($1::bigint[], $2::bigint[], $3::bigint[], $4::timestamptz[])
                AS ack(chat_id, message_id, user_id, created_at)
            JOIN messages message
                ON message.id = ack.message_id AND message.chat_id = ack.chat_id
            ON CONFLICT (chat_id, message_id, user_id) DO NOTHING
            ",
        )
        .bind(chat_ids)
        .bind(message_ids)
        .bind(user_ids)
        .bind(created_ats)
        .execute(&self.pool)
        .await
        .context("Failed to insert message acks")?;

        Ok(())
    }

    async fn get_acks(
        &self,
        chat_id: i64,
        message_id: i64,
    ) -> Result<Vec<MessageAck>, anyhow::Error> {
        let result = sqlx::query_as::<_, MessageAckDb>(
            "
            SELECT *
            FROM message_acks
            WHERE chat_id = $1 AND message_id = $2
            ",
        )
        .bind(chat_id)
        .bind(message_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch message acks")?
        .into_iter()
        .map(MessageAck::from)
        .collect::<Vec<_>>();

        Ok(result)
    }
}
