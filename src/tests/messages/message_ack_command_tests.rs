use chrono::{Duration, TimeZone, Utc};

use crate::{
    application::{
        RequestHandler,
        messages::{
            MAX_ACKS_PER_REQUEST, MessageAckCommand, MessageAckCommandHandler, MessageAckInput,
        },
    },
    assert_err,
    domain::messages::{Message, MessageType},
    error::Error,
    tests::common::TestContext,
};

#[tokio::test]
async fn test_ack_auto_fill() {
    let ctx = TestContext::new("test_ack_auto_fill").await;
    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let message1 = ctx.create_message(chat.id, user1.id, "message 1").await;
    let message2 = ctx.create_message(chat.id, user2.id, "message 2").await;
    let message3 = ctx.create_message(chat.id, user1.id, "message 3").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user1.id,
            chat_id: chat.id,
            last_read_message_id: Some(message3.message.id),
            acks: vec![],
        })
        .await
        .expect("Ack should succeed");

    for message_id in [
        message1.message.id,
        message2.message.id,
        message3.message.id,
    ] {
        let acks = ctx
            .app_state
            .message_ack_repository
            .get_acks(chat.id, message_id)
            .await
            .expect("Should fetch acks");

        assert_eq!(acks.len(), 1, "Message {} should have 1 ack", message_id);
        assert_eq!(acks[0].user_id, user1.id);
    }
}

#[tokio::test]
async fn test_ack_explicit() {
    let ctx = TestContext::new("test_ack_explicit").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let m1 = ctx.create_message(chat.id, user1.id, "message 1").await;
    let m2 = ctx.create_message(chat.id, user2.id, "message 2").await;
    let m3 = ctx.create_message(chat.id, user1.id, "message 3").await;

    let t1 = Utc.with_ymd_and_hms(2026, 6, 20, 10, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2026, 6, 20, 11, 0, 0).unwrap();
    let t3 = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user1.id,
            chat_id: chat.id,
            last_read_message_id: Some(m3.message.id),
            acks: vec![
                MessageAckInput {
                    message_id: m1.message.id,
                    acked_at: t1,
                },
                MessageAckInput {
                    message_id: m2.message.id,
                    acked_at: t2,
                },
                MessageAckInput {
                    message_id: m3.message.id,
                    acked_at: t3,
                },
            ],
        })
        .await
        .expect("Ack should succeed");

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, m1.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Message {} should have 1 ack", m1.message.id);
    assert_eq!(acks[0].user_id, user1.id);
    assert_eq!(acks[0].created_at, t1);

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, m2.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Message {} should have 1 ack", m2.message.id);
    assert_eq!(acks[0].user_id, user1.id);
    assert_eq!(acks[0].created_at, t2);

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, m3.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Message {} should have 1 ack", m3.message.id);
    assert_eq!(acks[0].user_id, user1.id);
    assert_eq!(acks[0].created_at, t3);
}

#[tokio::test]
async fn test_ack_partial_explicit() {
    let ctx = TestContext::new("test_ack_partial_explicit").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let m1 = ctx.create_message(chat.id, user1.id, "message 1").await;
    let m2 = ctx.create_message(chat.id, user2.id, "message 2").await;
    let m3 = ctx.create_message(chat.id, user1.id, "message 3").await;

    let t1 = Utc.with_ymd_and_hms(2026, 6, 20, 10, 0, 0).unwrap();
    let t3 = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user1.id,
            chat_id: chat.id,
            last_read_message_id: Some(m3.message.id),
            acks: vec![
                MessageAckInput {
                    message_id: m1.message.id,
                    acked_at: t1,
                },
                MessageAckInput {
                    message_id: m3.message.id,
                    acked_at: t3,
                },
            ],
        })
        .await
        .expect("Ack should succeed");

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, m1.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Message {} should have 1 ack", m1.message.id);
    assert_eq!(acks[0].user_id, user1.id);
    assert_eq!(acks[0].created_at, t1);

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, m2.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Message {} should have 1 ack", m2.message.id);
    assert_eq!(acks[0].user_id, user1.id);
    assert_eq!(acks[0].created_at, t3);

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, m3.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Message {} should have 1 ack", m3.message.id);
    assert_eq!(acks[0].user_id, user1.id);
    assert_eq!(acks[0].created_at, t3);
}

