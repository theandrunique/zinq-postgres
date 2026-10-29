use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::{
    chats::{Chat, ChatMember, ChatType, LastMessageInfo},
    messages::MessageType,
};

#[derive(Serialize)]
pub struct ChatMemberSchema {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub permissions: Option<String>,
}

impl From<ChatMember> for ChatMemberSchema {
    fn from(value: ChatMember) -> Self {
        Self {
            user_id: value.user_id.to_string(),
            username: value.username,
            display_name: value.display_name,
            avatar: value.avatar,
            permissions: value.permissions.map(|p| p.to_string()),
        }
    }
}

#[derive(Serialize)]
pub struct LastMessageInfoSchema {
    pub message_id: String,
    pub author_id: String,
    pub content: String,
    pub message_type: MessageType,
    pub edited_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<LastMessageInfo> for LastMessageInfoSchema {
    fn from(value: LastMessageInfo) -> Self {
        Self {
            message_id: value.message_id.to_string(),
            author_id: value.author_id.to_string(),
            content: value.content,
            message_type: value.message_type,
            edited_at: value.edited_at,
            created_at: value.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct ChatSchema {
    pub id: String,
    pub owner_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub permissions: String,
    pub created_at: DateTime<Utc>,
    pub members: Vec<ChatMemberSchema>,
    pub last_message: Option<LastMessageInfoSchema>,
    pub last_read_message_id: Option<String>,
    pub max_read_message_id: Option<String>,
}

impl ChatSchema {
    pub fn from_chat_for_user(chat: Chat, current_user_id: i64) -> Self {
        let current_member = chat.get_member(current_user_id);

        if current_member.is_none() {
            tracing::warn!("Current user is not a member of this chat");
        }

        let current_last_read = current_member
            .and_then(|m| m.last_read_message_id)
            .map(|id| id.to_string());

        let max_read_message_id = chat
            .members
            .iter()
            .filter(|m| m.user_id != current_user_id)
            .filter_map(|m| m.last_read_message_id)
            .max();

        Self {
            id: chat.id.to_string(),
            owner_id: chat.owner_id.map(|id| id.to_string()),
            name: chat.name,
            description: Some("".to_string()),
            image: chat.image,
            chat_type: chat.chat_type,
            last_message: chat.last_message.map(|v| v.into()),
            permissions: chat.permissions.to_string(),
            created_at: chat.created_at,
            members: chat.members.into_iter().map(|m| m.into()).collect(),
            last_read_message_id: current_last_read,
            max_read_message_id: max_read_message_id.map(|id| id.to_string()),
        }
    }
}

impl From<Chat> for ChatSchema {
    fn from(chat: Chat) -> Self {
        Self {
            id: chat.id.to_string(),
            owner_id: chat.owner_id.map(|id| id.to_string()),
            name: chat.name,
            description: Some("".to_string()),
            image: chat.image,
            chat_type: chat.chat_type,
            last_message: chat.last_message.map(|v| v.into()),
            permissions: chat.permissions.to_string(),
            created_at: chat.created_at,
            members: chat.members.into_iter().map(|m| m.into()).collect(),
            last_read_message_id: None,
            max_read_message_id: None,
        }
    }
}
