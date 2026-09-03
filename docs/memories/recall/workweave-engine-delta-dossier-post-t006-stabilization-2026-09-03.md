# WorkWeave Engine — Delta Dossier: Post-T006 Stabilization

**Status date:** 2026-09-03  
**Repository:** `misawsneto/ww-engine`  
**Baseline dossier:** `WorkWeave Engine — Chronological Project Dossier Through G003 T006`  
**Baseline commit:** `15251675d0af29fb883f6c098cd9874da19e6a94`  
**Current canonical head:** `b70b53832e1dff627f31408762405074d4f472d8`  
**Active Goal:** `G003 — Durable Agent Kernel`  
**Current next Task:** `G003/T007 — Implement tool contract, schema validation, policy, and replay fixtures`  
**Purpose:** record only the material changes since the T006 chronological dossier, including the D018 hardening experiment, its supersession by D019, the audited rollback, preservation of the technical findings, extraction of proposed G010, restoration of stable Goal numbering, and the final stabilization checkpoint before T007.

---

## 1. Delta executive summary

The previous dossier ended at a coherent T006 checkpoint:

```text
G003
T001 activation/bookkeeping                  complete
T002 provider protocol/assembler              complete
T003 Agent history + recovery reducer         complete
T004 Agent SQLite persistence                 complete
T005 common/Agent transaction seam            complete
T006 RecordedProvider                         complete
T007 tool/schema/policy/replay                 next
T008 functional Agent kernel                  open
T009 lifecycle + cancellation                 open
T010 durable limits                           open
T011 crash/restart matrix                     open
T012 evaluations + terminal review            open
```

At that point the implementation architecture remained aligned with the original Pi-derived Agent design: a provider-neutral stream seam, immutable context entries plus operational records, a deterministic recovery reducer, Agent-owned persistence, and a future small functional model→tool→model loop. The prior dossier also warned that the project should resist cleanup-driven Goal expansion and should keep Goal and Task identifiers stable.

Immediately after T006, an engineering review identified several legitimate durability weaknesses. Decision **D018** treated those findings as a blocking prerequisite and inserted five new cleanup Tasks before the existing tool/kernel work. The cleanup implementation itself was technically coherent and passed 75 tests, but the remediation changed the active Goal structure, reassigned already-used Task IDs, expanded G003 substantially, and modified shared-runtime behavior without an actual G003 Stop Condition requiring the interruption.

That approach was subsequently rejected.

Decision **D019** superseded D018 and restored the core rule:

> Review findings do not automatically become prerequisite Tasks. Interrupt an active Goal only when a finding violates accepted architecture, makes an existing acceptance criterion impossible, or triggers an existing Stop Condition. Otherwise map the finding to an existing Task or defer it to a future Goal.

The D018 code and Plan-v2 changes were then **ordinarily reverted**, without rewriting history. The complete D018 findings, implementation evidence, successful CI results, and finding-by-finding dispositions were preserved in a dated retrospective. The original G003 T001–T012 structure was restored exactly.

The storage-evolution findings that do not block G003 were extracted into **proposed G010 — Durable Storage Evolution and Recovery Hardening**. It has no accepted ADR, no implementation, and no automatic dependency relationship with G004.

A later bookkeeping review found that **G009** was already the post-D015 position for **Coordinated Deployment**. The proposed hardening Goal was therefore renamed from G009 to G010 before activation, preserving the existing roadmap. `CLAUDE.md` was also published as a real Git symlink to `AGENTS.md`.

Current `main` is now `b70b53832e1dff627f31408762405074d4f472d8`. Hosted CI run `33783458388` passed on that exact head. The repository is back on the **58/58-test post-revert T006 baseline**, and no D018 implementation code, lockfile changes, Plan-v2 task structure, or extra CI machinery remains.

The active engineering path is therefore again simple:

```text
T007 tool contract/schema/policy/replay
        ↓
T008 small functional Agent kernel
        ↓
T009 lifecycle/cancellation
        ↓
T010 durable limits
        ↓
T011 crash/restart matrix
        ↓
T012 evaluations + terminal review
```

---

## 2. Delta evidence ledger

