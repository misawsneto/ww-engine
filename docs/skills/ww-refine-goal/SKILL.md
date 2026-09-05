---
name: ww-refine-goal
description: Refine an existing WorkWeave Goal specification and downstream planning records under a human-authorized REPLAN_LOCK before implementation continues.
---

# ww-refine-goal

**Status:** approved

## Purpose

Make an existing Goal more precise without changing its Goal state or creating a second planning lifecycle.

```text
accepted requester Decision
        ↓
REPLAN_LOCK
        ↓
refine SPEC
        ↓
refine PLAN
        ↓
reconcile open TASKS + Verification/Evaluations
        ↓
requester approves complete refined packet
        ↓
record evidence
        ↓
remove REPLAN_LOCK
```

The lock is a flag, not a Goal state.

## Authorization

Do not start a new refinement without an **accepted requester-approved Decision** identifying the target Goal and reason.

Keep the Decision short. It authorizes the refinement; this skill defines the procedure.

D020 introduces this skill. It is not a substitute for the per-run Decision.

### Resume an incomplete refinement

If a refinement was interrupted or was unlocked before this skill's completion criteria were met:

- restore the same Goal/Decision lock when the existing Decision still covers the correction;
- resume the existing refinement;
- do not create a new Decision merely to repair incomplete reconciliation or bookkeeping.

Obtain a new Decision only when the requested refinement scope has materially changed.

## REPLAN_LOCK

The first repository mutation of a new refinement run is:

```markdown
## GNN REPLAN_LOCK
> only specification or planning related mutations allowed.
- goal: <goal-id>
- decision: <decision-id>
```

Publish the lock before changing the Goal packet.

Do not change Goal state.

While locked:

- implementation mutations for that Goal are forbidden;
- implementation Tasks for that Goal must not advance;
- specification, planning, Verification/Evaluations, review, question/decision evidence, handoff, and current-state mutations required by refinement are allowed;
- verification commands may run;
- unrelated Goals are not blocked.

If the same Goal is already locked by the same Decision, resume. If it is locked by another Decision, stop and ask the requester.

## Authority

Refinement may reduce ambiguity inside the governed Goal. It may not silently reshape the governed box.

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

Without separate approved governance, do not:

- broaden or replace the Goal boundary;
- change accepted architecture;
- add acceptance obligations to completed Tasks;
- reassign or renumber used Task IDs;
- turn non-blocking debt into a prerequisite cleanup gate;
- introduce a prerequisite Goal merely because an improvement is desirable.

If accepted architecture or Goal boundary must change, keep the lock and obtain the required Decision/ADR change before continuing.

If a finding does not violate architecture, make acceptance impossible, or trigger an existing Stop Condition, map it naturally to an open Task or defer it.

### Normative reconciliation safeguard

A new normative requirement must be introduced at the **highest authority layer it affects** and reconciled downward.

Examples:

- architecture, ownership, seam, lifecycle, failure, cancellation, durability → SPEC;
- implementation ordering/dependency/checkpoint → PLAN;
- bounded open-Task deliverable/acceptance → TASKS;
- observable proof → VERIFICATION;
- Goal-level conformance → EVALUATIONS;
- builder orientation → HANDOFF only.

Do not add an architectural `MUST` only to TASKS or HANDOFF when SPEC does not contain the governing contract.

A lower-authority record may be more concrete. It must not contradict flexibility preserved by higher authority.

## Procedure

### 1. Establish the refinement basis

Read the current canonical head and the target Goal packet:

- `AGENTS.md`;
- authorizing Decision and governing ADRs;
- `GOAL.md`;
- `SPEC.md`;
- `PLAN.md`;
- `TASKS.md`;
- `VERIFICATION.md`;
- `EVALUATIONS.md`;
- `HANDOFF.md` when present;
- `REVIEWS.md`;
- `PROJECT_STATE.md` and current README/guidance when relevant.

Identify complete and open Tasks. Completed Task acceptance/evidence are historical facts and remain unchanged.

If refinement follows `ww-dryrun-review`, carry forward only findings classified **refine current Goal**. Do not promote implementation guidance or deferred findings into requirements.

