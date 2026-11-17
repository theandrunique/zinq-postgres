use crate::{
    application::{
        RequestHandler,
        chats::{CreateChatCommand, CreateChatCommandHandler},
        chats::{DeleteChatMemberCommand, DeleteChatMemberCommandHandler},
        chats::{GetChatQuery, GetChatQueryHandler},
    },
    assert_err,
    error::Error,
    tests::common::TestContext,
};

#[tokio::test]
async fn test_get_chat_success() {
    let ctx = TestContext::new("test_get_chat_success").await;

    let owner = ctx.create_test_user("owner", "owner@test.com").await;
    let member1 = ctx.create_test_user("member1", "member1@test.com").await;
    let member2 = ctx.create_test_user("member2", "member2@test.com").await;
    let member3 = ctx.create_test_user("member3", "member3@test.com").await;

    let chat = ctx
        .create_group_chat(
            owner.id,
            "Test Group",
            vec![member1.id, member2.id, member3.id],
            None,
        )
        .await;

    let query_handler = GetChatQueryHandler::new(&ctx.app_state);
    let query = GetChatQuery {
        current_user_id: owner.id,
        chat_id: chat.id,
    };

    let result = query_handler.handle(query).await.expect("Should succeed");

    assert_eq!(result.chat.id, chat.id);
    assert_eq!(result.chat.name, Some("Test Group".to_string()));
    assert_eq!(result.chat.members.len(), 4);

    let member_ids: Vec<i64> = result.chat.members.iter().map(|m| m.user_id).collect();
    assert!(member_ids.contains(&owner.id));
    assert!(member_ids.contains(&member1.id));
    assert!(member_ids.contains(&member2.id));
    assert!(member_ids.contains(&member3.id));
}

#[tokio::test]
async fn test_get_chat_excludes_left_members() {
    let ctx = TestContext::new("test_get_chat_excludes_left_members").await;

    let owner = ctx.create_test_user("owner", "owner@test.com").await;
    let member = ctx.create_test_user("member", "member@test.com").await;
    let chat = ctx
        .create_group_chat(owner.id, "Test Group", vec![member.id], None)
        .await;

    let delete_handler = DeleteChatMemberCommandHandler::new(&ctx.app_state);
    delete_handler
        .handle(DeleteChatMemberCommand {
            current_user_id: owner.id,
            chat_id: chat.id,
            user_id: member.id,
        })
        .await
        .expect("Should remove member");

    let query_handler = GetChatQueryHandler::new(&ctx.app_state);
    let result = query_handler
        .handle(GetChatQuery {
            current_user_id: owner.id,
            chat_id: chat.id,
        })
        .await
        .expect("Should succeed");

    let member_ids: Vec<i64> = result.chat.members.iter().map(|m| m.user_id).collect();
    assert!(member_ids.contains(&owner.id));
    assert!(
        !member_ids.contains(&member.id),
        "Left members must not be returned as chat participants"
    );
}

#[tokio::test]
async fn test_get_chat_not_found() {
    let ctx = TestContext::new("test_get_chat_not_found").await;

    let current_user = ctx
        .create_test_user("currentuser", "currentuser@test.com")
        .await;

    let query_handler = GetChatQueryHandler::new(&ctx.app_state);
    let query = GetChatQuery {
        current_user_id: current_user.id,
        chat_id: 99999,
    };

    let err = query_handler
        .handle(query)
        .await
        .expect_err("Should fail - chat not found");

    assert_err!(err, Error::ChatNotFound(_));
}

#[tokio::test]
async fn test_get_chat_not_member() {
    let ctx = TestContext::new("test_get_chat_not_member").await;

    let owner = ctx.create_test_user("owner", "owner@test.com").await;
    let member = ctx.create_test_user("member", "member@test.com").await;
    let stranger = ctx.create_test_user("stranger", "stranger@test.com").await;
    let chat = ctx
        .create_group_chat(owner.id, "Test Group", vec![member.id], None)
        .await;

    let query_handler = GetChatQueryHandler::new(&ctx.app_state);
    let query = GetChatQuery {
        current_user_id: stranger.id,
        chat_id: chat.id,
    };

    let err = query_handler
        .handle(query)
        .await
        .expect_err("Should fail - not member");

    assert_err!(err, Error::UserNotMember { .. });
}
