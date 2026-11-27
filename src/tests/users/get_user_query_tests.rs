use crate::{
    application::{
        RequestHandler,
        users::{GetUserQuery, GetUserQueryHandler},
    },
    error::Error,
    tests::common::TestContext,
};

#[tokio::test]
async fn test_get_user_by_id_returns_user() {
    let ctx = TestContext::new("test_get_user_by_id_returns_user").await;

    let user = ctx
        .create_test_user("alexander", "alexander@test.com")
        .await;

    let handler = GetUserQueryHandler::new(&ctx.app_state);
    let found = handler
        .handle(GetUserQuery { user_id: user.id })
        .await
        .expect("Should find an existing user");

    assert_eq!(found.id, user.id);
    assert_eq!(found.username, user.username);
    assert_eq!(found.display_name, user.display_name);
}

#[tokio::test]
async fn test_get_user_by_id_not_found() {
    let ctx = TestContext::new("test_get_user_by_id_not_found").await;

    let handler = GetUserQueryHandler::new(&ctx.app_state);
    let result = handler
        .handle(GetUserQuery {
            user_id: 123_456_789,
        })
        .await;

    assert!(matches!(
        result,
        Err(Error::UserNotFound(user_id)) if user_id == 123_456_789
    ));
}
