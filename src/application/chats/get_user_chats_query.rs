use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    application::RequestHandler,
    domain::chats::{
        Chat, ChatMember,
        data::{ChatMemberRepository, ChatPageCursor, ChatRepository},
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
    pub referenced_members: HashMap<i64, ChatMember>,
}

pub struct GetUserChatsQueryHandler {
    chat_repository: Arc<dyn ChatRepository>,
    chat_member_repository: Arc<dyn ChatMemberRepository>,
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

        let referenced_pairs: Vec<(i64, i64)> = page
            .chats
            .iter()
            .flat_map(|chat| {
                chat.member_ids_for_last_message()
                    .into_iter()
                    .map(move |user_id| (chat.id, user_id))
            })
            .collect();

        let referenced_members = self
            .chat_member_repository
            .get_members_for_chats(&referenced_pairs)
            .await
            .map_err(Error::InternalServerError)?;

        Ok(GetUserChatsQueryResult {
            chats: page.chats,
            next_cursor: page.next_cursor,
            referenced_members,
        })
    }
}
