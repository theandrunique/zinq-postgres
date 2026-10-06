use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::{chats::{ChatPermissions, ChatType}, messages::MessageType};

#[derive(Clone, Debug)]
pub struct ChatInfo {
    pub id: i64,
    pub owner_id: Option<i64>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub chat_type: ChatType,
    pub permissions: ChatPermissions,
    pub created_at: DateTime<Utc>,

    pub members: Vec<ChatMemberWithUserInfo>,
    pub last_message: Option<LastMessageInfo>,
}

impl ChatInfo {
    pub fn get_member(&self, user_id: i64) -> Option<ChatMemberWithUserInfo> {
        self.members
            .iter()
            .find(|m| m.user_id == user_id && !m.is_leave)
            .cloned()
    }

    pub fn has_member(&self, user_id: i64) -> bool {
        self.get_member(user_id).is_some()
    }
}

#[derive(Clone, Debug)]
pub struct ChatMemberWithUserInfo {
    pub user_id: i64,
    pub last_read_message_id: Option<i64>,
    pub username: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub is_leave: bool,
    pub permissions: Option<ChatPermissions>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LastMessageInfo {
    pub message_id: i64,
    pub author_id: i64,
    pub content: String,
    pub message_type: MessageType,
    pub edited_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
