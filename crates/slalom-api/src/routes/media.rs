use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/media/status", get(status))
        .route("/api/media/preflight", get(preflight))
        .route("/api/media/preview/start", post(start_preview))
        .route("/api/media/arm", post(arm))
        .route("/api/media/disarm", post(disarm))
        .route("/api/media/encoder-profile", post(set_profile))
}

async fn status(State(st): State<AppState>) -> Json<serde_json::Value> {
    Json(json!(st.media.status().await))
}

async fn preflight(State(st): State<AppState>) -> Json<serde_json::Value> {
    Json(json!(st.media.preflight()))
}

async fn start_preview(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    st.media
        .start_preview()
        .await
        .map_err(slalom_core::error::CoreError::Validation)?;
    st.publish(slalom_core::events::VenueEvent::MediaState {
        state: "PreviewReady".into(),
    })
    .await;
    Ok(Json(json!({ "ok": true })))
}

async fn arm(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    st.media
        .arm()
        .await
        .map_err(slalom_core::error::CoreError::Validation)?;
    st.publish(slalom_core::events::VenueEvent::MediaState {
        state: "Armed".into(),
    })
    .await;
    Ok(Json(json!({ "ok": true })))
}

async fn disarm(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    st.media
        .disarm()
        .await
        .map_err(slalom_core::error::CoreError::Validation)?;
    Ok(Json(json!({ "ok": true })))
}

#[derive(Deserialize)]
struct ProfileBody {
    profile: String,
}

async fn set_profile(
    State(_st): State<AppState>,
    Json(body): Json<ProfileBody>,
) -> Json<serde_json::Value> {
    // MediaEngine needs interior mutability for profile — use tokio::sync::Mutex wrapper in state later
    Json(json!({ "profile": body.profile, "note": "restart to apply in alpha" }))
}
