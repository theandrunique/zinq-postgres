use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    application::RequestHandler,
    domain::chats::{
        Chat, ChatMember,
        data::{ChatMemberRepository, ChatRepository},
    },
    error::Error,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct GetChatQuery {
    pub current_user_id: i64,
    pub chat_id: i64,
}

#[derive(Debug, Clone)]
pub struct ChatWithReferences {
    pub chat: Chat,
    pub referenced_members: HashMap<i64, ChatMember>,
}

pub struct GetChatQueryHandler {
    chat_repository: Arc<dyn ChatRepository>,
    chat_member_repository: Arc<dyn ChatMemberRepository>,
}

impl GetChatQueryHandler {
    pub fn new(state: &AppState) -> Self {
        Self {
            chat_repository: Arc::clone(&state.chat_repository),
            chat_member_repository: Arc::clone(&state.chat_member_repository),
        }
    }
}

impl RequestHandler for GetChatQueryHandler {
    type Request = GetChatQuery;
    type Output = ChatWithReferences;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let chat = self
            .chat_repository
            .get_by_id(request.chat_id)
            .await
            .map_err(Error::InternalServerError)?
            .ok_or(Error::ChatNotFound(request.chat_id))?;

        if !chat.has_member(request.current_user_id) {
            return Err(Error::UserNotMember {
                user_id: request.current_user_id,
                chat_id: request.chat_id,
            });
        }

        let referenced_members = self
            .chat_member_repository
            .get_members_by_ids(chat.id, &chat.member_ids_for_last_message())
            .await
            .map_err(Error::InternalServerError)?;

        Ok(ChatWithReferences {
            chat,
            referenced_members,
        })
    }
}
