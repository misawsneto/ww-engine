# G003 Builder Handoff

- Status: current implementation orientation
- Goal: `G003 — Durable Agent Kernel`
- Next Task: `T007 — tool contract, schema validation, policy, and replay fixtures`
- Governing architecture: `ADR-0003`
- Planning basis: approved `SPEC v2`, `PLAN v2`, `TASKS.md`, `VERIFICATION.md`, and `EVALUATIONS.md`
- Refinement authority: `D021`

## Purpose

This is the live handoff for the implementing agent. It is an orientation layer, not a second specification or plan.

When this file conflicts with a governing record, follow this authority order:

```text
accepted Decisions + ADR-0003
        ↓
GOAL.md
        ↓
SPEC.md
        ↓
PLAN.md
        ↓
TASKS.md
        ↓
VERIFICATION.md / EVALUATIONS.md
        ↓
this HANDOFF.md
        ↓
implementation
```

Historical handoffs under `docs/memories/recall/` are evidence only. Do not use their old branch instructions or execution checkpoints as current authority.

## Builder profile

Operate as a **Rust systems builder specialized in CLI agents and workflow automation**.

Your expected strengths are:

- idiomatic Rust 2024, async traits, ownership, typed errors, deterministic state reduction, and testable ports;
- model/provider and tool execution seams for coding/CLI agents;
- durable execution, SQLite-backed recovery, optimistic concurrency, cancellation, limits, and restart behavior;
- command-line and workflow-engine architecture, while respecting that G003 itself deliberately adds no public CLI or Flow surface;
- small composable crates and dependency direction rather than framework-heavy abstractions;
- TDD and fault-oriented verification for behavior that must survive process failure.

Your role is **builder, not roadmap owner**. Implement the accepted architecture and Task outcome. Do not redesign the Goal because another design looks cleaner or more general.

## Engineering principles

1. **Simplest compliant path.** Prefer the smallest design that satisfies the Task acceptance and SPEC invariants.
2. **Contract before machinery.** Stabilize the narrow types/ports needed by the current Task before adding orchestration objects or helpers.
3. **Durable before ambiguity.** Provider or tool work may cross an ambiguity boundary only after the durable record that authorizes that work commits.
4. **Recovery from durable truth.** Restart behavior is derived from entries, records, common execution state, and pinned configuration—not process memory.
5. **Keep kernels separate.** Agent, Flow, provider, tools, common runtime, and Orchestration keep their ownership boundaries.
6. **Fail closed.** Unknown durable states, unsafe replay ambiguity, malformed model output, and invalid tool input do not get guessed into success.
7. **One coherent Task at a time.** Do not pull later Task acceptance forward merely because adjacent code is convenient to change.
8. **Always-green `main`.** A Task increment lands only after its focused proof and the complete permanent D017 gate pass.
9. **Reference implementations are evidence.** Preserve/adapt/reject their design lessons as specified; do not copy their product machinery or terminology indiscriminately.
10. **Debt is not automatically scope.** Non-blocking improvements are recorded or deferred, not converted into prerequisite cleanup work.

## Task-outcome decision ladder

Use this ladder whenever implementation produces an unexpected result.

### 1. The current Task acceptance is not yet satisfied

Stay inside the Task. Write or tighten the failing test, make the minimum compliant implementation change, and rerun the focused proof.

Do not escalate merely because implementation is difficult.

### 2. A test exposes an implementation defect inside the approved contract

Fix it with TDD inside the current Task.

Examples:

- invalid arguments reached policy;
- a stale writer could launch an effect;
- a provider stream was not finalized through EOF;
- ordering or result uniqueness is wrong.

No new Decision is required when the accepted SPEC already determines the correct behavior.

### 3. The SPEC permits more than one implementation

Choose the simplest implementation that preserves ownership, recovery, and verification semantics.

If the choice materially affects future maintainability or a public/internal contract, record a short rationale in the Task review. Do not create a durable Decision for ordinary local design.

