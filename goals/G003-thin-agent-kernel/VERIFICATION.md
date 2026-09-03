# G003 Verification

## Required deterministic checks

- [x] ADR-0003 is accepted before implementation is represented as active.
- [x] `cargo fmt --all -- --check`
- [x] Agent/common dependency-boundary checks
- [x] `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- [x] `cargo test --workspace --all-features --locked`

### Provider protocol

- [x] text-only recorded stream finalizes one immutable assistant message
- [x] complete recorded tool-call stream finalizes stable call IDs and exact JSON arguments
- [x] delta before start rejects
- [x] duplicate terminal event rejects
- [x] incomplete/truncated tool arguments cannot execute
- [x] provider disconnect before finalization becomes an interrupted/failed model attempt
- [x] normalized usage is immutable at finalized response boundary

### Durable Agent state

- [x] entries and operational records reconstruct identical `AgentRecoveryState` after SQLite reopen and OS process restart
- [x] stale Agent writer is rejected without partial Agent/common mutation
- [x] unknown references, duplicate finalization, duplicate logical tool result, and records after terminal result reject as corrupt history
- [x] finalized entries are immutable; retries create new attempt records

### Tool safety

- [ ] JSON arguments validate before policy and execution
- [ ] denied tool produces one durable model-visible error result and performs no effect
- [ ] replay policy and arguments/tool-version digests are durable before effect start
- [ ] one logical tool call has at most one committed model-visible result
- [ ] sequential results preserve provider source order

### Kernel and limits

- [ ] recorded provider completes text-only Agent run
- [ ] recorded provider completes model → deterministic tool → model terminal run
- [ ] durable cancellation reaches active provider token
- [ ] durable cancellation reaches active tool token
- [ ] deadline terminates deterministically
- [ ] max model requests, turns, and tool calls terminate deterministically
- [ ] configured token limit stops before the next model request after normalized usage reaches/exceeds budget
- [ ] Agent result and G002 execution terminal state reconcile after restart

### Recovery fault matrix

- [ ] restart after Agent/common creation commit continues exactly once
- [ ] restart after model-attempt start creates a distinct retry attempt when permitted
- [ ] restart after model finalization does not re-contact provider before pending tool processing
- [ ] replay-safe started tool without result resumes as a new attempt without duplicate committed result
- [ ] non-replayable started tool without result is never re-executed and yields `RequiresIntervention`
- [ ] durable tool result with missing model-visible entry repairs idempotently
- [ ] all tool results with missing turn commit repair idempotently
- [ ] durable Agent result with non-terminal common execution terminalizes common execution idempotently

### Architecture boundaries

- [ ] `ww-agent-core` has no concrete provider/HTTP, SQLite, filesystem, Flow/OWS, CLI/TUI/server, or WorkWeave Orchestration semantic dependency
- [x] Agent DTOs do not enter shared `ww-store` semantic contracts
- [x] no concrete network provider or public filesystem/process/network tool exists in G003
- [x] no Agent CLI/SDK product surface is added in G003

## Required Evaluations

All checks in `EVALUATIONS.md` required for G003 closure must have current passing EvaluationRuns on the final reviewed code basis.

## Evidence

- T002 verification: temporary verifier branch; provider boundary and clippy passed; full workspace tests passed with 15 `ww-agent-provider` assembler/conformance tests.
- T003 verification: temporary verifier branch; clippy and full workspace tests passed with 11 `ww-agent-core` recovery/corruption tests.
- T004 verification: temporary verifier branch; clippy and full workspace tests passed, including Agent SQLite rollback/reopen/version-conflict and real process-restart reconstruction.
- T005 verification: consolidated engineering commit `69f4ab7ecbed731d40a695dafcf487d62645b695`; full merge-target-equivalent gate passed on Rust 1.98.0: fmt, five architecture boundary checks, clippy `--locked -D warnings`, and 44/44 tests. Coordinator acceptance covers atomic create/link, injected mid-write rollback, and mismatched database path rejection.
