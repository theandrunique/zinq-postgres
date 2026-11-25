use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    application::{
        RequestHandler,
        messages::{MessageAckCommand, MessageAckCommandHandler, MessageAckInput},
    },
    error::Error,
    infra::AuthUser,
    state::AppState,
};

#[serde_as]
#[derive(Deserialize)]
pub struct AckMessagesRequest {
    /// Advances the read pointer. Omitted when only individual messages are
    /// acknowledged while scrolling.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub last_read_message_id: Option<i64>,
    #[serde(default)]
    pub acks: Vec<AckInputRequest>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct AckInputRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub message_id: i64,
    /// When the message was actually seen. Required: a receipt without the
    /// moment it happened is of no use.
    pub acked_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ChatIdParam {
    chat_id: i64,
}

#[axum::debug_handler]
pub async fn ack_messages(
    State(state): State<AppState>,
    AuthUser { claims }: AuthUser,
    Path(ChatIdParam { chat_id }): Path<ChatIdParam>,
    Json(body): Json<AckMessagesRequest>,
) -> Result<(), Error> {
    let handler = MessageAckCommandHandler::new(&state);

    let command = MessageAckCommand {
        current_user_id: claims.sub,
        chat_id,
        last_read_message_id: body.last_read_message_id,
        acks: body
            .acks
            .into_iter()
            .map(|a| MessageAckInput {
                message_id: a.message_id,
                acked_at: a.acked_at,
            })
            .collect(),
    };

    handler.handle(command).await?;

    Ok(())
}
