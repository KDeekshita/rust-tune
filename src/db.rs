use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct DbPool(pub Mutex<Connection>);

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS refresh_tokens (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            token TEXT NOT NULL UNIQUE,
            expires_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
    ")?;

    Ok(conn)
}