use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;
use slalom_core::repo::PassRepo;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/tournaments/{id}/passes", get(list).post(create))
        .route("/api/tournaments/{id}/passes/pending", get(pending))
        .route("/api/passes/{id}", get(get_one))
}

async fn list(
    State(st): State<AppState>,
    Path(tid): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let list = PassRepo::list_for_tournament(&st.db, tid).map_err(ApiError::from)?;
    Ok(Json(json!(list)))
}

async fn pending(
    State(st): State<AppState>,
    Path(tid): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let p = PassRepo::pending_for_tournament(&st.db, tid).map_err(ApiError::from)?;
    Ok(Json(json!(p)))
}

async fn get_one(
    State(st): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let p = PassRepo::get(&st.db, id).map_err(ApiError::from)?;
    Ok(Json(json!(p)))
}

#[derive(Deserialize)]
struct CreatePassBody {
    skier_id: i64,
    skier_name: String,
    rope_length: f32,
    #[serde(default = "one")]
    round_number: i32,
}

fn one() -> i32 {
    1
}

async fn create(
    State(st): State<AppState>,
    Path(tid): Path<i64>,
    Json(body): Json<CreatePassBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let (pass, evt) = PassRepo::create(
        &st.db,
        tid,
        body.skier_id,
        &body.skier_name,
        body.rope_length,
        body.round_number,
    )
    .map_err(ApiError::from)?;
    st.publish(evt).await;

    if let Ok(Some(_)) = st.media.on_pass_pending(pass.id, tid).await {
        let path = st
            .media
            .recordings_dir()
            .join(format!("pass_{}.mp4", pass.id));
        let rec = slalom_core::repo::RecordingRepo::create(
            &st.db,
            &path.to_string_lossy(),
            Some(pass.id),
            Some(tid),
        )
        .map_err(ApiError::from)?;
        st.publish(slalom_core::events::VenueEvent::RecordingLinked {
            recording_id: rec.id,
            pass_id: pass.id,
        })
        .await;
    }

    Ok(Json(json!(pass)))
}