Surface material unresolved choices rather than guessing.

### 2. Publish or restore the lock

Add or restore the Goal's `REPLAN_LOCK` before Goal-packet mutation.

Do not implement while the lock exists.

### 3. Refine SPEC

Create a new explicit SPEC version and keep it candidate/pending until requester approval.

Clarify only what is needed inside the accepted Goal, including where relevant:

- interfaces and ownership;
- invariants and ordering;
- allowed/forbidden behavior;
- failure/cancellation/recovery behavior;
- architecture constraints;
- testable success criteria;
- questions the implementer must not guess.

Every new normative requirement needs a defensible basis in governing records or a clarification required to make an existing open Task unambiguous.

Do not turn ordinary file/module/function naming into architecture unless the name itself is a governed interoperability or durable semantic contract.

### 4. Refine PLAN

Update PLAN to the refined SPEC version.

Make sequencing, dependencies, integration points, risks, and checkpoints explicit enough that implementation does not choose architecture implicitly.

Preserve Task topology and used IDs by default.

### 5. Reconcile open Tasks

Refine only open Tasks as needed so each has:

- focused outcome;
- explicit acceptance criteria;
- concrete verification;
- clear dependencies;
- likely files when useful, without turning file layout into accidental architecture.

Do not change completed Task meaning or evidence.

**Task sizing:** roughly five implementation files per coherent work unit is a strong decomposition signal, not a prohibition.

### 6. Reconcile Verification / Evaluations and current records

Make the packet internally consistent:

- each new normative requirement has an observable Verification check;
- proof is assigned to a Task capable of producing it;
- `EVALUATIONS.md` remains aligned with Goal-level behavior;
- HANDOFF reflects but does not strengthen the governed packet;
- questions resolved during refinement are reflected where they affect governed behavior;
- PROJECT_STATE/README/current guidance reflect locked/candidate status accurately;
- historical recall documents remain historical.

Keep Decision writing concise. Record detailed refinement history in review/verification evidence.

### 7. Consistency safeguard before unlock

Before requesting approval, check:

- SPEC → PLAN → TASKS → VERIFICATION/EVALUATIONS are mutually consistent;
- no architectural requirement exists only in a lower-authority record;
- no Task contradicts SPEC/PLAN flexibility;
- every Verification claim belongs to a Task capable of producing that evidence;
- HANDOFF introduces no stronger rule than the governed packet;
- current-state records do not claim implementation-ready/unlocked while the lock is active;
- no stale version/status reference remains in current records.

If any item fails, refinement is not ready for approval or unlock.

### 8. Verify and request approval

Run the complete permanent repository gate required by current governance on the exact candidate head.

Present the requester with a compact review of:

- SPEC changes;
- PLAN changes;
- open Task changes;
- Verification/Evaluation changes;
- proof ownership changes;
- assumptions/questions resolved or open;
- findings deliberately deferred;
- candidate commit and gate result.

The requester approves the **complete resulting packet**, not merely the Decision or an earlier partial recommendation.

Do not remove the lock until approval is explicit.

### 9. Unlock

After requester approval:

1. promote candidate status/version to approved;
2. record requester approval and evidence;
3. ensure current references point to approved records;
4. run any required exact-head verification after approval reconciliation;
5. remove only the target Goal's lock;
6. leave Goal state unchanged;
7. resume implementation from the current open Task.

If approval is withheld or work is interrupted, leave the lock in place.

## Completion criteria

The skill is complete only when:

- authorizing Decision is accepted;
- SPEC has a new explicit approved version;
- PLAN is reconciled to that SPEC;
- open Tasks are consistent with SPEC/PLAN;
- each new normative requirement has current Verification/Evaluation coverage;
- proof ownership matches the Task capable of producing the evidence;
- HANDOFF/current-state records are consistent;
- completed Task meanings and used IDs remain stable;
- required verification passes;
- requester approval of the complete packet is recorded;
- target Goal's `REPLAN_LOCK` is removed.

Until then, implementation for the locked Goal remains blocked.
