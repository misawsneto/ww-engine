# G002 Tasks

| Task | State | Acceptance |
| --- | --- | --- |
| T001 Close G001 and initialize G002 bookkeeping | complete | G001 review accepted; root state points to G002 |
| T002 Scaffold Rust workspace and dependency rules | complete | shared crates exist; Rust 1.98.0 pinned; common dependency boundary enforced in CI |
| T003 Implement shared IDs, lifecycle types, event envelope, and reducer | complete | lifecycle transitions and event reduction are executable and tested |
| T004 Implement transactional SQLite store and migrations | complete | state + event commit atomically; stale expected-version conflict leaves state/event history unchanged |
| T005 Implement durable cancellation and local cancellation registry | complete | cancel request is durable; local token signals; cancellation state survives reopen and process boundaries |
| T006 Implement content-addressed local artifacts | complete | digest-addressed bytes + metadata + dedupe tested |
| T007 Implement SDK inspection and event stream | complete | inspection, polling stream, and cursor reconnect tests pass |
| T008 Implement `ww run` / `ww artifact` CLI | complete | lifecycle, inspection, events, and artifacts go through `ww-sdk`; process-boundary integration test passes |
| T009 Add CI and executable verification | complete | format, architecture boundaries, clippy, and full tests pass on Rust 1.98.0 in CI run `33644225518` |
| T010 Independent architecture/implementation review | open | independent reviewer confirms no Agent/Flow semantic leakage and accepts Goal exit criteria |
