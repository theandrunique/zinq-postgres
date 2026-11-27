use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use crate::{routers::api_router, tests::common::TestContext};

#[tokio::test]
async fn test_get_user_requires_auth() {
    let ctx = TestContext::new("test_get_user_requires_auth").await;
    let app = api_router(ctx.app_state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/users/123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_search_users_requires_auth() {
    let ctx = TestContext::new("test_search_users_requires_auth").await;
    let app = api_router(ctx.app_state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/users/search?q=alex")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
