# G003 Reviews

## Implementation progress review — 2026-09-03

State: active. No terminal G003 review has occurred.

Verified slices:

- T002 provider-neutral protocol and pure stream assembler: provider boundary, clippy, and workspace tests passed; 15 assembler/conformance tests passed.
- T003 durable Agent entries/operational records and pure recovery reducer: clippy and workspace tests passed; 11 recovery/corruption tests passed.
- T004 Agent-owned SQLite persistence/reconstruction: clippy and workspace tests passed, including rollback/reopen/version-conflict and real OS-process restart reconstruction.
- T005 common/Agent SQLite transaction coordination: full merge-target-equivalent gate passed on consolidation commit `69f4ab7ecbed731d40a695dafcf487d62645b695`; atomic common-execution + Agent-run creation/linkage and rollback on injected half-write failure are verified.
- T006 RecordedProvider conformance: the full `main` gate passed at 58/58 tests; recorded fixtures cover text, tool calls, usage, failure, cancellation, truncation, interrupted attempts, determinism, request capture, and script violations.

## A003-reviewer durability/hygiene review — 2026-09-03

Disposition: blocking cleanup before the previously planned tool-contract T007. Decision D018 accepted; G003 Plan v2 inserts a bounded cleanup gate and renumbers only still-open tasks.

Findings that must be closed before tool semantics expand the durable model:

1. Agent durable entry/record/configuration payloads do not yet have an explicit schema/payload evolution contract; SQLite migration ownership is also not component-versioned for the shared physical database.
2. coordinated Agent creation commits before its post-commit reads, so a crash/read failure after commit can leave the caller uncertain whether creation succeeded; retry semantics must be idempotent and typed.
3. provider and durable tool calls carry both raw JSON and parsed JSON arguments; one representation must become authoritative before schema validation, policy, hashing, and replay semantics rely on it.
4. durable Agent history directly serializes normalized provider-crate types; an explicit provider-to-durable conversion boundary should make `ww-agent-core` the structural owner of its disk format.
5. provider stream consumption can bypass `ResponseAssembler::finish`; the kernel needs one production finalization path for normal completion, failure, cancellation, and interrupted EOF.
6. store error categories collapse invalid commands, persistence corruption, conflicts, and backend failures too aggressively for durable retry/recovery decisions.
7. runtime store, Agent store, and coordinator duplicate SQLite connection/configuration mechanics; physical-backend reuse should be introduced without moving Agent semantics into the shared store contract.
8. architecture CI relies heavily on textual grep; retain those semantic guards but add structural dependency-graph checks.
9. test-only binaries/recorded fixtures are exposed as normal package surfaces, and canonical project documents already drifted behind T006 completion.

This review does not change the accepted sibling-kernel, provider-neutral, recovery-first architecture. It tightens implementation discipline required to satisfy ADR-0003 before the tool/replay and functional-loop slices.

## A003-reviewer cleanup closure — 2026-09-03

Disposition: D018 cleanup gate accepted as complete; G003 may proceed to T012.

- T007: runtime, Agent, and coordinator migrations use one component ledger; configuration/entry/record/event payload versions are explicit and validated; future and gapped versions fail closed.
- T008: coordinated creation performs all reads before commit, returns the in-transaction result, accepts exact retries, rejects conflicting retries without mutation, and exposes recovery-relevant invalid/conflict/corrupt/transient/permanent failure categories.
- T009: tool-call arguments have one parsed JSON representation; Agent-owned completion/usage/tool-call types define the disk shape; explicit conversion rejects provider tool results.
- T010: `finalize_stream` consumes through EOF and gives completion, provider failure, cancellation, malformed protocol, and interrupted EOF one typed interpretation; RecordedProvider conformance uses that production path.
- T011: physical SQLite setup is shared without semantic leakage; structural Cargo dependency checks supplement semantic guards; RecordedProvider and the process fixture require opt-in test support; canonical records agree.

Additional audit hardening makes runtime inspection and Agent reconstruction read one SQLite snapshot, makes store-level lifecycle patches inseparable from their events, validates persisted event kind/version metadata, and gives every registration for one execution the same cancellation root.

Residual items are deliberately outside D018: artifact/file crash reconciliation and a long-lived SQLite connection pool remain future runtime-hardening work. Neither blocks the current recorded, single-process G003 kernel proof.

## Planned terminal review focus

- provider-neutral kernel ownership and dependency direction;
- deterministic recovery reducer and corrupt-history behavior;
- Agent/common SQLite transaction ownership without Agent DTO leakage into shared store contracts;
- durable schema/payload evolution and idempotent create/repair semantics;
- tool argument validation, policy, replay classification, and logical-result uniqueness;
- crash/restart behavior at every model/tool ambiguity boundary;
- cancellation and durable budget accounting;
- absence of concrete transport, filesystem, CLI/product surface, Flow/OWS, or WorkWeave Orchestration semantics in the kernel.
