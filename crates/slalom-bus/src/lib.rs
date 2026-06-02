use std::sync::Arc;

use serde::Serialize;
use slalom_core::events::VenueEvent;
use tokio::sync::{broadcast, RwLock};

const CAPACITY: usize = 256;

#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<String>,
    last_seq: Arc<RwLock<u64>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(CAPACITY);
        Self {
            tx,
            last_seq: Arc::new(RwLock::new(0)),
        }
    }
}

impl EventBus {
    pub async fn publish(&self, event: &VenueEvent) -> u64 {
        let mut seq = self.last_seq.write().await;
        *seq += 1;
        let envelope = Envelope {
            seq: *seq,
            event_type: event.type_name().to_string(),
            payload: event,
        };
        let json = serde_json::to_string(&envelope).unwrap_or_default();
        let _ = self.tx.send(json);
        *seq
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }
}

#[derive(Serialize)]
struct Envelope<'a> {
    seq: u64,
    event_type: String,
    payload: &'a VenueEvent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use slalom_core::events::VenueEvent;

    #[tokio::test]
    async fn broadcast_reaches_subscriber() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();
        bus.publish(&VenueEvent::MediaState {
            state: "Armed".into(),
        })
        .await;
        let msg = rx.try_recv().unwrap();
        assert!(msg.contains("media.state"));
    }
}
