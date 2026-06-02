use axum::{extract::State, routing::get, Json, Router};
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;
use slalom_registry::RegistryService;

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/officials", get(list))
}

async fn list(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    let list = RegistryService::list(&st.db).map_err(ApiError::from)?;
    Ok(Json(json!(list)))
}
