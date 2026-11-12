use chrono::{Duration, TimeZone, Utc};

use crate::{
    application::{
        RequestHandler,
        chats::{GetUserChatsQuery, GetUserChatsQueryHandler},
    },
    domain::chats::{
        ChatType,
        data::{ChatPageCursor, UserChatsPage},
    },
    tests::common::TestContext,
};

async fn get_page(
    ctx: &TestContext,
    user_id: i64,
    cursor: Option<ChatPageCursor>,
    limit: i32,
) -> UserChatsPage {
    let query_handler = GetUserChatsQueryHandler::new(&ctx.app_state);
    let query = GetUserChatsQuery {
        current_user_id: user_id,
        cursor,
        limit,
    };

    query_handler.handle(query).await.expect("Should succeed")
}

fn chat_ids(page: &UserChatsPage) -> Vec<i64> {
    page.chats.iter().map(|chat| chat.id).collect()
}

#[tokio::test]
async fn test_get_user_chats_empty() {
    let ctx = TestContext::new("test_get_user_chats_empty").await;

    let user = ctx.create_test_user("testuser", "user@test.com").await;

    let result = get_page(&ctx, user.id, None, 20).await;
    assert!(result.chats.is_empty());
    assert!(result.next_cursor.is_none());
}

#[tokio::test]
async fn test_get_user_chats_returns_chats() {
    let ctx = TestContext::new("test_get_user_chats_returns_chats").await;

    let current_user = ctx.create_test_user("owner", "owner@test.com").await;
    let member1 = ctx.create_test_user("member1", "member1@test.com").await;
    let member2 = ctx.create_test_user("member2", "member2@test.com").await;
    let member3 = ctx.create_test_user("member3", "member3@test.com").await;

    let group_chat = ctx
        .create_group_chat(
            current_user.id,
            "Test Group",
            vec![member1.id, member2.id],
            None,
        )
        .await;
    let _ = ctx
        .create_message(group_chat.id, current_user.id, "Hello group")
        .await;

    let dm_chat1 = ctx.get_or_create_dm_chat(current_user.id, member2.id).await;
    let _ = ctx
        .create_message(dm_chat1.id, current_user.id, "Hello DM 1")
        .await;

    let dm_chat2 = ctx.get_or_create_dm_chat(current_user.id, member3.id).await;
    let _ = ctx
        .create_message(dm_chat2.id, current_user.id, "Hello DM 2")
        .await;

    let result = get_page(&ctx, current_user.id, None, 20).await;

    assert_eq!(
        result.chats.len(),
        3,
        "Should return 3 chats (1 group + 2 DM)"
    );

    for chat in &result.chats {
        assert!(
            chat.last_message.is_some(),
            "All returned chats should have last_message_id set"
        );

        match chat.chat_type {
            ChatType::GroupDm => {
                assert!(
                    chat.members.len() == 1 && chat.members[0].user_id == current_user.id,
                    "GroupDm chat should have loaded only current user"
                );
            }
            ChatType::Dm => {
                assert!(
                    !chat.members.is_empty(),
                    "DM chat should have members loaded"
                );
                let member_ids: Vec<i64> = chat.members.iter().map(|m| m.user_id).collect();
                assert!(
                    member_ids.contains(&current_user.id),
                    "DM should contain current user"
                );
                assert!(
                    member_ids.len() >= 1 && member_ids.len() <= 2,
                    "DM chat should have 1 or 2 members (self-DM or with other user)"
                );
            }
        }
    }
}

#[tokio::test]
async fn test_get_user_chats_dm_without_message_filtered() {
    let ctx = TestContext::new("test_get_user_chats_dm_without_message_filtered").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let _dm_chat = ctx.get_or_create_dm_chat(user1.id, user2.id).await;

    let result = get_page(&ctx, user1.id, None, 20).await;
    assert!(result.chats.is_empty());
}

#[tokio::test]
async fn test_get_user_chats_orders_by_last_message_desc() {
    let ctx = TestContext::new("test_get_user_chats_orders_by_last_message_desc").await;

    let user = ctx.create_test_user("owner", "owner@test.com").await;

    let oldest = ctx.create_group_chat(user.id, "Oldest", vec![], None).await;
    let middle = ctx.create_group_chat(user.id, "Middle", vec![], None).await;
    let newest = ctx.create_group_chat(user.id, "Newest", vec![], None).await;

    let base = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    ctx.create_message_at(oldest.id, user.id, "old", base).await;
    ctx.create_message_at(middle.id, user.id, "mid", base + Duration::minutes(1))
        .await;
    ctx.create_message_at(newest.id, user.id, "new", base + Duration::minutes(2))
        .await;

    let page = get_page(&ctx, user.id, None, 20).await;

    assert_eq!(chat_ids(&page), vec![newest.id, middle.id, oldest.id]);
    assert!(page.next_cursor.is_none());
}