#[tokio::test]
async fn test_ack_idempotent() {
    let ctx = TestContext::new("test_ack_idempotent").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;
    let message = ctx.create_message(chat.id, user1.id, "message 1").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);

    for _ in 0..2 {
        handler
            .handle(MessageAckCommand {
                current_user_id: user1.id,
                chat_id: chat.id,
                last_read_message_id: Some(message.message.id),
                acks: vec![],
            })
            .await
            .expect("Ack should succeed");
    }

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, message.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1, "Should have only 1 ack (idempotent)");
}

#[tokio::test]
async fn test_ack_updates_last_read_message_id() {
    let ctx = TestContext::new("test_ack_updates_last_read_message_id").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;
    let message = ctx.create_message(chat.id, user1.id, "message 1").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user1.id,
            chat_id: chat.id,
            last_read_message_id: Some(message.message.id),
            acks: vec![],
        })
        .await
        .expect("Ack should succeed");

    let updated = ctx
        .app_state
        .chat_repository
        .get_by_id(chat.id)
        .await
        .expect("Should get chat")
        .expect("Chat should exists");

    let member = updated.get_member(user1.id).expect("Member should exist");

    assert_eq!(member.last_read_message_id, Some(message.message.id));
}

#[tokio::test]
async fn test_ack_not_member() {
    let ctx = TestContext::new("test_ack_not_member").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let outsider = ctx.create_test_user("outsider", "outsider@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;
    let message = ctx.create_message(chat.id, user1.id, "message 1").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    let err = handler
        .handle(MessageAckCommand {
            current_user_id: outsider.id,
            chat_id: chat.id,
            last_read_message_id: Some(message.message.id),
            acks: vec![],
        })
        .await
        .expect_err("Expected error");

    assert_err!(err, Error::UserNotMember { .. })
}

#[tokio::test]
async fn test_ack_chat_not_found() {
    let ctx = TestContext::new("test_ack_chat_not_found").await;

    let user = ctx.create_test_user("user1", "user@test.com").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    let err = handler
        .handle(MessageAckCommand {
            current_user_id: user.id,
            chat_id: 99999,
            last_read_message_id: Some(100),
            acks: vec![],
        })
        .await
        .expect_err("Expected error");

    assert_err!(err, Error::ChatNotFound { .. })
}

#[tokio::test]
async fn test_ack_skips_old_messages() {
    let ctx = TestContext::new("test_ack_skips_old_messages").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let old_id = ((Utc::now() - Duration::days(10)) - ctx.app_state.id_gen.get_epoch())
        .num_milliseconds()
        << 22;

    let old_message = Message {
        id: old_id,
        chat_id: chat.id,
        author_id: user1.id,
        content: "old message".to_string(),
        created_at: Utc::now() - Duration::days(10),
        edited_at: None,
        message_type: MessageType::Default,
    };

    ctx.app_state
        .message_repository
        .upsert(&old_message)
        .await
        .expect("Should insert old message");

    let recent_message = ctx
        .create_message(chat.id, user1.id, "recent message")
        .await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: Some(recent_message.message.id),
            acks: vec![],
        })
        .await
        .expect("Ack should succeed");

    let old_acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, old_id)
        .await
        .expect("Should fetch acks");
    assert_eq!(
        old_acks.len(),
        0,
        "Old message should not have ack (older than 7 days)"
    );

    let recent_acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, recent_message.message.id)
        .await
        .expect("Should fetch acks");
    assert_eq!(recent_acks.len(), 1, "Recent message should have ack");
}

#[tokio::test]
async fn test_ack_individual_messages_without_pointer_advance() {
    let ctx = TestContext::new("test_ack_individual_messages_without_pointer_advance").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let message = ctx.create_message(chat.id, user1.id, "message 1").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: None,
            acks: vec![MessageAckInput {
                message_id: message.message.id,
                acked_at: Utc::now(),
            }],
        })
        .await
        .expect("Ack should succeed");

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, message.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1);
    assert_eq!(acks[0].user_id, user2.id);

    let stored_chat = ctx
        .app_state
        .chat_repository
        .get_by_id(chat.id)
        .await
        .expect("Should get chat")
        .expect("Chat should exist");
    let member = stored_chat
        .get_member(user2.id)
        .expect("Member should exist");

    assert_eq!(
        member.last_read_message_id, None,
        "Per-message acks must not move the read pointer"
    );
}

