use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod attachment_repository;
pub mod chat_loader;
pub mod chat_member_repository;
pub mod chat_repotisory;
pub mod message_ack_repository;
pub mod message_repository;
pub mod user_repository;
pub mod user_session_repository;

pub async fn create_pool(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    Ok(pool)
}

/// Applies the migrations embedded in `src/infra/data/migrations`.
///
/// Until the project reaches MVP the schema is kept in two squashed migrations
/// (`001_auth.sql`, `002_chats.sql`). Change them in place and recreate the
/// local database (`docker compose down -v`) instead of adding new files.
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations...");

    sqlx::migrate!("src/infra/data/migrations")
        .run(pool)
        .await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}
