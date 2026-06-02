use rusqlite::params;
use serde::{Deserialize, Serialize};
use slalom_core::db::Database;
use slalom_core::error::{CoreError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialRow {
    pub id: i64,
    pub federation_code: Option<String>,
    pub first_name: String,
    pub surname: String,
    pub region: Option<String>,
    pub slalom_grade: Option<String>,
    pub is_active: bool,
    pub pin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMeta {
    pub version: String,
    pub imported_at: Option<String>,
    pub source: Option<String>,
}

pub struct RegistryService;

impl RegistryService {
    pub fn meta(db: &Database) -> Result<RegistryMeta> {
        db.with_conn(|conn| {
            conn.query_row(
                "SELECT version, imported_at, source FROM registry_meta WHERE id = 1",
                [],
                |row| {
                    Ok(RegistryMeta {
                        version: row.get(0)?,
                        imported_at: row.get(1)?,
                        source: row.get(2)?,
                    })
                },
            )
            .map_err(CoreError::from)
        })
    }

    pub fn list(db: &Database) -> Result<Vec<OfficialRow>> {
        db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, federation_code, first_name, surname, region, slalom_grade, is_active, pin
                 FROM officials WHERE is_active = 1 ORDER BY surname, first_name",
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(OfficialRow {
                    id: row.get(0)?,
                    federation_code: row.get(1)?,
                    first_name: row.get(2)?,
                    surname: row.get(3)?,
                    region: row.get(4)?,
                    slalom_grade: row.get(5)?,
                    is_active: row.get::<_, i32>(6)? != 0,
                    pin: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(CoreError::from)
        })
    }

    /// Import officials from CSV with headers:
    /// federation_code,first_name,surname,region,slalom_grade,certification_expires
    pub fn import_csv(db: &Database, csv_data: &str, version: &str, source: &str) -> Result<usize> {
        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        let mut count = 0usize;
        db.with_conn(|conn| {
            let tx = conn.unchecked_transaction()?;
            for result in rdr.records() {
                let record = result.map_err(|e| CoreError::Validation(e.to_string()))?;
                if record.len() < 3 {
                    continue;
                }
                tx.execute(
                    "INSERT INTO officials (federation_code, first_name, surname, region, slalom_grade, certification_expires, is_active)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)",
                    params![
                        nullable(&record, 0),
                        &record[1],
                        &record[2],
                        nullable(&record, 3),
                        nullable(&record, 4),
                        nullable(&record, 5),
                    ],
                )?;
                count += 1;
            }
            tx.execute(
                "UPDATE registry_meta SET version = ?1, imported_at = datetime('now'), source = ?2 WHERE id = 1",
                params![version, source],
            )?;
            tx.commit()?;
            Ok(count)
        })
    }
}

fn nullable(record: &csv::StringRecord, idx: usize) -> Option<String> {
    record.get(idx).filter(|s| !s.is_empty()).map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_csv_updates_version() {
        let db = Database::open_in_memory().unwrap();
        let csv = "federation_code,first_name,surname,region,slalom_grade,certification_expires\nNZTWSA,Jane,Doe,Canterbury,J2*,2027-12-31\n";
        let n = RegistryService::import_csv(&db, csv, "v1", "test").unwrap();
        assert_eq!(n, 1);
        let meta = RegistryService::meta(&db).unwrap();
        assert_eq!(meta.version, "v1");
        assert_eq!(RegistryService::list(&db).unwrap().len(), 1);
    }
}
