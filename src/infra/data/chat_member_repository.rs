use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::prelude::FromRow;

use crate::domain::chats::data::ChatMemberRepository;

#[derive(Debug, FromRow)]
struct ChatMemberStatus {
    chat_id: i64,
    is_leave: bool,
}

pub struct PostgresChatMemberRepository { }

impl PostgresChatMemberRepository {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl ChatMemberRepository for PostgresChatMemberRepository {
    async fn get_chat_ids_for_user(
        &self,
        user_id: i64,
        chat_ids: &[i64],
    ) -> Result<HashMap<i64, bool>, anyhow::Error> {
        if chat_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let query = "
            SELECT chat_id, is_leave
            FROM chat_users_by_user_id
            WHERE user_id = ? AND chat_id IN ?
        ";

        let rows: Vec<ChatMemberStatus> = self.common.exec_all(query, (user_id, chat_ids)).await?;

        let mut result = HashMap::new();
        for row in rows {
            result.insert(row.chat_id, row.is_leave);
        }

        Ok(result)
    }
}
