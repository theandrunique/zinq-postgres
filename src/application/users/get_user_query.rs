use std::sync::Arc;

use crate::{
    application::RequestHandler,
    domain::auth::{User, data::user_repository::UserRepository},
    error::Error,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct GetUserQuery {
    pub user_id: i64,
}

pub struct GetUserQueryHandler {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUserQueryHandler {
    pub fn new(state: &AppState) -> Self {
        Self {
            user_repository: Arc::clone(&state.user_repository),
        }
    }
}

impl RequestHandler for GetUserQueryHandler {
    type Request = GetUserQuery;
    type Output = User;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let user = self
            .user_repository
            .get_by_id(request.user_id)
            .await
            .map_err(Error::InternalServerError)?
            .ok_or(Error::UserNotFound(request.user_id))?;

        Ok(user)
    }
}
