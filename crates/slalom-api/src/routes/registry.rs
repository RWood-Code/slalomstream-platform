use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;
use slalom_core::events::VenueEvent;
use slalom_registry::RegistryService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/registry/meta", get(meta))
        .route("/api/registry/import", post(import_csv))
}

async fn meta(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    let m = RegistryService::meta(&st.db).map_err(ApiError::from)?;
    Ok(Json(json!(m)))
}

#[derive(Deserialize)]
struct ImportBody {
    version: String,
    source: String,
    csv: String,
}

async fn import_csv(
    State(st): State<AppState>,
    Json(body): Json<ImportBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let count =
        RegistryService::import_csv(&st.db, &body.csv, &body.version, &body.source)
            .map_err(ApiError::from)?;
    st.publish(VenueEvent::RegistryUpdated {
        version: body.version.clone(),
    })
    .await;
    Ok(Json(json!({ "imported": count, "version": body.version })))
}
