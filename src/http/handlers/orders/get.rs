use axum::{Json, extract::{Path, State}};
use uuid::Uuid;

use crate::application::orders::{dto::OrderDto, get::GetOrder};
use crate::http::handlers::orders::error::AppError;
use crate::http::state::AppState;

pub async fn get_order(
    State(state): State<AppState>,
    Path(order_id): Path<String>,
) -> Result<Json<OrderDto>, AppError> {
    let id = Uuid::parse_str(&order_id).map_err(|e| AppError(anyhow::Error::from(e)))?;

    let order = state
        .get_order
        .execute(GetOrder { order_id: id })
        .await?
        .ok_or_else(|| anyhow::anyhow!("order not found"))?;

    Ok(Json(order))
}
