CREATE TABLE IF NOT EXISTS tournaments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'upcoming',
  judge_count INTEGER NOT NULL DEFAULT 3,
  tournament_class TEXT NOT NULL DEFAULT 'G',
  num_rounds INTEGER NOT NULL DEFAULT 2,
  is_test INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS skiers (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  tournament_id INTEGER NOT NULL,
  first_name TEXT NOT NULL,
  surname TEXT NOT NULL,
  division TEXT,
  pin TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS passes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  tournament_id INTEGER NOT NULL,
  skier_id INTEGER NOT NULL,
  skier_name TEXT NOT NULL,
  division TEXT,
  rope_length REAL NOT NULL,
  speed_kph REAL,
  round_number INTEGER NOT NULL DEFAULT 1,
  buoys_scored REAL,
  status TEXT NOT NULL DEFAULT 'pending',
  notes TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS judge_scores (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pass_id INTEGER NOT NULL,
  tournament_id INTEGER NOT NULL,
  judge_name TEXT NOT NULL,
  judge_role TEXT NOT NULL,
  pass_score TEXT NOT NULL,
  submitted_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS recordings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pass_id INTEGER,
  tournament_id INTEGER,
  file_path TEXT NOT NULL,
  markers_json TEXT,
  started_at TEXT NOT NULL DEFAULT (datetime('now')),
  ended_at TEXT
);

CREATE TABLE IF NOT EXISTS app_settings (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  active_tournament_id INTEGER,
  surepath_enabled INTEGER NOT NULL DEFAULT 0,
  surepath_event_name TEXT,
  surepath_event_sub_id TEXT,
  surepath_observer_pin TEXT
);

INSERT OR IGNORE INTO app_settings (id) VALUES (1);

CREATE TABLE IF NOT EXISTS event_log (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  event_type TEXT NOT NULL,
  payload TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS officials (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  external_id TEXT,
  federation_code TEXT,
  first_name TEXT NOT NULL,
  surname TEXT NOT NULL,
  region TEXT,
  slalom_grade TEXT,
  certification_expires TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  pin TEXT,
  is_admin INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS registry_meta (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  version TEXT NOT NULL DEFAULT '0',
  imported_at TEXT,
  source TEXT
);

INSERT OR IGNORE INTO registry_meta (id, version) VALUES (1, '0');

CREATE INDEX IF NOT EXISTS idx_passes_tournament ON passes(tournament_id);
CREATE INDEX IF NOT EXISTS idx_passes_status ON passes(status);
CREATE INDEX IF NOT EXISTS idx_recordings_pass ON recordings(pass_id);
