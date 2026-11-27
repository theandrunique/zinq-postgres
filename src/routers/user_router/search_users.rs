use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{
    application::{
        RequestHandler,
        users::{DEFAULT_USER_SEARCH_LIMIT, SearchUsersQuery, SearchUsersQueryHandler},
    },
    error::Error,
    infra::AuthUser,
    routers::schemas::common::UserPublicSchema,
    state::AppState,
};

#[derive(Deserialize)]
pub struct SearchUsersQueryParams {
    q: String,
    limit: Option<i32>,
}

#[axum::debug_handler]
pub async fn search_users(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(params): Query<SearchUsersQueryParams>,
) -> Result<Json<Vec<UserPublicSchema>>, Error> {
    let handler = SearchUsersQueryHandler::new(&state);

    let query = SearchUsersQuery {
        query: params.q,
        limit: params.limit.unwrap_or(DEFAULT_USER_SEARCH_LIMIT),
    };

    let users = handler.handle(query).await?;

    Ok(Json(
        users.into_iter().map(UserPublicSchema::from).collect(),
    ))
}
