# G003 Verification

- Version: `v2`
- Approval: `pending requester approval under D021`
- Specification basis: `G003 SPEC v2`
- Completed T002–T006 checks/evidence below retain their original meaning.

## Permanent deterministic gate

- [x] ADR-0003 is accepted before implementation is represented as active.
- [x] `cargo fmt --all -- --check`
- [x] Agent/common dependency-boundary checks
- [x] `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- [x] `cargo test --workspace --all-features --locked`

The checked gate records completed evidence through T006. Every later Task closure must add exact-code evidence for the same gate.

## Completed foundation

### Provider protocol — T002/T006

- [x] text-only recorded stream finalizes one immutable assistant message
- [x] complete recorded tool-call stream finalizes stable call IDs and exact JSON arguments
- [x] delta before start rejects
- [x] duplicate terminal event rejects
- [x] incomplete/truncated tool arguments cannot execute
- [x] provider disconnect before finalization becomes an interrupted/failed model attempt
- [x] normalized usage is immutable at finalized response boundary
- [x] RecordedProvider detects mismatched, extra, and unused scripted exchanges
- [x] RecordedProvider captures provider-source tool/result ordering deterministically

### Durable Agent state — T003/T004/T005

- [x] entries and operational records reconstruct identical `AgentRecoveryState` after SQLite reopen and OS process restart
- [x] stale Agent writer is rejected without partial Agent/common mutation
- [x] unknown references, duplicate finalization, duplicate logical tool result, and records after terminal result reject as corrupt history
- [x] finalized entries are immutable; retries create new attempt records
- [x] common execution + Agent run + link commit atomically or roll back together

## V-T007 — Tool contract, validation, policy, replay

### Identity and registry

- [ ] `V-T007-01` empty ToolId/ToolVersion rejects
- [ ] `V-T007-02` duplicate ToolId rejects before run start
- [ ] `V-T007-03` exact pinned version resolves; unavailable/mismatched version rejects
- [ ] `V-T007-04` model-visible specs preserve configured tool order

### Schema profile

- [ ] `V-T007-05` valid self-contained Draft 2020-12 schema compiles once and validates repeatedly
- [ ] `V-T007-06` malformed schema rejects registry construction
- [ ] `V-T007-07` HTTP/file/other non-fragment `$ref` rejects with no retrieval
- [ ] `V-T007-08` local `#/$defs/...` reference validates
- [ ] `V-T007-09` invalid instance reports deterministic WorkWeave-owned instance path/message
- [ ] `V-T007-10` validation is non-coercing and leaves the parsed `Value` byte-equivalent after normalized serialization
- [ ] `V-T007-11` invalid arguments invoke policy zero times and executor zero times

### Canonical arguments, effect, replay, policy

- [ ] `V-T007-12` object key order does not change arguments digest
- [ ] `V-T007-13` different argument value changes digest
- [ ] `V-T007-14` validation occurs before effect/replay classification
- [ ] `V-T007-15` classification occurs before policy
- [ ] `V-T007-16` policy is evaluated exactly once per preparation attempt
- [ ] `V-T007-17` policy denial invokes executor/probe zero times
- [ ] `V-T007-18` denial yields exactly one ordered `policy_denied` model-visible result
- [ ] `V-T007-19` `test.echo` returns deterministic structured output and `ReplayPolicy::Safe`
- [ ] `V-T007-20` `test.unsafe_once` invokes its probe once per actual execute and is `ReplayPolicy::Never`

### Durable preparation and reducer

- [ ] `V-T007-21` pre-effect durable state contains source position, provider call ID, pinned tool/version, arguments digest, effect, replay, policy, attempt ID, reserved result ID, and `ToolEffectStarted`
- [ ] `V-T007-22` allowed effect is not invoked until the pre-effect append containing `ToolEffectStarted` commits
- [ ] `V-T007-23` unknown/invalid/classification/denied paths each produce one no-effect audited result with stable code and no effect-start/effect-completion record
- [ ] `V-T007-24` durable effect output can exist before model-visible result for later repair
- [ ] `V-T007-25` interrupted safe and intervention Never attempts are distinct states
- [ ] `V-T007-26` reducer rejects changed replay/policy across attempts
- [ ] `V-T007-27` reducer rejects wrong reserved result ID, effect on denied call, duplicate classification/result, and source-order violation
- [ ] `V-T007-28` Agent history reconstructs the same tool state after SQLite reopen
- [ ] `V-T007-29` `ww-agent-tools` imports no runtime/store/SQLite/filesystem/process/network/Flow/Orchestration dependency

