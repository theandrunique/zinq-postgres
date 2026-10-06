use std::sync::Arc;

use crate::{
    application::{
        meta_messages::{
            ChatCreateMetaMessage, ChatMemberAddedMetaMessage, ChatMemberRemovedMetaMessage,
        },
        services::{AttachmentService, AvatarService, ChannelImageService},
    },
    config::{self, Config},
    domain::{
        attachments::data::AttachmentRepository,
        auth::data::{
            user_repository::UserRepository, user_session_repository::UserSessionRepository,
        },
        chats::data::{ChatLoader, ChatMemberRepository, ChatRepository},
        events::Mediator,
        message_acks::data::MessageAckRepository,
        messages::data::MessageRepository,
    },
    infra::{
        auth::{
            hash_handler::{BcryptHandler, HashHandler},
            jwks_service::FileJwksService,
            jwt_handler::{JwtHandler, JwtService},
            totp_handler::{TotpHandler, TotpService},
        },
        data::{
            attachment_repository::PostgresAttachmentRepository, chat_loader::PostgresChatLoader,
            chat_member_repository::PostgresChatMemberRepository,
            chat_repotisory::PostgresChatRepository, create_pool,
            message_ack_repository::PostgresMessageAckRepository,
            message_repository::PostgresMessageRepository, user_repository::PostgresUserRepository,
            user_session_repository::PostgresUserSessionRepository,
        },
        id_generator::{IdGenerator, SnowflakeIdGenerator},
        s3::{AwsS3Service, S3Service},
        smtp_client::{SmtpClient, SmtpService},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub id_gen: Arc<dyn IdGenerator>,
    pub user_repository: Arc<dyn UserRepository>,
    pub user_session_repository: Arc<dyn UserSessionRepository>,
    pub chat_loader: Arc<dyn ChatLoader>,
    pub chat_member_repository: Arc<dyn ChatMemberRepository>,
    pub chat_repository: Arc<dyn ChatRepository>,
    pub message_repository: Arc<dyn MessageRepository>,
    pub attachment_repository: Arc<dyn AttachmentRepository>,
    pub message_ack_repository: Arc<dyn MessageAckRepository>,
    pub hash_handler: Arc<dyn HashHandler>,
    pub jwks_service: Arc<FileJwksService>,
    pub jwt_handler: Arc<dyn JwtHandler>,
    pub smtp_client: Arc<dyn SmtpClient>,
    pub totp_handler: Arc<dyn TotpHandler>,
    pub s3_service: Arc<dyn S3Service>,
    pub attachment_service: Arc<AttachmentService>,
    pub avatar_service: Arc<AvatarService>,
    pub channel_image_service: Arc<ChannelImageService>,
    pub mediator: Arc<Mediator>,
}

impl AppState {
    pub async fn register_handlers(&mut self) {
        self.mediator
            .register(ChatCreateMetaMessage::new(self))
            .await;
        self.mediator
            .register(ChatMemberAddedMetaMessage::new(self))
            .await;
        self.mediator
            .register(ChatMemberRemovedMetaMessage::new(self))
            .await;
    }
}

pub async fn init_state() -> AppState {
    let app_config = config::config();

    let jwks_service = FileJwksService::load_from_directory(&app_config.auth.keys_directory)
        .expect("Failed to init JwksService");

    let s3_service: Arc<dyn S3Service> = Arc::new(AwsS3Service::new(&app_config.s3).await);

    let id_gen: Arc<dyn IdGenerator> = Arc::new(SnowflakeIdGenerator::new());

    let attachment_service = Arc::new(AttachmentService::new(
        s3_service.clone(),
        id_gen.clone(),
        &app_config.s3,
    ));

    let avatar_service = Arc::new(AvatarService::new(s3_service.clone(), &app_config.s3));

    let channel_image_service =
        Arc::new(ChannelImageService::new(s3_service.clone(), &app_config.s3));

    let pool = create_pool().await.unwrap();

    let mut app_state = AppState {
        id_gen: id_gen.clone(),
        user_repository: Arc::new(PostgresUserRepository::new(pool.clone())),
        user_session_repository: Arc::new(PostgresUserSessionRepository::new(pool.clone())),
        chat_loader: Arc::new(PostgresChatLoader::new(pool.clone())),
        chat_member_repository: Arc::new(PostgresChatMemberRepository::new(pool.clone())),
        chat_repository: Arc::new(PostgresChatRepository::new()),
        message_repository: Arc::new(PostgresMessageRepository::new(pool.clone())),
        attachment_repository: Arc::new(PostgresAttachmentRepository::new(pool.clone())),
        message_ack_repository: Arc::new(PostgresMessageAckRepository::new(pool.clone())),
        hash_handler: Arc::new(BcryptHandler::new()),
        jwks_service: Arc::new(jwks_service.clone()),
        jwt_handler: Arc::new(JwtService::new(
            jwks_service,
            app_config.auth.access_token_expiration_seconds as i64,
        )),
        smtp_client: Arc::new(SmtpService::new(
            app_config.smtp.from.clone(),
            app_config.smtp.host.clone(),
            app_config.smtp.port,
            app_config.smtp.username.clone(),
            app_config.smtp.password.clone(),
        )),
        totp_handler: Arc::new(TotpService::new()),
        s3_service,
        attachment_service,
        avatar_service,
        channel_image_service,
        mediator: Arc::new(Mediator::new()),
    };

    app_state.register_handlers().await;

    app_state
}
