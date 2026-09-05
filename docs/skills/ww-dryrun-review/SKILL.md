---
name: ww-dryrun-review
description: Review a WorkWeave implementation dry run against the governed Goal packet, current code, and pinned reference architecture before implementation relies on it.
---

# ww-dryrun-review

**Status:** approved

## Purpose

Review a dry run as an implementation hypothesis before code relies on it.

A dry run is useful when it exposes hidden design choices, false-green tests, incorrect seam ownership, or proof obligations that the existing planning packet leaves implicit.

A dry run is **not** specification authority.

```text
current governed records
        +
current code
        +
pinned reference evidence
        ↓
dry run
        ↓
review
        ↓
proceed | implementation guidance | refine Goal | governance change | defer
```

The goal of the review is not to grade prose. It is to answer:

> If an implementing agent follows this dry run, will it build the governed design correctly and prove the right things at the right boundaries?

## Authorization

This skill is review-only by default and needs no new Decision.

Run it when the requester asks to review, critique, validate, or assess an implementation dry run.

Do not mutate governed Goal records merely because the review found an improvement.

If the requester asks to apply findings:

- implementation guidance already inside the accepted contract may remain implementation guidance;
- normative specification/planning changes use `ww-refine-goal`;
- Goal, domain-model, or accepted-architecture changes require the appropriate requester Decision/ADR change before reliance.

## Authority

Review against the repository's actual authority order:

```text
accepted Decisions + ADRs
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
dry-run artifact
        ↓
implementation sketch
```

The dry run may explain or challenge higher records. It may not silently override them.

When a dry-run recommendation conflicts with a higher-authority record, report the conflict. Do not ask the builder to choose which record to obey.

## Review basis

Read the smallest complete basis needed to judge the dry run:

- current canonical integration head;
- `AGENTS.md` and any active `REPLAN_LOCK`;
- the dry-run artifact;
- authorizing Decisions and governing ADRs;
- `GOAL.md`;
- current `SPEC.md`;
- `PLAN.md`;
- target Task in `TASKS.md`;
- matching `VERIFICATION.md` and `EVALUATIONS.md`;
- `HANDOFF.md` when present;
- `PROJECT_STATE.md` when readiness/current state matters;
- actual implementation files and dependency graph touched by the dry run;
- only the pinned reference-architecture source needed to validate material claims.

Do not rely on a historical handoff or stale branch when current records exist.

## Core review rule

Treat every dry-run prescription as one of four things:

1. **already governed** — directly required by higher authority;
2. **implementation choice** — one valid way to satisfy the governed contract;
3. **new normative requirement** — changes what implementation or evidence must satisfy;
4. **future/deferred improvement** — useful but not required for the current Task.

The review must not let category 2 or 4 silently become category 3.

## Procedure

### 1. Establish exact state

Confirm:

- exact current `main` head;
- target Goal and Task;
- whether implementation has started;
- whether a planning lock is active;
- whether the dry run is code-free, partially executable, or compiled/tested;
- which permanent gate applies.

A green repository proves only the current repository state. It does not prove that dry-run pseudocode compiles or that the proposed design satisfies unimplemented acceptance.

### 2. Read the dry run as a hypothesis

Extract the dry run's material claims:

- proposed seams and ownership;
- type/API shape;
- ordering and lifecycle;
- persistence/effect boundaries;
- failure and cancellation behavior;
- test strategy;
- file/crate changes;
- reference-project claims;
- questions and assumptions.

Ignore cosmetic code unless it implies an architectural or contract decision.

Do not promote illustrative snippets into normative API requirements merely because they are concrete.

### 3. Check repository realism

Verify the dry run against the actual codebase.

Look for:

- existing ports that already satisfy the need;
- invented ports, managers, state machines, or crates;
- dependency-direction violations;
- incorrect assumptions about current types/APIs;
- duplicated logic across boundaries;
- file/module plans that are unnecessarily prescriptive;
- tests that could pass accidentally because of current library behavior;
- pseudo-code that was never compile-validated.

Prefer reuse and deletion over added machinery.

If the existing seam is sufficient, flag a proposed replacement as unnecessary even when the replacement is elegant.

### 4. Check domain-model impact first

Before recommending planning changes, answer explicitly:

> Does the dry-run finding require a new domain entity, identity, relationship, state, lifecycle, authority, durable semantic record, or changed meaning of an existing one?

Classify:

- **No domain-model change** — implementation/specification architecture may still need refinement.
- **Domain-model change required** — stop ordinary Task hardening and require deeper validation/governance before implementation relies on it.

Do not confuse "no domain-model change" with "Task-only edit is sufficient."

### 5. Check reference fidelity

For each material upstream claim:

- verify it against the pinned source;
- distinguish production behavior from experimental/future scaffolding;
- distinguish observed reference behavior from WorkWeave adaptation;
- verify that WorkWeave is preserving the intended seam rather than copying product machinery;
- verify that deliberate deviations are explicit.

For Pi-oriented Agent work, pay particular attention to:

- small functional loop versus product/session façade;
- provider-neutral streaming seam;
- preparation versus execution separation;
- source-ordered tool results;
- cancellation propagation;
- entry/record separation and pure recovery reduction;
- replay ambiguity;
- avoiding queues/hooks/parallel/session machinery when the Goal excludes them.

Reference parity is behavioral. Package names and implementation structure do not need to match upstream.

### 6. Check seam and ownership quality

For every proposed seam, ask:

- who owns the decision?
- who owns durable identity?
- who owns execution?
- which direction may dependencies flow?
- does another layer duplicate the same logic?
- does the seam carry domain identities it does not need?
- is a generic manager/service object being introduced where a function/trait/port is enough?

