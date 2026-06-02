use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use slalom_api::{router, AppState};
use slalom_bus::EventBus;
use slalom_core::db::Database;
use slalom_registry::RegistryService;

fn data_dir() -> PathBuf {
    if let Ok(d) = std::env::var("VENUE_DATA_DIR") {
        return PathBuf::from(d);
    }
    #[cfg(windows)]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            return PathBuf::from(local).join("SlalomStream").join("venue");
        }
    }
    PathBuf::from(".data/venue")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,venue_node=debug".into()),
        )
        .init();

    let dir = data_dir();
    std::fs::create_dir_all(&dir)?;
    let db_path = dir.join("venue.db");
    let db = Arc::new(Database::open(&db_path)?);
    let bus = EventBus::new();
    let preview_port: u16 = std::env::var("VENUE_PREVIEW_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9878);
    let media = Arc::new(media_engine::MediaEngine::new(dir.clone(), preview_port));
    let registry = Arc::new(RegistryService);

    let state = AppState {
        db,
        bus,
        media,
        registry,
    };

    let port: u16 = std::env::var("VENUE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3010);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let app = router(state);

    tracing::info!("SlalomStream Venue Node listening on http://{addr}");
    tracing::info!("WebSocket: ws://127.0.0.1:{port}/ws");
    tracing::info!("Data directory: {}", dir.display());

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
