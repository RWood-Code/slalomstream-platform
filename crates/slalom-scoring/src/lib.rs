//! IWWF slalom scoring rules — ported from SlalomStream v2 `utils.ts`.

pub const VALID_IWWF_SCORES: &[&str] = &[
    "1", "1.5", "2", "2.5", "3", "3.5", "4", "4.5", "5", "5.5", "6", "6_no_gates",
];

pub const ROPE_LENGTHS: &[f32] = &[
    23.0, 18.25, 16.0, 14.25, 13.0, 12.0, 11.25, 10.75, 10.25, 9.75,
];

pub const SPEEDS_KPH: &[f32] = &[34.0, 37.0, 40.0, 43.0, 46.0, 49.0, 52.0, 55.0, 58.0];

pub const TOURNAMENT_CLASSES: &[&str] = &["G", "L", "R", "E", "EMS"];

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PanelStation {
    pub role: String,
    pub label: String,
    pub short_label: String,
    pub is_boat: bool,
    pub is_chief_only: bool,
}

/// Panel size normalised to 1, 3, or 5 judges (IWWF).
pub fn normalised_judge_count(judge_count: i32) -> usize {
    if judge_count <= 1 {
        1
    } else if judge_count <= 3 {
        3
    } else {
        5
    }
}

pub fn judging_panel(judge_count: i32) -> Vec<PanelStation> {
    let count = normalised_judge_count(judge_count);
    let letters = ['A', 'B', 'C', 'D', 'E'];
    (0..count)
        .map(|i| {
            let is_last = i == count - 1;
            let is_boat = count > 1 && is_last;
            let letter = letters[i];
            PanelStation {
                role: format!("judge_{}", letter.to_ascii_lowercase()),
                label: if is_boat {
                    format!("Judge {letter} / Boat")
                } else {
                    format!("Judge {letter}")
                },
                short_label: letter.to_string(),
                is_boat,
                is_chief_only: false,
            }
        })
        .collect()
}

pub fn scoring_roles(judge_count: i32) -> Vec<String> {
    judging_panel(judge_count)
        .into_iter()
        .map(|s| s.role)
        .collect()
}

pub fn is_valid_score(score: &str) -> bool {
    VALID_IWWF_SCORES.contains(&score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_sizes() {
        assert_eq!(judging_panel(1).len(), 1);
        assert_eq!(judging_panel(2).len(), 3);
        assert_eq!(judging_panel(5).len(), 5);
        assert!(judging_panel(3).last().unwrap().is_boat);
    }

    #[test]
    fn valid_scores() {
        assert!(is_valid_score("6"));
        assert!(is_valid_score("6_no_gates"));
        assert!(!is_valid_score("7"));
    }
}
