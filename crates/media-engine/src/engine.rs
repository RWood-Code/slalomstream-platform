use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::preflight::{run_preflight, PreflightReport};
use crate::state::MediaState;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MediaStatus {
    pub state: MediaState,
    pub preview_url: Option<String>,
    pub active_recording_id: Option<i64>,
    pub encoder_profile: String,
}

pub struct MediaEngine {
    state: Arc<RwLock<MediaState>>,
    data_dir: PathBuf,
    recordings_dir: PathBuf,
    preview_port: u16,
    active_recording_id: Arc<RwLock<Option<i64>>>,
    encoder_profile: String,
    pre_roll_secs: u32,
}

impl MediaEngine {
    pub fn new(data_dir: PathBuf, preview_port: u16) -> Self {
        let recordings_dir = data_dir.join("recordings");
        Self {
            state: Arc::new(RwLock::new(MediaState::Idle)),
            data_dir,
            recordings_dir,
            preview_port,
            active_recording_id: Arc::new(RwLock::new(None)),
            encoder_profile: "high".into(),
            pre_roll_secs: 5,
        }
    }

    pub async fn status(&self) -> MediaStatus {
        MediaStatus {
            state: *self.state.read().await,
            preview_url: Some(format!("http://127.0.0.1:{}/", self.preview_port)),
            active_recording_id: *self.active_recording_id.read().await,
            encoder_profile: self.encoder_profile.clone(),
        }
    }

    pub fn preflight(&self) -> PreflightReport {
        run_preflight(&self.data_dir, &self.recordings_dir, 10)
    }

    pub async fn start_preview(&self) -> Result<(), String> {
        let mut s = self.state.write().await;
        if *s == MediaState::Recording {
            return Err("cannot start preview while recording".into());
        }
        *s = MediaState::PreviewReady;
        Ok(())
    }

    pub async fn arm(&self) -> Result<(), String> {
        let report = self.preflight();
        if !report.ready {
            return Err("preflight failed".into());
        }
        let mut s = self.state.write().await;
        if *s == MediaState::Idle {
            *s = MediaState::PreviewReady;
        }
        *s = MediaState::Armed;
        Ok(())
    }

    pub async fn disarm(&self) -> Result<(), String> {
        let mut s = self.state.write().await;
        *s = MediaState::PreviewReady;
        Ok(())
    }

    /// Simulated record start — Phase 3 wires FFmpeg. Returns recording row id when armed.
    pub async fn start_recording(
        &self,
        pass_id: i64,
        tournament_id: i64,
    ) -> Result<i64, String> {
        let mut s = self.state.write().await;
        if *s != MediaState::Armed && *s != MediaState::PreviewReady {
            return Err(format!("cannot record in state {}", s.as_str()));
        }
        *s = MediaState::Recording;

        let filename = format!("pass_{pass_id}_{}.mp4", chrono_like_ts());
        let path = self.recordings_dir.join(&filename);
        std::fs::create_dir_all(&self.recordings_dir).map_err(|e| e.to_string())?;
        // Placeholder file — real implementation streams via FFmpeg
        std::fs::write(&path, b"").map_err(|e| e.to_string())?;

        let id = pass_id; // caller links via RecordingRepo; use pass_id as temp id until DB insert in API
        *self.active_recording_id.write().await = Some(id);
        let _ = tournament_id;
        Ok(id)
    }

    pub async fn stop_recording(&self, recording_id: i64) -> Result<(), String> {
        let mut s = self.state.write().await;
        *s = MediaState::Finalizing;
        *self.active_recording_id.write().await = None;
        *s = MediaState::Armed;
        let _ = recording_id;
        Ok(())
    }

    pub async fn on_pass_pending(
        &self,
        pass_id: i64,
        tournament_id: i64,
    ) -> Result<Option<i64>, String> {
        if *self.state.read().await != MediaState::Armed {
            return Ok(None);
        }
        let id = self.start_recording(pass_id, tournament_id).await?;
        Ok(Some(id))
    }

    pub fn set_encoder_profile(&mut self, profile: &str) {
        self.encoder_profile = profile.to_string();
    }

    pub fn pre_roll_secs(&self) -> u32 {
        self.pre_roll_secs
    }

    pub fn set_pre_roll_secs(&mut self, secs: u32) {
        self.pre_roll_secs = secs;
    }

    pub fn recordings_dir(&self) -> &PathBuf {
        &self.recordings_dir
    }
}

fn chrono_like_ts() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn arm_requires_preflight_pass() {
        let dir = std::env::temp_dir().join(format!("slalom-media-{}", uuid_simple()));
        let engine = MediaEngine::new(dir.clone(), 9878);
        engine.start_preview().await.unwrap();
        engine.arm().await.unwrap();
        assert_eq!(engine.status().await.state, MediaState::Armed);
        let _ = std::fs::remove_dir_all(dir);
    }

    fn uuid_simple() -> u32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }
}
