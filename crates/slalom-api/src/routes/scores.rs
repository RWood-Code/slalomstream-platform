use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;
use slalom_core::repo::ScoreRepo;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/passes/{id}/judge-scores", get(list).post(submit))
}

async fn list(
    State(st): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let scores = ScoreRepo::list_for_pass(&st.db, id).map_err(ApiError::from)?;
    Ok(Json(json!(scores)))
}

#[derive(Deserialize)]
struct SubmitBody {
    judge_name: String,
    judge_role: String,
    pass_score: String,
}

async fn submit(
    State(st): State<AppState>,
    Path(pass_id): Path<i64>,
    Json(body): Json<SubmitBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let (score, evts) = ScoreRepo::submit(
        &st.db,
        pass_id,
        &body.judge_name,
        &body.judge_role,
        &body.pass_score,
    )
    .map_err(ApiError::from)?;
    for e in evts {
        st.publish(e).await;
    }
    Ok(Json(json!(score)))
}
