use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::path::Path;
use std::sync::Mutex;

pub struct ResponseCache {
    conn: Mutex<Connection>,
}

impl ResponseCache {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS response_cache (
                cache_key TEXT PRIMARY KEY,
                payload TEXT NOT NULL,
                cached_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn get<T: DeserializeOwned>(&self, cache_key: &str) -> Result<Option<T>, Box<dyn Error>> {
        let now = Utc::now().timestamp();
        let conn = self.conn.lock().map_err(|_| "cache mutex poisoned")?;

        let mut stmt = conn.prepare(
            "SELECT payload FROM response_cache
             WHERE cache_key = ?1
               AND expires_at > ?2",
        )?;

        let mut rows = stmt.query(params![cache_key, now])?;
        if let Some(row) = rows.next()? {
            let payload: String = row.get(0)?;
            let parsed = serde_json::from_str::<T>(&payload)?;
            return Ok(Some(parsed));
        }

        Ok(None)
    }

    pub fn set<T: Serialize>(
        &self,
        cache_key: &str,
        payload: &T,
        ttl_seconds: i64,
    ) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().timestamp();
        let expires_at = now + ttl_seconds.max(1);
        let serialized = serde_json::to_string(payload)?;

        let conn = self.conn.lock().map_err(|_| "cache mutex poisoned")?;
        conn.execute(
            "INSERT INTO response_cache(cache_key, payload, cached_at, expires_at)
             VALUES(?1, ?2, ?3, ?4)
             ON CONFLICT(cache_key) DO UPDATE SET
                 payload = excluded.payload,
                 cached_at = excluded.cached_at,
                 expires_at = excluded.expires_at",
            params![cache_key, serialized, now, expires_at],
        )?;

        Ok(())
    }

    pub fn purge_expired(&self) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().timestamp();
        let conn = self.conn.lock().map_err(|_| "cache mutex poisoned")?;
        conn.execute(
            "DELETE FROM response_cache WHERE expires_at <= ?1",
            params![now],
        )?;
        Ok(())
    }
}
