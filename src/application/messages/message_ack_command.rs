use std::{cmp, collections::BTreeMap, sync::Arc};

use chrono::{DateTime, Utc};

use crate::{
    application::RequestHandler,
    domain::{
        chats::data::{ChatLoadOptions, ChatLoader, ChatRepository},
        events::{DomainEvent, Mediator},
        message_acks::{MessageAck, data::MessageAckRepository},
        messages::data::MessageRepository,
    },
    error::Error,
    infra::IdGenerator,
    state::AppState,
};

/// Upper bound of acks that are synthesized for a single read pointer jump.
/// The pointer itself always advances, only the per-message history is capped.
const ACK_FILL_LIMIT: i32 = 1000;

pub struct MessageAckCommand {
    pub current_user_id: i64,
    pub chat_id: i64,
    /// Moves the read pointer forward. `None` means "only store individual
    /// acks", which is what scrolling through the chat sends.
    pub last_read_message_id: Option<i64>,
    pub acks: Vec<MessageAckInput>,
}

pub struct MessageAckInput {
    pub message_id: i64,
    /// When the message was seen. Falls back to the server time when omitted.
    pub acked_at: Option<DateTime<Utc>>,
}

pub struct MessageAckCommandHandler {
    chat_loader: Arc<dyn ChatLoader>,
    chat_repository: Arc<dyn ChatRepository>,
    message_ack_repository: Arc<dyn MessageAckRepository>,
    message_repository: Arc<dyn MessageRepository>,
    id_gen: Arc<dyn IdGenerator>,
    mediator: Arc<Mediator>,
}

impl MessageAckCommandHandler {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            chat_loader: Arc::clone(&app_state.chat_loader),
            chat_repository: Arc::clone(&app_state.chat_repository),
            message_ack_repository: Arc::clone(&app_state.message_ack_repository),
            message_repository: Arc::clone(&app_state.message_repository),
            id_gen: Arc::clone(&app_state.id_gen),
            mediator: Arc::clone(&app_state.mediator),
        }
    }
}

impl RequestHandler for MessageAckCommandHandler {
    type Request = MessageAckCommand;
    type Output = ();
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

        let member = chat
            .get_member(request.current_user_id)
            .ok_or(Error::UserNotMember {
                user_id: request.current_user_id,
                chat_id: request.chat_id,
            })?;

        let now = Utc::now();
        let current_last_read = member.last_read_message_id.unwrap_or(0);

        // Client clocks can be behind or ahead of the server, so timestamps are
        // clamped to the moment the request is processed.
        let explicit: BTreeMap<i64, DateTime<Utc>> =
            request
                .acks
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, ack| {
                    let acked_at = ack.acked_at.filter(|at| *at <= now).unwrap_or(now);

                    acc.entry(ack.message_id)
                        .and_modify(|existing| *existing = (*existing).min(acked_at))
                        .or_insert(acked_at);

                    acc
                });

        let advance_to = request
            .last_read_message_id
            .filter(|last_read| *last_read > current_last_read);

        let mut acks: Vec<MessageAck> = explicit
            .iter()
            .map(|(message_id, acked_at)| MessageAck {
                chat_id: request.chat_id,
                message_id: *message_id,
                user_id: request.current_user_id,
                created_at: *acked_at,
            })
            .collect();

        if let Some(last_read_message_id) = advance_to {
            // Jumping the pointer means every message up to it is read, so the
            // missing per-message acks are synthesized from the oldest moment
            // the next seen message was acknowledged.
            let min_id = self.id_gen.min_id_for_days_ago(7);
            let range_start = cmp::max(current_last_read + 1, min_id);

            if range_start <= last_read_message_id {
                let message_ids = self
                    .message_repository
                    .get_message_ids_in_range(
                        request.chat_id,
                        range_start,
                        last_read_message_id,
                        ACK_FILL_LIMIT,
                    )
                    .await
                    .map_err(Error::InternalServerError)?;

                for message_id in message_ids {
                    if explicit.contains_key(&message_id) {
                        continue;
                    }

                    let created_at = explicit
                        .range(message_id..)
                        .next()
                        .map(|(_, acked_at)| *acked_at)
                        .unwrap_or(now);

                    acks.push(MessageAck {
                        chat_id: request.chat_id,
                        message_id,
                        user_id: request.current_user_id,
                        created_at,
                    });
                }
            }
        }

        if !acks.is_empty() {
            self.message_ack_repository.bulk_upsert(&acks).await?;
        }

        if let Some(last_read_message_id) = advance_to {
            self.chat_repository
                .update_last_read_message_id(
                    request.current_user_id,
                    request.chat_id,
                    last_read_message_id,
                )
                .await?;

            self.mediator
                .publish(&DomainEvent::MessageAck {
                    user_id: request.current_user_id,
                    chat_id: request.chat_id,
                    last_read_message_id,
                })
                .await?;
        }

        Ok(())
    }
}
