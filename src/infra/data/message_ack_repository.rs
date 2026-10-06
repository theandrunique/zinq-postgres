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
    async fn upsert(&self, message_ack: &MessageAck) -> Result<(), anyhow::Error> {
        sqlx::query(
            "
            INSERT INTO message_acks (
                chat_id,
                message_id,
                user_id,
                created_at
            ) VALUES ($1, $2, $3, $4)
            ",
        )
        .bind(&message_ack.chat_id)
        .bind(&message_ack.message_id)
        .bind(&message_ack.user_id)
        .bind(&message_ack.created_at)
        .execute(&self.pool)
        .await
        .context("Failed to insert message acks")?;

        Ok(())
    }

    async fn bulk_upsert(&self, message_acks: &[MessageAck]) -> Result<(), anyhow::Error> {
        for ack in message_acks {
            self.upsert(ack).await?;
        }
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
