use std::process::{Command, ExitCode};

use testcontainers::GenericImage;
use testcontainers::{ImageExt, runners::AsyncRunner};
use testcontainers_modules::minio::MinIO;
use testcontainers_modules::nats::{Nats, NatsServerCmd};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::scylladb::ScyllaDB;

#[tokio::main]
async fn main() -> ExitCode {
    let postgres = match Postgres::default().with_tag("16-alpine").start().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to start PostgreSQL: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let minio = match GenericImage::new("rustfs/rustfs", "latest")
        .with_env_var("MINIO_DEFAULT_BUCKETS", "test-bucket")
        .start()
        .await
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to start RustFS: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let pg_host = postgres.get_host().await.unwrap().to_string();
    let pg_port = postgres.get_host_port_ipv4(5432).await.unwrap().to_string();

    let minio_host = minio.get_host().await.unwrap().to_string();
    let minio_port = minio.get_host_port_ipv4(9000).await.unwrap();
    let minio_url = format!("http://{}:{}", minio_host, minio_port);

    let status = Command::new("cargo")
        .arg("test")
        .env("TEST_POSGRES_HOST", pg_host)
        .env("TEST_POSGRES_PORT", pg_port)
        .env("TEST_S3_ENDPOINT", minio_url)
        .env("TEST_S3_ACCESS_KEY", "minioadmin")
        .env("TEST_S3_SECRET_KEY", "minioadmin")
        .env("TEST_S3_BUCKET", "test-bucket")
        .env("TEST_S3_REGION", "us-east-1")
        .spawn()
        .expect("Failed to start cargo test")
        .wait()
        .expect("Failed to wait on cargo test");

    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
