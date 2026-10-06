use async_trait::async_trait;

use crate::domain::chats::{Chat, ChatInfo, ChatMember};

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

    async fn get_dm_chat_info(
        &self,
        user_id1: i64,
        user_id2: i64,
    ) -> Result<Option<ChatInfo>, anyhow::Error>;
    async fn get_info_by_id(&self, chat_id: i64) -> Result<Option<ChatInfo>, anyhow::Error>;
    async fn get_user_chat_infos(&self, user_id: i64) -> Result<Vec<ChatInfo>, anyhow::Error>;
}
