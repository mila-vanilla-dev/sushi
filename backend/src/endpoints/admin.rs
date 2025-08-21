/*
Admin API Endpoints (suggested):

- GET    /admin/orders           - List all orders
- GET    /admin/orders/{id}      - Get order details
- POST   /admin/orders           - Create a new order
- PUT    /admin/orders/{id}      - Update an order
- DELETE /admin/orders/{id}      - Delete/cancel an order

- GET    /admin/inventory        - List inventory items
- POST   /admin/inventory        - Add new inventory item
- PUT    /admin/inventory/{id}   - Update inventory item
- DELETE /admin/inventory/{id}   - Remove inventory item

- GET    /admin/shipments        - List all shipments
- POST   /admin/shipments        - Create a shipment
- PUT    /admin/shipments/{id}   - Update shipment status

- GET    /admin/payments         - List all payments
- POST   /admin/payments/refund  - Issue a refund

Other common admin tasks:
- GET    /admin/customers        - List customers
- GET    /admin/stats            - Get sales/statistics overview
*/
use axum::{Json, extract::State};
use reqwest::StatusCode;

use crate::{
    AppState,
    endpoints::auth::{CreateAdminRequest, MessageResponse, UserResponse},
};

/// POST /api/admin/users (admin only) - Create new admin user
pub async fn create_admin_endpoint(
    State(state): State<AppState>,
    Json(request): Json<CreateAdminRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<MessageResponse>)> {
    let mut user_store = state.user_store.write().await;

    match user_store.create_admin(request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse { message: error }),
        )),
    }
}
