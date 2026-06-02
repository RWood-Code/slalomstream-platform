use rusqlite::{params, OptionalExtension};

use crate::db::Database;
use crate::domain::{AppSettings, JudgeScore, Pass, Recording, Skier, Tournament};
use crate::error::{CoreError, Result};
use crate::events::VenueEvent;
use slalom_scoring::{is_valid_score, scoring_roles};

pub struct TournamentRepo;

impl TournamentRepo {
    pub fn list(db: &Database) -> Result<Vec<Tournament>> {
        db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, status, judge_count, tournament_class, num_rounds, is_test, created_at
                 FROM tournaments ORDER BY id DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Tournament {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    status: row.get(2)?,
                    judge_count: row.get(3)?,
                    tournament_class: row.get(4)?,
                    num_rounds: row.get(5)?,
                    is_test: row.get::<_, i32>(6)? != 0,
                    created_at: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(CoreError::from)
        })
    }

    pub fn create(db: &Database, name: &str, judge_count: i32) -> Result<(Tournament, VenueEvent)> {
        db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO tournaments (name, status, judge_count) VALUES (?1, 'upcoming', ?2)",
                params![name, judge_count],
            )?;
            let id = conn.last_insert_rowid();
            let t = Self::get_by_id_conn(conn, id)?;
            Ok((
                t.clone(),
                VenueEvent::TournamentCreated {
                    id,
                    name: name.to_string(),
                },
            ))
        })
    }

    pub fn get(db: &Database, id: i64) -> Result<Tournament> {
        db.with_conn(|conn| Self::get_by_id_conn(conn, id))
    }

    fn get_by_id_conn(conn: &rusqlite::Connection, id: i64) -> Result<Tournament> {
        conn.query_row(
            "SELECT id, name, status, judge_count, tournament_class, num_rounds, is_test, created_at
             FROM tournaments WHERE id = ?1",
            params![id],
            |row| {
                Ok(Tournament {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    status: row.get(2)?,
                    judge_count: row.get(3)?,
                    tournament_class: row.get(4)?,
                    num_rounds: row.get(5)?,
                    is_test: row.get::<_, i32>(6)? != 0,
                    created_at: row.get(7)?,
                })
            },
        )
        .optional()?
        .ok_or_else(|| CoreError::NotFound(format!("tournament {id}")))
    }

    pub fn set_active(db: &Database, tournament_id: i64) -> Result<()> {
        Self::get(db, tournament_id)?;
        db.with_conn(|conn| {
            conn.execute(
                "UPDATE app_settings SET active_tournament_id = ?1 WHERE id = 1",
                params![tournament_id],
            )?;
            Ok(())
        })
    }
}

pub struct SkierRepo;

impl SkierRepo {
    pub fn create(
        db: &Database,
        tournament_id: i64,
        first_name: &str,
        surname: &str,
        division: Option<&str>,
    ) -> Result<Skier> {
        TournamentRepo::get(db, tournament_id)?;
        db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO skiers (tournament_id, first_name, surname, division)
                 VALUES (?1, ?2, ?3, ?4)",
                params![tournament_id, first_name, surname, division],
            )?;
            let id = conn.last_insert_rowid();
            Ok(Skier {
                id,
                tournament_id,
                first_name: first_name.to_string(),
                surname: surname.to_string(),
                division: division.map(String::from),
                pin: None,
            })
        })
    }

    pub fn list_for_tournament(db: &Database, tournament_id: i64) -> Result<Vec<Skier>> {
        db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, tournament_id, first_name, surname, division, pin
                 FROM skiers WHERE tournament_id = ?1 ORDER BY surname, first_name",
            )?;
            let rows = stmt.query_map(params![tournament_id], |row| {
                Ok(Skier {
                    id: row.get(0)?,
                    tournament_id: row.get(1)?,
                    first_name: row.get(2)?,
                    surname: row.get(3)?,
                    division: row.get(4)?,
                    pin: row.get(5)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(CoreError::from)
        })
    }
}

pub struct PassRepo;

