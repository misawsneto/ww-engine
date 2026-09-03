# G003 T006/T007 engineering brief — 2026-09-03

Directive to the engineering line. Records the accepted operating model and a bounded path
through T006/T007 without reopening architecture.

Proceed from `main == g003-engineering @ e443c06`.

## Repository policy — accepted

`main` is the continuously integrated, always-green engineering line.

A Goal does not need to be terminally accepted before intermediate work lands on `main`.

Keep these concepts separate:

- **Merged to `main`** — implementation passed the complete engineering gate and is part of the
  integrated codebase.
- **Task complete** — that Task's declared acceptance evidence is satisfied.
- **Goal accepted/achieved** — the Goal's Verification/Evaluation/Review obligations are complete.
- **Architecture accepted** — governed by ADR state, not by branch placement.

Therefore G003 remains active, despite T001–T005 already being on `main`.

Recorded as `D016`. This supersedes the previous branch policy. No engine architecture ADR is
created for Git branching policy.

## Verification gate — accepted

Any task-specific or temporary verification path must execute the complete merge-target CI gate.
It may add checks, but it may not replace or omit target-branch checks.

For `main`, the minimum gate is currently:

```bash
cargo fmt --all -- --check
# architecture-boundary checks from .github/workflows/ci.yml
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Prefer using the permanent `ci.yml` rather than creating temporary workflows. If a temporary
verifier is genuinely needed, it must include the full gate above.

Recorded as `D017`, with `WARNINGS.md` 14 and the Verification section of
`GITHUB-SANDBOX-RULES.md`.

Old branch deletion is housekeeping. It does not block G003 and is not worth engineering time.

## Current G003 state

| Task | State |
| --- | --- |
| T001 activation/bookkeeping | complete |
| T002 provider-neutral protocol/assembler | complete |
| T003 durable history + recovery reducer | complete |
| T004 Agent SQLite persistence | complete |
| T005 common runtime + Agent transaction seam | complete |
| T006 RecordedProvider conformance | **READY** |
| T007 tool validation/policy/replay contracts | **READY** |
| T008 functional Agent kernel | BLOCKED on T006 + T007 |
| T009 lifecycle + cancellation | open |
| T010 durable limits | open |
| T011 crash/restart matrix | open |
| T012 Evaluations + terminal review | open |

Proceed with T006 and T007 independently/in parallel. Do not start T008 until both have passed
their acceptance gates.

## T006 — RecordedProvider conformance

Goal: provide a completely deterministic provider implementation capable of driving the Agent
kernel without HTTP, credentials, or provider nondeterminism.

Keep it behind the provider-neutral contracts established in T002. Implement a scripted/recorded
provider that deterministically emits the same normalized event protocol as a real provider.

Required scenarios, at minimum:

1. text-only completion;
2. one tool call followed by a later final response;
3. multiple tool calls with stable provider source ordering;
4. usage accounting;
5. provider-declared failure;
6. cancellation;
7. truncated/incomplete response;
8. interrupted model attempt / missing terminal event.

The fixture must allow assertions against the requests the Agent sends to it, not merely replay
outputs blindly:

```text
expected request 1
    ↓
stream events
    ↓
expected request 2
    ↓
stream events
    ↓
...
```

Reject unexpected requests or unexpected call order.

**Do not add:** OpenAI-specific HTTP; `reqwest`; API keys/secrets; filesystem capabilities; retry
policy beyond what the fixture contract needs; G004 functionality.

**Acceptance:** deterministic across runs; uses only the normalized provider protocol; existing
T002 protocol tests remain green; provider boundary checks remain green; new conformance tests
exercise all scenarios above; full `main` CI gate passes.

Update G003 task/evidence bookkeeping when complete.

## T007 — Tool contract, validation, policy and replay fixtures

Goal: establish the safety/recovery contract the functional Agent loop will use before any
consequential real tool exists. Keep this independent from filesystem/process/network tooling.

Implement the minimum abstractions needed to represent: tool identity; description/schema;
normalized tool input; logical tool-call identity; attempt identity; validation result; effect
classification; replay classification; policy decision; normalized tool result/error.

Use deterministic test tools only.

### Required fixtures

`test.echo` — a safe deterministic tool.

```text
ReplaySafe
no external side effect
deterministic structured result
```

Use it for ordinary model → tool → model tests.

`test.unsafe_once` — a synthetic non-replayable effect used only to prove recovery rules.

```text
ReplayNever
effect may have happened after start
interruption after start must never silently execute again
```

It does not need to perform a real dangerous external effect. Its purpose is to make the ambiguity
state testable.

### Required execution sequence

```text
receive proposed tool call
        ↓
parse / validate input
        ↓
classify effect + replay semantics
        ↓
policy decision
        ↓
persist execution intent/start boundary
        ↓
execute tool
        ↓
normalize result
        ↓
persist one logical result
```

Malformed arguments must fail before effect execution. A policy denial must not execute the tool.

For G003, keep policy deliberately small. Enough structure to establish the seam, not a policy
language.

The durable model must retain the replay/effect information needed to answer after restart:

- Can this logical call be retried safely?
- Did execution possibly begin?
- Does this state require intervention?

**Critical invariant:** one logical tool call may produce at most one committed model-visible
result.

**Do not add:** `fs.read`; shell/process tools; networking; MCP; approvals UI; general policy DSL;
plugin ABI; parallel tool execution. Those belong later.

**Acceptance:** schema/argument validation before effect execution; policy deny path proves zero
tool execution; `ReplaySafe` interruption can create a new audited attempt without a second logical
result; `ReplayNever` ambiguous interruption produces `RequiresIntervention` or the equivalent
accepted G003 recovery state and does not replay; duplicate committed results are rejected; full
`main` CI gate passes.

Update G003 task/evidence bookkeeping when complete.

## T008 gate

Do not begin the functional Agent loop merely because either T006 or T007 passes. T008 starts only
after both contracts are stable.

```text
T002 provider protocol
       │
       ▼
T006 RecordedProvider ─────┐
                           │
T003 recovery model        │
T004 persistence           ├──→ T008 Agent kernel
T005 transaction seam      │
                           │
T007 tools/policy/replay ──┘
```

T008 should then be a small orchestration kernel, not another AgentSession object:

```text
load durable Agent state
        ↓
check terminal/cancel/limits
        ↓
build provider request
        ↓
RecordedProvider stream
        ↓
finalize assistant response
        ↓
tool calls?
   ├─ no → terminal Agent result
   └─ yes
        ↓
validate/policy/persist/execute tools
        ↓
append results in provider source order
        ↓
request provider again
        ↓
terminal result
```

Do not move product/session/provider-specific concerns into that loop.

## Escalate before changing

Stop and raise for strategic review before changing any of:

- accepted ADR-0003 boundaries;
- the G003/G004 split;
- Agent versus Flow separation;
- provider neutrality;
- Agent persistence versus shared-runtime ownership;
- replay/intervention semantics;
- sequential tool semantics for G003;
- policy authority;
- introduction of HTTP/network/filesystem/process effects;
- new public SDK/CLI/API surfaces;
- any new crate or abstraction that materially changes the current C3/C4 architecture.

Implementation details within the accepted contracts are yours to resolve autonomously.

When you encounter a defect, fix it and record the evidence. When you encounter an architecture
question, do not silently encode the answer in code — surface it for strategic review.

No PR ceremony is required merely to continue development. Keep `main` green and keep the Goal
records synchronized with what has actually been proven.
