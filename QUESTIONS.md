# Questions

| ID | Question | State |
| --- | --- | --- |
| Q001 | What exact subset of strict jq must the first Rust OWS runtime support, and which jq implementation should be used? | open |
| Q002 | Should the first public SDK be Rust-only, or should a stable protocol be designed early for Python/TypeScript clients? | open |
| Q003 | Which process/filesystem/network sandbox guarantees belong in G002 versus later hardening? | resolved |
| Q004 | Should Agent conversational persistence and the common execution journal share one physical SQLite database in embedded mode? | open — proposed answer in ADR-0003 |
| Q005 | How much Pi-style extension dynamism should exist before a stable plugin ABI or WASI boundary is justified? | open |
| Q006 | Which A2A version/profile should define remote Agent interoperability when local Agent invocation already works? | open |
| Q007 | Should Flow timers and event waits be served by one embedded scheduler in the first release or only by a server profile? | open |

## Q003 Resolution

G002 contains no general process/filesystem/network sandbox. It proves only the neutral execution substrate and local artifact boundary. G003 introduces minimal tool allow/deny, schema validation, replay classification, and bounded `fs.read`; write/process/network sandboxing remains later hardening and must not be inferred from the G003 policy layer.

## Q004 Proposed direction

ADR-0003 proposes one physical SQLite database in embedded mode with separate common-runtime and Agent logical tables/repositories. Commits that must coordinate both models must share one backend transaction. The question remains open until ADR-0003 is accepted with G003 activation.
