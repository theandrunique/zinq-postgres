use std::{collections::HashMap, sync::Arc};

use crate::{
    application::RequestHandler,
    domain::auth::{User, data::user_repository::UserRepository},
    error::Error,
    state::AppState,
};

pub const DEFAULT_USER_SEARCH_LIMIT: i32 = 20;
pub const MAX_USER_SEARCH_LIMIT: i32 = 50;
pub const MAX_USER_SEARCH_QUERY_LENGTH: usize = 32;

#[derive(Debug, Clone)]
pub struct SearchUsersQuery {
    pub query: String,
    pub limit: i32,
}

pub struct SearchUsersQueryHandler {
    user_repository: Arc<dyn UserRepository>,
}

impl SearchUsersQueryHandler {
    pub fn new(state: &AppState) -> Self {
        Self {
            user_repository: Arc::clone(&state.user_repository),
        }
    }
}

fn validation_error(message: &str) -> Error {
    Error::InvalidRequestBody(HashMap::from([(
        "q".to_string(),
        vec![message.to_string()],
    )]))
}

impl RequestHandler for SearchUsersQueryHandler {
    type Request = SearchUsersQuery;
    type Output = Vec<User>;
    type Error = Error;

    async fn handle(&self, request: Self::Request) -> Result<Self::Output, Self::Error> {
        let query = request.query.trim();

        if query.is_empty() {
            return Err(validation_error("Field is required."));
        }

        if query.chars().count() > MAX_USER_SEARCH_QUERY_LENGTH {
            return Err(validation_error(
                "Search query must be between 1 and 32 characters.",
            ));
        }

        let limit = request.limit.clamp(1, MAX_USER_SEARCH_LIMIT);

        let users = self
            .user_repository
            .search_by_username(query, limit)
            .await
            .map_err(Error::InternalServerError)?;

        Ok(users)
    }
}
