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

## Q003 Resolution

G002 contains no general process/filesystem/network sandbox. It proves only the neutral execution substrate and local artifact boundary. G003 introduces only tool allow/deny, schema validation, replay classification, and deterministic/synthetic tool fixtures. G004 adds bounded `fs.read`. Write/process/network sandboxing remains later hardening and must not be inferred from either Goal's narrow policy layer.

## Q004 Resolution

ADR-0003 is accepted, so this direction is settled. One physical SQLite database is permitted in embedded mode; common runtime and Agent keep separate logical ownership, tables, and repositories. Commits that must atomically change both models coordinate at the SQLite backend transaction seam, and Agent DTOs stay out of `ww-store`. G003 T004 and T005 implement this direction.
