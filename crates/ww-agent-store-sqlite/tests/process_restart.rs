#![cfg(feature = "test-support")]

use serde_json::Value;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn agent_history_reconstructs_across_real_process_restart() {
    let temp = TempDir::new().expect("temp dir");
    let db = temp.path().join("runtime.db");
    let fixture = env!("CARGO_BIN_EXE_agent-store-fixture");

    let seed = Command::new(fixture)
        .arg("seed")
        .arg(&db)
        .output()
        .expect("seed process");
    assert!(
        seed.status.success(),
        "seed failed: {}",
        String::from_utf8_lossy(&seed.stderr)
    );
    let run_id = String::from_utf8(seed.stdout)
        .expect("seed stdout UTF-8")
        .trim()
        .to_owned();

    let inspect = Command::new(fixture)
        .arg("inspect")
        .arg(&db)
        .arg(&run_id)
        .output()
        .expect("inspect process");
    assert!(
        inspect.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let state: Value = serde_json::from_slice(&inspect.stdout).expect("state JSON");
    assert_eq!(state["phase"], "terminal");
    assert_eq!(state["context_entry_ids"].as_array().unwrap().len(), 2);
    assert_eq!(state["model_request_count"], 1);
}
