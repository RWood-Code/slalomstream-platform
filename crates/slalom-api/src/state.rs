use std::sync::Arc;

use media_engine::MediaEngine;
use slalom_bus::EventBus;
use slalom_core::db::Database;
use slalom_registry::RegistryService;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub bus: EventBus,
    pub media: Arc<MediaEngine>,
    pub registry: Arc<RegistryService>,
}

impl AppState {
    pub async fn publish(&self, event: slalom_core::events::VenueEvent) {
        let payload = serde_json::to_string(&event).unwrap_or_default();
        let _ = self.db.append_event(event.type_name(), &payload);
        self.bus.publish(&event).await;
    }
}