### 4. A finding is useful but not required for the current Task outcome

Record/defer it and continue.

Do not insert a cleanup gate, prerequisite Task, or future-Goal dependency unless existing acceptance or a Stop Condition actually requires it.

### 5. The current Task cannot be completed safely under the accepted architecture

Stop implementation when any of these is true:

- accepted architecture would be violated;
- the Task acceptance is impossible under the current SPEC;
- an explicit PLAN Stop Condition fires;
- satisfying the Task requires changing the Goal boundary, used Task identity, or accepted ADR direction.

Raise the conflict for requester review. If architecture changes, amend/supersede the governing ADR before implementation relies on the new direction.

### 6. Specification or planning must change

Do not edit the executable planning basis casually during implementation.

Use `docs/skills/ww-refine-goal/SKILL.md`:

```text
short requester-approved Decision
        ↓
Goal REPLAN_LOCK in AGENTS.md
        ↓
refine SPEC + PLAN + open Tasks + V&V
        ↓
requester approval + verification
        ↓
remove lock
        ↓
implementation resumes
```

### 7. Durable state is ambiguous and no approved repair exists

Fail closed. Do not invent recovery behavior in code.

For G003, a started `ReplayPolicy::Never` effect without durable completion is the canonical example: it requires intervention rather than silent replay.

## TDD execution discipline

Use test-driven development for each work unit.

### Before code

1. Read the current Task description, acceptance criteria, dependencies, and likely files.
2. Read the SPEC requirement IDs exercised by that Task.
3. Read the matching `V-T00N` checks and Evaluation expectations.
4. Inspect the existing implementation and the pinned reference seam only as needed.
5. Identify the smallest observable behavior that should fail first.

### Red

Add or tighten a focused test that expresses one Task requirement.

The test must fail for the expected missing/wrong behavior, not because the fixture is broken or compilation is unrelatedly failing.

For durability work, prefer tests that assert both:

- the required durable state/result; and
- prohibited provider/effect invocations.

### Green

Implement the smallest coherent change that makes the focused test pass while preserving the SPEC boundaries.

Do not implement later acceptance preemptively.

### Refactor

After the behavior is green:

- remove duplication;
- improve names and module seams;
- keep WorkWeave-owned errors/types at boundaries;
- preserve the same observable tests.

Do not use refactoring as a reason to expand architecture.

### Task checkpoint

Run the focused commands listed in `TASKS.md`, then the complete permanent gate:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

The permanent architecture-boundary checks in `.github/workflows/ci.yml` are also mandatory under D017.

A Task is not complete until the exact closure commit has hosted CI success and its Verification evidence is recorded.

## T007 starting orientation

T007 is the next implementation slice. Execute its internal work units in the PLAN order:

```text
A. ww-agent-tools identity + offline Draft 2020-12 schema contract
        ↓
B. registry + digest + effect/replay + Allow/Deny policy + fixtures
        ↓
C. Agent-owned durable preparation/effect/result vocabulary + reducer
        ↓
T007 verification + full gate
```

Keep these boundaries explicit:

- `ww-agent-tools` must not depend on `ww-agent-core`;
- parsed `serde_json::Value` is the sole executable argument authority;
- invalid/unknown/denied calls perform zero effect;
- allowed effect execution starts only after durable authorization state commits;
- `test.echo` is pure/Safe;
- `test.unsafe_once` is synthetic/Never;
- T007 does not implement the model→tool→model loop; that remains T008.

## Completion behavior

When a Task outcome is satisfied and verified:

1. record the exact evidence in `VERIFICATION.md`;
2. mark only that Task complete;
3. ensure `PROJECT_STATE.md` still identifies the correct next Task;
4. keep `main` green;
5. continue only to the dependency-ready next Task.

Task completion is not Goal acceptance. G003 closes only after T012 Evaluations/review and explicit requester acceptance.
