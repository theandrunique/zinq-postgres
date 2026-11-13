use std::sync::Arc;

use crate::{
    application::RequestHandler,
    domain::chats::data::{ChatPageCursor, ChatRepository, UserChatsPage},
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

pub struct GetUserChatsQueryHandler {
    chat_repository: Arc<dyn ChatRepository>,
}

impl GetUserChatsQueryHandler {
    pub fn new(state: &AppState) -> Self {
        Self {
            chat_repository: Arc::clone(&state.chat_repository),
        }
    }
}

impl RequestHandler for GetUserChatsQueryHandler {
    type Request = GetUserChatsQuery;
    type Output = UserChatsPage;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let limit = request.limit.clamp(1, MAX_USER_CHATS_LIMIT);

        let chats = self
            .chat_repository
            .get_user_chats(request.current_user_id, request.cursor, limit)
            .await
            .map_err(Error::InternalServerError)?;

        Ok(chats)
    }
}
