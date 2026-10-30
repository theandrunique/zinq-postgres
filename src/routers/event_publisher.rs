use std::sync::Arc;

use async_trait::async_trait;
use socketioxide::SocketIo;

use crate::{
    domain::{
        chats::data::ChatRepository,
        events::{DomainEvent, DomainEventHandler},
    },
    infra::{
        data::{chat_repotisory, message_ack_repository},
        id_generator,
    },
    routers::schemas::GatewayEventSchema,
};

pub struct EventPublisher {
    io: SocketIo,
    chat_repository: Arc<dyn ChatRepository>,
}

impl EventPublisher {
    pub fn new(io: SocketIo, chat_repository: Arc<dyn ChatRepository>) -> Self {
        Self {
            io,
            chat_repository,
        }
    }
}

#[async_trait]
impl DomainEventHandler for EventPublisher {
    async fn handle(&self, event: &DomainEvent) -> Result<(), anyhow::Error> {
        let (gateway_event, chat_id) = match event {
            DomainEvent::MessageCreate {
                initiator_id: _,
                message,
                attachments,
            } => (
                GatewayEventSchema::MessageCreate {
                    message: (message.clone(), attachments.clone()).into(),
                },
                message.chat_id,
            ),
            DomainEvent::MessageUpdate {
                initiator_id: _,
                message,
                attachments,
            } => (
                GatewayEventSchema::MessageUpdate {
                    message: (message.clone(), attachments.clone()).into(),
                },
                message.chat_id,
            ),
            DomainEvent::MessageDelete {
                chat_id,
                message_id,
            } => (
                GatewayEventSchema::MessageDelete {
                    message_id: message_id.to_string(),
                },
                *chat_id,
            ),
            DomainEvent::ChatCreate { chat } => (
                {
                    GatewayEventSchema::ChatCreate {
                        chat: chat.clone().into(),
                    }
                },
                chat.id,
            ),
            DomainEvent::ChatMemberAdd {
                initiator_id: _,
                chat_id,
                member,
            } => (
                GatewayEventSchema::ChatMemberAdd {
                    chat_id: chat_id.to_string(),
                    member: member.clone().into(),
                },
                *chat_id,
            ),
            DomainEvent::ChatMemberRemove {
                initiator_id: _,
                chat_id,
                member,
            } => (
                GatewayEventSchema::ChatMemberRemove {
                    chat_id: chat_id.to_string(),
                    member: member.clone().into(),
                },
                *chat_id,
            ),
            DomainEvent::MessageAck {
                user_id,
                chat_id,
                last_read_message_id,
            } => (
                GatewayEventSchema::MessageAck {
                    user_id: user_id.to_string(),
                    chat_id: chat_id.to_string(),
                    last_read_message_id: last_read_message_id.to_string(),
                },
                *chat_id,
            ),
            _ => return Ok(()),
        };

        let recipients = self
            .chat_repository
            .get_member_ids(chat_id)
            .await?
            .into_iter()
            .filter(|(_, is_leave)| !*is_leave)
            .map(|(user_id, _)| user_id)
            .collect::<Vec<_>>();

        for user_id in recipients {
            if let Err(e) = self
                .io
                .to(format!("user:{}", user_id))
                .emit("event", &gateway_event)
                .await
            {
                tracing::error!(error = ?e, user_id, "Failed to emit event to user");
            }
            tracing::info!(
                event_type = serde_json::to_string(&gateway_event)?,
                "Event published"
            );
        }

        Ok(())
    }
}
