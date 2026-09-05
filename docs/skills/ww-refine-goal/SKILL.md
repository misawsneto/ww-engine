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
consistency reconciliation
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

### Identity and version reconciliation safeguard

Refinement must preserve the identity and lineage conventions already used by each record family.

**Explicit identifiers**

- If a record family already publishes explicit identifiers such as Goal, Task, Decision, Question, Verification-check, or similar labels, a published identifier is **consumed** and remains attached to the same semantic subject.
- Do not renumber, compact, backfill, or reuse a published identifier because a record was cancelled, resolved, superseded, deprecated, or moved.
- Tightening wording may retain an identifier only when the underlying proposition or semantic subject remains the same.
- If the proposition or semantic subject changes materially, allocate a new identifier according to that family's existing convention and record an explicit supersession/replacement mapping when continuity matters.
- Do not move an existing identifier to another Task merely because proof ownership moved; preserve the old identity and map it to the new proof/check.
- If a record family is title-addressed and does not use explicit identifiers, do **not** introduce an ID scheme solely for refinement bookkeeping.

**Version and state lineage**

- Follow the record family's existing version/state convention. Do not introduce numeric versions or generic revisions into a family that does not already use them.
- If the Goal packet already uses explicit SPEC/PLAN versions, refinement advances the version identity once and tracks lifecycle separately: for example, `Version: v3` with `State: draft` while pending approval, then the same `v3` becomes active/approved.
- Do not encode lifecycle into the version identity merely for convenience, such as creating a separate `v3-candidate` identity when state/approval already expresses candidacy.
- Approval of a candidate promotes the same candidate generation; it does not create another version solely because state changed.
- A candidate does not supersede the currently approved generation until requester approval is recorded.
- Existing evidence remains attached to the contract/version under which it was produced. Refinement must not retroactively make old evidence prove a changed proposition.

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

Identify the existing addressability convention for each record family before creating or changing IDs. Identify the existing version/state convention before changing SPEC/PLAN generation labels.

If refinement follows `ww-dryrun-review`, carry forward only findings classified **refine current Goal**. Do not promote implementation guidance or deferred findings into requirements.

Surface material unresolved choices rather than guessing.

### 2. Publish or restore the lock

Add or restore the Goal's `REPLAN_LOCK` before Goal-packet mutation.

Do not implement while the lock exists.

### 3. Refine SPEC

Create the next candidate SPEC generation **only when the existing Goal packet uses explicit SPEC versions**. Otherwise preserve the record family's existing state-based convention and rely on Git history rather than inventing a version number.

Keep candidate lifecycle/state separate from version identity.

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

Update PLAN to the candidate SPEC generation or state used by this Goal packet.

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
- existing published check identifiers retain their semantic proposition; changed propositions receive new identifiers or explicit supersession mappings;
- `EVALUATIONS.md` remains aligned with Goal-level behavior;
- HANDOFF reflects but does not strengthen the governed packet;
- questions resolved during refinement are reflected where they affect governed behavior;
- PROJECT_STATE/README/current guidance reflect locked/candidate status accurately;
- historical recall documents remain historical.

Keep Decision writing concise. Record detailed refinement history in review/verification evidence.

### 7. Consistency reconciliation before unlock

Before requesting approval, validate the packet as one coherent generation:

**Authority**

- SPEC → PLAN → TASKS → VERIFICATION/EVALUATIONS are mutually consistent;
- no architectural requirement exists only in a lower-authority record;
- no Task contradicts SPEC/PLAN flexibility;
- HANDOFF introduces no stronger rule than the governed packet.

**Identity**

- no published explicit identifier was renumbered, reused, compacted, or silently repurposed;
- materially changed propositions use a new identifier or explicit supersession/replacement mapping;
- no new ID scheme was invented for a title-addressed family.

**Version / state lineage**

- SPEC, PLAN, Tasks, Verification/Evaluations, HANDOFF, and current-state guidance reference the same candidate generation/state;
- version identity and lifecycle state are not conflated;
- the last approved generation remains the approved basis until the candidate is approved;
- evidence remains attributed to the generation/proposition it actually proved.

**Proof ownership and readiness**

- every Verification claim belongs to a Task capable of producing that evidence;
- current-state records do not claim implementation-ready/unlocked while the lock is active;
- no stale version/status/reference remains in current records.

If any item fails, refinement is not ready for approval or unlock.

### 8. Verify and request approval

Run the complete permanent repository gate required by current governance on the exact candidate head.

Present the requester with a compact review of:

- SPEC changes;
- PLAN changes;
- open Task changes;
- Verification/Evaluation changes;
- identifier or version-lineage reconciliations;
- proof ownership changes;
- assumptions/questions resolved or open;
- findings deliberately deferred;
- candidate commit and gate result.

The requester approves the **complete resulting packet**, not merely the Decision or an earlier partial recommendation.

Do not remove the lock until approval is explicit.

### 9. Unlock

After requester approval:

1. promote the candidate generation/state to approved/active without minting another version solely for the state transition;
2. record requester approval and evidence;
3. ensure current references point to the approved generation/state;
4. run any required exact-head verification after approval reconciliation;
5. remove only the target Goal's lock;
6. leave Goal state unchanged;
7. resume implementation from the current open Task.

If approval is withheld or work is interrupted, leave the lock in place.

## Completion criteria

The skill is complete only when:

- authorizing Decision is accepted;
- SPEC/PLAN generation or state follows the Goal packet's established convention;
- open Tasks are consistent with SPEC/PLAN;
- each new normative requirement has current Verification/Evaluation coverage;
- published identifiers retain semantic continuity or explicit supersession mappings;
- no artificial ID/version scheme was introduced where the record family does not use one;
- proof ownership matches the Task capable of producing the evidence;
- HANDOFF/current-state records reference one coherent approved basis;
- completed Task meanings and used IDs remain stable;
- required verification passes;
- requester approval of the complete packet is recorded;
- target Goal's `REPLAN_LOCK` is removed.

Until then, implementation for the locked Goal remains blocked.