| Ref | Evidence |
| --- | --- |
| `[BASE-T006]` | `15251675d0af29fb883f6c098cd9874da19e6a94` — T006 RecordedProvider implementation and conformance fixtures |
| `[CI-T006]` | GitHub Actions run `33733236295` — T006 full `main` gate, 58/58 tests |
| `[D018-DEC]` | `e99c0f11cdf8825cd9ae5c3514e1df93137fe9be` — records D018 cleanup gate |
| `[D018-REVIEW]` | `dd695422ef49f3f821290e44667bf591ef5cd96f` — pre-tool durability/hygiene review |
| `[D018-PLAN]` | `c8974e446bfff88c9f73f5f11227f1d30134b091` — G003 Plan v2 cleanup-first sequence |
| `[D018-TASKS]` | `22df2c364c7c92ac4755f14c24c9c44a20157c2c` — inserted/renumbered cleanup Tasks |
| `[D018-STATE]` | `7636ab2cdf67533388695adfafd8c065be4db99e` — Plan-v2 project-state synchronization |
| `[D018-README]` | `d2208af89c03d05e59a44b84bc6a158fdb22b65c` — README Plan-v2 alignment |
| `[D018-IMPL]` | `0c48b20bd80163c78d26ed2c425c3779420148df` — D018 implementation, T007–T011 under Plan v2 |
| `[CI-D018]` | GitHub Actions run `33739897518` — D018 implementation gate, 75/75 tests |
| `[D018-EVID]` | `9f4d75fa69d89d9c004a9038df61b2fa61709e1a` — remote verification evidence |
| `[CI-D018-EVID]` | GitHub Actions run `33740047976` — evidence-head CI success |
| `[D019]` | `5c0b432a2ac91020d321dd9250dd6ca85376abdb` — supersedes D018 and restores bounded G003 structure |
| `[D018-RETRO]` | `723dd323e61101bac35bba06dba0d503e7738ad4` — preserves D018 retrospective and finding dispositions |
| `[D018-REVERT]` | `cc5dcbe96217969ae46cf4f3e795ae27862c5d9a` — ordinary audited revert of all eight D018-era commits |
| `[RECONCILE]` | `ae967bfb7d88623ab9d8075e97bcc493fea3619f` — restores T006/T007 bookkeeping, guardrails, and proposed hardening Goal |
| `[STABILIZE]` | `b70b53832e1dff627f31408762405074d4f472d8` — reserves G009, renames hardening Goal to G010, publishes `CLAUDE.md` symlink |
| `[CI-STABILIZE]` | GitHub Actions run `33783458388` — full hosted gate on `b70b538`, successful |

---

# Part I — The D018 experiment

## 3. Why the hardening review occurred

After T006, the Agent architecture was coherent enough to begin tool semantics. A reviewer deliberately inspected the durability substrate before T007 because the next slices would begin relying on persistent replay, policy, and effect boundaries.

The review found nine substantive weaknesses:

1. Agent configuration, entry, and record payloads lacked explicit schema/payload evolution contracts.
2. Coordinated Agent creation could commit successfully and then fail before acknowledgement, leaving retry semantics ambiguous.
3. Tool calls carried both raw JSON text and parsed JSON values, creating two possible authorities for validation, hashing, policy, and replay.
4. Durable Agent history directly serialized normalized provider-crate types instead of making the Agent durability boundary structurally explicit.
5. Provider consumers could bypass `ResponseAssembler::finish`, so interrupted EOF did not have one mandatory production interpretation.
6. Store errors did not distinguish enough recovery-relevant categories.
7. Runtime, Agent, and coordinator persistence repeated physical SQLite connection/configuration mechanics.
8. Architecture CI relied mainly on textual checks instead of also checking Cargo dependency structure.
9. Test fixtures and current-state documentation had accumulated ordinary hygiene debt.

These were technically real findings. The eventual mistake was **not discovering them**. The mistake was concluding that all of them had to become a blocking pre-T007 program of work.

---

## 4. D018 changed the remediation structure

D018 inserted a new five-Task durability/hygiene gate between T006 and the original T007. The former T007–T012 were renumbered T012–T017.

Conceptually, Plan v2 became:

```text
T006 RecordedProvider
        ↓
T007 schema/migration evolution
T008 idempotent creation + typed storage errors
T009 durable ownership + canonical arguments
T010 stream finalization
T011 backend/repository hygiene
        ↓
T012 tool policy/replay
T013 functional kernel
T014 lifecycle/cancellation
T015 limits
T016 crash/restart
T017 terminal review
```

The implementation then added:

- component-scoped migration tracking;
- payload/configuration versions;
- idempotent coordinated create retry;
- richer store error classes;
- provider→durable conversion types;
- one parsed argument representation;
- a centralized provider stream finalizer;
- snapshot-consistent storage reads;
- shared cancellation-token registration behavior;
- common lifecycle state/event coherence validation;
- physical SQLite helper reuse;
- Cargo dependency-graph checks;
- feature-gated test surfaces.

The code passed the full strengthened gate and 75 tests. This demonstrated that the implementation was internally coherent. It did **not** prove that its placement inside G003 was correct.

---

## 5. Why D018 was rejected despite green code

The strategic review separated **technical validity** from **roadmap validity**.

### 5.1 No G003 Stop Condition had fired

The original G003 Plan already contained explicit Stop Conditions for conditions such as Agent types leaking into shared storage, unsafe replay ambiguity, provider/tool finalization safety, or a Task becoming unmanageably large.

The review findings did not establish that G003 could not safely continue under the existing T007–T012 acceptance boundaries. D018 therefore treated engineering debt as an automatic blocker without satisfying the project's own interruption rule.

### 5.2 Stable Task identities were reassigned

Although the original T007–T012 Tasks were still open, their IDs had already appeared in plans, handoffs, dependencies, reviews, and project discussions. Reassigning those IDs made references such as `G003/T009` plan-version-dependent.

D019 consequently established a stronger record rule:

> Once a Task identifier has been used as part of an active Goal record, do not reassign it. New work receives a new identity rather than changing the meaning of an existing one.

### 5.3 The cleanup gate mixed two different categories

Some D018 findings were actually constraints of existing planned Tasks:

- canonical arguments naturally belong in T007;
- mandatory provider finalization naturally belongs in T008;
- cancellation/lifecycle agreement belongs in T009;
- Agent-level ambiguity recovery belongs in T011.

Other findings were broader storage-evolution concerns that could be proved independently later.

Combining both categories into one prerequisite gate created unnecessary coupling.

### 5.4 The gate expanded beyond its initial findings

The D018 implementation also changed snapshot semantics, common lifecycle/event validation, and cancellation-token registration behavior. These were defensible improvements, but their appearance confirmed the structural problem: a generic "hardening" phase encourages adjacent infrastructure changes to aggregate before productively bounded work resumes.

### 5.5 G003's original size discipline was weakened

ADR-0003 deliberately split concrete provider/filesystem/SDK/CLI work into G004 so G003 remained a focused durable-kernel proof. D018 effectively re-expanded G003 through infrastructure work, even though it did not add product breadth.

The correct conclusion was therefore:

> Preserve the findings and evidence; reject the inserted cleanup program.

---

# Part II — D019 and the audited rollback

## 6. D019 superseded the remediation, not the learning

D019 records four important principles:

1. D018 is superseded, not deleted.
2. G003 returns to its original bounded structure after T006.
3. The D018 implementation is reverted through normal Git history, not reset/force-pushed away.
4. The findings survive as review evidence, Task-local constraints, or a separate proposed Goal.

This creates the desired historical model:

```text
Decision made
    ↓
Implementation attempted
    ↓
Evidence collected
    ↓
Remediation judged structurally wrong
    ↓
Decision superseded
    ↓
Implementation reverted
    ↓
Learning preserved
```

The project therefore retains both the failed approach and the reason it was rejected.

---

## 7. The retrospective freezes the D018 episode

Before rollback, `[D018-RETRO]` created:

`docs/memories/recall/D018-DURABILITY-HYGIENE-RETROSPECTIVE-2026-09-03.md`

The retrospective explicitly identifies itself as historical recall rather than current planning authority. It records:

- the complete D018 chronology;
- all nine findings;
- the implementation commits;
- successful CI evidence;
- what the implementation proved technically;
- what failed structurally;
- D019's supersession rationale;
- a finding-by-finding durable destination.

This prevents the ordinary revert from erasing the intellectual work.

---

## 8. The ordinary revert restored the exact implementation baseline

`[D018-REVERT]` reverted the eight D018-era commits in normal Git history.

The result restored the pre-D018 code state rather than selectively retaining infrastructure changes whose authority would have become ambiguous.

After reconciliation, a comparison against `[BASE-T006]` showed no differences in:

- Rust code;
- `Cargo.lock`;
- `.github/workflows/ci.yml`;
- G003 `PLAN.md`;
- G003 `TASKS.md`.

Only intended governance/history material remained different.

