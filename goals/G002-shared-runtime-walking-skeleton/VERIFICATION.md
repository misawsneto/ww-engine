# G002 Verification

## Required checks

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`
- [ ] create → start → succeed persists ordered events and final state
- [ ] create → start → cancel request → cancelled persists ordered events and reason
- [ ] invalid transition leaves state/version/event count unchanged
- [ ] stale expected version is rejected
- [ ] reopened SQLite database yields identical inspection and reduced event projection
- [ ] event cursor resumes without duplicates
- [ ] local cancellation token is signalled after durable cancel request
- [ ] artifact bytes are content-addressed and duplicate content reuses the digest
- [ ] CLI inspection/events are produced through SDK APIs
- [ ] shared crates contain no Agent provider/message/tool-loop concepts
- [ ] shared crates contain no OWS/FlowToken/interpreter concepts

## Evidence

- CI run and test output to be recorded when T009 completes.
