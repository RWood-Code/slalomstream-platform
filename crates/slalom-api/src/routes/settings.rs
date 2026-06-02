use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;
use slalom_core::repo::SettingsRepo;

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/settings", get(get_settings))
}

async fn get_settings(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    let s = SettingsRepo::get(&st.db).map_err(ApiError::from)?;
    Ok(Json(json!(s)))
}
