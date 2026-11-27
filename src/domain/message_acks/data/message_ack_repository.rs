use async_trait::async_trait;

use crate::domain::message_acks::MessageAck;

#[async_trait]
pub trait MessageAckRepository: Send + Sync {
    /// Persists individual message acks. Already stored acks are kept as is,
    /// so repeated calls with the same messages are safe.
    async fn bulk_upsert(&self, message_acks: &[MessageAck]) -> Result<(), anyhow::Error>;

    async fn get_acks(
        &self,
        chat_id: i64,
        message_id: i64,
    ) -> Result<Vec<MessageAck>, anyhow::Error>;
}
