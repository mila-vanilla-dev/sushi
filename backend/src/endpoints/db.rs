// backend/src/endpoints/db.rs
use axum::{extract::State, Json};
use serde::Serialize;
use crate::AppState; // use the AppState defined in src/lib.rs

#[derive(Serialize)]
pub struct DbHealth {
    pub db_ok: bool,
}

pub async fn db_health(State(app_state): State<AppState>) -> Json<DbHealth> {
    // perform a small runtime query (no compile-time sqlx macros)
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&app_state.db_pool)
        .await
        .is_ok();

    Json(DbHealth { db_ok })
}
