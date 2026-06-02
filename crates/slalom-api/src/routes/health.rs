use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/healthz", get(healthz))
}

async fn healthz() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "venue-node",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