impl PassRepo {
    pub fn create(
        db: &Database,
        tournament_id: i64,
        skier_id: i64,
        skier_name: &str,
        rope_length: f32,
        round_number: i32,
    ) -> Result<(Pass, VenueEvent)> {
        TournamentRepo::get(db, tournament_id)?;
        db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO passes (tournament_id, skier_id, skier_name, rope_length, round_number, status)
                 VALUES (?1, ?2, ?3, ?4, ?5, 'pending')",
                params![tournament_id, skier_id, skier_name, rope_length, round_number],
            )?;
            let id = conn.last_insert_rowid();
            let p = Self::get_by_id_conn(conn, id)?;
            Ok((
                p.clone(),
                VenueEvent::PassCreated {
                    id,
                    tournament_id,
                    status: p.status,
                },
            ))
        })
    }

    pub fn list_for_tournament(db: &Database, tournament_id: i64) -> Result<Vec<Pass>> {
        db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, tournament_id, skier_id, skier_name, division, rope_length, speed_kph,
                        round_number, buoys_scored, status, notes, created_at
                 FROM passes WHERE tournament_id = ?1 ORDER BY id DESC",
            )?;
            let rows = stmt.query_map(params![tournament_id], map_pass)?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(CoreError::from)
        })
    }

    pub fn get(db: &Database, id: i64) -> Result<Pass> {
        db.with_conn(|conn| Self::get_by_id_conn(conn, id))
    }

    fn get_by_id_conn(conn: &rusqlite::Connection, id: i64) -> Result<Pass> {
        conn.query_row(
            "SELECT id, tournament_id, skier_id, skier_name, division, rope_length, speed_kph,
                    round_number, buoys_scored, status, notes, created_at
             FROM passes WHERE id = ?1",
            params![id],
            map_pass,
        )
        .optional()?
        .ok_or_else(|| CoreError::NotFound(format!("pass {id}")))
    }

    pub fn pending_for_tournament(db: &Database, tournament_id: i64) -> Result<Option<Pass>> {
        db.with_conn(|conn| {
            conn.query_row(
                "SELECT id, tournament_id, skier_id, skier_name, division, rope_length, speed_kph,
                        round_number, buoys_scored, status, notes, created_at
                 FROM passes WHERE tournament_id = ?1 AND status = 'pending'
                 ORDER BY id DESC LIMIT 1",
                params![tournament_id],
                map_pass,
            )
            .optional()
            .map_err(CoreError::from)
        })
    }

    pub fn update_status(db: &Database, id: i64, status: &str) -> Result<(Pass, VenueEvent)> {
        db.with_conn(|conn| {
            let existing = Self::get_by_id_conn(conn, id)?;
            conn.execute(
                "UPDATE passes SET status = ?1 WHERE id = ?2",
                params![status, id],
            )?;
            let p = Self::get_by_id_conn(conn, id)?;
            Ok((
                p.clone(),
                VenueEvent::PassUpdated {
                    id,
                    tournament_id: existing.tournament_id,
                    status: status.to_string(),
                },
            ))
        })
    }
}

fn map_pass(row: &rusqlite::Row<'_>) -> rusqlite::Result<Pass> {
    Ok(Pass {
        id: row.get(0)?,
        tournament_id: row.get(1)?,
        skier_id: row.get(2)?,
        skier_name: row.get(3)?,
        division: row.get(4)?,
        rope_length: row.get(5)?,
        speed_kph: row.get(6)?,
        round_number: row.get(7)?,
        buoys_scored: row.get(8)?,
        status: row.get(9)?,
        notes: row.get(10)?,
        created_at: row.get(11)?,
    })
}

pub struct ScoreRepo;

