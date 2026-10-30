use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, prelude::FromRow};

use crate::domain::{
    chats::{Chat, ChatMember, ChatPermissions, ChatType, LastMessageInfo, data::ChatRepository},
    messages::MessageType,
};

#[derive(Debug, FromRow)]
pub struct ChatMemberDb {
    chat_id: i64,
    user_id: i64,
    username: String,
    display_name: String,
    avatar: Option<String>,
    last_read_message_id: Option<i64>,
    permission_overwrites: Option<i64>,
    is_leave: bool,
}

impl From<ChatMemberDb> for ChatMember {
    fn from(value: ChatMemberDb) -> Self {
        ChatMember {
            user_id: value.user_id,
            username: value.username,
            display_name: value.display_name,
            avatar: value.avatar,
            last_read_message_id: value.last_read_message_id,
            is_leave: value.is_leave,
            permissions: value
                .permission_overwrites
                .map(ChatPermissions::from_bits_truncate),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ChatDb {
    id: i64,
    chat_type: i16,
    owner_id: Option<i64>,
    name: Option<String>,
    image: Option<String>,

    last_message_id: Option<i64>,
    last_message_content: Option<String>,
    last_message_type: Option<String>,
    last_message_author_id: Option<i64>,
    last_message_created_at: Option<DateTime<Utc>>,
    last_message_edited_at: Option<DateTime<Utc>>,

    permissions: i64,
    created_at: DateTime<Utc>,
}

impl TryFrom<ChatDb> for Chat {
    type Error = anyhow::Error;

    fn try_from(value: ChatDb) -> Result<Self, Self::Error> {
        let chat_type = match value.chat_type {
            0 => ChatType::Dm,
            1 => ChatType::GroupDm,
            _ => return Err(anyhow::anyhow!("Unknown chat_type: {}", value.chat_type)),
        };

        let last_message = match value.last_message_id {
            Some(id) => {
                let author_id = value.last_message_author_id.ok_or_else(|| {
                    anyhow::anyhow!(
                        "last_message_id is present but author_id is null for chat {}",
                        value.id
                    )
                })?;
                let message_type: MessageType =
                    serde_json::from_str(&value.last_message_type.ok_or_else(|| {
                        anyhow::anyhow!(
                            "last_message_id is present but last_message_type is null for chat {}",
                            value.id
                        )
                    })?)?;
                let last_message_created_at = value.last_message_created_at.ok_or_else(|| anyhow::anyhow!(
                    "last_message_id is present but last_message_created_at is null for chat {}", value.id
                ))?;

                Some(LastMessageInfo {
                    message_id: id,
                    content: value.last_message_content.unwrap_or_default(),
                    message_type: message_type,
                    author_id: author_id,
                    created_at: last_message_created_at,
                    edited_at: value.last_message_edited_at,
                })
            }
            None => None, // В чате еще нет сообщений
        };

        Ok(Chat {
            id: value.id,
            owner_id: value.owner_id,
            name: value.name,
            image: value.image,
            chat_type: chat_type,
            last_message: last_message,
            created_at: value.created_at,
            permissions: ChatPermissions::from_bits_truncate(value.permissions),
            members: Vec::new(),
        })
    }
}

impl TryFrom<(ChatDb, Vec<ChatMember>)> for Chat {
    type Error = anyhow::Error;

    fn try_from((chat, members): (ChatDb, Vec<ChatMember>)) -> Result<Self, Self::Error> {
        let mut chat = Chat::try_from(chat)?;
        chat.members = members;
        Ok(chat)
    }
}

pub struct PostgresChatRepository {
    pool: Pool<Postgres>,
}

impl PostgresChatRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatRepository for PostgresChatRepository {
    async fn save(&self, chat: &Chat) -> Result<(), anyhow::Error> {
        let query_chat = "
            INSERT INTO chats (
                id,
                chat_type,
                name,
                owner_id,
                image,
                permissions,
                created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        ";
        let chat_type = match chat.chat_type {
            ChatType::Dm => 0,
            ChatType::GroupDm => 1,
        };

        sqlx::query(query_chat)
            .bind(&chat.id)
            .bind(&chat_type)
            .bind(&chat.name)
            .bind(&chat.owner_id)
            .bind(&chat.image)
            .bind(chat.permissions.bits())
            .bind(&chat.created_at)
            .execute(&self.pool)
            .await
            .context("Failed to insert chat")?;

        // For DM chats also maintain mapping in private_chats
        if chat.chat_type == ChatType::Dm && !chat.members.is_empty() {
            let user_id1 = chat.members[0].user_id;
            // Self-DM has a single member, so it maps to itself
            let user_id2 = chat.members.get(1).map(|m| m.user_id).unwrap_or(user_id1);
            let (u1, u2) = if user_id1 < user_id2 {
                (user_id1, user_id2)
            } else {
                (user_id2, user_id1)
            };

            let query_private = "
                INSERT INTO private_chats (
                    user_id1,
                    user_id2,
                    chat_id
                ) VALUES ($1, $2, $3)
                ON CONFLICT (user_id1, user_id2) DO NOTHING
            ";

            sqlx::query(query_private)
                .bind(u1)
                .bind(u2)
                .bind(chat.id)
                .execute(&self.pool)
                .await
                .context("Failed to insert private chat")?;
        }

        let query_member = "
            INSERT INTO chat_members (
                chat_id,
                user_id,
                username,
                display_name,
                avatar,
                last_read_message_id,
                permission_overwrites,
                is_leave
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ";

        for member in &chat.members {
            sqlx::query(query_member)
                .bind(&chat.id)
                .bind(&member.user_id)
                .bind(&member.username)
                .bind(&member.display_name)
                .bind(&member.avatar)
                .bind(&member.last_read_message_id)
                .bind(member.permissions.as_ref().map(|p| p.bits()))
                .bind(member.is_leave)
                .execute(&self.pool)
                .await
                .context("Failed to insert into chat_members")?;
        }

        Ok(())
    }

    async fn get_by_id(&self, chat_id: i64) -> Result<Option<Chat>, anyhow::Error> {
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
        .context("Failed to fetch chat")?;

        let chat_db = match chat_db {
            Some(c) => c,
            None => return Ok(None),
        };

        let members_db =
            sqlx::query_as::<_, ChatMemberDb>("SELECT * FROM chat_members WHERE chat_id = $1")
                .bind(chat_id)
                .fetch_all(&self.pool)
                .await
                .context("Failed to fetch chat members")?;

        let members: Vec<ChatMember> = members_db
            .into_iter()
            .map(|m| ChatMember::from(m))
            .collect();

        let mut chat = Chat::try_from(chat_db)?;
        chat.members = members;
        Ok(Some(chat))
    }

    async fn get_dm_chat(
        &self,
        user_id1: i64,
        user_id2: i64,
    ) -> Result<Option<Chat>, anyhow::Error> {
        let values = if user_id1 < user_id2 {
            (user_id1, user_id2)
        } else {
            (user_id2, user_id1)
        };

        let result = sqlx::query_as::<_, (i64,)>(
            "SELECT chat_id from private_chats WHERE user_id1 = $1 AND user_id2 = $2",
        )
        .bind(values.0)
        .bind(values.1)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch from private_chats")?;

        match result {
            Some(result) => self.get_by_id(result.0).await,
            None => Ok(None),
        }
    }

    async fn get_member_ids(&self, chat_id: i64) -> Result<Vec<(i64, bool)>, anyhow::Error> {
        sqlx::query_as::<_, (i64, bool)>(
            "SELECT user_id, is_leave FROM chat_members WHERE chat_id = $1",
        )
        .bind(chat_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch member ids")
    }

    async fn get_user_chats(&self, user_id: i64) -> Result<Vec<Chat>, anyhow::Error> {
        let user_members =
            sqlx::query_as::<_, ChatMemberDb>("SELECT * FROM chat_members WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(&self.pool)
                .await
                .context("Failed to fetch chat members")?;

        let chat_ids: Vec<i64> = user_members.iter().map(|v| v.chat_id).collect();

        if chat_ids.is_empty() {
            return Ok(vec![]);
        }

        let chats_db = sqlx::query_as::<_, ChatDb>(
            "SELECT id, chat_type, owner_id, name, image,
                    last_message_id, last_message_content,
                    last_message_type::text AS last_message_type,
                    last_message_author_id, last_message_created_at, last_message_edited_at,
                    permissions, created_at
             FROM chats
             WHERE id = ANY($1)
               AND (chat_type <> 0 OR last_message_id IS NOT NULL)",
        )
        .bind(chat_ids)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch user chats")?;

        let dm_chat_ids: Vec<i64> = chats_db
            .iter()
            .filter(|c| c.chat_type == 0)
            .map(|c| c.id)
            .collect();

        let mut members_by_chat: HashMap<i64, Vec<ChatMember>> = HashMap::new();

        for chat_user in user_members {
            members_by_chat
                .entry(chat_user.chat_id)
                .or_default()
                .push(ChatMember::from(chat_user));
        }

        if !dm_chat_ids.is_empty() {
            let members_db = sqlx::query_as::<_, ChatMemberDb>(
                "SELECT * FROM chat_members WHERE chat_id = ANY($1)",
            )
            .bind(dm_chat_ids)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch chat_members")?;

            for member in members_db {
                if member.user_id == user_id {
                    continue;
                }
                members_by_chat
                    .entry(member.chat_id)
                    .or_default()
                    .push(ChatMember::from(member));
            }
        }

        let chats: Vec<Chat> = chats_db
            .into_iter()
            .map(|chat_db| {
                let members = members_by_chat.remove(&chat_db.id).unwrap_or_default();
                Chat::try_from((chat_db, members))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(chats)
    }

    async fn upsert_chat_member(
        &self,
        chat_id: i64,
        member: &ChatMember,
    ) -> Result<(), anyhow::Error> {
        let query = "
            INSERT INTO chat_members (
                chat_id,
                user_id,
                username,
                display_name,
                avatar,
                last_read_message_id,
                permission_overwrites,
                is_leave
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (chat_id, user_id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                avatar = EXCLUDED.avatar,
                last_read_message_id = EXCLUDED.last_read_message_id,
                permission_overwrites = EXCLUDED.permission_overwrites,
                is_leave = EXCLUDED.is_leave
        ";
        sqlx::query(query)
            .bind(chat_id)
            .bind(&member.user_id)
            .bind(&member.username)
            .bind(&member.display_name)
            .bind(&member.avatar)
            .bind(&member.last_read_message_id)
            .bind(member.permissions.as_ref().map(|p| p.bits()))
            .bind(member.is_leave)
            .execute(&self.pool)
            .await
            .context("Failed to insert into chat_members")?;
        Ok(())
    }

    async fn update_is_leave_status(
        &self,
        user_id: i64,
        chat_id: i64,
        is_leave: bool,
    ) -> Result<(), anyhow::Error> {
        sqlx::query("UPDATE chat_members SET is_leave = $1 WHERE user_id = $2 AND chat_id = $3")
            .bind(is_leave)
            .bind(user_id)
            .bind(chat_id)
            .execute(&self.pool)
            .await
            .context("Failed to update is_leave in chat_members")?;
        Ok(())
    }

    // async fn update_owner_id(&self, chat_id: i64, owner_id: i64) -> Result<(), anyhow::Error> {
    //     sqlx::query("UPDATE chats SET owner_id = ? WHERE chat_id = ?")
    //         .bind(owner_id)
    //         .bind(chat_id)
    //         .execute(&self.pool)
    //         .await
    //         .context("Failed to update owner_id in chats")?;
    //     Ok(())
    // }

    async fn update_last_read_message_id(
        &self,
        user_id: i64,
        chat_id: i64,
        message_id: i64,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            "UPDATE chat_members SET last_read_message_id = $1 WHERE user_id = $2 AND chat_id = $3",
        )
        .bind(message_id)
        .bind(user_id)
        .bind(chat_id)
        .execute(&self.pool)
        .await
        .context("Failed to update last_read_message_id in chat_members")?;
        Ok(())
    }
}
