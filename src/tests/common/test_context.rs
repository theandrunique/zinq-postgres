use std::{fs, sync::Arc};

use sqlx::{Pool, Postgres, query};

use crate::{
    application::{
        RequestHandler,
        auth::{RegisterComandHandler, RegisterCommand},
        chats::{
            CreateChatCommand, CreateChatCommandHandler, GetDMChatCommand, GetDMChatCommandHandler,
        },
        messages::{
            AddOrEditMessageCommand, AddOrEditMessageCommandHandler, AddOrEditMessageCommandResult,
        },
        meta_messages::{
            ChatCreateMetaMessage, ChatMemberAddedMetaMessage, ChatMemberRemovedMetaMessage,
        },
        services::{AttachmentService, AvatarService, ChannelImageService},
    }, config::S3Config, domain::{
        auth::User,
        chats::{Chat, ChatPermissions, data::ChatLoader},
        events::Mediator,
        messages::{CreateMessageRequest, Message},
    }, infra::{
        auth::{
            hash_handler::BcryptHandler, jwks_service::FileJwksService, jwt_handler::JwtService,
            totp_handler::TotpService,
        }, data::{attachment_repository::PostgresAttachmentRepository, chat_loader::PostgresChatLoader, chat_member_repository::PostgresChatMemberRepository, chat_repotisory::PostgresChatRepository, create_pool, message_ack_repository::PostgresMessageAckRepository, message_repository::PostgresMessageRepository, run_migrations, user_repository::PostgresUserRepository, user_session_repository::PostgresUserSessionRepository}, id_generator::SnowflakeIdGenerator, s3::AwsS3Service, smtp_client::SmtpService,
    }, state::AppState, tests::common::test_config::{
        POSTGRES_HOST_ENV, POSTGRES_PORT_ENV, S3_ACCESS_KEY_ENV, S3_BUCKET_ENV, S3_ENDPOINT_ENV, S3_REGION_ENV, S3_SECRET_KEY_ENV,
    },
};

pub struct TestContext {
    pub app_state: AppState,
}

impl TestContext {
    pub async fn new(database: &str) -> Self {
        let pg_host = std::env::var(POSTGRES_HOST_ENV).expect("TEST_POSGRES_HOST not set");
        let pg_port = std::env::var(POSTGRES_PORT_ENV).expect("TEST_POSGRES_PORT not set");


        let master_url = format!("postgres://postgres:postgres@{}:{}/postgres", pg_host, pg_port);
        let master_pool = create_pool(&master_url).await.unwrap();
        Self::setup_database(master_pool.clone(), database).await;
        master_pool.close().await;

        let pg_url = format!("postgres://postgres:postgres@{}:{}/{}", pg_host, pg_port, database);
        let pool = create_pool(&pg_url).await.unwrap();
        run_migrations(&pool).await.unwrap();

        let jwks_service = FileJwksService::load_from_directory("keys").unwrap();
        let jwks_service_clone = FileJwksService::load_from_directory("keys").unwrap();

        let s3_endpoint = std::env::var(S3_ENDPOINT_ENV).expect("TEST_S3_ENDPOINT not set");
        let s3_access_key = std::env::var(S3_ACCESS_KEY_ENV).expect("TEST_S3_ACCESS_KEY not set");
        let s3_secret_key = std::env::var(S3_SECRET_KEY_ENV).expect("TEST_S3_SECRET_KEY not set");
        let s3_bucket = std::env::var(S3_BUCKET_ENV).expect("TEST_S3_BUCKET not set");
        let s3_region = std::env::var(S3_REGION_ENV).unwrap_or("us-east-1".to_string());

        let s3_config = S3Config {
            access_key: s3_access_key,
            secret_key: s3_secret_key,
            service_url: s3_endpoint,
            force_path_style: true,
            bucket_name: s3_bucket,
            region: s3_region,
        };

        let s3_service: Arc<dyn crate::infra::s3::S3Service> =
            Arc::new(AwsS3Service::new(&s3_config).await);

        let _ = s3_service.create_bucket(&s3_config.bucket_name).await;

        let id_gen = Arc::new(SnowflakeIdGenerator::new());

        let attachment_service = Arc::new(AttachmentService::new(
            s3_service.clone(),
            id_gen.clone(),
            &s3_config,
        ));

        let avatar_service = Arc::new(AvatarService::new(s3_service.clone(), &s3_config));

        let channel_image_service =
            Arc::new(ChannelImageService::new(s3_service.clone(), &s3_config));

        let mut state = AppState {
            id_gen,
            user_repository: Arc::new(PostgresUserRepository::new(pool.clone())),
            user_session_repository: Arc::new(PostgresUserSessionRepository::new(pool.clone())),
            chat_loader: Arc::new(PostgresChatLoader::new(pool.clone())),
            chat_member_repository: Arc::new(PostgresChatMemberRepository::new(pool.clone())),
            chat_repository: Arc::new(PostgresChatRepository::new(pool.clone())),
            message_repository: Arc::new(PostgresMessageRepository::new(pool.clone())),
            attachment_repository: Arc::new(PostgresAttachmentRepository::new(pool.clone())),
            message_ack_repository: Arc::new(PostgresMessageAckRepository::new(pool.clone())),
            hash_handler: Arc::new(BcryptHandler::new()),
            jwks_service: Arc::new(jwks_service),
            jwt_handler: Arc::new(JwtService::new(jwks_service_clone, 3600)),
            smtp_client: Arc::new(SmtpService::new(
                "test@test.com".to_string(),
                "smtp.test.com".to_string(),
                587,
                "test".to_string(),
                "test".to_string(),
            )),
            totp_handler: Arc::new(TotpService::new()),
            s3_service,
            attachment_service,
            avatar_service,
            channel_image_service,
            mediator: Arc::new(Mediator::new()),
        };

        state.register_handlers().await;

        Self { app_state: state }
    }

