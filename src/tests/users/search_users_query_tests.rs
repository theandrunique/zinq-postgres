use sqlx::query;

use crate::{
    application::{
        RequestHandler,
        users::{SearchUsersQuery, SearchUsersQueryHandler},
    },
    domain::auth::User,
    error::Error,
    tests::common::TestContext,
};

async fn search(ctx: &TestContext, query: &str, limit: i32) -> Result<Vec<User>, Error> {
    let handler = SearchUsersQueryHandler::new(&ctx.app_state);

    handler
        .handle(SearchUsersQuery {
            query: query.to_string(),
            limit,
        })
        .await
}

#[tokio::test]
async fn test_search_users_by_exact_username() {
    let ctx = TestContext::new("test_search_users_by_exact_username").await;

    let alexander = ctx
        .create_test_user("alexander", "alexander@test.com")
        .await;
    ctx.create_test_user("bobmarley", "bob@test.com").await;

    let users = search(&ctx, "alexander", 20).await.expect("Should succeed");

    assert!(users.iter().any(|user| user.id == alexander.id));
    assert!(!users.iter().any(|user| user.username == "bobmarley"));
}

#[tokio::test]
async fn test_search_users_tolerates_typos() {
    let ctx = TestContext::new("test_search_users_tolerates_typos").await;

    ctx.create_test_user("aleksey", "aleksey@test.com").await;
    ctx.create_test_user("bobmarley", "bob@test.com").await;

    let users = search(&ctx, "alekx", 20).await.expect("Should succeed");

    assert!(users.iter().any(|user| user.username == "aleksey"));
    assert!(!users.iter().any(|user| user.username == "bobmarley"));
}

#[tokio::test]
async fn test_search_users_supports_short_prefixes() {
    let ctx = TestContext::new("test_search_users_supports_short_prefixes").await;

    ctx.create_test_user("alexander", "alexander@test.com")
        .await;
    ctx.create_test_user("aleksey", "aleksey@test.com").await;
    ctx.create_test_user("bobmarley", "bob@test.com").await;

    let users = search(&ctx, "al", 20).await.expect("Should succeed");

    assert!(users.iter().any(|user| user.username == "alexander"));
    assert!(users.iter().any(|user| user.username == "aleksey"));
    assert!(!users.iter().any(|user| user.username == "bobmarley"));
}

#[tokio::test]
async fn test_search_users_respects_limit() {
    let ctx = TestContext::new("test_search_users_respects_limit").await;

    ctx.create_test_user("aliceone", "alice1@test.com").await;
    ctx.create_test_user("alicetwo", "alice2@test.com").await;
    ctx.create_test_user("alicethree", "alice3@test.com").await;

    let users = search(&ctx, "alice", 2).await.expect("Should succeed");
    assert_eq!(users.len(), 2);

    let users = search(&ctx, "alice", 0).await.expect("Should succeed");
    assert_eq!(users.len(), 1);
}

#[tokio::test]
async fn test_search_users_excludes_inactive_users() {
    let ctx = TestContext::new("test_search_users_excludes_inactive_users").await;

    let user = ctx
        .create_test_user("inactiveuser", "inactive@test.com")
        .await;

    query("UPDATE users SET is_active = FALSE WHERE id = $1")
        .bind(user.id)
        .execute(&ctx.pool)
        .await
        .expect("Should deactivate user");

    let users = search(&ctx, "inactiveuser", 20)
        .await
        .expect("Should succeed");

    assert!(users.is_empty());
}

#[tokio::test]
async fn test_search_users_rejects_blank_query() {
    let ctx = TestContext::new("test_search_users_rejects_blank_query").await;

    let result = search(&ctx, "   ", 20).await;

    assert!(matches!(result, Err(Error::InvalidRequestBody(_))));
}

#[tokio::test]
async fn test_search_users_rejects_too_long_query() {
    let ctx = TestContext::new("test_search_users_rejects_too_long_query").await;

    let result = search(&ctx, &"a".repeat(33), 20).await;

    assert!(matches!(result, Err(Error::InvalidRequestBody(_))));
}

#[tokio::test]
async fn test_search_users_treats_like_wildcards_literally() {
    let ctx = TestContext::new("test_search_users_treats_like_wildcards_literally").await;

    ctx.create_test_user("alexander", "alexander@test.com")
        .await;

    let users = search(&ctx, "al%", 20).await.expect("Should succeed");

    assert!(users.is_empty());
}
