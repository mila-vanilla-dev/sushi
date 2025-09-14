use axum::{extract::State, response::Json};
use serde::Serialize;
use crate::AppState;

#[derive(Serialize)]
pub struct DbHealth {
    db_ok: bool,
}

pub async fn db_health(State(app_state): State<AppState>) -> Json<DbHealth> {
    if let Some(pool) = &app_state.db_pool {
        let res = sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(pool)
            .await;
        Json(DbHealth { db_ok: res.is_ok() })
    } else {
        Json(DbHealth { db_ok: false })
    }
}