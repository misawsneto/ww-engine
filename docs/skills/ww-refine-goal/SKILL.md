---
name: ww-refine-goal
description: Refine an existing WorkWeave Goal specification and downstream planning records under a human-authorized REPLAN_LOCK before implementation continues.
---

# ww-refine-goal

**Status:** approved

## Purpose

Make an existing Goal more precise without changing its Goal state or creating a second planning lifecycle.

One run owns the complete refinement transaction:

```text
accepted requester Decision
        ↓
REPLAN_LOCK
        ↓
refine SPEC
        ↓
refine PLAN
        ↓
reconcile open TASKS + verification/evaluations
        ↓
authority + proof-ownership sweep
        ↓
requester approves complete refined packet
        ↓
record evidence
        ↓
remove REPLAN_LOCK
```

The lock is a flag, not a Goal state.

## Authorization

Do not start a new refinement without an **accepted Decision approved by the requester** that identifies the target Goal and reason.

Keep the Decision short. The Decision authorizes the refinement; this skill defines the procedure.

Example:

```text
D021 | Refine G003 specification and downstream plan before T007 to reduce implementation ambiguity while preserving the accepted Goal boundary, completed Tasks, and used Task IDs. | accepted |
```

D020 introduces this skill. It is not a substitute for the per-run Decision.

### Resume an incomplete refinement

If later review shows that an authorized refinement was interrupted or unlocked before this skill's completion criteria were met:

- if the original Decision still covers the correction, restore the same Goal/Decision lock and **resume that Decision**;
- do not create a new Decision merely to repair incomplete bookkeeping or authority reconciliation;
- if the requested refinement scope has materially changed, obtain a new requester-approved Decision.

A prior approval of the Decision or general direction is not approval of an incomplete resulting packet.

## REPLAN_LOCK

The first repository mutation of a refinement run is to place this block at the top of `AGENTS.md` and make it durable before changing the Goal packet:

```markdown
## GNN REPLAN_LOCK
> only specification or planning related mutations allowed.
- goal: <goal-id>
- decision: <decision-id>
```

Do not change the Goal state.

While the lock exists for a Goal:

- implementation mutations for that Goal are forbidden;
- implementation Tasks for that Goal must not advance or be marked complete;
- specification, planning, verification, review, decision/evidence, handoff, and current-state mutations directly required by refinement are allowed;
- tests and verification commands may run;
- unrelated Goals are not blocked by this Goal-scoped lock.

If the Goal is already locked by the same Decision, resume. If it is locked by a different Decision, stop and ask the requester. Preserve locks for other Goals.

While locked, current-state records MUST NOT claim that the Goal or affected Task is implementation-ready.

## Authority

Refinement may reduce ambiguity inside the governed Goal. It may not silently reshape the governed box.

Authority remains:

