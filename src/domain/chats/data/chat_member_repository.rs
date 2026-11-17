use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::chats::ChatMember;

#[async_trait]
pub trait ChatMemberRepository: Send + Sync {
    async fn get_chat_ids_for_user(
        &self,
        user_id: i64,
        chat_ids: &[i64],
    ) -> Result<HashMap<i64, bool>, anyhow::Error>;

    /// Загружает профили участников чата по их идентификаторам.
    ///
    /// Умышленно **не** фильтрует `is_leave`: профиль нужен и для авторов
    /// сообщений, и для участников системных сообщений, которые уже вышли из
    /// чата.
    async fn get_members(
        &self,
        chat_id: i64,
        user_ids: &[i64],
    ) -> Result<HashMap<i64, ChatMember>, anyhow::Error>;
}
