---
name: ww-dryrun-review
description: Review a WorkWeave implementation dry run against the governed Goal packet, current code, and pinned reference architecture before implementation relies on it.
---

# ww-dryrun-review

**Status:** approved

## Purpose

Review a dry run as an **implementation hypothesis** before code relies on it.

A dry run is not specification authority. It may expose:

- hidden implementation choices;
- seam or ownership mistakes;
- reference-architecture drift;
- false-green tests;
- proof assigned to the wrong Task;
- unresolved failure, cancellation, or recovery behavior;
- questions the implementing agent should not guess.

This skill reviews and classifies findings. It does **not** perform replanning.

```text
governed Goal packet
        +
current code
        +
pinned reference evidence
        ↓
dry run
        ↓
review
        ↓
implement | implementation guidance | ww-refine-goal | governance change | defer
```

## Authority

Review against the current repository authority:

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
```

The dry run may challenge a governed record. It may not override one.

When a dry-run recommendation conflicts with higher authority, report the conflict. Do not leave the implementing agent to choose which instruction wins.

## Review basis

Read only the material needed to judge the target Task:

- current canonical integration head;
- `AGENTS.md` and any active `REPLAN_LOCK`;
- dry-run artifact;
- governing Decision/ADR;
- Goal, SPEC, PLAN, target Task;
- matching Verification/Evaluations;
- current handoff when present;
- actual code and dependency seams implicated by the dry run;
- pinned reference source for material reference claims.

Prefer current governed records over historical handoffs or recall material.

## Procedure

### 1. Establish exact state

Confirm:

- exact current head;
- target Goal and Task;
- implementation status;
- active planning lock, if any;
- whether the dry run is prose/pseudocode, compiled code, or tested code;
- permanent verification gate that applies.

A green repository does not prove dry-run pseudocode compiles or that an unimplemented design satisfies acceptance.

### 2. Extract material dry-run claims

Ignore cosmetic code shape unless it implies a contract decision.

Extract claims about:

- seams and ownership;
- public/internal API shape;
- dependency direction;
- ordering and lifecycle;
- persistence/effect boundaries;
- failure and cancellation behavior;
- replay/recovery behavior;
- tests and evidence;
- file/crate changes;
- reference-project behavior;
- open questions and assumptions.

Treat illustrative code as a proposed shape, not source authority.

### 3. Check domain-model impact first

Ask:

> Does this finding require a new domain entity, identity, relationship, state, lifecycle, authority, durable semantic record, or changed meaning of an existing one?

Classify:

- **no domain-model change** — implementation or specification architecture may still need refinement;
- **domain-model change required** — require deeper validation/governance before implementation relies on it.

Do not infer that "no domain-model change" means a Task-only edit is sufficient.

### 4. Check repository realism

Compare the dry run with actual code.

Look for:

- existing ports that already satisfy the need;
- invented managers, services, state machines, crates, or transaction seams;
- dependency-direction violations;
- duplicate logic across layers;
- incorrect assumptions about current APIs;
- unnecessary file/module prescription;
- pseudocode presented as if compile-proven.

Prefer reuse and deletion over added machinery.

### 5. Check reference alignment

Verify material claims against the pinned reference revision.

Distinguish:

- observed production behavior;
- future/experimental reference scaffolding;
- WorkWeave adaptation;
- deliberate WorkWeave deviation.

For Pi-oriented Agent work, pay particular attention to:

- small functional loop versus product/session façade;
- provider-neutral streaming;
- preparation versus execution;
- source-ordered tool results;
- cancellation propagation;
- entry/record separation and pure reduction;
- replay ambiguity;
- avoiding queues, hooks, parallelism, or session machinery when the Goal excludes them.

Reference parity is behavioral, not package or file-layout parity.

### 6. Check seam and ownership quality

For each proposed seam, answer:

- who owns the decision?
- who owns durable identity?
- who owns execution?
- what dependency direction is legal?
- is the same logic duplicated elsewhere?
- does the seam carry identities it does not need?
- is a manager/service object being introduced where a function, trait, or existing port is enough?

A strong dry run leaves one obvious authority for each responsibility.

### 7. Check proof ownership

A Task can prove only a boundary it actually exercises.

Examples:

- preparation can prove classification ordering;
- the real kernel proves commit-before-effect execution ordering;
- lifecycle integration proves durable cancellation reaches live work;
- process-restart tests prove crash recovery.

Flag any acceptance or Verification check assigned to a Task that cannot produce the evidence.

A handcrafted history proves reducer behavior. It does not prove the production driver persisted that history correctly.

### 8. Check failure and recovery algebra

Ensure required outcomes remain distinguishable:

- ordinary operational error;
- cooperative cancellation;
- timeout/budget termination;
- panic/invariant violation;
- retryable interruption;
- unsafe ambiguity requiring intervention;
- corruption.

If the API cannot represent a distinction required by governed behavior, classify it as a specification/architecture gap rather than leaving the builder to improvise.

### 9. Check tests for false confidence

Look for:

- equality tests that do not observe the actual canonical bytes/order;
- mocks that bypass the production seam;
- handcrafted history used as production-order proof;
- registration order accidentally matching configured order;
- disabled library features mistaken for a stable WorkWeave error contract;
- metadata present in types but never behaviorally consumed;
- helper tests where acceptance requires the production boundary.

Prefer the smallest test that observes the real invariant.

### 10. Classify every material finding

Use exactly one disposition:

- **implementation guidance** — already inside the governed contract; no planning mutation required;
- **refine current Goal** — normative SPEC/PLAN/Task/Verification/Evaluation clarification is required; hand off to `ww-refine-goal`;
- **governance change required** — Goal/domain model/accepted architecture must change; Decision/ADR first;
- **defer** — useful but not required for current acceptance;
- **reject dry-run claim** — unsupported, incorrect, or contrary to governing authority.

Do not turn an implementation preference or deferred improvement into a new requirement.

### 11. Answer dry-run questions

Resolve questions when existing authority/evidence determines the answer.

If the question requires requester choice, say so explicitly.

Do not mutate `QUESTIONS.md` unless the requester asks to apply the review findings.

## Output

Return:

1. **Verdict** — implementation-ready, ready with guidance, needs refinement, or blocked by governance;
2. **What the dry run got right**;
3. **Findings** — severity, issue, evidence/rationale, disposition;
4. **Reference-alignment assessment**;
5. **Domain-model impact**;
6. **Open questions resolved or still requiring requester choice**;
7. **Recommended next action**.

Keep implementation guidance separate from governed changes.

## Applying findings

This skill does not mutate the Goal packet by itself.

If the requester asks to apply findings:

- implementation guidance may be handed directly to the builder;
- `refine current Goal` findings use `ww-refine-goal`;
- governance findings require the appropriate Decision/ADR first;
- deferred findings remain deferred.

Do not reproduce the `ww-refine-goal` procedure inside this skill.

## Completion criteria

The review is complete when:

- the dry run was checked against current authority, code, and material reference evidence;
- domain-model impact is explicit;
- seam/ownership and proof ownership were assessed;
- false-confidence tests and unresolved failure/recovery semantics were considered;
- every material finding has one disposition;
- implementation readiness and next action are explicit.
