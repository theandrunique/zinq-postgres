mod auth_router;
mod chat_router;
mod emoji_router;
mod event_publisher;
mod ping_router;
mod schemas;
mod user_router;
mod well_known_router;

use axum::Router;

use crate::{error::Error, state::AppState};

pub use auth_router::auth_router;
pub use chat_router::chat_router;
pub use emoji_router::emoji_router;
pub use event_publisher::EventPublisher;
pub use ping_router::ping_router;
pub use user_router::user_router;
pub use well_known_router::well_known_router;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", user_router(state.clone()))
        .nest("/auth", auth_router(state.clone()))
        .nest("/emoji-packs", emoji_router(state.clone()))
        .nest("/chats", chat_router(state))
        .fallback(api_not_found)
}

async fn api_not_found() -> Error {
    Error::RouteNotFound
}
