use axum::{extract::State, response::Json};
use serde::Serialize;
use sushi::AppState;

#[derive(Serialize)]
struct DbHealth {
    db_ok: bool,
}

pub async fn db_health(State(app_state): State<AppState>) -> Json<DbHealth> {
    if let Some(pool) = &app_state.db_pool {
        let rest = sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(pool)
            .await;
        Json(DbHealth { db_ok: res.is_ok() })
    } else {
        Json(DbHealth { db_ok: false })
    }
}