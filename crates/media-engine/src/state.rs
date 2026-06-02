use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MediaState {
    Idle,
    PreviewReady,
    Armed,
    Recording,
    Finalizing,
}

impl MediaState {
    pub fn as_str(self) -> &'static str {
        match self {
            MediaState::Idle => "Idle",
            MediaState::PreviewReady => "PreviewReady",
            MediaState::Armed => "Armed",
            MediaState::Recording => "Recording",
            MediaState::Finalizing => "Finalizing",
        }
    }
}