```text
accepted ADRs + Decisions
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

Without a separate approved governance change, this skill must not:

- broaden or replace the Goal boundary;
- change accepted architecture;
- add new acceptance obligations to completed Tasks;
- reassign or renumber used Task IDs;
- turn non-blocking review debt into a prerequisite or cleanup gate;
- introduce a prerequisite Goal merely because an improvement is desirable.

If refinement reveals that accepted architecture or the Goal boundary must change, keep the lock in place and stop. Obtain the required Decision/ADR change before continuing.

If a finding does not violate accepted architecture, make existing acceptance impossible, or trigger an existing Stop Condition, map it naturally to an existing open Task or defer it. Do not redesign the roadmap around it.

### Authority reconciliation rule

A new normative requirement must live at the **highest authority layer it affects**, then be reconciled downstream.

Examples:

- ownership, seam, lifecycle, failure, cancellation, durability, or architectural behavior → SPEC first;
- implementation order/dependency/checkpoint → PLAN;
- bounded deliverable/acceptance for an open Task → TASKS, consistent with SPEC/PLAN;
- observable proof → VERIFICATION with stable check identity;
- Goal-level behavioral conformance → EVALUATIONS;
- builder orientation → HANDOFF, never stronger than the packet above it.

Do not leave an architectural MUST only in `TASKS.md` or `HANDOFF.md` when SPEC does not contain the governing contract.

A lower-authority record may be more concrete, but it MUST NOT contradict flexibility explicitly preserved by a higher-authority record.

## Procedure

### 1. Establish the refinement basis

Read the current canonical integration head and target Goal packet before editing:

- `AGENTS.md`;
- `DECISIONS.md` and authorizing Decision;
- governing ADRs;
- `GOAL.md`;
- `SPEC.md`;
- `PLAN.md`;
- `TASKS.md`;
- `VERIFICATION.md`;
- `EVALUATIONS.md`;
- `HANDOFF.md` when present;
- `REVIEWS.md`;
- `PROJECT_STATE.md`, README, and other current-state guidance when relevant.

Identify completed and open Tasks. Completed Task acceptance/evidence are historical facts and remain unchanged by ordinary refinement.

If the refinement was triggered by `ww-dryrun-review`, carry forward only findings whose disposition is `refine-current-goal`. Do not convert deferred or implementation-guidance findings into normative scope.

Surface material assumptions or unresolved choices instead of silently guessing. If the requester must decide something outside the authorizing Decision, keep the lock and ask.

### 2. Publish or restore the lock

Add/restore the `REPLAN_LOCK` before any other repository mutation for the refinement and publish it on the canonical engineering line.

Do not begin or resume implementation while the lock exists.

### 3. Refine the specification

Create a new explicit SPEC version and mark it candidate/pending until requester approval.

If the existing SPEC has no version, treat its accepted contents as `v1` and make the refined document `v2`.

The refined SPEC should clarify, where relevant:

- objective and success conditions already inside Goal boundary;
- interfaces and ownership;
- invariants and ordering;
- allowed/forbidden behavior;
- implementation constraints that are genuinely architectural;
- failure, cancellation, recovery, and ambiguity behavior;
- testable success criteria;
- open questions that must not be guessed.

Do not turn ordinary file/module/function naming into architecture unless the name itself is a governed interoperability or durable semantic contract.

Every new normative requirement needs a defensible basis: governing ADR/Decision/Goal requirement or a clarification necessary to make existing **open** acceptance unambiguous.

### 4. Refine the plan

Update PLAN so it explicitly targets the refined SPEC version.

Make implementation order, dependencies, integration points, risks, and verification checkpoints clear enough that the implementing agent is not deciding architecture or sequencing implicitly.

Preserve Task topology and used IDs by default. Do not introduce prerequisite work unless an existing Stop Condition requires it and the requester has approved the governed change.

### 5. Reconcile open Tasks

Refine only open Tasks as needed so each has:

- focused outcome;
- explicit acceptance criteria;
- concrete verification method;
- clear dependencies;
- implementation-file expectations when useful, but not as accidental architecture.

Do not change completed Task meanings/evidence.

**Task sizing:** strongly prefer roughly five or fewer implementation files per coherent work unit. This is a decomposition signal, not a prohibition.

### 6. Reconcile proof ownership

Every acceptance claim must be assigned to a Task capable of producing the evidence.

Do not let:

- a pure preparation Task claim production effect ordering;
- handcrafted reducer history claim the production driver persisted that history correctly;
- a unit test claim process-restart behavior;
- an integration Task claim Goal-level conformance without an Evaluation.

Move the proof obligation to the Task that crosses the actual boundary while keeping Task IDs/topology stable when possible.

### 7. Reconcile verification, evaluations, and current records

Make the packet internally consistent:

- every new normative requirement has an observable, stable Verification check;
- changed requirements update or supersede stale check wording instead of creating an untracked side-list;
- `EVALUATIONS.md` remains aligned with Goal-level behavior and changed proof ownership;
- `HANDOFF.md` reflects the refined packet without becoming a second SPEC;
- `PROJECT_STATE.md`, README/current guidance, reviews, and cross-references reflect candidate/locked readiness accurately;
- historical recall documents are not rewritten merely to make current records look cleaner.

Keep Decision writing concise. Put the detailed refinement trail in Goal review/verification evidence.

At minimum record:

- authorizing Decision;
- prior and candidate SPEC/PLAN versions;
- open Tasks materially refined;
- proof obligations moved between Tasks, if any;
- verification/evaluation changes;
- findings deliberately deferred;
- requester approval of the resulting packet;
- relevant commit/verification evidence.

### 8. Run the authority and contradiction sweep

Before requesting approval, verify all of the following:

- no new normative requirement exists only in a lower-authority record when it changes a higher-level contract;
- TASKS does not contradict SPEC/PLAN flexibility;
- Verification check ownership matches the Task capable of producing the proof;
- Evaluations test the current contract, not a superseded one;
- HANDOFF does not introduce stronger rules than SPEC/PLAN/TASKS;
- Questions resolved during refinement are reflected consistently in the governed records they affect;
- current-state docs do not say implementation-ready/unlocked while the lock is active;
- no stale version/status reference remains in the current packet;
- no new Goal/Task/ADR/prerequisite was introduced accidentally.

If any check fails, the refinement is not ready for approval or unlock.

### 9. Verify and request approval

Run the repository's complete permanent verification gate required by current governance on the exact candidate head.

Present the requester with a compact review of:

- SPEC changes;
- PLAN changes;
- open Task acceptance/proof-ownership changes;
- Verification/Evaluation changes;
- assumptions/questions resolved or still open;
- findings deliberately deferred;
- exact candidate commit and gate result.

The requester must approve the **complete candidate packet**. Approval of the idea, Decision, earlier partial draft, or one recommendation is not enough to unlock unless it clearly covers the complete resulting packet.

Do not remove the lock until that approval is explicit.

### 10. Unlock

After requester approval:

1. promote candidate version/status to approved;
2. record approval/evidence in canonical review/verification trail;
3. ensure all current planning references point to approved versions;
4. run the required exact-head verification after approval reconciliation when current governance requires it;
5. remove only the target Goal's `REPLAN_LOCK`;
6. leave Goal state unchanged;
7. implementation may resume from the current open Task.

If approval is withheld or the run is interrupted, leave the lock in place. A later run resumes from the same Decision when its scope still applies.

## Completion criteria

The skill is complete only when:

- authorizing Decision is accepted;
- SPEC has a new explicit approved version;
- PLAN is reconciled to that SPEC;
- open Tasks are consistent with SPEC/PLAN;
- proof ownership is assigned to Tasks capable of producing evidence;
- Verification/Evaluations contain stable current coverage for every new normative requirement;
- HANDOFF/current-state records contain no contradictory or stale readiness/version statement;
- no orphan lower-level normative requirement remains;
- completed Task meanings/used IDs remain stable;
- required repository verification passes;
- requester approval of the complete packet is recorded;
- target Goal's `REPLAN_LOCK` is removed.

Until all of these are true, implementation for the locked Goal remains blocked.