#[tokio::test]
async fn test_ack_batch_is_idempotent() {
    let ctx = TestContext::new("test_ack_batch_is_idempotent").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let m1 = ctx.create_message(chat.id, user1.id, "message 1").await;
    let m2 = ctx.create_message(chat.id, user1.id, "message 2").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);

    for _ in 0..3 {
        handler
            .handle(MessageAckCommand {
                current_user_id: user2.id,
                chat_id: chat.id,
                last_read_message_id: None,
                acks: vec![
                    MessageAckInput {
                        message_id: m1.message.id,
                        acked_at: Utc::now(),
                    },
                    MessageAckInput {
                        message_id: m2.message.id,
                        acked_at: Utc::now(),
                    },
                ],
            })
            .await
            .expect("Ack should succeed");
    }

    for message_id in [m1.message.id, m2.message.id] {
        let acks = ctx
            .app_state
            .message_ack_repository
            .get_acks(chat.id, message_id)
            .await
            .expect("Should fetch acks");

        assert_eq!(acks.len(), 1, "Repeated acks must stay idempotent");
    }
}

#[tokio::test]
async fn test_ack_clamps_future_timestamp() {
    let ctx = TestContext::new("test_ack_clamps_future_timestamp").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let message = ctx.create_message(chat.id, user1.id, "message 1").await;
    let future = Utc::now() + Duration::days(1);

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: None,
            acks: vec![MessageAckInput {
                message_id: message.message.id,
                acked_at: future,
            }],
        })
        .await
        .expect("Ack should succeed");

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, message.message.id)
        .await
        .expect("Should fetch acks");

    assert_eq!(acks.len(), 1);
    assert!(
        acks[0].created_at <= Utc::now(),
        "Future timestamps must be clamped to the server time"
    );
}

#[tokio::test]
async fn test_ack_ignores_messages_from_another_chat() {
    let ctx = TestContext::new("test_ack_ignores_messages_from_another_chat").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;
    let other_chat = ctx
        .create_group_chat(user1.id, "Other chat", vec![user2.id], None)
        .await;

    let foreign_message = ctx.create_message(other_chat.id, user1.id, "foreign").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: None,
            acks: vec![MessageAckInput {
                message_id: foreign_message.message.id,
                acked_at: Utc::now(),
            }],
        })
        .await
        .expect("Ack should succeed");

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, foreign_message.message.id)
        .await
        .expect("Should fetch acks");

    assert!(
        acks.is_empty(),
        "Ack for a message from another chat must be ignored"
    );
}

#[tokio::test]
async fn test_ack_does_not_move_pointer_backwards() {
    let ctx = TestContext::new("test_ack_does_not_move_pointer_backwards").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let m1 = ctx.create_message(chat.id, user1.id, "message 1").await;
    let m2 = ctx.create_message(chat.id, user1.id, "message 2").await;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);

    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: Some(m2.message.id),
            acks: vec![],
        })
        .await
        .expect("Ack should succeed");

    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: Some(m1.message.id),
            acks: vec![],
        })
        .await
        .expect("Ack should succeed");

    let stored_chat = ctx
        .app_state
        .chat_repository
        .get_by_id(chat.id)
        .await
        .expect("Should get chat")
        .expect("Chat should exist");
    let member = stored_chat
        .get_member(user2.id)
        .expect("Member should exist");

    assert_eq!(
        member.last_read_message_id,
        Some(m2.message.id),
        "Read pointer must only move forward"
    );
}

#[tokio::test]
async fn test_ack_rejects_too_many_acks() {
    let ctx = TestContext::new("test_ack_rejects_too_many_acks").await;

    let acks: Vec<MessageAckInput> = (0..=MAX_ACKS_PER_REQUEST)
        .map(|message_id| MessageAckInput {
            message_id: message_id as i64,
            acked_at: Utc::now(),
        })
        .collect();

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    let err = handler
        .handle(MessageAckCommand {
            current_user_id: 1,
            chat_id: 1,
            last_read_message_id: None,
            acks,
        })
        .await
        .expect_err("Expected validation error");

    assert_err!(err, Error::InvalidRequestBody(_));
}

