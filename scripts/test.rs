use std::process::{Command, ExitCode};

use testcontainers::{GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;

/// RustFS image tag (S3-compatible storage used by the tests).
const RUSTFS_TAG: &str = "1.0.4";
/// RustFS root credentials. `rustfsadmin` is the built-in default.
const S3_ACCESS_KEY: &str = "rustfsadmin";
const S3_SECRET_KEY: &str = "rustfsadmin";
const S3_BUCKET: &str = "test-bucket";

#[tokio::main]
async fn main() -> ExitCode {
    let postgres = match Postgres::default().with_tag("16-alpine").start().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to start PostgreSQL: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let s3 = match GenericImage::new("rustfs/rustfs", RUSTFS_TAG)
        .with_env_var("RUSTFS_ACCESS_KEY", S3_ACCESS_KEY)
        .with_env_var("RUSTFS_SECRET_KEY", S3_SECRET_KEY)
        // By default RustFS writes logs into /logs, so testcontainers never sees
        // them. An empty directory makes the entrypoint log to stdout instead.
        .with_env_var("RUSTFS_OBS_LOG_DIRECTORY", "")
        .with_env_var("RUSTFS_OBS_LOGGER_LEVEL", "info")
        .with_ready_conditions(vec![WaitFor::message_on_stdout(
            "RustFS Service is running",
        )])
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

    let s3_host = s3.get_host().await.unwrap().to_string();
    let s3_port = s3.get_host_port_ipv4(9000).await.unwrap();
    let s3_url = format!("http://{}:{}", s3_host, s3_port);

    let status = Command::new("cargo")
        .arg("test")
        .env("TEST_POSGRES_HOST", pg_host)
        .env("TEST_POSGRES_PORT", pg_port)
        .env("TEST_S3_ENDPOINT", s3_url)
        .env("TEST_S3_ACCESS_KEY", S3_ACCESS_KEY)
        .env("TEST_S3_SECRET_KEY", S3_SECRET_KEY)
        .env("TEST_S3_BUCKET", S3_BUCKET)
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
