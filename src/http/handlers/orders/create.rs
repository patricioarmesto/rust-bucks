use axum::{Json, extract::State};
use serde::Deserialize;

use crate::application::orders::create::CreateOrder;
use crate::http::handlers::orders::dto::OrderResponse;
use crate::http::handlers::orders::error::AppError;
use crate::http::state::AppState;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub customer_name: String,
    pub drink: String,
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = state
        .create_order
        .execute(CreateOrder {
            customer_name: request.customer_name,
            drink: request.drink,
        })
        .await?;

    Ok(Json(order.into()))
}
