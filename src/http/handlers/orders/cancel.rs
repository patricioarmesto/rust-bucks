use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::application::orders::cancel::CancelOrder;
use crate::http::handlers::orders::error::AppError;
use crate::http::state::AppState;

pub async fn cancel_order(
    State(state): State<AppState>,
    Path(order_id): Path<String>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let id = Uuid::parse_str(&order_id).map_err(|e| AppError(anyhow::Error::from(e)))?;

    state
        .cancel_order
        .execute(CancelOrder { order_id: id })
        .await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(serde_json::json!({})),
    ))
}