A strong dry run makes one authority obvious for each responsibility.

### 7. Check proof ownership

Every acceptance claim must belong to a Task capable of producing the evidence.

Examples:

- a pure preparation Task can prove classification order and reducer grammar;
- only the real kernel can prove an external effect occurs after a durable commit;
- only lifecycle integration can prove durable cancellation reaches live work;
- only restart tests can prove process-loss behavior.

Flag a verification check when the assigned Task cannot actually produce its evidence.

Do not allow a manually constructed history to masquerade as proof that the production driver persisted that history in the required order.

### 8. Check failure, cancellation, and recovery algebra

Inspect the dry run for conflated outcomes.

Keep distinct when the governed design requires it:

- ordinary operational error;
- cooperative cancellation;
- timeout/budget termination;
- panic/invariant violation;
- retryable interruption;
- unsafe ambiguity requiring intervention;
- corruption.

If the API cannot represent distinctions required by the higher-level behavior, treat that as an architecture-contract gap rather than leaving the implementer to improvise.

### 9. Check tests for false confidence

Look for tests that prove the wrong thing or can pass accidentally.

Typical risks:

- equality without observing canonical bytes/order;
- mocks that bypass the production seam;
- manually constructed history used as production ordering proof;
- fixture registration order accidentally matching configured order;
- disabled network features assumed to provide a stable WorkWeave error contract;
- a type carrying metadata that no behavior actually consumes;
- tests of helpers when acceptance requires the production boundary.

Require the smallest test that observes the real invariant.

### 10. Check authority consistency

Compare dry-run recommendations with SPEC/PLAN/TASKS/V&V/HANDOFF.

Flag:

- new MUSTs that exist only in a Task or handoff;
- Task wording that contradicts SPEC flexibility;
- verification requirements with no stable check ID;
- Evaluations that still test an older contract;
- current-state docs saying implementation-ready while a refinement is incomplete;
- a review finding recorded as mandatory scope without higher-authority basis.

No implementing agent should have to decide which governed record wins.

### 11. Classify findings

Use severity based on implementation risk, not prose quality.

| Severity | Meaning |
| --- | --- |
| **Blocker** | implementation would violate Goal/ADR/domain model, cross an unsafe boundary, or rely on contradictory authority |
| **High** | likely to misdirect architecture, assign proof to the wrong Task, or leave cancellation/recovery/effect behavior undefined |
| **Medium** | material conformance/test/realism gap that should be corrected before Task closure |
| **Low** | maintainability, naming, organization, or documentation issue with no acceptance/safety effect |

For each finding record one disposition:

- `implementation-guidance`;
- `refine-current-goal`;
- `governance-change-required`;
- `defer`;
- `reject-dry-run-claim`.

### 12. Decide implementation readiness

End with one explicit status:

- **ready** — no blocking/high unresolved architecture or authority issue;
- **ready with implementation guidance** — contract is complete; findings do not require record changes;
- **not ready — refinement required** — governed records must be reconciled through `ww-refine-goal`;
- **not ready — governance change required** — Goal/domain/ADR change must be settled first.

Do not equate green CI with implementation readiness.

## Integration with `ww-refine-goal`

When this review finds a material normative gap and the requester asks to apply it:

```text
dry-run finding
        ↓
domain / ADR / Goal change?
    yes → governance first
    no
        ↓
accepted refinement Decision exists?
    yes → resume/use that Decision
    no  → requester-approved refinement Decision
        ↓
ww-refine-goal
        ↓
SPEC → PLAN → TASKS → V&V → HANDOFF/current state
        ↓
requester approval
        ↓
unlock
```

Do not patch TASKS alone when the finding changes architecture, ownership, failure behavior, or acceptance proof.

If a prior refinement was unlocked prematurely and the original Decision still covers the correction, restore the lock and resume the same Decision instead of creating a bookkeeping Decision.

## Record handling

By default, the review may be delivered in chat without repository mutation.

When the requester asks to persist it:

- record current findings/disposition in the Goal's canonical `REVIEWS.md`;
- leave the original dry-run artifact unchanged as evidence;
- record builder-raised unresolved questions in the canonical Questions record when useful;
- do not rewrite historical recall files to match current conclusions.

If the review triggers refinement, let `ww-refine-goal` own the governed record mutation transaction.

## Output contract

A useful dry-run review contains:

1. **Verdict** — ready/not ready and why.
2. **Domain-model impact** — explicit yes/no plus affected concepts if yes.
3. **Reference alignment** — preserve/adapt/reject assessment for material seams.
4. **Findings** — severity, evidence, risk, disposition.
5. **Proof ownership** — which Task must prove each disputed invariant.
6. **Record impact** — no change / implementation guidance / records needing refinement / governance change.
7. **Open questions** — only decisions the implementation agent must not guess.

Keep the conclusion direct. Do not bury a blocker under general praise.

## Stop conditions

Stop and require governance/refinement before implementation when:

- the dry run requires a domain-model or accepted-architecture change;
- higher-authority records contradict the proposed implementation;
- one required safety/durability invariant has no owner capable of proving it;
- cancellation/recovery/effect ambiguity cannot be represented by the accepted contract;
- the dry run introduces a new prerequisite/Task/Goal without an existing Stop Condition;
- an implementation agent would have to choose between contradictory normative records.

## Completion criteria

The review is complete when:

- exact repository and dry-run basis are identified;
- material dry-run claims are checked against current code and governing records;
- relevant reference claims are grounded in pinned source;
- domain-model impact is explicitly classified;
- seam ownership and proof ownership are assessed;
- false-green and recovery/cancellation risks are assessed;
- findings have severity and disposition;
- implementation readiness is explicit;
- any required refinement/governance route is identified without silently mutating scope.
