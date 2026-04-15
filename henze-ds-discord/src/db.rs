//! Database module for storing bet runs and bets.
//!
//! Uses SQLite for persistence with rusqlite.

use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::info;

/// Thread-safe database connection wrapper.
pub type DbPool = Arc<Mutex<Connection>>;

/// Bet run status values.
#[derive(Debug, Clone, PartialEq)]
pub enum RunStatus {
    Active,
    Lost,
    Completed,
}

impl RunStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RunStatus::Active => "active",
            RunStatus::Lost => "lost",
            RunStatus::Completed => "completed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(RunStatus::Active),
            "lost" => Some(RunStatus::Lost),
            "completed" => Some(RunStatus::Completed),
            _ => None,
        }
    }
}

/// Bet status values.
#[derive(Debug, Clone, PartialEq)]
pub enum BetStatus {
    Pending,
    Won,
    Lost,
    Void,
}

impl BetStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BetStatus::Pending => "pending",
            BetStatus::Won => "won",
            BetStatus::Lost => "lost",
            BetStatus::Void => "void",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(BetStatus::Pending),
            "won" => Some(BetStatus::Won),
            "lost" => Some(BetStatus::Lost),
            "void" => Some(BetStatus::Void),
            _ => None,
        }
    }
}

/// A bet run record.
#[derive(Debug, Clone)]
pub struct BetRun {
    pub id: i64,
    pub guild_id: String,
    pub channel_id: String,
    pub created_by: String,
    pub starting_amount: f64,
    pub current_amount: f64,
    pub status: RunStatus,
    pub created_at: String,
}

/// A bet record.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Bet {
    pub id: i64,
    pub run_id: i64,
    pub event_id: String,
    pub outcome_id: String,
    pub market_id: String,
    pub event_name: String,
    pub market_name: String,
    pub outcome_name: String,
    pub odds: f64,
    pub stake: f64,
    pub potential_return: f64,
    pub status: BetStatus,
    pub placed_by: String,
    pub placed_at: String,
    pub settled_at: Option<String>,
}

/// Initialize the database connection and create tables if they don't exist.
pub fn init_db(path: &Path) -> Result<DbPool> {
    info!("Initializing database at {:?}", path);
    let conn = Connection::open(path)?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS bet_runs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id TEXT NOT NULL,
            channel_id TEXT NOT NULL,
            created_by TEXT NOT NULL,
            starting_amount REAL NOT NULL,
            current_amount REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS bets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id INTEGER NOT NULL REFERENCES bet_runs(id),
            event_id TEXT NOT NULL,
            outcome_id TEXT NOT NULL,
            market_id TEXT NOT NULL,
            event_name TEXT NOT NULL,
            market_name TEXT NOT NULL,
            outcome_name TEXT NOT NULL,
            odds REAL NOT NULL,
            stake REAL NOT NULL,
            potential_return REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            placed_by TEXT NOT NULL,
            placed_at TEXT NOT NULL DEFAULT (datetime('now')),
            settled_at TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_bet_runs_guild ON bet_runs(guild_id);
        CREATE INDEX IF NOT EXISTS idx_bet_runs_status ON bet_runs(status);
        CREATE INDEX IF NOT EXISTS idx_bets_run_id ON bets(run_id);
        CREATE INDEX IF NOT EXISTS idx_bets_status ON bets(status);
        "#,
    )?;

    info!("Database initialized successfully");
    Ok(Arc::new(Mutex::new(conn)))
}

/// Create a new bet run.
pub fn create_bet_run(
    db: &DbPool,
    guild_id: &str,
    channel_id: &str,
    created_by: &str,
    starting_amount: f64,
) -> Result<BetRun> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO bet_runs (guild_id, channel_id, created_by, starting_amount, current_amount)
         VALUES (?1, ?2, ?3, ?4, ?4)",
        params![guild_id, channel_id, created_by, starting_amount],
    )?;

    let id = conn.last_insert_rowid();
    let run = get_bet_run_by_id_internal(&conn, id)?;
    Ok(run.expect("Just inserted row should exist"))
}