    async fn setup_database(pool: Pool<Postgres>, database: &str) {
        let mut drop_builder = sqlx::QueryBuilder::new("DROP DATABASE IF EXISTS ");
        drop_builder.push(database);
        drop_builder.push(" WITH (FORCE)");

        drop_builder
            .build()
            .execute(&pool)
            .await
            .expect("Failed to drop database");

        let mut query_builder = sqlx::QueryBuilder::new("CREATE DATABASE ");
        query_builder.push(database);

        query_builder
            .build()
            .execute(&pool)
            .await
            .expect("Failed to create database");
    }

    pub async fn create_test_user(&self, username: &str, email: &str) -> User {
        let cmd = RegisterCommand {
            username: username.to_string(),
            password: "Test123!".to_string(),
            global_name: username.to_string(),
            email: email.to_string(),
        };

        let handler = RegisterComandHandler::new(&self.app_state);
        handler
            .handle(cmd)
            .await
            .expect(&format!("Failed to create test user {}", username))
    }

    pub async fn create_group_chat(
        &self,
        owner_id: i64,
        name: &str,
        members: Vec<i64>,
        permissions: Option<ChatPermissions>,
    ) -> Chat {
        let cmd = CreateChatCommand {
            current_user_id: owner_id,
            name: name.to_string(),
            members,
            permissions,
        };

        let handler = CreateChatCommandHandler::new(&self.app_state);
        handler
            .handle(cmd)
            .await
            .expect(&format!("Failed to create group chat {}", name))
    }

    pub async fn get_or_create_dm_chat(&self, user1_id: i64, user2_id: i64) -> Chat {
        let cmd = GetDMChatCommand {
            current_user_id: user1_id,
            user_id: user2_id,
        };

        let handler = GetDMChatCommandHandler::new(&self.app_state);
        handler.handle(cmd).await.expect(&format!(
            "Failed to get DM chat between {} and {}",
            user1_id, user2_id
        ))
    }

    pub async fn create_message(
        &self,
        chat_id: i64,
        author_id: i64,
        content: &str,
    ) -> AddOrEditMessageCommandResult {
        let cmd = AddOrEditMessageCommand {
            current_user_id: author_id,
            message_id: None,
            referenced_message_id: None,
            chat_id: chat_id,
            content: content.to_string(),
            attachments: vec![],
        };

        let msg_handler = AddOrEditMessageCommandHandler::new(&self.app_state);

        msg_handler
            .handle(cmd)
            .await
            .expect("Failed to create message")
    }
}
