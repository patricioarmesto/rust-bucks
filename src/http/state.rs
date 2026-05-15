use std::sync::Arc;

use crate::application::orders::cancel::CancelOrderUseCase;
use crate::application::orders::create::CreateOrderUseCase;
use crate::application::orders::get::GetOrderUseCase;

#[derive(Clone)]
pub struct AppState {
    pub create_order: Arc<CreateOrderUseCase>,
    pub get_order: Arc<GetOrderUseCase>,
    pub cancel_order: Arc<CancelOrderUseCase>,
}
