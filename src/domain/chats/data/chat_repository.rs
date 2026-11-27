use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::chats::{Chat, ChatMember};

/// Opaque position inside the user's chat list.
///
/// Chats are ordered by `last_message_created_at DESC NULLS LAST, id DESC`,
/// so the cursor has to remember which group the last returned chat belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatPageCursor {
    /// Last returned chat had at least one message.
    Dated {
        last_message_at: DateTime<Utc>,
        chat_id: i64,
    },
    /// Last returned chat had no messages and lives in the tail of the list.
    WithoutMessages { chat_id: i64 },
}

#[derive(Debug, Default)]
pub struct UserChatsPage {
    pub chats: Vec<Chat>,
    pub next_cursor: Option<ChatPageCursor>,
}

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn save(&self, chat: &Chat) -> Result<(), anyhow::Error>;
    async fn get_by_id(&self, chat_id: i64) -> Result<Option<Chat>, anyhow::Error>;
    async fn get_member_ids(&self, chat_id: i64) -> Result<Vec<(i64, bool)>, anyhow::Error>;
    async fn upsert_chat_member(
        &self,
        chat_id: i64,
        member: &ChatMember,
    ) -> Result<(), anyhow::Error>;
    async fn update_is_leave_status(
        &self,
        user_id: i64,
        chat_id: i64,
        is_leave: bool,
    ) -> Result<(), anyhow::Error>;
    async fn update_last_read_message_id(
        &self,
        user_id: i64,
        chat_id: i64,
        message_id: i64,
    ) -> Result<(), anyhow::Error>;

    async fn get_dm_chat(
        &self,
        user_id1: i64,
        user_id2: i64,
    ) -> Result<Option<Chat>, anyhow::Error>;

    async fn get_user_chats(
        &self,
        user_id: i64,
        after: Option<ChatPageCursor>,
        limit: i32,
    ) -> Result<UserChatsPage, anyhow::Error>;
}
