# G002 Tasks

| Task | State | Acceptance |
| --- | --- | --- |
| T001 Close G001 and initialize G002 bookkeeping | complete | G001 review accepted; root state points to G002 |
| T002 Scaffold Rust workspace and dependency rules | active | shared crates exist; toolchain pinned; dependency direction documented |
| T003 Implement shared IDs, lifecycle types, event envelope, and reducer | open | valid transitions/reduction covered by tests |
| T004 Implement transactional SQLite store and migrations | open | current state + event commit atomically; version conflicts tested |
| T005 Implement durable cancellation and local cancellation registry | open | cancel request durable; local token signals; restart state preserved |
| T006 Implement content-addressed local artifacts | open | digest-addressed bytes + metadata + dedupe tested |
| T007 Implement SDK inspection and event stream | open | inspect and cursor/reconnect tests pass |
| T008 Implement `ww run` / `ww artifact` CLI | open | lifecycle and event commands operate only through SDK |
| T009 Add CI and executable verification | open | fmt, clippy, test, restart/fault tests pass on Rust 1.98.0 |
| T010 Independent architecture/implementation review | open | no Agent/Flow semantic leakage; Goal exit criteria satisfied |
