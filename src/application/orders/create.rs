use std::sync::Arc;

use tracing::{info, instrument};

use crate::domain::order::{entity::Order, repository::OrderRepository};

#[derive(Debug)]
pub struct CreateOrder {
    pub customer_name: String,
    pub drink: String,
}

pub struct CreateOrderUseCase {
    repository: Arc<dyn OrderRepository>,
}

impl CreateOrderUseCase {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: CreateOrder) -> anyhow::Result<Order> {
        let order = Order::create(command.customer_name, command.drink);

        self.repository.save(&order)?;

        info!(order_id = %order.id, "order created");

        Ok(order)
    }
}
