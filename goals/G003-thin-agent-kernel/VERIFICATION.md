# G003 Verification

- Version: `v3`
- State: `draft`
- Approval: `pending requester approval under resumed D022`
- Specification basis: `G003 SPEC v3 (draft)`
- Completed T002–T006 evidence retains its original meaning.

## Permanent deterministic gate

Every later Task closure MUST run the complete merge-target gate required by D017:

```bash
cargo fmt --all -- --check
# permanent architecture-boundary checks from CI
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Focused checks are additive and never replace this gate.

## Completed foundation

### Provider protocol — T002/T006

- [x] text-only recorded stream finalizes one immutable assistant message
- [x] complete tool-call stream finalizes stable call IDs and exact parsed JSON arguments
- [x] invalid event ordering/duplicate finalization/truncated or incomplete tool calls fail closed
- [x] disconnect before finalization is not success
- [x] normalized usage is immutable at finalization
- [x] RecordedProvider detects mismatched/extra/unused exchanges and preserves source order deterministically

### Durable Agent state — T003/T004/T005

- [x] entries/records reconstruct identical recovery state after SQLite reopen and OS restart
- [x] stale Agent writer rejects without partial mutation
- [x] impossible references/order/duplicate logical result/post-terminal records fail closed
- [x] finalized entries are immutable; retries append new attempts
- [x] common execution + Agent run + link commit atomically or roll back together

## V-T007 — Tool preparation, policy, replay, and durable grammar

### Identity and configured order

- [ ] `V-T007-01` empty ToolId/ToolVersion rejects
- [ ] `V-T007-02` duplicate ToolId rejects before run start
- [ ] `V-T007-03` exact pinned version resolves; missing/mismatched version rejects with no substitution
- [ ] `V-T007-04` registry availability/registration order differs from run pin order; projection returns only exact configured pins in configured order

### Schema profile

- [ ] `V-T007-05` valid self-contained Draft 2020-12 schema compiles once and validates repeatedly
- [ ] `V-T007-06` malformed schema rejects registry construction
- [ ] `V-T007-07` non-fragment `$ref` and non-fragment `$dynamicRef` reject before validator compilation with no retrieval; `$id` alone does not reject or retrieve
- [ ] `V-T007-08` local fragment `$ref` and local `$dynamicRef`/`$dynamicAnchor` fixture validates
- [ ] `V-T007-09` invalid instance reports deterministic WorkWeave-owned path/message ordering
- [ ] `V-T007-10` validation is non-coercing and leaves the authoritative parsed Value unchanged
- [ ] `V-T007-11` invalid arguments invoke classification 0, policy 0, execution 0

### Canonical arguments and preparation ordering

- [ ] `V-T007-12` two nested objects differing only in insertion/key order produce equal canonical bytes and equal digest
- [ ] `V-T007-13` semantically different parsed value produces different canonical bytes/digest
- [ ] `V-T007-14` validation occurs before digest/effect/replay classification
- [ ] `V-T007-15` effect/replay classification occurs before policy
- [ ] `V-T007-16` policy evaluates exactly once per preparation attempt
- [ ] `V-T007-17` policy Deny invokes executor/probe 0 times
- [ ] `V-T007-19` `test.echo` returns deterministic structured output and is Safe
- [ ] `V-T007-20` `test.unsafe_once` invokes its probe once per direct execute and is Never

### Durable grammar and reducer

- [ ] `V-T007-24` durable `ToolEffectCompleted` with reserved model-visible result absent reconstructs a repairable completed-awaiting-result state
- [ ] `V-T007-25` interrupted Safe and intervention Never histories are distinct
- [ ] `V-T007-26` reducer rejects changed tool/version/digest/effect/replay/policy across attempts of one logical call
- [ ] `V-T007-27` reducer rejects wrong reserved result ID, effect on NoEffect, duplicate preparation/result, and source-order violation
- [ ] `V-T007-28` Agent history reconstructs the same tool state after SQLite reopen
- [ ] `V-T007-29` tools crate imports no Agent core/runtime/store/SQLite/filesystem/process/network/Flow/Orchestration dependency
- [ ] `V-T007-30` tools public request/context types contain no Agent run/logical-call/attempt/entry identity and no core dependency
- [ ] `V-T007-31` Resolve/Validate/Classify terminate as Rejected; only Policy Deny terminates as Denied

### D022 preparation-contract checks

- [ ] `V-T007-32` exactly one production tools preparation seam is exercised end-to-end; core exposes no competing preparation pipeline
- [ ] `V-T007-33` an effect/replay-aware policy fixture changes decision based on `EffectDescriptor` and/or `ReplayPolicy`; omitting/substituting/late classification metadata fails the proof
- [ ] `V-T007-34` canonicalization test asserts nested canonical serialized bytes directly; digest equality alone cannot satisfy the check
- [ ] `V-T007-35` Q008 placement is exact: `ToolCallPrepared::NoEffect.failed_at=Policy` + Deny; `ToolAttemptDenied` has no duplicate stage field
- [ ] `V-T007-36` tool execution contract represents Output, OrdinaryError, and Cancelled as machine-distinguishable normal outcomes; panic/invariant is outside that outcome contract
- [ ] `V-T007-37` policy Deny returns stable `NoEffect(Policy)` data containing `policy_denied` code/message and durable `PolicyDecision::Deny`; this check does not claim model-visible result persistence
- [ ] `V-T007-38` handcrafted executable history contains source/call/attempt/reserved-result identities plus pinned tool/version, digest, effect, replay, policy, and `ToolEffectStarted`; reducer reconstructs the expected effect-in-flight ambiguity state
- [ ] `V-T007-39` reducer treats `ToolEffectStarted` as an ambiguity boundary, not evidence that an external effect definitely occurred; T007 performs no commit-before-effect production proof
- [ ] `V-T007-40` Resolve/Validate/Classify/Policy `NoEffect` histories contain no effect-start/effect-completion record and reconstruct the matching no-effect state

### Published check identities superseded by D022

These identifiers were published under v2 and remain consumed. D022 changed proof ownership; their meanings are not reused.

- `V-T007-18` — v2 proposition: denial yields exactly one ordered `policy_denied` model-visible result. Production proof now lives in `V-T008-09`.
- `V-T007-21` — v2 proposition: pre-effect durable state contains the complete tool preparation/effect-start identity set. The grammar/reducer portion is now `V-T007-38`; production sequencing is proved by `V-T008-24`.
- `V-T007-22` — v2 proposition: allowed effect is not invoked until the append containing `ToolEffectStarted` commits. Production proof now lives in `V-T008-24`.
- `V-T007-23` — v2 proposition: unknown/invalid/classification/denied paths produce one no-effect audited result with no effect-start/completion. The grammar portion is now `V-T007-40`; production result settlement is `V-T008-09`.

Focused evidence:

```bash
cargo test -p ww-agent-tools --test preparation --locked
cargo test -p ww-agent-tools --locked
cargo test -p ww-agent-core --test recovery --locked
```

## V-T008 — Functional Agent kernel

### Request and stream

- [ ] `V-T008-01` typed stored configuration decodes before provider/tool work
- [ ] `V-T008-02` request maps ordered durable entries and exact pinned tool specs
- [ ] `V-T008-03` provider/model/request digest attempt state commits before provider call
- [ ] `V-T008-04` stream drains through EOF and finalizes exactly once
- [ ] `V-T008-05` unexpected EOF, stream error, post-terminal event, malformed order, or truncated tool call creates no assistant entry/effect
- [ ] `V-T008-06` provider Failed/Aborted has one typed attempt/Agent control outcome
- [ ] `V-T008-07` finalized assistant entry/usage commits before tool handling

### Tool loop and result settlement

- [ ] `V-T008-08` logical IDs allocate once in provider source order and survive reconstruction
- [ ] `V-T008-09` invalid/unknown/classification/denied paths each persist exactly one ordered model-visible error result and execute 0 times
- [ ] `V-T008-10` allowed calls execute sequentially
- [ ] `V-T008-11` provider call order equals model-visible result order in the next request
- [ ] `V-T008-12` durable effect completion precedes/repairs model-visible result
- [ ] `V-T008-13` `TurnCommitted` contains exactly ordered result IDs
- [ ] `V-T008-14` text-only RecordedProvider run commits expected successful Agent result
- [ ] `V-T008-15` RecordedProvider model→test.echo→model run commits expected successful Agent result
- [ ] `V-T008-16` Length completion is audited but not successful
- [ ] `V-T008-17` kernel imports no concrete provider transport, SQLite, capability, Flow/OWS, CLI/TUI/server, or Orchestration type
- [ ] `V-T008-18` outer provider-dispatch error commits one typed failed/interrupted attempt and creates no assistant/tool/effect/automatic retry
- [ ] `V-T008-19` stale expected-version append launches no external work; reload/reduction follows winning durable state
- [ ] `V-T008-20` ordinary returned tool error creates exactly one durable model-visible `is_error=true` result and may be included in next provider request
- [ ] `V-T008-21` tool Cancelled outcome enters cancellation/interruption control flow and is not converted into `ToolEffectCompleted::Error` or a model-visible ordinary tool error
- [ ] `V-T008-22` panic/impossible invariant failure is not normalized into ordinary model-visible tool error; recovery starts from last durable boundary

### D022 production-boundary checks

- [ ] `V-T008-23` kernel calls the single T007 production preparation seam; instrumentation proves no duplicate core preparation stages execute
- [ ] `V-T008-24` allowed fixture executor/probe is invoked only after the append containing `ToolAttemptStarted + ToolCallPrepared::Executable + ToolEffectStarted` commits successfully
- [ ] `V-T008-25` a failed/conflicted pre-effect append invokes executor/probe 0 times
- [ ] `V-T008-26` registry registration order differs from run pin order and provider request exposes tools exactly in run pin order
- [ ] `V-T008-27` cancellation already observable before pre-effect append prevents `ToolEffectStarted` and invocation; post-start cancellation remains a distinct control outcome for T009

Focused evidence:

```bash
cargo test -p ww-agent-core --test kernel --locked
cargo test -p ww-agent-provider --test recorded_provider --locked
cargo test -p ww-agent-tools --locked
```

## V-T009 — Lifecycle and durable cancellation

- [ ] `V-T009-01` missing/mismatched/non-agent common link rejects before work
- [ ] `V-T009-02` Pending starts once; Running/Waiting resumes; matching terminal performs no work
- [ ] `V-T009-03` durable cancellation commits before root-token signal
- [ ] `V-T009-04` repeated registration observes one root; consumer child cancellation cannot cancel siblings
- [ ] `V-T009-05` pre-launch cancellation calls provider/tool 0
- [ ] `V-T009-06` active provider receives cancellation
- [ ] `V-T009-07` active Safe tool receives cancellation and Cancelled remains distinct from ordinary error
- [ ] `V-T009-08` active Never tool with no durable completion settles RequiresIntervention
- [ ] `V-T009-09` completed durable result is not discarded by later cancellation
- [ ] `V-T009-10` Agent terminal dispositions map to matching common statuses
- [ ] `V-T009-11` Agent-terminal/common-nonterminal repair is idempotent and calls provider/tool 0
- [ ] `V-T009-12` shared runtime API contains no Agent DTO/semantic type
- [ ] `V-T009-13` cancellation durable before final pre-effect check prevents marker/invocation; cancellation after effect start obeys replay ambiguity
- [ ] `V-T009-14` same conflicting conditions before/after reopen select same SPEC precedence disposition

Focused evidence:

```bash
cargo test -p ww-agent-core --test lifecycle --locked
cargo test -p ww-runtime --locked
cargo test -p ww-agent-store-sqlite --test coordinator --locked
```

## V-T010 — Deadlines and budgets

- [ ] `V-T010-01` zero count limit rejects configuration
- [ ] `V-T010-02` model-request count includes every durable attempt start
- [ ] `V-T010-03` completed-model-turn count is distinct from T003 `turn_count`
- [ ] `V-T010-04` logical-tool-call count derives from finalized assistant calls and is distinct from T003 `tool_attempt_count`
- [ ] `V-T010-05` counts reconstruct identically after reopen
- [ ] `V-T010-06` provider request at limit is allowed only when reserved; `limit+1` never launches
- [ ] `V-T010-07` `now == ExecutionRecord.deadline` is expired
- [ ] `V-T010-08` expired canonical deadline before launch calls provider/tool 0
- [ ] `V-T010-09` active deadline expiry cancels provider/tool child token
- [ ] `V-T010-10` normalized input/output/total usage accumulates durably
- [ ] `V-T010-11` reaching/exceeding token limit prevents next provider call
- [ ] `V-T010-12` BudgetExhausted and TimedOut are distinct audited Agent/common outcomes
- [ ] `V-T010-13` Never ambiguity settles intervention rather than timeout/cancel
- [ ] `V-T010-14` no limit decision depends on process-local counters
- [ ] `V-T010-15` simultaneous cancel/deadline/budget observations resolve identically before/after reopen
- [ ] `V-T010-16` common deadline is authoritative; Agent snapshot mismatch fails closed before work
- [ ] `V-T010-17` token limits + `usage == false` reject before provider I/O
- [ ] `V-T010-18` usage-capable provider omitting finalized usage fails closed before next provider request
- [ ] `V-T010-19` over-budget finalized multi-call batch prepares/executes 0 tools and settles BudgetExhausted
- [ ] `V-T010-20` exactly fitting batch is admitted in source order with no partial admission path

## V-T011 — Recovery matrix

- [ ] `V-T011-F1` creation restart continues existing run once
- [ ] `V-T011-F2` started model attempt becomes interrupted/new attempt only when permitted
- [ ] `V-T011-F3` finalized model response is not re-requested before pending handling
- [ ] `V-T011-F4` Safe effect-start/no-completion creates new attempt and one logical result
- [ ] `V-T011-F5` Never effect-start/no-completion performs 0 re-execution and requires intervention
- [ ] `V-T011-F6` durable effect completion repairs reserved result without execution
- [ ] `V-T011-F7` missing turn commit repairs once without provider/tool work
- [ ] `V-T011-F8` Agent terminal/common nonterminal repairs once without provider/tool work
- [ ] `V-T011-09` F1–F8 resume in distinct OS process on same SQLite database
- [ ] `V-T011-10` second restart adds no effect/logical result/duplicate terminal event
- [ ] `V-T011-11` impossible history outside matrix fails closed
- [ ] `V-T011-12` F2 pre-event and transient-partial-delta losses leave no durable assistant entry
- [ ] `V-T011-13` competing resume drivers cannot both authorize external work

## V-T012 — Evaluation and terminal review

- [ ] `V-T012-01` every active Evaluation check has a current passing EvaluationRun
- [ ] `V-T012-02` each EvaluationRun pins exact commit, command/fixture, date, mode, result, evidence
- [ ] `V-T012-03` permanent gate passes locally and hosted on exact reviewed commit
- [ ] `V-T012-04` terminal review maps every SPEC requirement family to evidence
- [ ] `V-T012-05` review finds no unsafe replay, duplicate logical result, or undefined repair state
- [ ] `V-T012-06` review finds no concrete transport/capability/product/Flow/Orchestration leakage
- [ ] `V-T012-07` residual findings are classified without automatically changing roadmap
- [ ] `V-T012-08` no G003 Stop Condition remains active
- [ ] `V-T012-09` requester explicitly accepts/rejects G003
- [ ] `V-T012-10` review traces each open Task to reference evidence, WorkWeave adoption, and deferrals

## Architecture boundary checks for final review

- [ ] provider crate remains provider-neutral and transport-free
- [ ] tools crate remains capability-free and independent from Agent core/runtime/store
- [ ] Agent core contains no SQLite or concrete capability/transport
- [ ] Agent DTOs do not enter shared `ww-store`
- [ ] no Agent crate depends on Flow/OWS
- [ ] no public Agent SDK/CLI/TUI/server is added
- [ ] no secret value or hidden chain-of-thought is persisted

## Evidence retained through T006

- T002: provider boundary + assembler/conformance tests passed.
- T003: recovery/corruption tests passed.
- T004: rollback/reopen/version-conflict and process-restart reconstruction passed.
- T005: atomic create/link + injected rollback passed under full gate.
- T006: RecordedProvider conformance + full `main` gate passed with 58/58 tests at its closure checkpoint.