impl ScoreRepo {
    pub fn submit(
        db: &Database,
        pass_id: i64,
        judge_name: &str,
        judge_role: &str,
        pass_score: &str,
    ) -> Result<(JudgeScore, Vec<VenueEvent>)> {
        if !is_valid_score(pass_score) {
            return Err(CoreError::Validation(format!(
                "invalid IWWF score: {pass_score}"
            )));
        }
        let pass = PassRepo::get(db, pass_id)?;
        let tournament = TournamentRepo::get(db, pass.tournament_id)?;
        let roles = scoring_roles(tournament.judge_count);
        if !roles.iter().any(|r| r == judge_role) {
            return Err(CoreError::Validation(format!(
                "role {judge_role} not in panel"
            )));
        }

        db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO judge_scores (pass_id, tournament_id, judge_name, judge_role, pass_score)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    pass_id,
                    pass.tournament_id,
                    judge_name,
                    judge_role,
                    pass_score
                ],
            )?;
            let id = conn.last_insert_rowid();
            let score = JudgeScore {
                id,
                pass_id,
                tournament_id: pass.tournament_id,
                judge_name: judge_name.to_string(),
                judge_role: judge_role.to_string(),
                pass_score: pass_score.to_string(),
                submitted_at: chrono::Utc::now().to_rfc3339(),
            };

            let count: i64 = conn.query_row(
                "SELECT COUNT(DISTINCT judge_role) FROM judge_scores WHERE pass_id = ?1",
                params![pass_id],
                |r| r.get(0),
            )?;

            let mut events = vec![VenueEvent::ScoreSubmitted {
                pass_id,
                tournament_id: pass.tournament_id,
                judge_role: judge_role.to_string(),
            }];

            if count >= roles.len() as i64 {
                conn.execute(
                    "UPDATE passes SET status = 'scored' WHERE id = ?1",
                    params![pass_id],
                )?;
                events.push(VenueEvent::PassUpdated {
                    id: pass_id,
                    tournament_id: pass.tournament_id,
                    status: "scored".into(),
                });
            }

            Ok((score, events))
        })
    }

    pub fn list_for_pass(db: &Database, pass_id: i64) -> Result<Vec<JudgeScore>> {
        db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, pass_id, tournament_id, judge_name, judge_role, pass_score, submitted_at
                 FROM judge_scores WHERE pass_id = ?1",
            )?;
            let rows = stmt.query_map(params![pass_id], |row| {
                Ok(JudgeScore {
                    id: row.get(0)?,
                    pass_id: row.get(1)?,
                    tournament_id: row.get(2)?,
                    judge_name: row.get(3)?,
                    judge_role: row.get(4)?,
                    pass_score: row.get(5)?,
                    submitted_at: row.get(6)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(CoreError::from)
        })
    }
}

pub struct RecordingRepo;

impl RecordingRepo {
    pub fn create(
        db: &Database,
        file_path: &str,
        pass_id: Option<i64>,
        tournament_id: Option<i64>,
    ) -> Result<Recording> {
        db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO recordings (pass_id, tournament_id, file_path)
                 VALUES (?1, ?2, ?3)",
                params![pass_id, tournament_id, file_path],
            )?;
            let id = conn.last_insert_rowid();
            Ok(Recording {
                id,
                pass_id,
                tournament_id,
                file_path: file_path.to_string(),
                markers_json: None,
                started_at: chrono::Utc::now().to_rfc3339(),
                ended_at: None,
            })
        })
    }

    pub fn link_pass(db: &Database, recording_id: i64, pass_id: i64) -> Result<VenueEvent> {
        db.with_conn(|conn| {
            conn.execute(
                "UPDATE recordings SET pass_id = ?1 WHERE id = ?2",
                params![pass_id, recording_id],
            )?;
            Ok(VenueEvent::RecordingLinked {
                recording_id,
                pass_id,
            })
        })
    }

    pub fn finalize(
        db: &Database,
        recording_id: i64,
        markers_json: Option<&str>,
    ) -> Result<()> {
        db.with_conn(|conn| {
            conn.execute(
                "UPDATE recordings SET ended_at = datetime('now'), markers_json = ?1 WHERE id = ?2",
                params![markers_json, recording_id],
            )?;
            Ok(())
        })
    }
}

pub struct SettingsRepo;

impl SettingsRepo {
    pub fn get(db: &Database) -> Result<AppSettings> {
        db.with_conn(|conn| {
            conn.query_row(
                "SELECT active_tournament_id, surepath_enabled, surepath_event_name
                 FROM app_settings WHERE id = 1",
                [],
                |row| {
                    Ok(AppSettings {
                        active_tournament_id: row.get(0)?,
                        surepath_enabled: row.get::<_, i32>(1)? != 0,
                        surepath_event_name: row.get(2)?,
                    })
                },
            )
            .map_err(CoreError::from)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tournament_pass_score_flow() {
        let db = Database::open_in_memory().unwrap();
        let (t, _) = TournamentRepo::create(&db, "Test Open", 3).unwrap();
        TournamentRepo::set_active(&db, t.id).unwrap();
        let skier = SkierRepo::create(&db, t.id, "Jane", "Doe", Some("Open Women")).unwrap();
        let (pass, _) =
            PassRepo::create(&db, t.id, skier.id, "Jane Doe", 11.25, 1).unwrap();
        assert_eq!(pass.status, "pending");

        for role in scoring_roles(3) {
            ScoreRepo::submit(&db, pass.id, "Judge", &role, "4").unwrap();
        }
        let updated = PassRepo::get(&db, pass.id).unwrap();
        assert_eq!(updated.status, "scored");
    }
}
