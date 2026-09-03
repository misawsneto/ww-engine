# G003 Reviews

## Implementation progress review — 2026-09-03

State: active. No terminal G003 review has occurred.

Verified slices:

- T002 provider-neutral protocol and pure stream assembler: provider boundary, clippy, and workspace tests passed; 15 assembler/conformance tests passed.
- T003 durable Agent entries/operational records and pure recovery reducer: clippy and workspace tests passed; 11 recovery/corruption tests passed.
- T004 Agent-owned SQLite persistence/reconstruction: clippy and workspace tests passed, including rollback/reopen/version-conflict and real OS-process restart reconstruction.

Current checkpoint:

- T006 RecordedProvider conformance is complete and verified. The full `main` gate passed on Rust 1.98.0: rustfmt, five architecture-boundary checks, clippy with `--locked -D warnings`, and 58/58 tests.
- T005 remains scoped only to atomic common-execution + Agent-run creation/linkage and rollback on injected half-write failure. Terminal repair remains owned by T009.
- T007 tool contract, schema validation, policy, and replay fixtures is the next open implementation slice.

## D018 experiment disposition — 2026-09-03

D018 introduced a durability-hygiene cleanup gate after T006. The implementation produced useful evidence, but review found that inserting and renumbering Tasks changed the accepted G003 structure without an existing acceptance criterion or Stop Condition requiring the interruption. D019 therefore superseded D018, and the D018-era implementation was ordinarily reverted without rewriting history. The canonical G003 sequence remains T001–T012, with T007 next. Technical findings are mapped either to an existing G003 Task or proposed G009; the full evidence and disposition remain in `docs/memories/recall/D018-DURABILITY-HYGIENE-RETROSPECTIVE-2026-09-03.md`.

## Planned terminal review focus

- provider-neutral kernel ownership and dependency direction;
- deterministic recovery reducer and corrupt-history behavior;
- Agent/common SQLite transaction ownership without Agent DTO leakage into shared store contracts;
- tool argument validation, policy, replay classification, and logical-result uniqueness;
- crash/restart behavior at every model/tool ambiguity boundary;
- cancellation and durable budget accounting;
- absence of concrete transport, filesystem, CLI/product surface, Flow/OWS, or WorkWeave Orchestration semantics in the kernel.
