use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, prelude::FromRow};

use crate::{
    domain::attachments::{Attachment, data::AttachmentRepository},
};

#[derive(Debug, FromRow)]
struct AttachmentDb {
    id: i64,
    message_id: i64,
    chat_id: i64,

    content_type: String,
    duration_secs: Option<f32>,
    filename: String,
    is_spoiler: bool,
    placeholder: Option<String>,
    storage_key: String,
    size: i64,
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
        let query = "
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
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";

        sqlx::query(query)
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
}
