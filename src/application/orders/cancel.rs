use std::sync::Arc;

use tracing::{info, instrument};
use uuid::Uuid;

use crate::domain::order::repository::OrderRepository;

#[derive(Debug)]
pub struct CancelOrder {
    pub order_id: Uuid,
}

pub struct CancelOrderUseCase {
    repository: Arc<dyn OrderRepository>,
}

impl CancelOrderUseCase {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: CancelOrder) -> anyhow::Result<()> {
        let mut order = self
            .repository
            .find_by_id(command.order_id)?
            .ok_or_else(|| anyhow::anyhow!("order not found"))?;

        order.cancel()?;

        self.repository.save(&order)?;

        info!(order_id = %order.id, "order cancelled");

        Ok(())
    }
}
