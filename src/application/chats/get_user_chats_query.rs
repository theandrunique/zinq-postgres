use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    application::{RequestHandler, chats::references::load_referenced_members},
    domain::chats::{
        Chat, ChatMember,
        data::{ChatPageCursor, ChatRepository},
    },
    error::Error,
    state::AppState,
};

pub const DEFAULT_USER_CHATS_LIMIT: i32 = 20;
pub const MAX_USER_CHATS_LIMIT: i32 = 100;

#[derive(Debug, Clone)]
pub struct GetUserChatsQuery {
    pub current_user_id: i64,
    pub cursor: Option<ChatPageCursor>,
    pub limit: i32,
}

#[derive(Debug, Clone)]
pub struct GetUserChatsQueryResult {
    pub chats: Vec<Chat>,
    pub next_cursor: Option<ChatPageCursor>,
    /// Профили, на которые ссылаются последние сообщения чатов, но которых нет
    /// среди активных участников (вышедшие авторы и участники системных
    /// сообщений).
    pub referenced_members: HashMap<i64, ChatMember>,
}

pub struct GetUserChatsQueryHandler {
    chat_repository: Arc<dyn ChatRepository>,
    chat_member_repository: Arc<dyn crate::domain::chats::data::ChatMemberRepository>,
}

impl GetUserChatsQueryHandler {
    pub fn new(state: &AppState) -> Self {
        Self {
            chat_repository: Arc::clone(&state.chat_repository),
            chat_member_repository: Arc::clone(&state.chat_member_repository),
        }
    }
}

impl RequestHandler for GetUserChatsQueryHandler {
    type Request = GetUserChatsQuery;
    type Output = GetUserChatsQueryResult;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let limit = request.limit.clamp(1, MAX_USER_CHATS_LIMIT);

        let page = self
            .chat_repository
            .get_user_chats(request.current_user_id, request.cursor, limit)
            .await
            .map_err(Error::InternalServerError)?;

        let referenced_members =
            load_referenced_members(self.chat_member_repository.as_ref(), &page.chats)
                .await
                .map_err(Error::InternalServerError)?;

        Ok(GetUserChatsQueryResult {
            chats: page.chats,
            next_cursor: page.next_cursor,
            referenced_members,
        })
    }
}