#[tokio::test]
async fn test_ack_drops_acks_below_read_pointer() {
    let ctx = TestContext::new("test_ack_drops_acks_below_read_pointer").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let m1 = ctx.create_message(chat.id, user1.id, "message 1").await;
    let m2 = ctx.create_message(chat.id, user1.id, "message 2").await;

    // Advance the pointer directly so no acks are synthesized for the range.
    ctx.app_state
        .chat_repository
        .update_last_read_message_id(user2.id, chat.id, m2.message.id)
        .await
        .expect("Should move the read pointer");

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: None,
            acks: vec![
                MessageAckInput {
                    message_id: m1.message.id,
                    acked_at: Utc::now(),
                },
                MessageAckInput {
                    message_id: m2.message.id,
                    acked_at: Utc::now(),
                },
            ],
        })
        .await
        .expect("Ack should succeed");

    for message_id in [m1.message.id, m2.message.id] {
        let acks = ctx
            .app_state
            .message_ack_repository
            .get_acks(chat.id, message_id)
            .await
            .expect("Should fetch acks");

        assert!(
            acks.is_empty(),
            "Ack for message {} below the read pointer must be dropped",
            message_id
        );
    }
}

#[tokio::test]
async fn test_ack_drops_acks_older_than_window() {
    let ctx = TestContext::new("test_ack_drops_acks_older_than_window").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    let old_id = ((Utc::now() - Duration::days(10)) - ctx.app_state.id_gen.get_epoch())
        .num_milliseconds()
        << 22;

    let old_message = Message {
        id: old_id,
        chat_id: chat.id,
        author_id: user1.id,
        content: "old message".to_string(),
        created_at: Utc::now() - Duration::days(10),
        edited_at: None,
        message_type: MessageType::Default,
    };

    ctx.app_state
        .message_repository
        .upsert(&old_message)
        .await
        .expect("Should insert old message");

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: None,
            acks: vec![MessageAckInput {
                message_id: old_id,
                acked_at: Utc::now(),
            }],
        })
        .await
        .expect("Ack should succeed");

    let acks = ctx
        .app_state
        .message_ack_repository
        .get_acks(chat.id, old_id)
        .await
        .expect("Should fetch acks");

    assert!(
        acks.is_empty(),
        "Ack older than the receipt window must be dropped"
    );
}

#[tokio::test]
async fn test_ack_fill_respects_limit() {
    let ctx = TestContext::new("test_ack_fill_respects_limit").await;

    let user1 = ctx.create_test_user("user1", "user1@test.com").await;
    let user2 = ctx.create_test_user("user2", "user2@test.com").await;
    let chat = ctx
        .create_group_chat(user1.id, "Test chat", vec![user2.id], None)
        .await;

    // Must stay in sync with `ACK_FILL_LIMIT`.
    const FILL_LIMIT: usize = 30;
    const TOTAL: usize = FILL_LIMIT + 5;

    let mut messages = Vec::with_capacity(TOTAL);
    for index in 0..TOTAL {
        messages.push(
            ctx.create_message(chat.id, user1.id, &format!("message {index}"))
                .await,
        );
    }

    let newest = messages.last().expect("Should create messages").message.id;

    let handler = MessageAckCommandHandler::new(&ctx.app_state);
    handler
        .handle(MessageAckCommand {
            current_user_id: user2.id,
            chat_id: chat.id,
            last_read_message_id: Some(newest),
            acks: vec![],
        })
        .await
        .expect("Ack should succeed");

    let skipped = TOTAL - FILL_LIMIT;

    for (index, message) in messages.iter().enumerate() {
        let acks = ctx
            .app_state
            .message_ack_repository
            .get_acks(chat.id, message.message.id)
            .await
            .expect("Should fetch acks");

        if index < skipped {
            assert!(
                acks.is_empty(),
                "Message {} is outside the fill limit",
                index
            );
        } else {
            assert_eq!(acks.len(), 1, "Message {} should be filled", index);
        }
    }

    let stored_chat = ctx
        .app_state
        .chat_repository
        .get_by_id(chat.id)
        .await
        .expect("Should get chat")
        .expect("Chat should exist");
    let member = stored_chat
        .get_member(user2.id)
        .expect("Member should exist");

    assert_eq!(
        member.last_read_message_id,
        Some(newest),
        "The pointer must advance regardless of the fill limit"
    );
}