Focused evidence:

```bash
cargo test -p ww-agent-tools --locked
cargo test -p ww-agent-core --test recovery --locked
```

## V-T008 — Functional Agent kernel

### Request and stream

- [ ] `V-T008-01` typed stored configuration decodes before provider/tool work
- [ ] `V-T008-02` request maps ordered entries and pinned tool specs exactly
- [ ] `V-T008-03` provider/model/request digest attempt state commits before provider call
- [ ] `V-T008-04` stream is drained through EOF and then finalized
- [ ] `V-T008-05` unexpected EOF, stream error, post-terminal event, malformed order, or truncated tool call creates no assistant entry/effect
- [ ] `V-T008-06` provider Failed/Aborted has one typed attempt/Agent disposition
- [ ] `V-T008-07` finalized assistant entry and usage commit before any tool effect

### Tool loop and terminal result

- [ ] `V-T008-08` logical IDs allocate once in provider source order and survive reconstruction
- [ ] `V-T008-09` unknown/invalid/denied/tool-error each returns one ordered model-visible error result
- [ ] `V-T008-10` allowed calls execute sequentially
- [ ] `V-T008-11` provider call order equals model-visible result order in the next request
- [ ] `V-T008-12` durable effect output precedes/repairs model-visible result
- [ ] `V-T008-13` `TurnCommitted` contains exactly the ordered result IDs
- [ ] `V-T008-14` text-only RecordedProvider run commits expected successful Agent result
- [ ] `V-T008-15` RecordedProvider model→test.echo→model run commits expected successful Agent result
- [ ] `V-T008-16` Length completion is audited but not successful
- [ ] `V-T008-17` kernel imports no concrete provider transport, SQLite, filesystem/process/network, Flow/OWS, CLI/TUI/server, or Orchestration type

Focused evidence:

```bash
cargo test -p ww-agent-core --test kernel --locked
cargo test -p ww-agent-provider --test recorded_provider --locked
```

## V-T009 — Lifecycle and cancellation

- [ ] `V-T009-01` missing/mismatched/non-agent common link rejects before work
- [ ] `V-T009-02` Pending starts once; Running/Waiting resumes; matching terminal state performs no work
- [ ] `V-T009-03` durable cancellation commits before root-token signal
- [ ] `V-T009-04` repeated registration observes one root while consumer child cancellation cannot cancel siblings
- [ ] `V-T009-05` pre-launch cancellation calls provider/tool zero times
- [ ] `V-T009-06` active provider receives cancellation
- [ ] `V-T009-07` active safe tool receives cancellation and is not retried after caller cancellation
- [ ] `V-T009-08` active Never tool with no durable result settles RequiresIntervention
- [ ] `V-T009-09` completed durable result is not discarded by later cancellation
- [ ] `V-T009-10` Agent terminal dispositions map to matching common statuses
- [ ] `V-T009-11` Agent-terminal/common-nonterminal repair is idempotent and calls provider/tool zero times
- [ ] `V-T009-12` shared runtime API contains no Agent DTO or semantic type

Focused evidence:

```bash
cargo test -p ww-agent-core --test lifecycle --locked
cargo test -p ww-runtime --locked
cargo test -p ww-agent-store-sqlite --test coordinator --locked
```

## V-T010 — Deadlines and budgets

- [ ] `V-T010-01` zero count limit rejects configuration
- [ ] `V-T010-02` model-request count includes every durable attempt start
- [ ] `V-T010-03` a distinct completed-model-turn count includes every durable `ModelAttemptCompleted`, including terminal assistant responses, while T003 `turn_count` remains `TurnCommitted` count
- [ ] `V-T010-04` tool-call count includes every durable handling/execution attempt
- [ ] `V-T010-05` counts reconstruct identically after reopen
- [ ] `V-T010-06` operation at the limit is allowed only if reserved; operation `limit + 1` is never launched
- [ ] `V-T010-07` `now == deadline` is expired
- [ ] `V-T010-08` deadline before launch calls provider/tool zero times
- [ ] `V-T010-09` active deadline expiry cancels provider/tool child token
- [ ] `V-T010-10` normalized input/output/total usage accumulates durably
- [ ] `V-T010-11` reaching/exceeding token limit prevents the next provider call
- [ ] `V-T010-12` BudgetExhausted and TimedOut are distinct audited Agent/common terminal outcomes
- [ ] `V-T010-13` Never ambiguity settles intervention rather than falsely timing out/cancelling
- [ ] `V-T010-14` no limit decision depends on process-local counters

