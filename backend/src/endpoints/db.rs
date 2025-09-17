use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::{Pool, Postgres};

#[derive(Serialize)]
pub struct DbHealth {
    pub db_ok: bool,
}

pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

pub async fn db_health(State(app_state): State<AppState>) -> Json<DbHealth> {
    let db_ok = sqlx::query("SELECT 1")
        .execute(&app_state.db_pool)
        .await
        .is_ok();

    Json(DbHealth { db_ok })
}