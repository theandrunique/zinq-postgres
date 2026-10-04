use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod attachment_repository;
pub mod chat_loader;
pub mod chat_member_repository;
pub mod chat_repotisory;
pub mod message_ack_repository;
pub mod message_repository;
pub mod user_repository;
pub mod user_session_repository;

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations...");

    sqlx::migrate!("src/infra/data/migrations").run(pool).await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}
