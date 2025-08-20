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
