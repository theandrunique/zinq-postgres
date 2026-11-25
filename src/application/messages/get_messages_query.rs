use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::{
    application::RequestHandler,
    domain::{
        attachments::{Attachment, data::AttachmentRepository},
        chats::{
            ChatMember,
            data::{ChatLoadOptions, ChatLoader, ChatMemberRepository},
        },
        messages::{Message, data::MessageRepository},
    },
    error::Error,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct GetMessagesQuery {
    pub current_user_id: i64,
    pub chat_id: i64,
    pub before: Option<i64>,
    pub limit: i32,
}

#[derive(Debug, Clone)]
pub struct GetMessagesQueryResult {
    pub messages: Vec<Message>,
    pub attachments: HashMap<i64, Vec<Attachment>>,
    pub members: HashMap<i64, ChatMember>,
}

pub struct GetMessagesQueryHandler {
    chat_loader: Arc<dyn ChatLoader>,
    chat_member_repository: Arc<dyn ChatMemberRepository>,
    message_repository: Arc<dyn MessageRepository>,
    attachment_repository: Arc<dyn AttachmentRepository>,
}

impl GetMessagesQueryHandler {
    pub fn new(state: &AppState) -> Self {
        Self {
            chat_loader: Arc::clone(&state.chat_loader),
            chat_member_repository: Arc::clone(&state.chat_member_repository),
            message_repository: Arc::clone(&state.message_repository),
            attachment_repository: Arc::clone(&state.attachment_repository),
        }
    }
}

impl RequestHandler for GetMessagesQueryHandler {
    type Request = GetMessagesQuery;
    type Output = GetMessagesQueryResult;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let chat = self
            .chat_loader
            .load(
                ChatLoadOptions::default()
                    .with_chat_id(request.chat_id)
                    .with_member(request.current_user_id),
            )
            .await
            .map_err(Error::InternalServerError)?
            .ok_or(Error::ChatNotFound(request.chat_id))?;

        if !chat.has_member(request.current_user_id) {
            return Err(Error::UserNotMember {
                user_id: request.current_user_id,
                chat_id: request.chat_id,
            });
        }

        let before = request.before.unwrap_or(i64::MAX);

        let messages = self
            .message_repository
            .get_messages(request.chat_id, before, request.limit)
            .await
            .map_err(Error::InternalServerError)?;

        if messages.is_empty() {
            return Ok(GetMessagesQueryResult {
                messages,
                attachments: HashMap::new(),
                members: HashMap::new(),
            });
        }

        let message_ids: Vec<i64> = messages.iter().map(|m| m.id).collect();
        let member_ids: Vec<i64> = messages
            .iter()
            .flat_map(|message| {
                [
                    Some(message.author_id),
                    message.message_type.subject_user_id(),
                ]
                .into_iter()
                .flatten()
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let (attachments, members) = tokio::try_join!(
            async {
                self.attachment_repository
                    .get_by_message_ids(request.chat_id, &message_ids)
                    .await
            },
            async {
                self.chat_member_repository
                    .get_members_by_ids(request.chat_id, &member_ids)
                    .await
            },
        )
        .map_err(Error::InternalServerError)?;

        let mut attachments_by_message: HashMap<i64, Vec<Attachment>> = HashMap::new();
        for attachment in attachments {
            attachments_by_message
                .entry(attachment.message_id)
                .or_default()
                .push(attachment);
        }

        Ok(GetMessagesQueryResult {
            messages,
            attachments: attachments_by_message,
            members,
        })
    }
}
