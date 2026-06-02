use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::error::ApiError;
use crate::state::AppState;
use slalom_core::repo::TournamentRepo;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/tournaments", get(list).post(create))
        .route("/api/tournaments/{id}", get(get_one))
        .route("/api/tournaments/{id}/activate", post(activate))
}

async fn list(State(st): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    let list = TournamentRepo::list(&st.db).map_err(ApiError::from)?;
    Ok(Json(json!(list)))
}

async fn get_one(
    State(st): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let t = TournamentRepo::get(&st.db, id).map_err(ApiError::from)?;
    Ok(Json(json!(t)))
}

#[derive(Deserialize)]
struct CreateBody {
    name: String,
    #[serde(default = "default_judges")]
    judge_count: i32,
}

fn default_judges() -> i32 {
    3
}

async fn create(
    State(st): State<AppState>,
    Json(body): Json<CreateBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let (t, evt) = TournamentRepo::create(&st.db, &body.name, body.judge_count).map_err(ApiError::from)?;
    st.publish(evt).await;
    Ok(Json(json!(t)))
}

async fn activate(
    State(st): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    TournamentRepo::set_active(&st.db, id).map_err(ApiError::from)?;
    st.publish(slalom_core::events::VenueEvent::TournamentUpdated { id })
        .await;
    Ok(Json(json!({ "active_tournament_id": id })))
}
