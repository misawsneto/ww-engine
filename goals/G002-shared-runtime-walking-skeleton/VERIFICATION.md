# G002 Verification

## Required checks

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] `cargo test --workspace --all-features`
- [x] create → start → succeed persists ordered events and final state
- [x] create → start → cancel request → cancelled persists ordered events and reason
- [x] invalid transition leaves state/version/event count unchanged
- [x] stale expected version is rejected without partial state/event commit
- [x] reopened SQLite database yields identical inspection and reduced event projection
- [x] separate CLI processes create/start/cancel/settle/inspect the same durable execution
- [x] event cursor resumes without duplicates, including after CLI process reconnect
- [x] local cancellation token is signalled after durable cancel request
- [x] artifact bytes are content-addressed and duplicate content reuses the digest
- [x] CLI inspection/events are produced through SDK APIs
- [x] shared crates contain no Agent provider/message/tool-loop concepts
- [x] shared crates contain no OWS/FlowToken/interpreter concepts

## Evidence

- CI workflow: `.github/workflows/ci.yml`
- Verified code head: `c19c4c6a6c071190ddbcef23299fd10aecabb0a4`
- CI run: `33644225518` — success
- Rust toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, Cargo `1.98.0`
- `cargo fmt --all -- --check`: pass
- architecture-boundary grep checks: pass
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: pass
- `cargo test --workspace --all-features`: pass
- Runtime unit/property tests: 7 passed
- CLI cross-process integration: `lifecycle_survives_real_process_boundaries_and_cursor_reconnect` — pass
- SQLite optimistic concurrency integration: `stale_expected_version_is_rejected_without_partial_commit` — pass

## Remaining acceptance action

Executable acceptance evidence is complete. G002 remains active only because T010 requires an independent architecture/implementation review before terminal completion.
