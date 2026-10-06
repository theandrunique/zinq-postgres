use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use sqlx::prelude::FromRow;

use crate::domain::chats::data::ChatMemberRepository;

#[derive(Debug, FromRow)]
struct ChatMemberStatus {
    chat_id: i64,
    is_leave: bool,
}

pub struct PostgresChatMemberRepository {
    pool: Pool<Postgres>
}

impl PostgresChatMemberRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
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

        let rows = sqlx::query_as::<_, (i64, bool)>("
            SELECT chat_id, is_leave
            FROM chat_users
            WHERE user_id = ? AND chat_id IN ?
            ")
            .bind(user_id)
            .bind(chat_ids)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch chat members")?
            .into_iter()
            .collect::<Vec<_>>();

        let mut result = HashMap::new();
        for row in rows {
            result.insert(row.0, row.1);
        }

        Ok(result)
    }
}
