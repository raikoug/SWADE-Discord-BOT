use crate::initiative::{
    deserialize_deck, serialize_deck, InitiativeDraw, InitiativeSession, ParticipantKind,
};
use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Clone)]
pub struct BennyRow {
    pub user_id: u64,
    pub amount: i64,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;

            CREATE TABLE IF NOT EXISTS bennies (
                guild_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                amount INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (guild_id, user_id)
            );

            CREATE TABLE IF NOT EXISTS roll_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                guild_id TEXT,
                channel_id TEXT,
                user_id TEXT,
                command_name TEXT NOT NULL,
                summary TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS initiative_sessions (
                guild_id TEXT PRIMARY KEY,
                active INTEGER NOT NULL DEFAULT 0,
                round INTEGER NOT NULL DEFAULT 1,
                deck TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS initiative_draws (
                guild_id TEXT NOT NULL,
                participant_kind TEXT NOT NULL,
                participant_id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                card TEXT NOT NULL,
                on_hold INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (guild_id, participant_kind, participant_id)
            );
            "#,
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add_bennies(&self, guild_id: u64, user_id: u64, amount: i64) -> Result<i64> {
        if amount < 1 {
            return Err(anyhow!("amount must be at least 1"));
        }

        let conn = self.lock_conn()?;
        conn.execute(
            r#"
            INSERT INTO bennies (guild_id, user_id, amount, updated_at)
            VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
            ON CONFLICT(guild_id, user_id)
            DO UPDATE SET amount = amount + excluded.amount, updated_at = CURRENT_TIMESTAMP
            "#,
            params![guild_id.to_string(), user_id.to_string(), amount],
        )?;

        self.get_bennies_locked(&conn, guild_id, user_id)
    }

    pub fn spend_bennies(&self, guild_id: u64, user_id: u64, amount: i64) -> Result<i64> {
        if amount < 1 {
            return Err(anyhow!("amount must be at least 1"));
        }

        let conn = self.lock_conn()?;
        let current = self
            .get_bennies_locked(&conn, guild_id, user_id)
            .unwrap_or(0);
        if current < amount {
            return Err(anyhow!(
                "Bennies insufficienti: disponibili {current}, richiesti {amount}"
            ));
        }

        conn.execute(
            r#"
            UPDATE bennies
            SET amount = amount - ?3, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ?1 AND user_id = ?2
            "#,
            params![guild_id.to_string(), user_id.to_string(), amount],
        )?;

        Ok(current - amount)
    }

    pub fn list_bennies(&self, guild_id: u64) -> Result<Vec<BennyRow>> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT user_id, amount
            FROM bennies
            WHERE guild_id = ?1
            ORDER BY user_id ASC
            "#,
        )?;

