use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;

use crate::{
    application::{
        RequestHandler,
        users::{GetUserQuery, GetUserQueryHandler},
    },
    error::Error,
    infra::AuthUser,
    routers::schemas::common::UserPublicSchema,
    state::AppState,
};

#[derive(Deserialize)]
pub struct UserIdParam {
    user_id: i64,
}

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(UserIdParam { user_id }): Path<UserIdParam>,
) -> Result<Json<UserPublicSchema>, Error> {
    let handler = GetUserQueryHandler::new(&state);

    let query = GetUserQuery { user_id };

    let user = handler.handle(query).await?;

    Ok(Json(UserPublicSchema::from(user)))
}
