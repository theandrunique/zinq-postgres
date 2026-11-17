use std::collections::HashMap;

use crate::domain::chats::{Chat, ChatMember, data::ChatMemberRepository};

/// Пользователи, на которых ссылается последнее сообщение чата: автор и
/// участник системного сообщения (если он есть).
pub fn referenced_member_ids(chat: &Chat) -> Vec<i64> {
    chat.last_message
        .as_ref()
        .map(|last_message| {
            [
                Some(last_message.author_id),
                last_message.message_type.referenced_user_id(),
            ]
            .into_iter()
            .flatten()
            .collect()
        })
        .unwrap_or_default()
}

/// Догружает профили, которых нет среди активных участников чата (например,
/// автор последнего сообщения уже вышел из чата), чтобы их можно было отдать
/// в схеме чата.
pub async fn load_referenced_members(
    repository: &dyn ChatMemberRepository,
    chats: &[Chat],
) -> Result<HashMap<i64, ChatMember>, anyhow::Error> {
    let mut referenced: HashMap<i64, ChatMember> = HashMap::new();

    for chat in chats {
        let missing: Vec<i64> = referenced_member_ids(chat)
            .into_iter()
            .filter(|user_id| !chat.members.iter().any(|m| m.user_id == *user_id))
            .filter(|user_id| !referenced.contains_key(user_id))
            .collect();

        if missing.is_empty() {
            continue;
        }

        referenced.extend(repository.get_members(chat.id, &missing).await?);
    }

    Ok(referenced)
}
