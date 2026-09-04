# Questions

| ID | Question | State |
| --- | --- | --- |
| Q001 | What exact subset of strict jq must the first Rust OWS runtime support, and which jq implementation should be used? | open |
| Q002 | Should the first public SDK be Rust-only, or should a stable protocol be designed early for Python/TypeScript clients? | open |
| Q003 | Which process/filesystem/network sandbox guarantees belong in G002 versus later hardening? | resolved |
| Q004 | Should Agent conversational persistence and the common execution journal share one physical SQLite database in embedded mode? | resolved |
| Q005 | How much Pi-style extension dynamism should exist before a stable plugin ABI or WASI boundary is justified? | open |
| Q006 | Which A2A version/profile should define remote Agent interoperability when local Agent invocation already works? | open |
| Q007 | Should Flow timers and event waits be served by one embedded scheduler in the first release or only by a server profile? | open |
| Q008 | For G003 T007 policy denial, where does `failed_at: Policy` live, and should `ToolAttemptDenied` gain a `failed_at` field? Raised by A004-builder during the T007 dry run. | resolved |

## Q003 Resolution

G002 contains no general process/filesystem/network sandbox. It proves only the neutral execution substrate and local artifact boundary. G003 introduces only tool allow/deny, schema validation, replay classification, and deterministic/synthetic tool fixtures. G004 adds bounded `fs.read`. Write/process/network sandboxing remains later hardening and must not be inferred from either Goal's narrow policy layer.

## Q004 Resolution

ADR-0003 is accepted, so this direction is settled. One physical SQLite database is permitted in embedded mode; common runtime and Agent keep separate logical ownership, tables, and repositories. Commits that must atomically change both models coordinate at the SQLite backend transaction seam, and Agent DTOs stay out of `ww-store`. G003 T004 and T005 implement this direction.

## Q008 Resolution

Raised by `A004-builder` in `artifacts/A004-builder-T007-claude-opus-5-dryrun-01.md`.

For a policy denial, `failed_at: Policy` belongs in the already-required `ToolCallPrepared::NoEffect` disposition together with the durable `PolicyDecision::Deny`. The final attempt record remains the existing `ToolAttemptDenied { attempt_id, result_entry_id }` shape. Do **not** add a duplicate `failed_at` field to `ToolAttemptDenied`.

Resolve, Validate, and Classify failures continue to terminate through `ToolAttemptRejected`, whose `failed_at` identifies the actual failed preparation stage. This keeps one semantic authority for the preparation stage while preserving the final-attempt taxonomy: Rejected means Resolve/Validate/Classify; Denied means Policy.

This resolution is a nomenclature/record-placement clarification under D022. It introduces no new domain entity, state, relationship, or durable record variant.

The dry run's other question—the builder actor identifier—was already resolved by the requester as `A004-builder` and recorded in commit `c0c684d580d1e24bb746b7c46b1c7aaa4119639e`.
