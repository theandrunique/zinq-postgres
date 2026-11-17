use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    domain::chats::{Chat, ChatMember, ChatType, LastMessageInfo},
    routers::schemas::common::{MessageAuthorSchema, MessageTypeSchema},
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
            permissions: value.permissions.map(|p| p.bits().to_string()),
        }
    }
}

#[derive(Serialize)]
pub struct LastMessageInfoSchema {
    pub message_id: String,
    pub author_id: String,
    pub author: MessageAuthorSchema,
    pub content: String,
    pub message_type: MessageTypeSchema,
    pub edited_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl LastMessageInfoSchema {
    fn from_info(
        value: LastMessageInfo,
        members: &HashMap<i64, ChatMember>,
    ) -> Result<Self, anyhow::Error> {
        let author = members.get(&value.author_id).ok_or_else(|| {
            anyhow::anyhow!(
                "Author {} not found for last message {}",
                value.author_id,
                value.message_id
            )
        })?;

        Ok(Self {
            message_id: value.message_id.to_string(),
            author_id: value.author_id.to_string(),
            author: MessageAuthorSchema::from(author),
            content: value.content,
            message_type: MessageTypeSchema::from_type(value.message_type, members)?,
            edited_at: value.edited_at,
            created_at: value.created_at,
        })
    }
}

#[derive(Serialize)]
pub struct UserChatsPageSchema {
    pub chats: Vec<ChatSchema>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
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
    pub fn from_chat(
        chat: Chat,
        referenced_members: &HashMap<i64, ChatMember>,
    ) -> Result<Self, anyhow::Error> {
        Self::build(chat, referenced_members, None, None)
    }

    pub fn from_chat_for_user(
        chat: Chat,
        current_user_id: i64,
        referenced_members: &HashMap<i64, ChatMember>,
    ) -> Result<Self, anyhow::Error> {
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
            .max()
            .map(|id| id.to_string());

        Self::build(
            chat,
            referenced_members,
            current_last_read,
            max_read_message_id,
        )
    }

    fn build(
        chat: Chat,
        referenced_members: &HashMap<i64, ChatMember>,
        last_read_message_id: Option<String>,
        max_read_message_id: Option<String>,
    ) -> Result<Self, anyhow::Error> {
        let mut members_by_id: HashMap<i64, ChatMember> = chat
            .members
            .iter()
            .map(|member| (member.user_id, member.clone()))
            .collect();

        members_by_id.extend(
            referenced_members
                .iter()
                .map(|(user_id, member)| (*user_id, member.clone())),
        );

        let last_message = chat
            .last_message
            .map(|info| LastMessageInfoSchema::from_info(info, &members_by_id))
            .transpose()?;

        Ok(Self {
            id: chat.id.to_string(),
            owner_id: chat.owner_id.map(|id| id.to_string()),
            name: chat.name,
            description: Some("".to_string()),
            image: chat.image,
            chat_type: chat.chat_type,
            last_message,
            permissions: chat.permissions.bits().to_string(),
            created_at: chat.created_at,
            members: chat.members.into_iter().map(|m| m.into()).collect(),
            last_read_message_id,
            max_read_message_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        chats::{ChatPermissions, LastMessageInfo},
        messages::MessageType,
    };
    use chrono::Utc;

    fn member(user_id: i64, name: &str) -> ChatMember {
        ChatMember {
            user_id,
            last_read_message_id: None,
            username: name.to_string(),
            display_name: name.to_string(),
            avatar: None,
            is_leave: false,
            permissions: None,
        }
    }

    #[test]
    fn serializes_last_message_type_as_payload() {
        let author = member(10, "author");
        let chat = Chat {
            id: 1,
            owner_id: Some(owner_id()),
            name: None,
            image: None,
            chat_type: ChatType::GroupDm,
            last_message: Some(LastMessageInfo {
                message_id: 5,
                author_id: 10,
                content: String::new(),
                message_type: MessageType::MemberRemove { user_id: 20 },
                edited_at: None,
                created_at: Utc::now(),
            }),
            permissions: ChatPermissions::empty(),
            created_at: Utc::now(),
            members: vec![author],
        };

        let referenced = HashMap::from([(20, member(20, "subject"))]);
        let schema = ChatSchema::from_chat_for_user(chat, owner_id(), &referenced)
            .expect("Should build a schema");

        let json = serde_json::to_value(&schema).expect("Should serialize to json");

        assert_eq!(json["last_message"]["author"]["username"], "author");
        assert_eq!(
            json["last_message"]["message_type"]["type"],
            "member_remove"
        );
        assert_eq!(
            json["last_message"]["message_type"]["payload"]["user"]["username"],
            "subject"
        );
    }

    fn owner_id() -> i64 {
        10
    }
}
