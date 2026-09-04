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
requester approves refined Goal packet
        ↓
record evidence
        ↓
remove REPLAN_LOCK
```

The lock is a flag, not a Goal state.

## Authorization

Do not run this skill without an **accepted Decision approved by the requester** that identifies the target Goal and the reason for refinement.

Keep the Decision short. The Decision authorizes the refinement; this skill defines the procedure.

Example:

```text
D021 | Refine G003 specification and downstream plan before T007 to reduce implementation ambiguity while preserving the accepted Goal boundary, completed Tasks, and used Task IDs. | accepted |
```

D020 introduces this skill. It is not a substitute for the per-run requester Decision.

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
- specification, planning, verification, review, decision/evidence, and current-state mutations directly required by the refinement are allowed;
- tests and verification commands may run;
- unrelated Goals are not blocked by this Goal-scoped lock.

If the same Goal is already locked by the same Decision, resume the refinement. If it is locked by a different Decision, stop and ask the requester. Preserve locks for other Goals.

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
implementation
```

Without a separate approved governance change, this skill must not:

- broaden or replace the Goal boundary;
- change accepted architecture;
- add new acceptance obligations to completed Tasks;
- reassign or renumber used Task IDs;
- turn non-blocking review debt into a new prerequisite or cleanup gate;
- introduce a prerequisite Goal merely because an improvement is desirable.

If refinement reveals that accepted architecture or the Goal boundary must change, keep the lock in place and stop. Obtain the required Decision/ADR change before continuing.

If a finding does not violate accepted architecture, make existing acceptance impossible, or trigger an existing Stop Condition, map it naturally to an existing open Task or defer it. Do not redesign the roadmap around it.

## Procedure

### 1. Establish the refinement basis

Read the current canonical integration head and the target Goal packet before editing:

- `AGENTS.md`;
- `DECISIONS.md` and the authorizing Decision;
- governing ADRs;
- `GOAL.md`;
- `SPEC.md`;
- `PLAN.md`;
- `TASKS.md`;
- `VERIFICATION.md`;
- `EVALUATIONS.md`;
- `REVIEWS.md`;
- `PROJECT_STATE.md` and other current-state guidance when relevant.

Identify which Tasks are complete and which remain open. Completed Task acceptance and evidence are historical facts and remain unchanged by ordinary refinement.

Surface material assumptions or unresolved choices instead of silently guessing. If the requester must decide something outside the authorizing Decision, keep the lock and ask.

### 2. Publish the lock

Add the `REPLAN_LOCK` block before any other repository mutation for the refinement and publish it on the canonical engineering line.

Do not begin implementation while the lock exists.

### 3. Refine the specification

Create a new explicit SPEC version.

If the existing SPEC has no version, treat its accepted contents as `v1` and make the refined document `v2`.

The refined SPEC should make the remaining open contract materially more executable by clarifying, where relevant:

- objective and success conditions already inside the Goal boundary;
- interfaces and ownership;
- invariants and ordering requirements;
- allowed and forbidden behavior;
- important implementation constraints already implied or selected by governing records;
- failure behavior;
- testable success criteria;
- open questions that must not be guessed.

Do not duplicate repository-wide commands, style, structure, or policy unless the Goal needs a specific addition or override. Reference existing repository authority where possible.

Every new normative requirement must have a defensible basis: a governing ADR/Decision/Goal requirement, or a clarification necessary to make an existing **open** Task acceptance unambiguous.

### 4. Refine the plan

Update PLAN so it is explicitly based on the refined SPEC version.

Make implementation order, dependencies, integration points, risks, and verification checkpoints clear enough that the implementing agent is not deciding architecture or sequencing implicitly.

Preserve the existing Task topology and used IDs by default. Do not introduce new prerequisite work unless an existing Stop Condition requires it and the requester has approved the governed change.

### 5. Reconcile open Tasks

Refine only open Tasks as needed so each has:

- a focused deliverable;
- explicit acceptance criteria;
- a concrete verification method;
- clear dependencies;
- implementation-file expectations when useful.

Do not change the meaning of completed Tasks or their evidence.

**Task sizing:** strongly prefer Tasks that touch roughly five or fewer implementation files. Exceeding this is a decomposition signal, not a prohibition. Keep a larger Task intact when splitting it would weaken one coherent acceptance boundary.

### 6. Reconcile verification and current records

Make the refined packet internally consistent:

- each new normative requirement has observable verification;
- `VERIFICATION.md` proves Task/Goal requirements rather than incidental implementation shape;
- `EVALUATIONS.md` remains aligned with Goal-level behavior;
- `PROJECT_STATE.md`, README/current guidance, reviews, and cross-references are updated only when the refinement makes them stale;
- no historical recall document is rewritten merely to make current records look cleaner.

Keep Decision writing concise. Record the detailed refinement trail in the Goal's review/verification evidence rather than expanding the Decision into a process log.

At minimum record:

- authorizing Decision;
- prior and refined SPEC/PLAN versions;
- open Tasks materially refined;
- verification/evaluation changes;
- requester approval of the resulting packet;
- relevant commit and verification evidence.

### 7. Verify and request approval

Run the repository's complete permanent verification gate required by current governance.

Present the requester with a compact review of:

- SPEC changes;
- PLAN changes;
- open Task acceptance changes;
- verification/evaluation changes;
- assumptions resolved or still open;
- findings deliberately deferred rather than added to active scope.

Do not remove the lock until the requester explicitly approves the refined Goal packet.

### 8. Unlock

After requester approval:

1. record the approval/evidence in the Goal's canonical review/verification trail;
2. ensure current planning references point to the refined versions;
3. remove only the target Goal's `REPLAN_LOCK` block from `AGENTS.md`;
4. leave the Goal state unchanged;
5. implementation may resume from the current open Task.

If approval is withheld or the run is interrupted, leave the lock in place. A later run resumes by reading the Decision, current Git state, and partially refined packet.

## Completion criteria

The skill is complete only when:

- the authorizing Decision is accepted;
- the Goal's SPEC has a new explicit approved version;
- PLAN is reconciled to that SPEC version;
- open Tasks and verification/evaluations are consistent with the refined contract;
- completed Task meanings and used Task IDs remain stable;
- required repository verification passes;
- requester approval is recorded;
- the target Goal's `REPLAN_LOCK` has been removed.

Until all of these are true, implementation for the locked Goal remains blocked.
