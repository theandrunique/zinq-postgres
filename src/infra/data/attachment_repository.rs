use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, QueryBuilder, prelude::FromRow};

use crate::domain::attachments::{Attachment, data::AttachmentRepository};

#[derive(Debug, FromRow)]
struct AttachmentDb {
    id: i64,
    message_id: i64,
    chat_id: i64,

    storage_key: String,
    size: i64,
    filename: String,
    content_type: String,

    duration_secs: Option<f32>,
    is_spoiler: bool,
    placeholder: Option<String>,
    waveform: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<AttachmentDb> for Attachment {
    fn from(value: AttachmentDb) -> Self {
        Attachment {
            id: value.id,
            message_id: value.message_id,
            chat_id: value.chat_id,

            content_type: value.content_type,
            duration_secs: value.duration_secs,
            filename: value.filename,
            size: value.size,
            storage_key: value.storage_key,
            placeholder: value.placeholder,
            waveform: value.waveform,
            is_spoiler: value.is_spoiler,
            created_at: value.created_at,
        }
    }
}

pub struct PostgresAttachmentRepository {
    pool: Pool<Postgres>,
}

impl PostgresAttachmentRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AttachmentRepository for PostgresAttachmentRepository {
    async fn save(&self, attachment: &Attachment) -> Result<(), anyhow::Error> {
        sqlx::query(
            "
            INSERT INTO attachments (
                id,
                message_id,
                chat_id,
                content_type,
                duration_secs,
                filename,
                is_spoiler,
                placeholder,
                storage_key,
                size,
                waveform,
                created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ",
        )
        .bind(&attachment.id)
        .bind(&attachment.message_id)
        .bind(&attachment.chat_id)
        .bind(&attachment.content_type)
        .bind(&attachment.duration_secs)
        .bind(&attachment.filename)
        .bind(&attachment.is_spoiler)
        .bind(&attachment.placeholder)
        .bind(&attachment.storage_key)
        .bind(&attachment.size)
        .bind(&attachment.waveform)
        .bind(&attachment.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn bulk_save(&self, attachments: &[Attachment]) -> Result<(), anyhow::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "
            INSERT INTO attachments (
                id,
                message_id,
                chat_id,
                content_type,
                duration_secs,
                filename,
                is_spoiler,
                placeholder,
                storage_key,
                size,
                waveform,
                created_at
            ) ",
        );
        query_builder.push_values(attachments, |mut b, attachment| {
            b.push_bind(&attachment.id)
                .push_bind(&attachment.message_id)
                .push_bind(&attachment.chat_id)
                .push_bind(&attachment.content_type)
                .push_bind(&attachment.duration_secs)
                .push_bind(&attachment.filename)
                .push_bind(&attachment.is_spoiler)
                .push_bind(&attachment.placeholder)
                .push_bind(&attachment.storage_key)
                .push_bind(&attachment.size)
                .push_bind(&attachment.waveform)
                .push_bind(&attachment.created_at);
        });

        let query = query_builder.build();
        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn get_by_id(&self, attachment_id: i64) -> Result<Option<Attachment>, anyhow::Error> {
        let attachment =
            sqlx::query_as::<_, AttachmentDb>("SELECT * FROM attachments WHERE id = $1")
                .bind(attachment_id)
                .fetch_optional(&self.pool)
                .await
                .context("Failed to fetch attachment by id")?
                .map(Attachment::from);

        return Ok(attachment);
    }

    async fn get_chat_attachments(
        &self,
        chat_id: i64,
        before_id: i64,
        limit: i32,
    ) -> Result<Vec<Attachment>, anyhow::Error> {
        let attachments = sqlx::query_as::<_, AttachmentDb>(
            "
            SELECT * FROM attachments
            WHERE chat_id = $1 AND id < $2
            LIMIT $3",
        )
        .bind(chat_id)
        .bind(before_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch chat attachment by chat_id")?
        .into_iter()
        .map(Attachment::from)
        .collect::<Vec<_>>();

        return Ok(attachments);
    }

    async fn get_by_message_ids(
        &self,
        chat_id: i64,
        message_ids: &[i64],
    ) -> Result<Vec<Attachment>, anyhow::Error> {
        let attachments = sqlx::query_as::<_, AttachmentDb>(
            "
            SELECT * FROM attachments
            WHERE chat_id = $1 AND message_id = ANY($2)",
        )
        .bind(chat_id)
        .bind(message_ids)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch attachment by chat_id and message_ids")?
        .into_iter()
        .map(Attachment::from)
        .collect::<Vec<_>>();

        return Ok(attachments);
    }
}
