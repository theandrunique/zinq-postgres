use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::DateTime;
use serde::Deserialize;

use crate::{
    application::{
        RequestHandler,
        chats::{DEFAULT_USER_CHATS_LIMIT, GetUserChatsQuery, GetUserChatsQueryHandler},
    },
    domain::chats::data::ChatPageCursor,
    error::Error,
    infra::AuthUser,
    routers::schemas::common::{ChatSchema, UserChatsPageSchema},
    state::AppState,
};

#[derive(Deserialize)]
pub struct GetUserChatsQueryParams {
    cursor: Option<String>,
    limit: Option<i32>,
}

fn invalid_cursor() -> Error {
    Error::InvalidRequestBody(HashMap::from([(
        "cursor".to_string(),
        vec!["Invalid cursor".to_string()],
    )]))
}

fn encode_cursor(cursor: ChatPageCursor) -> String {
    let raw = match cursor {
        ChatPageCursor::Dated {
            last_message_at,
            chat_id,
        } => format!("d:{}:{}", last_message_at.timestamp_micros(), chat_id),
        ChatPageCursor::WithoutMessages { chat_id } => format!("e:{}", chat_id),
    };

    URL_SAFE_NO_PAD.encode(raw)
}

fn decode_cursor(cursor: &str) -> Result<ChatPageCursor, Error> {
    let decoded = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| invalid_cursor())?;
    let raw = String::from_utf8(decoded).map_err(|_| invalid_cursor())?;

    let mut parts = raw.split(':');

    match parts.next() {
        Some("d") => {
            let micros = parts
                .next()
                .ok_or_else(invalid_cursor)?
                .parse::<i64>()
                .map_err(|_| invalid_cursor())?;
            let chat_id = parts
                .next()
                .ok_or_else(invalid_cursor)?
                .parse::<i64>()
                .map_err(|_| invalid_cursor())?;

            if parts.next().is_some() {
                return Err(invalid_cursor());
            }

            let last_message_at =
                DateTime::from_timestamp_micros(micros).ok_or_else(invalid_cursor)?;

            Ok(ChatPageCursor::Dated {
                last_message_at,
                chat_id,
            })
        }
        Some("e") => {
            let chat_id = parts
                .next()
                .ok_or_else(invalid_cursor)?
                .parse::<i64>()
                .map_err(|_| invalid_cursor())?;

            if parts.next().is_some() {
                return Err(invalid_cursor());
            }

            Ok(ChatPageCursor::WithoutMessages { chat_id })
        }
        _ => Err(invalid_cursor()),
    }
}

#[axum::debug_handler]
pub async fn get_user_chats(
    State(state): State<AppState>,
    AuthUser { claims }: AuthUser,
    Query(params): Query<GetUserChatsQueryParams>,
) -> Result<Json<UserChatsPageSchema>, Error> {
    let cursor = params.cursor.as_deref().map(decode_cursor).transpose()?;

    let handler = GetUserChatsQueryHandler::new(&state);

    let command = GetUserChatsQuery {
        current_user_id: claims.sub,
        cursor,
        limit: params.limit.unwrap_or(DEFAULT_USER_CHATS_LIMIT),
    };

    let result = handler.handle(command).await?;

    let next_cursor = result.next_cursor.map(encode_cursor);
    let has_more = next_cursor.is_some();

    let chats = result
        .chats
        .into_iter()
        .map(|chat| ChatSchema::from_chat_for_user(chat, claims.sub))
        .collect::<Vec<ChatSchema>>();

    Ok(Json(UserChatsPageSchema {
        chats,
        next_cursor,
        has_more,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_roundtrip_dated() {
        let cursor = ChatPageCursor::Dated {
            last_message_at: DateTime::from_timestamp_micros(1_700_000_000_123_456).unwrap(),
            chat_id: 42,
        };

        assert_eq!(decode_cursor(&encode_cursor(cursor)).unwrap(), cursor);
    }

    #[test]
    fn cursor_roundtrip_without_messages() {
        let cursor = ChatPageCursor::WithoutMessages { chat_id: 7 };

        assert_eq!(decode_cursor(&encode_cursor(cursor)).unwrap(), cursor);
    }

    #[test]
    fn rejects_invalid_cursors() {
        for raw in ["", "not-base64!!!", "abc", "e:not-a-number", "e:1:2", "x:1"] {
            assert!(matches!(
                decode_cursor(raw),
                Err(Error::InvalidRequestBody(_))
            ));
        }
    }
}