Focused evidence:

```bash
cargo test -p ww-agent-core --test limits --locked
cargo test -p ww-runtime --locked
```

## V-T011 — Recovery matrix

- [ ] `V-T011-F1` restart after creation commit continues existing run once
- [ ] `V-T011-F2` restart after model start appends interruption/new attempt only when permitted
- [ ] `V-T011-F3` restart after model finalization makes zero provider calls before pending tool/terminal handling
- [ ] `V-T011-F4` Safe `ToolEffectStarted`/no effect result creates a new attempt and one logical result
- [ ] `V-T011-F5` Never `ToolEffectStarted`/no effect result performs zero re-execution and settles RequiresIntervention
- [ ] `V-T011-F6` effect output/no model-visible entry appends exactly the reserved entry without execution
- [ ] `V-T011-F7` all results/no turn appends one TurnCommitted without provider/tool work
- [ ] `V-T011-F8` Agent terminal/common nonterminal terminalizes common once without provider/tool work
- [ ] `V-T011-09` F1–F8 resume in a distinct OS process using the same SQLite database
- [ ] `V-T011-10` second restart after every repair adds no effect, logical result, or duplicate terminal event
- [ ] `V-T011-11` impossible history outside the matrix fails closed

Focused evidence:

```bash
cargo test -p ww-agent-store-sqlite --test recovery_matrix --locked
```

## V-T012 — Evaluation and terminal review

- [ ] `V-T012-01` every active check in `EVALUATIONS.md` has a passing current EvaluationRun appended under that check
- [ ] `V-T012-02` each EvaluationRun pins exact commit, command/fixture, date, mode, result, and evidence
- [ ] `V-T012-03` permanent gate passes locally and in hosted CI on exact reviewed commit
- [ ] `V-T012-04` terminal review maps every SPEC requirement family to evidence
- [ ] `V-T012-05` review finds no unsafe replay, duplicate logical result, or undefined repair state
- [ ] `V-T012-06` review finds no concrete transport/filesystem/process/network/product/Flow/Orchestration leakage
- [ ] `V-T012-07` residual findings are classified without automatically changing the active roadmap
- [ ] `V-T012-08` no G003 Stop Condition remains active
- [ ] `V-T012-09` requester explicitly accepts or rejects G003; acceptance is not inferred from branch placement

## Architecture boundary checks for final review

- [ ] `ww-agent-provider` remains provider-neutral and transport-free
- [ ] `ww-agent-tools` remains capability-free and independent from Agent core/runtime/store
- [ ] `ww-agent-core` contains no SQLite or concrete capability/transport
- [ ] Agent DTOs do not enter shared `ww-store`
- [ ] no Agent crate depends on Flow/OWS
- [ ] no public Agent SDK/CLI/TUI/server is added
- [ ] no hidden chain-of-thought or secret value is persisted

## Required Evaluations

All checks in `EVALUATIONS.md` required for G003 closure must have current passing EvaluationRuns on the final reviewed code basis.

## Evidence retained through T006

- T002 verification: temporary verifier branch; provider boundary and clippy passed; full workspace tests passed with 15 `ww-agent-provider` assembler/conformance tests.
- T003 verification: temporary verifier branch; clippy and full workspace tests passed with 11 `ww-agent-core` recovery/corruption tests.
- T004 verification: temporary verifier branch; clippy and full workspace tests passed, including Agent SQLite rollback/reopen/version-conflict and real process-restart reconstruction.
- T005 verification: consolidated engineering commit `69f4ab7ecbed731d40a695dafcf487d62645b695`; full merge-target-equivalent gate passed on Rust 1.98.0: fmt, five architecture boundary checks, clippy `--locked -D warnings`, and 44/44 tests. Coordinator acceptance covers atomic create/link, injected mid-write rollback, and mismatched database path rejection.
- T006 verification: full `main` gate on Rust 1.98.0 — formatting, five architecture checks, locked clippy, and 58/58 tests. RecordedProvider covers the eight required scenarios plus transport-unavailable, determinism, request capture, and script violations.