This is a key stabilization property: **the current implementation is not a hybrid of the T006 architecture and selected D018 mechanics.**

---

## 9. G003's original structure is again canonical

The restored canonical Tasks are:

```text
T001 Accept G002 review and activate G003                  complete
T002 provider-neutral protocol and stream assembler        complete
T003 Agent entries, operational records, recovery reducer  complete
T004 Agent SQLite persistence and reconstruction           complete
T005 common/Agent SQLite transaction coordination          complete
T006 RecordedProvider and conformance fixtures             complete
T007 tool contract/schema/policy/replay fixtures            NEXT
T008 functional recorded-provider model→tool→model kernel  open
T009 G002 lifecycle and durable cancellation               open
T010 durable deadlines and execution budgets               open
T011 crash/restart and ambiguous-effect recovery matrix    open
T012 EvaluationRuns and terminal architecture review       open
```

No Plan-v2 numbering remains current.

The D018 retrospective is the only place where the temporary numbering remains important, and it labels that numbering as historical.

---

# Part III — Learning disposition and G010

## 10. Findings mapped back into existing G003 Tasks

The following D018 findings remain relevant to current G003 acceptance, but they do **not** justify new Tasks.

### T007 — canonical tool arguments and real JSON Schema validation

T007 must establish one authoritative executable JSON value before validation, effect derivation, policy, hashing, or replay semantics rely on arguments.

The validation contract remains the original ADR-0003 contract:

```text
complete JSON arguments
        ↓
JSON Schema validation
        ↓
effect descriptor / replay classification
        ↓
policy decision
        ↓
durable attempt boundary
        ↓
execution or denial
```

The architecture dossier already selected the `jsonschema` crate for dynamic validation at provider/tool boundaries. Therefore T007 should use the real validator rather than a hand-written subset. Keep the library internal to `ww-agent-tools` and disable external file/HTTP resolution features for G003.

### T008 — mandatory fail-closed provider finalization

The kernel must not consume a provider stream and infer success from EOF. T008 owns the actual production consumption path and must guarantee that every provider attempt reaches one typed terminal/interrupted interpretation through the assembler/finalization contract.

This is an implementation requirement of the existing kernel Task, not a separate persistence project.

### T009 — lifecycle and cancellation correctness

The D018 experiment's cancellation observations remain useful, but the existing T009 already owns durable cancellation propagation and common/Agent terminal repair.

Any required cancellation-token mechanics should be implemented only to satisfy T009's acceptance contract.

### T011 — Agent-level ambiguity recovery

T011 already owns crash/restart proof. It should test any ambiguous creation/model/tool/settlement windows that are required to prove G003's recovery thesis. Broader generic storage API evolution is not required merely because a reusable solution may eventually be desirable.

---

## 11. Independent storage evolution moved to proposed G010

The non-blocking storage findings now have a dedicated proposed home:

**G010 — Durable Storage Evolution and Recovery Hardening**

G010 is intentionally outside the active sequence and currently has:

- state `proposed`;
- draft Plan/Spec/Evaluations;
- no governing ADR yet;
- no implementation;
- no automatic dependency relationship with G004.

Its intended problem boundary is:

```text
IN
  durable schema/payload evolution
  known-old compatibility fixtures
  future-version fail-closed behavior
  idempotent create/ensure after acknowledgement loss
  recovery-oriented store error classification
  reusable physical SQLite mechanics
  cross-adapter storage conformance

OUT
  Agent tool semantics
  Agent provider protocol
  Agent functional loop
  Flow/OWS semantics
  SDK/CLI/TUI
  filesystem/network capability
  WorkWeave Orchestration semantics
```

This preserves the learning without allowing it to block current G003 by default.

Activation must occur only after a concrete scheduling decision and a Goal-specific ADR.

---

# Part IV — Goal numbering and canonical roadmap

## 12. G009 reservation was restored

The prior stabilization pass temporarily used G009 for the hardening Goal. A subsequent review recognized that the original architecture sequence already contained:

```text
G007 local product experience
G008 coordinated deployment
```

D015 inserted `G004 — Agent Provider and Surface`, shifting later Goals by one in the current sequence. Therefore the post-D015 position for Coordinated Deployment is G009.

The current sequence is now recorded as:

