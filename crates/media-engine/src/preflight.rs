use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PreflightStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreflightCheck {
    pub name: String,
    pub status: PreflightStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreflightReport {
    pub ready: bool,
    pub checks: Vec<PreflightCheck>,
}

pub fn run_preflight(data_dir: &Path, recordings_dir: &Path, min_free_gb: u64) -> PreflightReport {
    let mut checks = Vec::new();

    let data_ok = write_probe(data_dir);
    checks.push(PreflightCheck {
        name: "data_dir_writable".into(),
        status: if data_ok {
            PreflightStatus::Pass
        } else {
            PreflightStatus::Fail
        },
        detail: data_dir.display().to_string(),
    });

    let rec_ok = write_probe(recordings_dir);
    checks.push(PreflightCheck {
        name: "recordings_dir_writable".into(),
        status: if rec_ok {
            PreflightStatus::Pass
        } else {
            PreflightStatus::Fail
        },
        detail: recordings_dir.display().to_string(),
    });

    checks.push(PreflightCheck {
        name: "disk_space".into(),
        status: PreflightStatus::Pass,
        detail: format!("Assumed OK in dev (min {min_free_gb} GB policy)"),
    });

    checks.push(PreflightCheck {
        name: "capture_device".into(),
        status: PreflightStatus::Warn,
        detail: "FFmpeg device probe runs when preview starts (Phase 3)".into(),
    });

    let ready = checks.iter().all(|c| c.status != PreflightStatus::Fail);
    PreflightReport { ready, checks }
}

fn write_probe(dir: &Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    let probe = dir.join(".slalomstream-write-probe");
    std::fs::write(&probe, b"ok").is_ok() && std::fs::remove_file(&probe).is_ok()
}
