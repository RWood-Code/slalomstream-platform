use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TournamentStatus {
    Upcoming,
    Active,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassStatus {
    Pending,
    Scored,
    Complete,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub judge_count: i32,
    pub tournament_class: String,
    pub num_rounds: i32,
    pub is_test: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skier {
    pub id: i64,
    pub tournament_id: i64,
    pub first_name: String,
    pub surname: String,
    pub division: Option<String>,
    pub pin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pass {
    pub id: i64,
    pub tournament_id: i64,
    pub skier_id: i64,
    pub skier_name: String,
    pub division: Option<String>,
    pub rope_length: f32,
    pub speed_kph: Option<f32>,
    pub round_number: i32,
    pub buoys_scored: Option<f32>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeScore {
    pub id: i64,
    pub pass_id: i64,
    pub tournament_id: i64,
    pub judge_name: String,
    pub judge_role: String,
    pub pass_score: String,
    pub submitted_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub id: i64,
    pub pass_id: Option<i64>,
    pub tournament_id: Option<i64>,
    pub file_path: String,
    pub markers_json: Option<String>,
    pub started_at: String,
    pub ended_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub active_tournament_id: Option<i64>,
    pub surepath_enabled: bool,
    pub surepath_event_name: Option<String>,
}