```text
G002 Shared Runtime                          achieved
  ↓
G003 Durable Agent Kernel                    active
  ↓
G004 Agent Provider and Surface              proposed
  ↓
G005 Deterministic OWS Flow Kernel           future
  ↓
G006 Flow → Agent integration                future
  ↓
G007 Full frozen OWS profile                 future
  ↓
G008 Local product experience                future
  ↓
G009 Coordinated Deployment                  reserved
```

G010 sits outside that sequence as proposed non-blocking hardening.

The G010 packet was renamed while still proposed, before ADR acceptance or implementation, so no active Goal identity was disturbed.

---

## 13. Architecture dossier numbering remains a baseline, not current Goal bookkeeping

The primary Engine architecture dossier remains the architecture authority for topology, ownership, runtime behavior, and implementation direction. Its original Goal-number headings reflect the 2026-09-01 baseline before D015.

Those headings were deliberately not retroactively rewritten during the G009/G010 correction.

The appropriate interpretation is:

- preserve the original dossier text as the architecture baseline;
- treat later accepted Decisions as superseding its Goal-number mapping where applicable;
- use `DECISIONS.md` and `PROJECT_STATE.md` for current Goal numbering.

A small non-destructive annotation in the dossier's implementation-sequence section would improve reader ergonomics, but it is not a blocker for T007 and should not become another cleanup task.

---

# Part V — Repository operating environment

## 14. `CLAUDE.md → AGENTS.md` is now durable repository state

`[STABILIZE]` publishes `CLAUDE.md` as an actual Git symlink:

```text
mode 120000
CLAUDE.md → AGENTS.md
```

It is not a duplicated copy of instructions.

This ensures Claude-based engineering sessions resolve the same repository operating rules as other agents without creating a second policy surface.

---

## 15. Current verification gate

D017 remains governing:

> Any task-specific or temporary verification path must execute the complete merge-target CI gate. It may add checks but may not omit target checks.

The current permanent `main` gate is again the post-revert T006 gate:

```text
cargo fmt --all -- --check
five architecture boundary checks from ci.yml
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

No D018 `check-architecture.sh` structural checker remains in current code or CI. No orphaned `.github/scripts/` directory remains.

`[CI-STABILIZE]` ran on the exact current head `b70b538...` and succeeded. The test count is again **58/58**, matching the coherent post-T006 implementation baseline.

---

# Part VI — Current anti-drift rules

## 16. The review-to-work rule

The most important new guardrail is now durable in project warnings/learnings:

```text
review finding
    ↓
Does it violate accepted architecture?
OR make an existing acceptance criterion impossible?
OR trigger an explicit Stop Condition?

    ├─ yes → stop/escalate/change governed plan
    └─ no
         ↓
     Does it naturally belong in an existing Task?
         ├─ yes → handle inside that Task
         └─ no  → record/defer to future Goal
```

A green implementation of a review finding is not evidence that it belongs before the current Goal.

---

## 17. Stable record identity rule

Active Goal Task IDs are now treated as stable records.

Do not reuse or reassign an existing Task ID merely because the Task is still open.

If genuinely new prerequisite work is required and a Stop Condition justifies interrupting the Goal, use a new stable identity rather than changing the meaning of references that already exist in handoffs, reviews, decisions, or evidence.

---

## 18. No second cleanup gate in G003

The D018 episode establishes a strong practical constraint:

> G003 should not receive another generic hardening or hygiene phase.

T007–T012 should execute the original proof. If implementation discovers non-blocking debt, record it. If a concrete issue makes the Task's accepted behavior impossible or unsafe, use the existing Stop Conditions and architecture escalation process.

This is essential to avoid repeatedly postponing the actual Agent kernel proof.

---

# Part VII — Current engineering handoff

## 19. T007 is ready

Current next work is the original:

**G003/T007 — Implement tool contract, schema validation, policy, and replay fixtures.**

Required concepts remain:

```text
ToolId / ToolVersion
ToolSpec
logical ToolCall identity
ToolAttempt identity
EffectDescriptor
ReplayPolicy::Safe | Never
PolicyDecision::Allow | Deny
ToolResult / normalized error
```

Required fixtures:

- `test.echo` — deterministic, replay-safe, no external effect;
- `test.unsafe_once` — synthetic ReplayNever ambiguity fixture.

Required order:

```text
finalized tool call
    ↓
resolve tool
    ↓
use canonical parsed JSON arguments
    ↓
real JSON Schema validation
    ↓
effect/replay classification
    ↓
policy decision
    ↓
persist attempt boundary
    ↓
execute or deny
    ↓
