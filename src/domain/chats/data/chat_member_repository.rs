use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::chats::ChatMember;

#[async_trait]
pub trait ChatMemberRepository: Send + Sync {
    /// Loads `chat_members` records of a single chat by `user_ids`, keyed by
    /// `user_id`. Missing ids are omitted, and `is_leave` is intentionally not
    /// filtered: profiles are still needed for users who have left the chat.
    async fn get_members_by_ids(
        &self,
        chat_id: i64,
        user_ids: &[i64],
    ) -> Result<HashMap<i64, ChatMember>, anyhow::Error>;

    /// Loads `chat_members` records for `(chat_id, user_id)` pairs in one query,
    /// keyed by `user_id`. `is_leave` is intentionally not filtered.
    async fn get_members_for_chats(
        &self,
        pairs: &[(i64, i64)],
    ) -> Result<HashMap<i64, ChatMember>, anyhow::Error>;
}
