use async_trait::async_trait;

use crate::domain::events::{DomainEvent, DomainEventHandler};

pub struct EventPublisher {

}

#[async_trait]
impl DomainEventHandler for EventPublisher {
    async fn handle(&self, event: &DomainEvent) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