#[tokio::test]
async fn test_get_user_chats_pagination_ignores_new_message_in_unloaded_chat() {
    let ctx =
        TestContext::new("test_get_user_chats_pagination_ignores_new_message_in_unloaded_chat")
            .await;

    let user = ctx.create_test_user("owner", "owner@test.com").await;

    let chat1 = ctx.create_group_chat(user.id, "Chat 1", vec![], None).await;
    let chat2 = ctx.create_group_chat(user.id, "Chat 2", vec![], None).await;
    let chat3 = ctx.create_group_chat(user.id, "Chat 3", vec![], None).await;
    let chat4 = ctx.create_group_chat(user.id, "Chat 4", vec![], None).await;

    let base = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    ctx.create_message_at(chat1.id, user.id, "m1", base).await;
    ctx.create_message_at(chat2.id, user.id, "m2", base + Duration::minutes(1))
        .await;
    ctx.create_message_at(chat3.id, user.id, "m3", base + Duration::minutes(2))
        .await;
    ctx.create_message_at(chat4.id, user.id, "m4", base + Duration::minutes(3))
        .await;

    let first_page = get_page(&ctx, user.id, None, 2).await;
    assert_eq!(chat_ids(&first_page), vec![chat4.id, chat3.id]);
    let cursor = first_page
        .next_cursor
        .expect("First page must have a cursor");

    // A message arrives in a chat that was not loaded on the first page.
    // The chat jumps to the top of the list and must not shift later pages.
    ctx.create_message_at(chat1.id, user.id, "m1-new", base + Duration::minutes(10))
        .await;

    let second_page = get_page(&ctx, user.id, Some(cursor), 2).await;

    assert_eq!(chat_ids(&second_page), vec![chat2.id]);
    assert!(second_page.next_cursor.is_none());

    let first_ids = chat_ids(&first_page);
    assert!(
        chat_ids(&second_page)
            .iter()
            .all(|id| !first_ids.contains(id)),
        "Pages must not overlap"
    );
}

#[tokio::test]
async fn test_get_user_chats_empty_chats_are_last_and_paginated() {
    let ctx = TestContext::new("test_get_user_chats_empty_chats_are_last_and_paginated").await;

    let user = ctx.create_test_user("owner", "owner@test.com").await;

    let with_message = ctx
        .create_group_chat(user.id, "With message", vec![], None)
        .await;
    let empty_first = ctx
        .create_group_chat(user.id, "Empty 1", vec![], None)
        .await;
    let empty_second = ctx
        .create_group_chat(user.id, "Empty 2", vec![], None)
        .await;

    // Group chat creation publishes a meta message, so remove messages to
    // exercise the "chat without messages" tail of the ordering.
    ctx.clear_chat_messages(empty_first.id).await;
    ctx.clear_chat_messages(empty_second.id).await;

    let base = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    ctx.create_message_at(with_message.id, user.id, "hello", base)
        .await;

    let first_page = get_page(&ctx, user.id, None, 1).await;
    assert_eq!(chat_ids(&first_page), vec![with_message.id]);
    let first_cursor = first_page
        .next_cursor
        .expect("First page must have a cursor");
    assert!(matches!(first_cursor, ChatPageCursor::Dated { .. }));

    let second_page = get_page(&ctx, user.id, Some(first_cursor), 1).await;
    assert_eq!(chat_ids(&second_page), vec![empty_second.id]);
    let second_cursor = second_page
        .next_cursor
        .expect("Second page must have a cursor");
    assert!(matches!(
        second_cursor,
        ChatPageCursor::WithoutMessages { chat_id } if chat_id == empty_second.id
    ));

    let third_page = get_page(&ctx, user.id, Some(second_cursor), 1).await;
    assert_eq!(chat_ids(&third_page), vec![empty_first.id]);
    assert!(third_page.next_cursor.is_none());
}

#[tokio::test]
async fn test_get_user_chats_tie_breaks_by_id_desc() {
    let ctx = TestContext::new("test_get_user_chats_tie_breaks_by_id_desc").await;

    let user = ctx.create_test_user("owner", "owner@test.com").await;

    let first = ctx.create_group_chat(user.id, "First", vec![], None).await;
    let second = ctx.create_group_chat(user.id, "Second", vec![], None).await;

    let same_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    ctx.create_message_at(first.id, user.id, "first", same_time)
        .await;
    ctx.create_message_at(second.id, user.id, "second", same_time)
        .await;

    let page = get_page(&ctx, user.id, None, 20).await;

    assert_eq!(chat_ids(&page), vec![second.id, first.id]);
}
