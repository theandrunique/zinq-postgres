use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    application::messages::GetMessagesQueryResult,
    domain::{
        attachments::Attachment,
        chats::ChatMember,
        messages::{Message, MessageType},
    },
};

#[derive(Serialize)]
pub struct AttachmentSchema {
    pub id: String,
    pub message_id: String,
    pub chat_id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub storage_key: String,
    pub created_at: DateTime<Utc>,
}

impl From<Attachment> for AttachmentSchema {
    fn from(value: Attachment) -> Self {
        Self {
            id: value.id.to_string(),
            message_id: value.message_id.to_string(),
            chat_id: value.chat_id.to_string(),
            filename: value.filename,
            content_type: value.content_type,
            size: value.size,
            storage_key: value.storage_key,
            created_at: value.created_at,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageAuthorSchema {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub avatar: Option<String>,
}

impl From<&ChatMember> for MessageAuthorSchema {
    fn from(value: &ChatMember) -> Self {
        Self {
            user_id: value.user_id.to_string(),
            username: value.username.clone(),
            display_name: value.display_name.clone(),
            avatar: value.avatar.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct MessageSchema {
    pub id: String,
    pub chat_id: String,
    pub author_id: String,
    pub author: MessageAuthorSchema,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub message_type: MessageTypeSchema,
    pub attachments: Vec<AttachmentSchema>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum MessageTypeSchema {
    Default,
    Reply { referenced_message_id: String },
    MemberAdd { user: MessageAuthorSchema },
    MemberRemove { user: MessageAuthorSchema },
    MemberLeave { user: MessageAuthorSchema },
    ChatNameUpdate { new_name: String },
    ChatImageUpdate { new_image: String },
    ChatPinnedMessage,
    ChatUnpinMessage,
    ChatCreate { chat_name: String },
    Forward,
}

impl MessageTypeSchema {
    pub fn from_type(
        value: MessageType,
        members: &HashMap<i64, ChatMember>,
    ) -> Result<Self, anyhow::Error> {
        fn require(
            members: &HashMap<i64, ChatMember>,
            user_id: i64,
        ) -> Result<MessageAuthorSchema, anyhow::Error> {
            members
                .get(&user_id)
                .map(MessageAuthorSchema::from)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Chat member {} not found while serializing a message payload",
                        user_id
                    )
                })
        }

        Ok(match value {
            MessageType::Default => MessageTypeSchema::Default,
            MessageType::Reply {
                referenced_message_id,
            } => MessageTypeSchema::Reply {
                referenced_message_id: referenced_message_id.to_string(),
            },
            MessageType::MemberAdd { user_id } => MessageTypeSchema::MemberAdd {
                user: require(members, user_id)?,
            },
            MessageType::MemberRemove { user_id } => MessageTypeSchema::MemberRemove {
                user: require(members, user_id)?,
            },
            MessageType::MemberLeave { user_id } => MessageTypeSchema::MemberLeave {
                user: require(members, user_id)?,
            },
            MessageType::ChatNameUpdate { new_name } => {
                MessageTypeSchema::ChatNameUpdate { new_name }
            }
            MessageType::ChatImageUpdate { new_image } => {
                MessageTypeSchema::ChatImageUpdate { new_image }
            }
            MessageType::ChatPinnedMessage => MessageTypeSchema::ChatPinnedMessage,
            MessageType::ChatUnpinMessage => MessageTypeSchema::ChatUnpinMessage,
            MessageType::ChatCreate { chat_name } => MessageTypeSchema::ChatCreate { chat_name },
            MessageType::Forward => MessageTypeSchema::Forward,
        })
    }
}

impl MessageSchema {
    pub fn from_message(
        message: Message,
        attachments: Vec<Attachment>,
        author: &ChatMember,
        subject: Option<&ChatMember>,
    ) -> Result<Self, anyhow::Error> {
        let mut members = HashMap::from([(author.user_id, author.clone())]);

        if let Some(subject) = subject {
            members.insert(subject.user_id, subject.clone());
        }

        Self::from_parts(message, attachments, author, &members)
    }

    pub fn from_parts(
        message: Message,
        attachments: Vec<Attachment>,
        author: &ChatMember,
        members: &HashMap<i64, ChatMember>,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id: message.id.to_string(),
            chat_id: message.chat_id.to_string(),
            author_id: message.author_id.to_string(),
            author: MessageAuthorSchema::from(author),
            content: message.content,
            created_at: message.created_at,
            edited_at: message.edited_at,
            message_type: MessageTypeSchema::from_type(message.message_type, members)?,
            attachments: attachments.into_iter().map(Into::into).collect(),
        })
    }
}

impl TryFrom<GetMessagesQueryResult> for Vec<MessageSchema> {
    type Error = anyhow::Error;

    fn try_from(result: GetMessagesQueryResult) -> Result<Self, anyhow::Error> {
        let GetMessagesQueryResult {
            messages,
            attachments,
            members,
        } = result;

        messages
            .into_iter()
            .map(|message| {
                let attachments = attachments.get(&message.id).cloned().unwrap_or_default();
                let author = members.get(&message.author_id).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Author {} not found for message {}",
                        message.author_id,
                        message.id
                    )
                })?;

                MessageSchema::from_parts(message, attachments, author, &members)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn message(message_type: MessageType) -> Message {
        Message {
            id: 1,
            chat_id: 2,
            author_id: 10,
            content: String::new(),
            created_at: Utc::now(),
            edited_at: None,
            message_type,
        }
    }

    #[test]
    fn serializes_author_and_member_subject() {
        let author = member(10, "author");
        let subject = member(20, "subject");
        let members = HashMap::from([(10, author.clone()), (20, subject)]);

        let schema = MessageSchema::from_parts(
            message(MessageType::MemberAdd { user_id: 20 }),
            vec![],
            &author,
            &members,
        )
        .expect("Should build a schema");

        let json = serde_json::to_value(&schema).expect("Should serialize to json");

        assert_eq!(json["author"]["display_name"], "author");
        assert_eq!(json["type"]["type"], "member_add");
        assert_eq!(json["type"]["payload"]["user"]["user_id"], "20");
        assert_eq!(json["type"]["payload"]["user"]["username"], "subject");
    }

    #[test]
    fn fails_when_member_subject_is_missing() {
        let author = member(10, "author");
        let members = HashMap::from([(10, author.clone())]);

        let result = MessageSchema::from_parts(
            message(MessageType::MemberAdd { user_id: 99 }),
            vec![],
            &author,
            &members,
        );

        assert!(
            result.is_err(),
            "Missing member must be an application error"
        );
    }
}
