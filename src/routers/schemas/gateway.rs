use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::routers::schemas::common::{ChatMemberSchema, ChatSchema, MessageSchema};

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum GatewayEventSchema {
    MessageCreate {
        message: MessageSchema,
    },
    MessageUpdate {
        message: MessageSchema,
    },
    MessageDelete {
        message_id: String,
    },
    ChatCreate {
        chat: ChatSchema,
    },
    ChatMemberAdd {
        chat_id: String,
        member: ChatMemberSchema,
    },
    ChatMemberRemove {
        chat_id: String,
        member: ChatMemberSchema,
    },
    MessageAck {
        user_id: String,
        chat_id: String,
        last_read_message_id: String,
    },
}
