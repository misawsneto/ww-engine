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
- [x] cancellation cannot settle without a prior durable cancel request
- [x] terminal result artifact and error payloads are reconstructed and compared against the execution row
- [x] artifact bytes are content-addressed and duplicate content reuses the digest
- [x] CLI inspection/events are produced through SDK APIs
- [x] shared crates contain no Agent provider/message/tool-loop concepts
- [x] shared crates contain no OWS/FlowToken/interpreter concepts

## Evidence

- CI workflow: `.github/workflows/ci.yml`
- Final reviewed implementation code head: `9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc`
- Evidence-recording head verified by CI: `bb2cb831fe42342afcfc93cf7e8757a9206c1947`
- Final CI run: `33646651848` — success
- Rust toolchain: Rust/Cargo 1.98.0
- `cargo fmt --all -- --check`: pass
- architecture-boundary grep checks: pass
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: pass
- `cargo test --workspace --all-features`: pass
- Runtime unit/property tests include two-phase cancellation regression and projection/reopen coverage
- CLI cross-process integration: `lifecycle_survives_real_process_boundaries_and_cursor_reconnect` — pass
- SQLite optimistic concurrency integration: `stale_expected_version_is_rejected_without_partial_commit` — pass

## Review finding resolved

The architecture review found that cancellation settlement could previously occur without a durable request and that terminal payloads were not fully represented in the reducer projection. The reviewed fix now enforces `request_cancel` → `settle_cancelled`, reuses the persisted cancellation reason, projects terminal result/error payloads, and rejects settlement without a prior durable request. CI run `33646651848` verifies the corrected implementation.

## Remaining acceptance action

All executable acceptance evidence is complete. G002 remains active only because T010 requires an independent architecture/implementation review before terminal completion.
