# D018 Durability/Hygiene Retrospective — 2026-09-03

## Purpose

Preserve the complete D018 episode as historical evidence after D019 superseded its sequencing and remediation approach. This is a dated recall record, not current planning authority. `DECISIONS.md`, `PROJECT_STATE.md`, and active Goal records remain canonical.

## Historical sequence

```text
T006 coherent baseline
        ↓
D018 cleanup-gate decision
        ↓
G003 Plan v2 + T007–T011 insertion
        ↓
D018 implementation + verification
        ↓
D019 supersession review
        ↓
ordinary audited revert
        ↓
original G003 T007–T012 resumes
```

## Decision and implementation evidence

- D018 decision commit: `e99c0f1`.
- Pre-T007 review commit: `dd69542`.
- Plan-v2/task/current-state commits: `c8974e4`, `22df2c3`, `7636ab2`, and `d2208af`.
- Implementation commit: `0c48b20bd80163c78d26ed2c425c3779420148df`.
- Implementation CI: GitHub Actions run `33739897518`, successful at 75/75 tests plus formatting, semantic boundaries, structural dependency checks, and locked clippy.
- Evidence commit: `9f4d75fa69d89d9c004a9038df61b2fa61709e1a`.
- Evidence-commit CI: GitHub Actions run `33740047976`, successful.
- Superseding decision: D019 at `5c0b432a2ac91020d321dd9250dd6ca85376abdb`.

The implementation and both successful CI runs remain permanently reachable in Git history even after their ordinary revert.

## Why D018 was adopted

The A003 review found real durability and repository-hygiene weaknesses immediately before tool/replay semantics would expand the durable Agent model. D018 interpreted those findings as a bounded prerequisite gate intended to prevent unstable persistence contracts, ambiguous retries, provider/durable ownership leakage, and bypassable stream finalization from becoming foundations for later work.

## Original nine findings

1. Agent entry, record, and configuration payloads lacked explicit evolution contracts; migration ownership was not component-versioned for the shared SQLite file.
2. Coordinated Agent creation committed before fallible response reads, leaving committed-but-unacknowledged success ambiguous and retries non-idempotent.
3. Provider and durable tool calls carried both raw and parsed JSON arguments, allowing divergent representations before validation, policy, hashing, and replay.
4. Durable Agent history serialized normalized provider-crate types rather than an explicitly Agent-owned disk vocabulary.
5. Provider stream consumers could bypass `ResponseAssembler::finish`, allowing interrupted EOF to escape the fail-closed path.
6. Store errors did not adequately distinguish invalid commands, conflicts, durable corruption, migration incompatibility, and retry-relevant backend failures.
7. Runtime store, Agent store, and coordinator duplicated physical SQLite connection/configuration mechanics.
8. Architecture CI relied mainly on textual grep rather than also checking the Cargo dependency graph.
9. RecordedProvider/process fixtures were exposed as ordinary package surfaces, while canonical current-state documents had drifted behind T006.

## What was technically useful

- The experiment produced executable proofs for v1 upgrade, future/gapped migration rejection, durable payload-kind/version validation, exact/conflicting create retry behavior, provider-to-durable conversion, interrupted/post-terminal stream rejection, state/event mismatch rejection, and shared cancellation registration.
- It demonstrated that common and Agent persistence can share physical SQLite mechanics without moving Agent DTOs into `ww-store`.
- It clarified which findings are intrinsic constraints of the already-planned Agent tasks and which are independent storage-evolution work.
- The 75-test green result showed that the implementation was technically coherent; D019 rejects its roadmap placement and scope, not the validity of every technique.

## What went wrong structurally

- No accepted G003 stop condition had actually fired. The findings were treated as automatic blockers rather than classified against existing acceptance criteria and stop conditions.
- Plan v2 reassigned already-used T007–T012 identifiers. Even though those Tasks were open, their identifiers already appeared in handoffs, plans, reviews, and dependencies and were therefore stable records.
- The cleanup gate mixed task-local constraints with independent infrastructure evolution. Canonical tool arguments, stream finalization, cancellation, and ambiguous restart behavior naturally belong in original T007, T008, T009, and T011.
- The implementation expanded beyond the nine approved findings into snapshot, lifecycle/event, and cancellation changes. Those were useful but confirmed that the supposedly bounded gate encouraged scope aggregation.
- The approach made all later G003 work depend on infrastructure hardening without evidence that G003 acceptance was impossible or unsafe without completing it first.

## D019 supersession rationale

D019 restores the bounded pre-D018 G003 structure, preserves all D018 evidence, forbids reassignment of used Task identifiers, and requires an ordinary revert rather than history rewriting. A review finding may interrupt an active Goal only when it violates accepted architecture, makes an existing acceptance criterion impossible, or triggers an explicit stop condition. Other valid findings must map naturally into an existing Task or move to a separate proposed Goal.

## Finding dispositions

| Finding | Disposition | Durable destination |
| --- | --- | --- |
| 1. Payload/schema migration evolution | defer to hardening Goal | proposed G010 |
| 2. Committed-but-unacknowledged create idempotency | defer to hardening Goal | proposed G010; original T011 may still test ambiguous restart behavior at the Agent level |
| 3. Raw/parsed tool-argument divergence | handle in existing G003 task | original T007 validation/hash/replay contract uses one canonical parsed value |
| 4. Durable format coupled to adjacent provider DTOs | defer to hardening Goal | proposed G010 |
| 5. Bypassable provider finalization | handle in existing G003 task | original T008 kernel owns mandatory fail-closed stream consumption |
| 6. Recovery-oriented storage error taxonomy | defer to hardening Goal | proposed G010 |
| 7. Repeated physical SQLite plumbing | defer to hardening Goal | proposed G010 |
| 8. Structural dependency checks | defer to hardening Goal | proposed G010 storage/conformance work may add the narrow structural guards it requires |
| 9. Test-surface/document hygiene | discard as a Goal-level blocker | maintain through ordinary fixture and bookkeeping discipline |

The experiment also addressed three adjacent findings. Snapshot-consistent persistence reads defer to proposed G010; lifecycle/event agreement and cancellation-token behavior are implementation constraints of original T009.

## Guardrail

> Review findings do not automatically become prerequisite Tasks.

Interrupt an active Goal only when a finding violates its accepted architecture, makes an existing acceptance criterion impossible, or triggers an explicit stop condition. Otherwise record and classify the finding, map it to an existing Task when naturally required, or defer it to a future Goal without stopping the active Goal.
