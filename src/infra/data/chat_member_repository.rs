use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres};

use crate::domain::chats::ChatMember;
use crate::domain::chats::data::ChatMemberRepository;
use crate::infra::data::chat_repotisory::ChatMemberDb;

pub struct PostgresChatMemberRepository {
    pool: Pool<Postgres>,
}

impl PostgresChatMemberRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatMemberRepository for PostgresChatMemberRepository {
    async fn get_members_by_ids(
        &self,
        chat_id: i64,
        user_ids: &[i64],
    ) -> Result<HashMap<i64, ChatMember>, anyhow::Error> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let members = sqlx::query_as::<_, ChatMemberDb>(
            "SELECT * FROM chat_members WHERE chat_id = $1 AND user_id = ANY($2)",
        )
        .bind(chat_id)
        .bind(user_ids)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch chat members by ids")?
        .into_iter()
        .map(ChatMember::from)
        .map(|member| (member.user_id, member))
        .collect::<HashMap<_, _>>();

        Ok(members)
    }

    async fn get_members_for_chats(
        &self,
        pairs: &[(i64, i64)],
    ) -> Result<HashMap<i64, ChatMember>, anyhow::Error> {
        if pairs.is_empty() {
            return Ok(HashMap::new());
        }

        let chat_ids: Vec<i64> = pairs.iter().map(|(chat_id, _)| *chat_id).collect();
        let user_ids: Vec<i64> = pairs.iter().map(|(_, user_id)| *user_id).collect();

        let members = sqlx::query_as::<_, ChatMemberDb>(
            "
            SELECT cm.*
            FROM UNNEST($1::bigint[], $2::bigint[]) AS pair(chat_id, user_id)
            JOIN chat_members cm
                ON cm.chat_id = pair.chat_id AND cm.user_id = pair.user_id
            ",
        )
        .bind(chat_ids)
        .bind(user_ids)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch chat members for chats")?
        .into_iter()
        .map(ChatMember::from)
        .map(|member| (member.user_id, member))
        .collect::<HashMap<_, _>>();

        Ok(members)
    }
}
