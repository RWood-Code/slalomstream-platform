use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use media_engine::MediaEngine;
use slalom_api::{router, AppState};
use slalom_bus::EventBus;
use slalom_core::db::Database;
use slalom_registry::RegistryService;
use tower::ServiceExt;

fn test_state() -> AppState {
    let db = Arc::new(Database::open_in_memory().unwrap());
    let dir = std::env::temp_dir().join("slalom-api-test");
    let _ = std::fs::remove_dir_all(&dir);
    AppState {
        db,
        bus: EventBus::new(),
        media: Arc::new(MediaEngine::new(dir, 9879)),
        registry: Arc::new(RegistryService),
    }
}

async fn body_json(body: Body) -> serde_json::Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn health_and_tournament_flow() {
    let app = router(test_state());
    let res = app
        .clone()
        .oneshot(Request::get("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = app
        .clone()
        .oneshot(
            Request::post("/api/tournaments")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"QA Open","judge_count":3}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let json = body_json(res.into_body()).await;
    assert_eq!(json["name"], "QA Open");
}
