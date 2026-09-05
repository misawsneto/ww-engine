# G003 Builder Handoff

- Status: current implementation orientation; the D022 `REPLAN_LOCK` is removed
- Goal: `G003 — Durable Agent Kernel`
- Next implementation Task: `T008`
- Governing architecture: `ADR-0003`
- Approved planning basis: `SPEC v3`, `PLAN v3`, `TASKS.md`, `VERIFICATION.md`, `EVALUATIONS.md`
- Refinement authority: D021 baseline + resumed D022

## Purpose

This is orientation, not a second specification.

Authority:

```text
accepted Decisions + ADR-0003
        ↓
GOAL
        ↓
SPEC
        ↓
PLAN
        ↓
TASKS
        ↓
VERIFICATION / EVALUATIONS
        ↓
HANDOFF
        ↓
implementation
```

Historical handoffs and dry-run artifacts are evidence only. The A004 dry-run snippets are not source-code authority.

## Builder profile

Operate as a **Rust systems builder specialized in CLI agents and workflow automation**.

Expected strengths:

- idiomatic Rust 2024, async traits, ownership, typed errors, pure reduction, testable ports;
- provider/tool seams for coding/CLI agents;
- durable execution, SQLite recovery, optimistic concurrency, cancellation, limits, restart behavior;
- small composable crates and dependency direction rather than framework-heavy abstractions;
- TDD and fault-oriented verification.

You are the builder, not the roadmap owner. Implement the governed Task outcome; do not redesign the Goal because another architecture looks more general.

## Engineering principles

1. **Simplest compliant path.** Delete unnecessary machinery before adding another layer.
2. **Contract before machinery.** Stabilize the current Task seam first.
3. **One preparation authority.** Tool preparation lives once in `ww-agent-tools`; core consumes it.
4. **Durable before ambiguity.** Provider/effect work starts only after authorizing state commits.
5. **Recovery from durable truth.** Never infer restart state from process memory.
6. **Fail closed.** Unknown/corrupt/unsafe ambiguity is not guessed into success.
7. **One coherent Task at a time.** Do not pull T008/T009 acceptance into T007 for convenience.
8. **Always-green main.** Focused proof + complete D017 gate before Task closure.
9. **Reference projects are evidence.** Preserve/adapt/reject their seams; do not clone product machinery.
10. **Debt is not scope.** Defer non-blocking hardening rather than inserting cleanup gates.

## Decision ladder

### 1. Current Task acceptance is not satisfied

Stay in the Task. Add/tighten the failing test, make the minimum compliant change, rerun focused proof.

### 2. A test exposes an implementation defect inside the accepted contract

Fix it with TDD. No new Decision is required.

### 3. SPEC permits multiple implementations

Choose the smallest implementation preserving ownership, recovery, and verification. Record a short Task-review rationale only when the choice materially affects a contract.

### 4. Finding is useful but not required for current acceptance

Record/defer it. Do not create a cleanup gate or prerequisite.

### 5. Task cannot be completed safely under accepted architecture

Stop when architecture/Goal/ADR would be violated, acceptance is impossible, or an explicit Stop Condition fires. Raise the conflict before implementation relies on a changed direction.

### 6. SPEC/PLAN must change

Use `ww-refine-goal`; do not patch lower-authority records around a higher-authority conflict.

### 7. Durable state is ambiguous and no governed repair exists

Fail closed. A started Never effect without durable completion is the canonical intervention case.

## TDD discipline

Before code:

1. read Task outcome/acceptance;
2. read matching SPEC requirement IDs;
3. read matching `V-T00N` checks and Evaluations;
4. inspect current implementation/reference seam only as needed;
5. choose the smallest observable red test.

Red → Green → Refactor:

- Red must fail for the intended missing behavior.
- Green is the smallest coherent implementation.
- Refactor removes duplication without expanding architecture.

Every work-unit checkpoint runs focused tests plus the complete permanent gate.

## T007 orientation — after approval/unlock

T007 proves **tool preparation + durable grammar**, not real Agent effect ordering.

```text
A. identity + schema + configured-order projection
        ↓
B. canonical bytes/digest + one tools preparation seam
        ↓
C. effect/replay-aware policy + fixtures + distinct execution outcome contract
        ↓
D. Agent-owned durable tool grammar + reducer
        ↓
T007 Verification + full gate
```

Hard boundaries:

- parsed `serde_json::Value` is sole executable argument authority;
- one tools preparation seam owns resolve → validate → digest → effect/replay → policy;
- `ToolPreparationDisposition` and `ToolPreparationStage` are defined in `ww-agent-tools` and returned by that seam;
- `ww-agent-core` embeds those exact tools-owned types in Agent-owned durable records; it does not duplicate the preparation taxonomy;
- there is no tools→core dependency;
- run configured pin order outranks registry registration order;
- non-fragment `$ref` and `$dynamicRef` are forbidden in G003; `$id` alone is not;
- policy input requires effect/replay structurally, and behavioral conformance proves exact observation plus substitution sensitivity;
- Output, OrdinaryError, and Cancelled are distinct normal tool execution outcomes;
- `ToolExecutionError` never means cancellation;
- `ToolEffectStarted` is an ambiguity marker only;
- Q008: Policy stage lives in `ToolCallPrepared::NoEffect`; `ToolAttemptDenied` gains no stage field;
- T007 may directly execute fixtures for fixture tests, but does not claim commit-before-effect proof.

Function/module names are conventional. Prefer direct Rust naming; do not add a manager/service layer just to wrap the preparation seam.

## T008 orientation

T008 is where production execution begins.

```text
tools-owned T007 preparation outcome
        ↓
core embeds it in Agent-owned attempt/preparation state
        ↓
Executable? persist ToolEffectStarted + COMMIT
        ↓
only then execute
        ↓
map Output | OrdinaryError | Cancelled distinctly
        ↓
one ordered model-visible result when applicable
```

T008 must prove with an effect probe that failed/conflicted pre-effect commit invokes executor zero times.

## Completion behavior

When one Task is satisfied:

1. record exact Verification evidence;
2. mark only that Task complete;
3. update current state/next Task;
4. keep `main` green;
5. continue only to dependency-ready work.

Task completion is not Goal acceptance.
