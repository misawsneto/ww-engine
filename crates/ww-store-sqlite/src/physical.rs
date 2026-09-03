use rusqlite::{Connection, TransactionBehavior, params};
use std::time::Duration;
use thiserror::Error;

const MIGRATION_LEDGER: &str = r#"
create table if not exists ww_schema_migrations (
    component   text not null,
    version     integer not null,
    applied_at  text not null,
    primary key(component, version)
);
"#;

#[derive(Clone, Copy, Debug)]
pub struct ComponentMigration {
    pub version: u32,
    pub sql: &'static str,
}

#[derive(Debug, Error)]
pub enum SqlitePhysicalError {
    #[error("transient SQLite backend error: {0}")]
    TransientBackend(String),
    #[error("permanent SQLite backend error: {0}")]
    PermanentBackend(String),
    #[error(
        "database component {component} is at future version {current}; this binary supports {supported}"
    )]
    FutureVersion {
        component: String,
        current: u32,
        supported: u32,
    },
    #[error("migration sequence for {component} expected version {expected}, got {actual}")]
    MigrationGap {
        component: String,
        expected: u32,
        actual: u32,
    },
}

impl SqlitePhysicalError {
    pub const fn is_transient(&self) -> bool {
        matches!(self, Self::TransientBackend(_))
    }
}

pub fn configure_connection(connection: &Connection) -> Result<(), SqlitePhysicalError> {
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(backend)?;
    connection
        .execute_batch("pragma foreign_keys = on; pragma journal_mode = wal;")
        .map_err(backend)?;
    Ok(())
}

pub fn apply_component_migrations(
    connection: &mut Connection,
    component: &str,
    migrations: &[ComponentMigration],
) -> Result<(), SqlitePhysicalError> {
    for (index, migration) in migrations.iter().enumerate() {
        let expected = u32::try_from(index + 1)
            .map_err(|_| SqlitePhysicalError::PermanentBackend("too many migrations".to_owned()))?;
        if migration.version != expected {
            return Err(SqlitePhysicalError::MigrationGap {
                component: component.to_owned(),
                expected,
                actual: migration.version,
            });
        }
    }
    connection
        .execute_batch(MIGRATION_LEDGER)
        .map_err(backend)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(backend)?;
    let mut statement = transaction
        .prepare("select version from ww_schema_migrations where component = ?1 order by version")
        .map_err(backend)?;
    let rows = statement
        .query_map([component], |row| row.get::<_, i64>(0))
        .map_err(backend)?;
    let mut current: u32 = 0;
    for row in rows {
        let actual = u32::try_from(row.map_err(backend)?).map_err(|_| {
            SqlitePhysicalError::PermanentBackend("negative migration version".to_owned())
        })?;
        let expected = current.checked_add(1).ok_or_else(|| {
            SqlitePhysicalError::PermanentBackend("migration version overflow".to_owned())
        })?;
        if actual != expected {
            return Err(SqlitePhysicalError::MigrationGap {
                component: component.to_owned(),
                expected,
                actual,
            });
        }
        current = actual;
    }
    drop(statement);
    let supported = migrations.last().map_or(0, |migration| migration.version);
    if current > supported {
        return Err(SqlitePhysicalError::FutureVersion {
            component: component.to_owned(),
            current,
            supported,
        });
    }

    let mut applied = current;
    for migration in migrations
        .iter()
        .filter(|migration| migration.version > current)
    {
        let expected = applied.checked_add(1).ok_or_else(|| {
            SqlitePhysicalError::PermanentBackend("migration version overflow".to_owned())
        })?;
        if migration.version != expected {
            return Err(SqlitePhysicalError::MigrationGap {
                component: component.to_owned(),
                expected,
                actual: migration.version,
            });
        }
        transaction.execute_batch(migration.sql).map_err(backend)?;
        transaction
            .execute(
                "insert into ww_schema_migrations (component, version, applied_at) values (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
                params![component, i64::from(migration.version)],
            )
            .map_err(backend)?;
        applied = migration.version;
    }
    transaction.commit().map_err(backend)
}

fn backend(error: rusqlite::Error) -> SqlitePhysicalError {
    if is_transient_sqlite_error(&error) {
        SqlitePhysicalError::TransientBackend(error.to_string())
    } else {
        SqlitePhysicalError::PermanentBackend(error.to_string())
    }
}

pub fn is_transient_sqlite_error(error: &rusqlite::Error) -> bool {
    matches!(
        error,
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error {
                code: rusqlite::ErrorCode::DatabaseBusy
                    | rusqlite::ErrorCode::DatabaseLocked
                    | rusqlite::ErrorCode::FileLockingProtocolFailed
                    | rusqlite::ErrorCode::SchemaChanged,
                ..
            },
            _,
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_retryable_sqlite_failures() {
        let busy = rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_BUSY),
            None,
        );
        let constraint = rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            None,
        );

        assert!(is_transient_sqlite_error(&busy));
        assert!(!is_transient_sqlite_error(&constraint));
    }
}
