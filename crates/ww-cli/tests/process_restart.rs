use serde_json::Value;
use std::process::{Command, Output};
use tempfile::TempDir;

fn run_ww(temp: &TempDir, args: &[&str]) -> Output {
    let db = temp.path().join("runtime.db");
    let artifacts = temp.path().join("artifacts");
    let output = Command::new(env!("CARGO_BIN_EXE_ww"))
        .arg("--db")
        .arg(&db)
        .arg("--artifacts")
        .arg(&artifacts)
        .args(args)
        .output()
        .expect("run ww process");
    assert!(
        output.status.success(),
        "ww {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn json_stdout(output: Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("valid JSON stdout")
}

#[test]
fn lifecycle_survives_real_process_boundaries_and_cursor_reconnect() {
    let temp = TempDir::new().expect("temp dir");

    let created = json_stdout(run_ww(&temp, &["--json", "run", "create"]));
    let id = created["id"].as_str().expect("execution id").to_owned();
    assert_eq!(created["status"], "pending");
    assert_eq!(created["version"], 1);

    let started = json_stdout(run_ww(&temp, &["--json", "run", "start", &id]));
    assert_eq!(started["status"], "running");
    assert_eq!(started["version"], 2);

    let cancelled_requested = json_stdout(run_ww(
        &temp,
        &[
            "--json",
            "run",
            "cancel",
            &id,
            "--code",
            "operator",
            "--message",
            "stop",
        ],
    ));
    assert_eq!(cancelled_requested["status"], "running");
    assert_eq!(cancelled_requested["cancel_requested"], true);
    assert_eq!(cancelled_requested["version"], 3);

    let settled = json_stdout(run_ww(&temp, &["--json", "run", "settle-cancelled", &id]));
    assert_eq!(settled["status"], "cancelled");
    assert_eq!(settled["version"], 4);

    let inspected = json_stdout(run_ww(&temp, &["--json", "run", "inspect", &id]));
    assert_eq!(inspected["status"], "cancelled");
    assert_eq!(inspected["cancel_requested"], true);
    assert_eq!(inspected["version"], 4);

    let all_events = run_ww(
        &temp,
        &["run", "events", &id, "--after", "0", "--limit", "100"],
    );
    let events: Vec<Value> = String::from_utf8(all_events.stdout)
        .expect("UTF-8 events")
        .lines()
        .map(|line| serde_json::from_str(line).expect("JSONL event"))
        .collect();
    assert_eq!(
        events
            .iter()
            .map(|event| event["sequence"].as_u64().expect("event sequence"))
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );

    let resumed = run_ww(
        &temp,
        &["run", "events", &id, "--after", "2", "--limit", "100"],
    );
    let resumed_events: Vec<Value> = String::from_utf8(resumed.stdout)
        .expect("UTF-8 resumed events")
        .lines()
        .map(|line| serde_json::from_str(line).expect("JSONL resumed event"))
        .collect();
    assert_eq!(
        resumed_events
            .iter()
            .map(|event| event["sequence"].as_u64().expect("event sequence"))
            .collect::<Vec<_>>(),
        vec![3, 4]
    );
}