/// Get a bet run by internal ID.
fn get_bet_run_by_id_internal(conn: &Connection, id: i64) -> Result<Option<BetRun>> {
    let mut stmt = conn.prepare(
        "SELECT id, guild_id, channel_id, created_by, starting_amount, current_amount, status, created_at
         FROM bet_runs WHERE id = ?1",
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(BetRun {
            id: row.get(0)?,
            guild_id: row.get(1)?,
            channel_id: row.get(2)?,
            created_by: row.get(3)?,
            starting_amount: row.get(4)?,
            current_amount: row.get(5)?,
            status: RunStatus::from_str(&row.get::<_, String>(6)?).unwrap_or(RunStatus::Active),
            created_at: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

/// Get a bet run by internal ID (public).
pub fn get_bet_run_by_id(db: &DbPool, id: i64) -> Result<Option<BetRun>> {
    let conn = db.lock().unwrap();
    get_bet_run_by_id_internal(&conn, id)
}

/// Get run number for a bet run within a guild (1-based sequence).
pub fn get_run_number(db: &DbPool, guild_id: &str, run_id: i64) -> Result<Option<i64>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM bet_runs WHERE guild_id = ?1 AND id <= ?2 ORDER BY id",
    )?;
    let count: i64 = stmt.query_row(params![guild_id, run_id], |row| row.get(0))?;
    if count > 0 {
        Ok(Some(count))
    } else {
        Ok(None)
    }
}

/// Get a bet run by its guild-specific run number (1, 2, 3...).
pub fn get_bet_run_by_number(db: &DbPool, guild_id: &str, run_number: i64) -> Result<Option<BetRun>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, guild_id, channel_id, created_by, starting_amount, current_amount, status, created_at
         FROM bet_runs WHERE guild_id = ?1 ORDER BY id LIMIT 1 OFFSET ?2",
    )?;

    let offset = run_number - 1;
    let mut rows = stmt.query(params![guild_id, offset])?;
    if let Some(row) = rows.next()? {
        Ok(Some(BetRun {
            id: row.get(0)?,
            guild_id: row.get(1)?,
            channel_id: row.get(2)?,
            created_by: row.get(3)?,
            starting_amount: row.get(4)?,
            current_amount: row.get(5)?,
            status: RunStatus::from_str(&row.get::<_, String>(6)?).unwrap_or(RunStatus::Active),
            created_at: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

/// Get the latest active bet run for a guild.
pub fn get_active_run(db: &DbPool, guild_id: &str) -> Result<Option<BetRun>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, guild_id, channel_id, created_by, starting_amount, current_amount, status, created_at
         FROM bet_runs WHERE guild_id = ?1 AND status = 'active' ORDER BY id DESC LIMIT 1",
    )?;

    let mut rows = stmt.query(params![guild_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(BetRun {
            id: row.get(0)?,
            guild_id: row.get(1)?,
            channel_id: row.get(2)?,
            created_by: row.get(3)?,
            starting_amount: row.get(4)?,
            current_amount: row.get(5)?,
            status: RunStatus::from_str(&row.get::<_, String>(6)?).unwrap_or(RunStatus::Active),
            created_at: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

/// Update bet run status and current amount.
pub fn update_bet_run(db: &DbPool, run_id: i64, status: RunStatus, current_amount: f64) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE bet_runs SET status = ?1, current_amount = ?2 WHERE id = ?3",
        params![status.as_str(), current_amount, run_id],
    )?;
    Ok(())
}

/// Create a new bet.
pub fn create_bet(
    db: &DbPool,
    run_id: i64,
    event_id: &str,
    outcome_id: &str,
    market_id: &str,
    event_name: &str,
    market_name: &str,
    outcome_name: &str,
    odds: f64,
    stake: f64,
    placed_by: &str,
) -> Result<Bet> {
    let potential_return = stake * odds;
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO bets (run_id, event_id, outcome_id, market_id, event_name, market_name, outcome_name, odds, stake, potential_return, placed_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![run_id, event_id, outcome_id, market_id, event_name, market_name, outcome_name, odds, stake, potential_return, placed_by],
    )?;

    let id = conn.last_insert_rowid();
    let bet = get_bet_by_id_internal(&conn, id)?;
    Ok(bet.expect("Just inserted row should exist"))
}

/// Get a bet by internal ID.
fn get_bet_by_id_internal(conn: &Connection, id: i64) -> Result<Option<Bet>> {
    let mut stmt = conn.prepare(
        "SELECT id, run_id, event_id, outcome_id, market_id, event_name, market_name, outcome_name, 
                odds, stake, potential_return, status, placed_by, placed_at, settled_at
         FROM bets WHERE id = ?1",
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Bet {
            id: row.get(0)?,
            run_id: row.get(1)?,
            event_id: row.get(2)?,
            outcome_id: row.get(3)?,
            market_id: row.get(4)?,
            event_name: row.get(5)?,
            market_name: row.get(6)?,
            outcome_name: row.get(7)?,
            odds: row.get(8)?,
            stake: row.get(9)?,
            potential_return: row.get(10)?,
            status: BetStatus::from_str(&row.get::<_, String>(11)?).unwrap_or(BetStatus::Pending),
            placed_by: row.get(12)?,
            placed_at: row.get(13)?,
            settled_at: row.get(14)?,
        }))
    } else {
        Ok(None)
    }
}

/// Get all bets for a run.
pub fn get_bets_for_run(db: &DbPool, run_id: i64) -> Result<Vec<Bet>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, run_id, event_id, outcome_id, market_id, event_name, market_name, outcome_name, 
                odds, stake, potential_return, status, placed_by, placed_at, settled_at
         FROM bets WHERE run_id = ?1 ORDER BY id",
    )?;

    let rows = stmt.query_map(params![run_id], |row| {
        Ok(Bet {
            id: row.get(0)?,
            run_id: row.get(1)?,
            event_id: row.get(2)?,
            outcome_id: row.get(3)?,
            market_id: row.get(4)?,
            event_name: row.get(5)?,
            market_name: row.get(6)?,
            outcome_name: row.get(7)?,
            odds: row.get(8)?,
            stake: row.get(9)?,
            potential_return: row.get(10)?,
            status: BetStatus::from_str(&row.get::<_, String>(11)?).unwrap_or(BetStatus::Pending),
            placed_by: row.get(12)?,
            placed_at: row.get(13)?,
            settled_at: row.get(14)?,
        })
    })?;

    rows.collect()
}

/// Check if a run has a pending bet.
pub fn has_pending_bet(db: &DbPool, run_id: i64) -> Result<bool> {
    let conn = db.lock().unwrap();
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM bets WHERE run_id = ?1 AND status = 'pending'",
        params![run_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Get all pending bets across all runs (for settlement polling).
pub fn get_all_pending_bets(db: &DbPool) -> Result<Vec<(Bet, BetRun)>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT b.id, b.run_id, b.event_id, b.outcome_id, b.market_id, b.event_name, b.market_name, 
                b.outcome_name, b.odds, b.stake, b.potential_return, b.status, b.placed_by, b.placed_at, b.settled_at,
                r.id, r.guild_id, r.channel_id, r.created_by, r.starting_amount, r.current_amount, r.status, r.created_at
         FROM bets b
         JOIN bet_runs r ON b.run_id = r.id
         WHERE b.status = 'pending'",
    )?;

    let rows = stmt.query_map([], |row| {
        let bet = Bet {
            id: row.get(0)?,
            run_id: row.get(1)?,
            event_id: row.get(2)?,
            outcome_id: row.get(3)?,
            market_id: row.get(4)?,
            event_name: row.get(5)?,
            market_name: row.get(6)?,
            outcome_name: row.get(7)?,
            odds: row.get(8)?,
            stake: row.get(9)?,
            potential_return: row.get(10)?,
            status: BetStatus::from_str(&row.get::<_, String>(11)?).unwrap_or(BetStatus::Pending),
            placed_by: row.get(12)?,
            placed_at: row.get(13)?,
            settled_at: row.get(14)?,
        };
        let run = BetRun {
            id: row.get(15)?,
            guild_id: row.get(16)?,
            channel_id: row.get(17)?,
            created_by: row.get(18)?,
            starting_amount: row.get(19)?,
            current_amount: row.get(20)?,
            status: RunStatus::from_str(&row.get::<_, String>(21)?).unwrap_or(RunStatus::Active),
            created_at: row.get(22)?,
        };
        Ok((bet, run))
    })?;

    rows.collect()
}

/// Update bet status and settlement time.
pub fn update_bet_status(db: &DbPool, bet_id: i64, status: BetStatus) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE bets SET status = ?1, settled_at = datetime('now') WHERE id = ?2",
        params![status.as_str(), bet_id],
    )?;
    Ok(())
}

/// Count total runs for a guild (for run numbering).
pub fn count_runs_for_guild(db: &DbPool, guild_id: &str) -> Result<i64> {
    let conn = db.lock().unwrap();
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM bet_runs WHERE guild_id = ?1",
        params![guild_id],
        |row| row.get(0),
    )?;
    Ok(count)
}