persist one logical result
```

Critical invariant:

> One logical tool call may have multiple attempts but at most one committed model-visible result.

A policy denial performs zero effect and produces one durable model-visible error result.

A non-replayable ambiguous started effect is never silently replayed.

T007 must remain deterministic and must not add filesystem, process, network, OpenAI, MCP, plugin, Flow, OWS, SDK/CLI, or Orchestration scope.

---

## 20. JSON Schema decision

The architecture choice should be treated as settled rather than reopened during implementation.

ADR-0003 normatively requires JSON Schema validation before policy or execution. The primary Engine dossier selects the `jsonschema` crate for dynamic schema validation at provider/tool boundaries.

T007 should therefore:

- use the real `jsonschema` implementation rather than a hand-written structural subset;
- keep third-party validator types behind WorkWeave-owned tool contracts;
- disable default external resolution features so G003 does not acquire HTTP/file schema capability;
- keep schemas self-contained/offline;
- reject malformed tool schemas as tool-definition errors;
- validate invocation arguments before effect derivation/policy/execution;
- avoid adding `schemars` unless a concrete T007 acceptance need requires schema derivation.

This is implementation of the accepted contract, not a new architecture decision.

---

## 21. T008 remains gated on T007

The functional kernel should not begin merely because provider work is complete.

Current dependency shape:

```text
T002 provider protocol
       │
       ▼
T006 RecordedProvider ─────┐
                           │
T003 recovery model        │
T004 persistence           ├──→ T008 functional Agent kernel
T005 transaction seam      │
                           │
T007 tools/policy/replay ──┘
```

T008 should remain a small Pi-style functional driver, not a broad `AgentSession` object.

---

# Part VIII — Strategic items unchanged from the previous dossier

## 22. G004-vs-Flow sequencing remains a future decision

Nothing in the D018/D019 episode resolves the previously identified roadmap question.

The original architecture's strategic sequence favored:

```text
shared substrate
→ thin Agent
→ Flow
→ immediate Flow→Agent composition
→ breadth later
```

Current D015 bookkeeping instead places:

```text
G003 Durable Agent
→ G004 concrete Agent provider/fs.read/SDK/CLI
→ G005 Flow
→ G006 Flow→Agent
```

This remains deliberate but strategically reviewable.

Do **not** reopen it during T007. Revisit it before G004 activation or during G003 terminal planning, when the durable Agent proof is complete and the trade-off can be assessed from actual evidence.

---

# Part IX — Final stabilized state

## 23. Canonical state at `b70b538`

```text
Repository
  main = b70b53832e1dff627f31408762405074d4f472d8
  hosted CI = 33783458388 success
  tests = 58/58
  CLAUDE.md -> AGENTS.md (Git mode 120000)

Architecture
  Orchestration remains above Engine
  Agent and Flow remain sibling kernels
  Agent remains provider-neutral
  no Flow/OWS leakage into Agent
  no concrete network/filesystem product surface in G003

G003
  original T001–T012 structure restored
  T001–T006 complete
  T007 next
  D018 implementation absent from current code
  D018 historical evidence preserved
  D019 governs remediation discipline

Future Goals
  G004 Agent Provider and Surface       proposed
  G005 Deterministic OWS Flow Kernel    future
  G006 Flow → Agent integration         future
  G007 Full frozen OWS profile          future
  G008 Local product experience         future
  G009 Coordinated Deployment           reserved
  G010 Storage Evolution/Hardening      proposed, non-blocking
```

---

## 24. Delta conclusion

The post-T006 period was not wasted work. It served as a live test of the project's governance model.

The technical reviewer correctly found durability weaknesses. The project then made an incorrect inference: that useful findings should automatically become blocking prerequisite work. D018 implemented that inference successfully at the code level but changed the structure and referential integrity of the active Goal.

D019 corrected the governance failure without erasing the evidence.

The important resulting discipline is now explicit:

```text
findings are evidence
not automatic roadmap authority
```

The repository has returned to the exact intended G003 implementation structure, while preserving a concrete future home for storage-evolution work. Goal numbering has been repaired before the new Goal became active. Repository instructions now resolve consistently for Claude through a real symlink. Hosted CI is green on the exact current head.

**Disposition:** environment stabilized. Proceed with `G003/T007` under the original Plan and ADR-0003 boundaries. Do not insert another cleanup gate unless an existing G003 Stop Condition actually fires.
