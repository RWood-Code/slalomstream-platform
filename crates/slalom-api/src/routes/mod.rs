mod health;
mod media;
mod officials;
mod passes;
mod registry;
mod scores;
mod settings;
mod tournaments;
mod ws;

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(tournaments::routes())
        .merge(passes::routes())
        .merge(scores::routes())
        .merge(settings::routes())
        .merge(media::routes())
        .merge(officials::routes())
        .merge(registry::routes())
        .route("/ws", get(ws::ws_handler))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}
