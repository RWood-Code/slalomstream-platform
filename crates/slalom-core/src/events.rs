use serde::{Deserialize, Serialize};

/// Append-only venue events — broadcast on WebSocket as `venue.event`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum VenueEvent {
    #[serde(rename = "tournament.created")]
    TournamentCreated { id: i64, name: String },
    #[serde(rename = "tournament.updated")]
    TournamentUpdated { id: i64 },
    #[serde(rename = "pass.created")]
    PassCreated { id: i64, tournament_id: i64, status: String },
    #[serde(rename = "pass.updated")]
    PassUpdated { id: i64, tournament_id: i64, status: String },
    #[serde(rename = "score.submitted")]
    ScoreSubmitted {
        pass_id: i64,
        tournament_id: i64,
        judge_role: String,
    },
    #[serde(rename = "recording.linked")]
    RecordingLinked { recording_id: i64, pass_id: i64 },
    #[serde(rename = "media.state")]
    MediaState { state: String },
    #[serde(rename = "registry.updated")]
    RegistryUpdated { version: String },
}

impl VenueEvent {
    pub fn type_name(&self) -> &'static str {
        match self {
            VenueEvent::TournamentCreated { .. } => "tournament.created",
            VenueEvent::TournamentUpdated { .. } => "tournament.updated",
            VenueEvent::PassCreated { .. } => "pass.created",
            VenueEvent::PassUpdated { .. } => "pass.updated",
            VenueEvent::ScoreSubmitted { .. } => "score.submitted",
            VenueEvent::RecordingLinked { .. } => "recording.linked",
            VenueEvent::MediaState { .. } => "media.state",
            VenueEvent::RegistryUpdated { .. } => "registry.updated",
        }
    }
}
