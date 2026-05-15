use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::domain::order::{entity::Order, repository::OrderRepository};

#[derive(Debug)]
pub struct GetOrder {
    pub order_id: Uuid,
}

pub struct GetOrderUseCase {
    repository: Arc<dyn OrderRepository>,
}

impl GetOrderUseCase {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, query: GetOrder) -> anyhow::Result<Option<Order>> {
        let order = self.repository.find_by_id(query.order_id)?;

        Ok(order)
    }
}
