use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::order::entity::Order;

#[derive(Debug, Serialize)]
pub struct OrderDto {
    pub id: Uuid,
    pub customer_name: String,
    pub drink: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
}

impl From<Order> for OrderDto {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            customer_name: order.customer_name,
            drink: order.drink,
            state: format!("{:?}", order.state),
            created_at: order.created_at,
        }
    }
}
