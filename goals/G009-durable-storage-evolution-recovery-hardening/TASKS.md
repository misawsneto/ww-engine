# G009 Tasks

These Task IDs are a draft proposal and become stable only if G009 is activated.

| Task | State | Acceptance | Dependencies |
| --- | --- | --- | --- |
| T001 Decide execution order, accept governing ADR, and activate G009 | open | post-G003 evidence records why G009 is scheduled; governing ADR accepted; project state points to active G009 | G003 terminal review |
| T002 Define component-owned durable version and migration contracts | open | schema and payload ownership/version envelopes are explicit without semantic DTO leakage into shared storage | T001 |
| T003 Prove known-old upgrades and future-version rejection | open | committed old-state fixtures upgrade deterministically; unknown future versions fail closed without partial mutation | T002 |
| T004 Make create/ensure idempotent across uncertain acknowledgement | open | retry after committed-but-unacknowledged creation returns the same logical resource without duplicate durable identity | T002 |
| T005 Define recovery-oriented store failure classification | open | callers distinguish retryable, conflict, corruption, unsupported-version, and permanent failures without backend-specific branching | T002 |
| T006 Extract reusable physical SQLite mechanics | open | transaction/open/migration primitives are reused across adapters while schemas and durable DTOs remain component-owned | T003, T004, T005 |
| T007 Prove cross-adapter conformance and perform terminal review | open | migration, ambiguity, retry, reopen, and concurrency Evaluations pass; review finds no Agent/Flow/Orchestration semantic change | T006 |
