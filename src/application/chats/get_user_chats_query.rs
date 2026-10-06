use std::sync::Arc;

use crate::{
    application::RequestHandler, domain::chats::{Chat, ChatInfo, ChatType, data::ChatRepository}, error::Error, state::AppState,
};

#[derive(Debug, Clone)]
pub struct GetUserChatsQuery {
    pub current_user_id: i64,
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
    type Output = Vec<ChatInfo>;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let chats = self
            .chat_repository
            .get_user_chat_infos(request.current_user_id)
            .await
            .map_err(Error::InternalServerError)?;

        Ok(chats)
    }
}
