use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres};

use crate::domain::chats::data::{ChatLoadOptions, ChatLoader};
use crate::domain::chats::{Chat, ChatMember, ChatPermissions, ChatType};
use crate::domain::messages::MessageType;
use crate::infra::data::chat_repotisory::{ChatDb, ChatMemberDb};

pub struct PostgresChatLoader {
    pool: Pool<Postgres>,
}

impl PostgresChatLoader {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatLoader for PostgresChatLoader {
    async fn load(&self, options: ChatLoadOptions) -> Result<Option<Chat>, anyhow::Error> {
        let chat_id = options
            .chat_id
            .ok_or_else(|| anyhow::anyhow!("chat_id is required"))?;

        let chat_db = sqlx::query_as::<_, ChatDb>(
            "SELECT id, chat_type, owner_id, name, image,
                    last_message_id, last_message_content,
                    last_message_type::text AS last_message_type,
                    last_message_author_id, last_message_created_at, last_message_edited_at,
                    permissions, created_at
             FROM chats WHERE id = $1",
        )
            .bind(chat_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch chat by id")?;

        let chat_db = match chat_db {
            Some(c) => c,
            None => return Ok(None),
        };

        let members: Vec<ChatMember> = if options.member_ids.is_empty() {
            Vec::new()
        } else {
            let members_db: Vec<ChatMember> = sqlx::query_as::<_, ChatMemberDb>(
                "SELECT * FROM chat_members WHERE chat_id = $1 AND user_id = ANY($2)",
            )
            .bind(chat_id)
            .bind(options.member_ids)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch chat members")?
            .into_iter()
            .map(ChatMember::from)
            .collect::<Vec<_>>();

            members_db
        };

        let mut chat = Chat::try_from(chat_db)?;
        chat.members = members;
        Ok(Some(chat))
    }
}