        let rows = stmt.query_map(params![guild_id.to_string()], |row| {
            let user_id: String = row.get(0)?;
            let amount: i64 = row.get(1)?;
            Ok(BennyRow {
                user_id: user_id.parse::<u64>().unwrap_or_default(),
                amount,
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn reset_bennies(&self, guild_id: u64, amount: i64) -> Result<usize> {
        if amount < 0 {
            return Err(anyhow!("amount cannot be negative"));
        }

        let conn = self.lock_conn()?;
        let changed = conn.execute(
            r#"
            UPDATE bennies
            SET amount = ?2, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ?1
            "#,
            params![guild_id.to_string(), amount],
        )?;
        Ok(changed)
    }

    pub fn insert_roll_history(
        &self,
        guild_id: Option<u64>,
        channel_id: u64,
        user_id: u64,
        command_name: &str,
        summary: &str,
    ) -> Result<()> {
        let conn = self.lock_conn()?;
        conn.execute(
            r#"
            INSERT INTO roll_history (guild_id, channel_id, user_id, command_name, summary)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                guild_id.map(|id| id.to_string()),
                channel_id.to_string(),
                user_id.to_string(),
                command_name,
                summary,
            ],
        )?;
        Ok(())
    }

    pub fn get_active_initiative_session(
        &self,
        guild_id: u64,
    ) -> Result<Option<InitiativeSession>> {
        let conn = self.lock_conn()?;
        let maybe_session = conn
            .query_row(
                r#"
                SELECT round, deck
                FROM initiative_sessions
                WHERE guild_id = ?1 AND active = 1
                "#,
                params![guild_id.to_string()],
                |row| {
                    let round: u32 = row.get(0)?;
                    let deck: String = row.get(1)?;
                    Ok((round, deck))
                },
            )
            .optional()?;

        let Some((round, deck)) = maybe_session else {
            return Ok(None);
        };

        let mut stmt = conn.prepare(
            r#"
            SELECT participant_kind, participant_id, display_name, card, on_hold
            FROM initiative_draws
            WHERE guild_id = ?1
            "#,
        )?;
        let draws = stmt
            .query_map(params![guild_id.to_string()], |row| {
                Ok(InitiativeDraw {
                    kind: ParticipantKind::from_str(row.get::<_, String>(0)?.as_str())
                        .map_err(to_sql_error)?,
                    participant_id: row.get(1)?,
                    display_name: row.get(2)?,
                    card: crate::initiative::Card::from_code(row.get::<_, String>(3)?.as_str())
                        .map_err(to_sql_error)?,
                    on_hold: row.get::<_, i64>(4)? != 0,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(Some(InitiativeSession::from_parts(
            round,
            deserialize_deck(&deck)?,
            draws,
        )))
    }

    pub fn save_active_initiative_session(
        &self,
        guild_id: u64,
        session: &InitiativeSession,
    ) -> Result<()> {
        let conn = self.lock_conn()?;
        let transaction = conn.unchecked_transaction()?;

        transaction.execute(
            r#"
            INSERT INTO initiative_sessions (guild_id, active, round, deck, updated_at)
            VALUES (?1, 1, ?2, ?3, CURRENT_TIMESTAMP)
            ON CONFLICT(guild_id)
            DO UPDATE SET active = 1, round = excluded.round, deck = excluded.deck, updated_at = CURRENT_TIMESTAMP
            "#,
            params![
                guild_id.to_string(),
                i64::from(session.round),
                serialize_deck(&session.deck)
            ],
        )?;

        transaction.execute(
            r#"
            DELETE FROM initiative_draws
            WHERE guild_id = ?1
            "#,
            params![guild_id.to_string()],
        )?;

        for draw in &session.draws {
            transaction.execute(
                r#"
                INSERT INTO initiative_draws (guild_id, participant_kind, participant_id, display_name, card, on_hold)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    guild_id.to_string(),
                    draw.kind.as_str(),
                    draw.participant_id,
                    draw.display_name,
                    draw.card.code(),
                    if draw.on_hold { 1 } else { 0 },
                ],
            )?;
        }

        transaction.commit()?;
        Ok(())
    }

    pub fn end_initiative_session(&self, guild_id: u64) -> Result<bool> {
        let conn = self.lock_conn()?;
        let transaction = conn.unchecked_transaction()?;

        let updated = transaction.execute(
            r#"
            UPDATE initiative_sessions
            SET active = 0, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ?1 AND active = 1
            "#,
            params![guild_id.to_string()],
        )?;

        transaction.execute(
            r#"
            DELETE FROM initiative_draws
            WHERE guild_id = ?1
            "#,
            params![guild_id.to_string()],
        )?;

        transaction.commit()?;
        Ok(updated > 0)
    }

    fn get_bennies_locked(&self, conn: &Connection, guild_id: u64, user_id: u64) -> Result<i64> {
        conn.query_row(
            r#"
            SELECT amount
            FROM bennies
            WHERE guild_id = ?1 AND user_id = ?2
            "#,
            params![guild_id.to_string(), user_id.to_string()],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    fn lock_conn(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        self.conn
            .lock()
            .map_err(|err| anyhow!("SQLite mutex poisoned: {err}"))
            .context("cannot lock SQLite connection")
    }
}

fn to_sql_error(err: anyhow::Error) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(
        0,
        rusqlite::types::Type::Text,
        Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            err.to_string(),
        )),
    )
}
