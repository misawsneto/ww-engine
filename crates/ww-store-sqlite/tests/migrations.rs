use rusqlite::{Connection, params};
use ww_store_sqlite::{ComponentMigration, SqlitePhysicalError, apply_component_migrations};

const MIGRATIONS: &[ComponentMigration] = &[ComponentMigration {
    version: 1,
    sql: "create table component_data (id integer primary key);",
}];

#[test]
fn future_component_version_fails_closed_without_rewriting_ledger() {
    let mut connection = Connection::open_in_memory().expect("open database");
    apply_component_migrations(&mut connection, "test_component", MIGRATIONS)
        .expect("apply supported migration");
    connection
        .execute(
            "insert into ww_schema_migrations (component, version, applied_at) values (?1, 2, 'future')",
            ["test_component"],
        )
        .expect("seed future version");

    let error = apply_component_migrations(&mut connection, "test_component", MIGRATIONS)
        .expect_err("future version must fail");
    assert!(matches!(
        error,
        SqlitePhysicalError::FutureVersion {
            current: 2,
            supported: 1,
            ..
        }
    ));
    let versions: Vec<u32> = connection
        .prepare("select version from ww_schema_migrations where component = ?1 order by version")
        .expect("prepare ledger query")
        .query_map(params!["test_component"], |row| row.get(0))
        .expect("query ledger")
        .collect::<Result<_, _>>()
        .expect("read ledger");
    assert_eq!(versions, vec![1, 2]);
}

#[test]
fn migration_gaps_are_rejected_before_schema_changes() {
    const GAPPED: &[ComponentMigration] = &[ComponentMigration {
        version: 2,
        sql: "create table should_not_exist (id integer primary key);",
    }];
    let mut connection = Connection::open_in_memory().expect("open database");
    let error = apply_component_migrations(&mut connection, "gapped", GAPPED)
        .expect_err("migration gap must fail");
    assert!(matches!(
        error,
        SqlitePhysicalError::MigrationGap {
            expected: 1,
            actual: 2,
            ..
        }
    ));
    let table_count: u32 = connection
        .query_row(
            "select count(*) from sqlite_master where type = 'table' and name = 'should_not_exist'",
            [],
            |row| row.get(0),
        )
        .expect("query schema");
    assert_eq!(table_count, 0);
}
