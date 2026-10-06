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

impl ChatType {
    pub fn as_i16(&self) -> i16 {
        match self {
            ChatType::Dm => 1,
            ChatType::GroupDm => 2,
        }
    }

    pub fn from_i16(i: i16) -> Option<Self> {
        match i {
            1 => Some(ChatType::Dm),
            2 => Some(ChatType::GroupDm),
            _ => None,
        }
    }
}

#[derive(Debug, FromRow)]
struct ChatMemberDb {
    chat_id: i64,
    user_id: i64,
    last_read_message_id: Option<i64>,
    permission_overwrites: Option<i64>,
    is_leave: bool,
}

impl From<ChatMemberDb> for ChatMember {
    fn from(value: ChatMemberDb) -> Self {
        ChatMember {
            user_id: value.user_id,
            last_read_message_id: value.last_read_message_id,
            is_leave: value.is_leave,
            permissions: value
                .permission_overwrites
                .map(ChatPermissions::from_bits_truncate),
        }
    }
}

#[derive(Debug, FromRow)]
struct ChatDb {
    id: i64,
    chat_type: i16,
    name: Option<String>,
    owner_id: Option<i64>,
    image: Option<String>,
    last_message_id: Option<i64>,
    last_message_created_at: DateTime<Utc>,
    last_message_edited_at: DateTime<Utc>,
    last_message_content: String,
    last_message_type: String,
    last_message_author_id: i64,
    permissions: i64,
    created_at: DateTime<Utc>,
}

impl TryFrom<ChatDb> for Chat {
    type Error = anyhow::Error;

    fn try_from(value: ChatDb) -> Result<Self, Self::Error> {
        let chat_type = ChatType::from_i16(value.chat_type)
            .ok_or_else(|| anyhow::anyhow!("Unknown chat_type value: {}", value.chat_type))?;

        Ok(Chat {
            id: value.id,
            owner_id: value.owner_id,
            name: value.name,
            image: value.image,
            chat_type: chat_type,
            last_message_id: value.last_message_id,
            created_at: value.created_at,
            permissions: ChatPermissions::from_bits_truncate(value.permissions),
            members: Vec::new(),
        })
    }
}

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

        let chat_db = sqlx::query_as::<_, ChatDb>("SELECT * FROM chats WHERE chat_id = $1")
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
                "SELECT * FROM chat_users WHERE chat_id = $1 AND user_id IN $2",
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
