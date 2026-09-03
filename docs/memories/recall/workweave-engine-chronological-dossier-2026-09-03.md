# WorkWeave Engine — Chronological Project Dossier

**Status date:** 2026-09-03  
**Project:** WorkWeave Engine  
**Primary workspace:** `/mnt/data/ww-engine`  
**Current local branch:** `g003-resync`  
**Current local checkpoint:** `3a3fb1e8f63f3098ca42fd1509b4afe5a53ed1c5`  
**Purpose:** preserve a chronological, evidence-oriented account of the architecture, implementation, reviews, failures, recoveries, and current state of WorkWeave Engine through G003 T005.

---

## 1. Executive summary

WorkWeave Engine began by separating three concerns that had initially been at risk of being conflated:

1. **WorkWeave Agent** — a bounded probabilistic LLM/tool worker.
2. **WorkWeave Flow** — a deterministic durable workflow execution engine whose authored workflow authority is OWS.
3. **WorkWeave Orchestration** — the layer above execution that owns Goal/Task/Question/Decision/Evaluation/Review and broader epistemic, deontic, and temporal work semantics.

The resulting architectural thesis is one Rust execution platform with **two sibling kernels** on a **shared operational substrate**, but **not one universal state machine**. The Agent kernel is inspired primarily by Pi's provider-neutral model/tool loop; the Flow kernel is defined by WorkWeave's OWS profile and informed by LangGraph's runtime techniques such as checkpointing, interrupts, explicit execution phases, and resumability. [R01][R02][ADR1]

The work has proceeded in four major stages:

- **G001 — WorkWeave Execution Architecture:** source-pinned Pi, WorkWeave Orchestration, OWS, and LangGraph research; C1–C4 architecture; Rust crate boundaries; persistence, audit, SDK/CLI/TUI, and implementation sequencing. G001 is achieved and accepted. [G001-1][G001-2][SRC]
- **G002 — Shared Runtime Walking Skeleton:** implemented and verified a semantically neutral Rust runtime with durable execution identity, ordered audit events, SQLite persistence, optimistic concurrency, two-phase cancellation, content-addressed artifacts, SDK inspection, and CLI/process-restart evidence. G002 is achieved after independent owner approval. [G002-1][G002-2][CI-G002]
- **G003 planning split:** the first G003 draft was judged too large because it mixed durable Agent recovery, real network provider integration, filesystem policy, and product surfaces. It was split into **G003 Durable Agent Kernel** and **G004 Agent Provider and Surface**. ADR-0003 was accepted for G003; ADR-0004 remains proposed for G004. [ADR3][ADR4][D015]
- **G003 implementation:** T002 provider protocol, T003 Agent durable-history/recovery reducer, and T004 Agent SQLite persistence are verified. T005 common/Agent SQLite transaction coordination is implemented but not yet verified because the latest CI gate stops at one clippy `clone_on_copy` warning. [R01][G003-TASKS][CI-T002][CI-T003][CI-T004][CI-T005-B]

The current local checkpoint was created after the sandbox repeatedly remounted only a subset of project files and after implementation had temporarily diverged across local snapshots, GitHub `main`, and a verification branch. The resync checkpoint deliberately consolidates the latest deep G003/G004 plan and the T002–T005 production code while excluding temporary CI/export machinery. [R07][OPS2]

---

## 2. Citation and evidence convention

This dossier favors durable records over conversational recollection.

- **`[Rxx]`** — current local repository record at checkpoint `3a3fb1e...`, with path and line range.
- **`[Gxxx-x]`** — Goal-owned record such as GOAL, TASKS, VERIFICATION, or REVIEWS.
- **`[ADRn]`** — Architecture Decision Record.
- **`[C-...]`** — WorkWeave Engine commit.
- **`[CI-...]`** — GitHub Actions run.
- **`[S-...]`** — immutable external reference source pinned by commit.
- **`[OPSx]`** — operational evidence or reconstruction checkpoint.

Where a design decision was made conversationally before the explicit ADR rule existed, the later accepted ADR is treated as the durable record and is marked retrospective where appropriate. ADR-0001 and ADR-0002 explicitly say they were backfilled after the ADR-per-Goal rule was introduced. [ADR1][ADR2]

---

# Part I — Architecture convergence

## 3. Late August to 2026-09-01 — correcting the execution/domain boundary

The critical conceptual correction was that an Agent should **not** own WorkWeave's higher-level epistemic/deontic/temporal semantics.

The durable architecture now states:

```text
WorkWeave Orchestration
  owns governed work semantics
  Goal / Task / Question / Decision / Evaluation / Review
                    |
                    v
             WorkWeave Engine
                    |
          +---------+---------+
          |                   |
          v                   v
   WorkWeave Flow       WorkWeave Agent
   deterministic        probabilistic
          |                   |
          +---------+---------+
                    v
           shared Rust substrate
```

The Agent is a **probabilistic atomic worker**: it receives bounded input, calls a model, may invoke tools, records what happened, and returns a bounded result. Flow is its deterministic sibling. Orchestration chooses and governs execution but does not become the Agent's internal state model. This is now captured in the repository purpose, decisions D002–D004, and ADR-0001. [R01][R03][ADR1]

This boundary also clarified that:

- Agent completion is not Task completion.
- Flow completion is not Goal achievement.
- model context, workflow context, execution audit, and WorkWeave Domain truth are distinct stores of meaning.
- ordinary model/tool calls belong to execution audit/observability rather than becoming orchestration semantic records. [R04][R05]

### Architecture mapping that emerged

| Reference concept | WorkWeave interpretation |
|---|---|
| Pi Agent | WorkWeave Agent reference for the probabilistic kernel |
| Pi future Harness | durability/reducer/coordination reference, not current production Pi and not semantic-equivalent to WorkWeave Orchestration |
| OWS | authored workflow-definition authority for WorkWeave Flow |
| LangGraph | runtime-mechanics reference for deterministic durability, checkpoints, interrupts, resume, streaming |

This mapping is explicit in `README.md`, ADR-0001, and the reference architecture documents. [R02][ADR1][PI-REF][FLOW-REF]

---

## 4. Canonical WorkWeave Orchestration alignment

The engine project deliberately does **not** fork WorkWeave Orchestration's canonical semantic model. The canonical reference is pinned to `misawsneto/ww-orchestration` revision `21aac374d28e6ad39944214866780a74b39f8e24`. [SRC][ORCH-REF]

The v0.5 boundary consumed by the engine is:

```text
OWS workflow definition
        |
        v
WorkWeave Flow runtime
        |
        v
guarded Domain services / commands
        |
        v
WorkWeave Domain truth
```

The engine repository owns the architecture beneath and beside this boundary: Agent runs, physical Flow runtime implementation, workers, persistence, timers, signal correlation, external execution adapters, audit, and deployment. It does not redefine Goal/Task/Question/Decision/Evaluation/Review semantics. [ORCH-REF]

This prevented a recurring architectural failure mode: turning workflow context or model context into semantic truth. The project warnings now explicitly prohibit that drift. [R05]

---

# Part II — G001: reference research and engine architecture

## 5. 2026-09-01 — source-pinned Pi research

G001 treated Pi as implementation evidence rather than a package layout to port.

The Pi source revision was pinned to:

`6c87d9a026677b601e8278030dcf1ad97fe0bd86` [SRC]

### 5.1 Pi production architecture

The Pi reverse engineering established several implementation patterns that became foundational for WorkWeave Agent:

1. **Provider-neutral streaming seam.** Pi's `StreamFn` decouples the Agent loop from concrete providers. [PI-REF][S-PI-TYPES]
2. **Small functional model/tool loop.** The low-level loop streams an assistant response, validates/finalizes tool calls, executes tools, appends tool-result messages, and calls the provider again until terminal. [PI-REF][S-PI-LOOP]
3. **Stateful façade over a smaller control loop.** Pi's `Agent` owns runtime state/listeners/queues while the actual loop remains more functional. [PI-REF][S-PI-AGENT]
4. **Tool validation and interception before effect.** Tool calls are validated, can be preflight-blocked, can execute sequentially or in parallel, and return model-visible failures rather than necessarily aborting the whole Agent. [PI-REF]
5. **Product/session composition is a larger layer.** Pi's `AgentSession` includes skills, session state, extensions, retry, compaction, tools, model selection, and UI translation. WorkWeave explicitly chose not to start there. [PI-REF]

### 5.2 Pi future Harness

The analysis separated Pi's current production Agent from the incomplete future Harness. The Harness is interesting because it introduces:

- immutable context-like entries;
- operational records;
- durable operation identity;
- reducers that reconstruct state and detect corrupt histories;
- explicit tool-start/replay information.

But the public Harness façade was incomplete at the pinned revision, so WorkWeave adopted the **entry/record/reducer idea**, not the unfinished product façade. [PI-REF][S-PI-HARNESS][S-PI-REDUCER]

That qualification later became directly relevant to G003 T003.

---

## 6. 2026-09-01 — OWS and LangGraph Flow research

The Flow research established two separate authorities:

- **OWS determines what the workflow means.**
- **LangGraph informs how a durable deterministic runtime may execute, checkpoint, interrupt, stream, and resume.** [FLOW-REF]

OWS was pinned to revision `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`; LangGraph to `11ee185999b86bfea2d8c0e69cef9a5e37acf686`. [SRC]

The frozen WorkWeave profile already identified native OWS mechanisms for `call`, `for`, `fork`, `listen`, `run.workflow`, `set`, `switch`, A2A, and MCP. The design therefore rejected a second authored graph language. [SRC][FLOW-REF]

### LangGraph mechanics selected for adaptation

The useful LangGraph lessons were:

- explicit plan/execute/update phases;
- checkpoint state as a durable resume mechanism;
- first-class interrupt/resume;
- scoped nested execution;
- event/stream projections distinct from checkpoint state. [FLOW-REF][S-LG-PREGEL][S-LG-CKPT]

The rejected ideas were equally important:

- no LangGraph graph DSL as WorkWeave Flow authority;
- no graph state as WorkWeave Domain truth;
- no framework-specific node vocabulary where OWS already defines behavior;
- no conflation of checkpoint state and audit history. [FLOW-REF]

---

## 7. G001 architecture result

G001's stated goal was to define one Rust execution platform with a probabilistic Agent kernel and deterministic OWS Flow kernel on a shared substrate. It is recorded as achieved. [G001-1]

The architecture dossier expanded this into C1–C4, including:

- C1 system context;
- C2 embedded/local-daemon/coordinated/remote deployment forms;
- C3 shared runtime, Agent, and Flow components;
- C4 Rust workspace and dependency rules;
- execution identity, attempts, budgets, cancellation, event journal, artifacts;
- Agent and Flow state models;
- provider and tool seams;
- OWS definition ingestion and compiled-plan boundaries;
- persistence and recovery contracts;
- SDK/CLI/TUI surface strategy. [ARCH]

### G001 non-negotiable decisions

ADR-0001 records the accepted baseline:

- build in Rust;
- Agent and Flow are sibling kernels;
- do not force one state machine;
- keep Orchestration above execution;
- Pi Agent is the primary Agent reference;
- Pi Harness is only a durability/coordination reference;
- OWS remains Flow-definition authority;
- LangGraph contributes runtime mechanics, not authored semantics;
- Flow-to-Agent keeps an A2A-shaped logical seam even when local;
- durable audit is product data distinct from telemetry;
- Agent and Flow ultimately deserve first-class SDK/CLI/TUI surfaces. [ADR1]

### G001 build-order decision

The architecture dossier deliberately chose a dependency sequence rather than "Agent product first" or "workflow product first":

```text
shared runtime substrate
        ↓
thin Agent kernel
        ↓
deterministic Flow kernel
        ↓
restart-safe Flow → Agent integration
```

The reason was architectural: the Agent is the first concrete consumer that stresses provider/tool/cancellation/audit boundaries, while Flow later exercises deterministic position/wait/checkpoint semantics. The integrated milestone remains more important than either isolated engine. [ARCH]

### G001 verification and acceptance

G001 verified all source pins, sibling-kernel separation, OWS authority, A2A local/remote seam, audit/observability separation, first-class product surfaces, and a bounded G002 spike. The user architecture review was recorded as accepted. [G001-2]

Relevant durable commits:

- `5b1e99025e5c374fa53c16b79035d706469c5cd5` — establish WorkWeave Engine architecture baseline. [C-G001-BASE]
- `7e688b58fa4b715e24b83463165b190061948786` — deepen WorkWeave Engine implementation dossier. [C-G001-DEEP]

---

# Part III — moving into the clean `ww-engine` repository

## 8. Re-baselining the clean repository

The clean `misawsneto/ww-engine` repository became the implementation home rather than continuing inside the orchestration starter.

The migration adopted the starter's **bookkeeping philosophy**, not its stale product identity or every copied semantic assumption. Specifically, the engine kept:

- Goal packets: GOAL / SPEC / PLAN / TASKS / VERIFICATION / REVIEWS;
- root state / decisions / questions / learnings / warnings;
- simplest-path development rules;
- evidence-first verification discipline;
- reusable authoring templates. [STARTER]

The starter's old `ww-*` skill snapshot was deliberately **not** imported because several skills still encoded v0.4/FEEL-era Flow assumptions. The repository explicitly records that those skills require requalification against WorkWeave Orchestration v0.5 before use. [STARTER]

This was an important drift-control decision: the new repository consumes canonical orchestration v0.5 rather than becoming an accidental fork of stale starter semantics. [STARTER][ORCH-REF]

---

# Part IV — G002: shared runtime walking skeleton

## 9. 2026-09-02 — G002 scope

G002 existed to falsify one assumption before either kernel grew: **can Agent and Flow share a durable Rust execution substrate without importing either engine's semantics?** [G002-1]

Its boundaries were intentionally strict:

- no LLM/tool Agent loop;
- no OWS interpreter or Flow tokens;
- no Agent-owned or Flow-owned state in the shared aggregate;
- SQLite first;
- durable audit distinct from transient telemetry. [G002-1]

The first implemented workspace contained:

```text
ww-types
ww-store
ww-store-sqlite
ww-runtime
ww-sdk
ww-cli
```

This design later became ADR-0002. [ADR2]

---

## 10. G002 implementation details

### 10.1 Shared execution aggregate

G002 established a generic execution identity/lifecycle with durable ordered events. Every lifecycle mutation was designed to update current state and append one corresponding event atomically. Optimistic version checks prevent stale writers from partially mutating state. [G002-1][ADR2]

### 10.2 SQLite first, portable semantics later

SQLite was chosen for embedded mode, but the semantic storage contract was kept separate from the physical adapter so PostgreSQL can later implement the same engine behavior. [ADR2]

### 10.3 Durable audit and reducible state

The execution row is not trusted alone. Event history is reducible, and inspection compares the reconstructed projection with the current row. This later caught a cancellation/audit mismatch that ordinary lifecycle tests had missed. [ADR2][R04]

### 10.4 Cancellation

The final G002 lifecycle is explicitly two-phase:

```text
request_cancel
  persist intent + reason
  signal local CancellationToken
        ↓
settle_cancelled
  allowed only after durable request exists
```

This is now part of ADR-0002 and the runtime invariant. [ADR2]

### 10.5 Artifacts, SDK, and CLI

G002 also proved:

- SHA-256 content-addressed artifact identity and deduplication;
- SDK inspection/event streaming without direct DB access by callers;
- CLI lifecycle operations through `ww-sdk` rather than SQLite coupling;
- cursor-based event reconnect across processes. [G002-1][G002-2]

---

## 11. G002 verification deepening

The first G002 implementation was not accepted on compile success alone. The verification was strengthened with explicit evidence for:

- real CLI process-boundary lifecycle;
- event cursor reconnect without duplicates;
- stale expected-version rejection with no partial commit;
- SQLite reopen projection equality;
- local cancellation token signaling after durable request;
- artifact deduplication;
- architecture greps that forbid Agent/provider/tool-loop and Flow/OWS/token concepts in the shared crates. [G002-2]

Relevant implementation baseline:

- `6b9e14ae13d96de9c8ed6cf0c2bfd9bc24b5bebc` — establish G002 shared runtime walking skeleton. [C-G002]

---

## 12. G002 architecture review found a real defect

A separate review pass found three related problems before Agent work was allowed to proceed:

1. `settle_cancelled` could terminalize without a prior durable `request_cancel`.
2. a second cancellation reason could diverge from the request reason.
3. the reducer did not project terminal `result_artifact` and `error`, allowing audit/current-row disagreement to go undetected. [G002-REV]

The fix:

- required durable request before terminal cancellation;
- reused the persisted reason at settlement;
- projected terminal result/error in the reducer;
- compared those values during inspection;
- added a regression test forbidding settlement without prior request. [G002-REV]

Reviewed implementation code head:

`9ea9d58f4dcafa2f5d5073beb6be65b7ab690bcc` [G002-REV]

Final CI evidence head:

`bb2cb831fe42342afcfc93cf7e8757a9206c1947` [G002-2]

Permanent CI run `33646651848` passed format, architecture boundaries, clippy with warnings denied, and the complete workspace test suite. [CI-G002]

This review produced two durable learnings that shape G003:

- current row + event reducer comparison is a valuable corruption detector;
- cancellation intent and cancellation terminalization must remain separate durable phases. [R04]

---

## 13. G002 owner review and terminal acceptance

G002 T010 required an independent architecture/implementation review. The project owner reviewed and explicitly approved the Goal on 2026-09-02. The Goal packet now records T010 complete and G002 achieved. [G002-REV][G002-TASKS]

This approval unblocked G003 activation.

---

# Part V — adding ADR discipline and preparing G003

## 14. ADR-per-Goal governance

During the transition from G002 to G003, the project adopted an explicit architectural governance rule:

> No Goal becomes active without at least one referenced ADR; if architecture changes materially during the Goal, the ADR must be amended or superseded before the changed direction is relied on.

This is decision D014 and is also an operating rule in `AGENTS.md`. [R03][R06]

Because the rule was introduced after G001/G002 had already started, ADR-0001 and ADR-0002 were recorded retrospectively. Both files explicitly say so. [ADR1][ADR2]

---

## 15. Initial G003 was too large

The first G003 proposal attempted to prove all of the following in one Goal:

- provider-neutral Agent protocol;
- durable Agent entries and operational records;
- SQLite persistence/recovery;
- tool policy and replay semantics;
- actual model → tool → model loop;
- cancellation and budgets;
- first concrete OpenAI adapter;
- bounded filesystem tool;
- SDK and CLI product surface.

Planning review judged that scope materially larger than G002 and containing independently risky proof domains. The project therefore made decision D015: split Agent delivery into **G003 durable kernel** and **G004 concrete provider/SDK/CLI surface**. [R03][R04][ADR3][ADR4]

### Resulting sequence

```text
G002 Shared Runtime                     achieved
  ↓
G003 Durable Agent Kernel               active
  ↓
G004 Agent Provider and Surface         proposed
  ↓
G005 Deterministic OWS Flow Kernel
  ↓
G006 Flow → Agent integration
  ↓
G007 Full frozen OWS profile
  ↓
G008 Local product experience
```

This sequence is now in `PROJECT_STATE.md`. [R01]

---

## 16. ADR-0003 — recovery before real effects

ADR-0003 was accepted after G002 approval. Its central thesis is that the next risk is not OpenAI connectivity or CLI UX; it is **whether the probabilistic loop can be made restart-safe without leaking provider, tool, SQLite, or Flow semantics into shared runtime or the core loop**. [ADR3]

### Core G003 decisions

- one `AgentRun` maps to one G002 common execution;
- `ww-agent-core` owns the functional loop, durable Agent model, recovery reducer, limits, and settlement;
- concrete provider/network/filesystem/CLI work is deferred;
- provider stream assembly is a pure state machine;
- finalized context entries are immutable;
- execution attempts are append-only operational records;
- retries create new attempts rather than rewriting history;
- one logical tool call can have multiple attempts but at most one model-visible committed result;
- safe ambiguous effects may retry as new attempts;
- non-replayable ambiguity becomes `RequiresIntervention` rather than silent re-execution;
- Agent data may share the physical SQLite file but remains logically Agent-owned;
- Agent DTOs must not enter shared `ww-store` semantic contracts. [ADR3]

This directly combines the strongest Pi production-loop idea with the strongest future-Harness reducer idea while rejecting Pi's larger product/session layer. [ADR3][PI-REF]

---

# Part VI — G003 implementation chronology

## 17. G003 activation

After G002 owner approval:

- G002 became achieved;
- ADR-0003 changed to accepted;
- G003 became active;
- G004 remained proposed under ADR-0004. [R01][ADR3][ADR4][G003-1]

The active G003 Goal is intentionally fixture-driven: recorded provider only, deterministic/synthetic tools only, no network provider, no user filesystem tool, no Agent public CLI/SDK surface, no Flow/OWS dependency. [G003-1]

---

## 18. T002 — provider-neutral protocol and pure stream assembler

### 18.1 Implementation

T002 created `ww-agent-provider`, now part of the workspace. [CODE-WS]

The crate owns normalized types for:

- `ProviderId`, `ModelId`, `ToolCallId`;
- `ModelCapabilities`;
- messages and model-visible content;
- tool specifications;
- `ModelRequest`;
- `ProviderContext` with cancellation token;
- completion reasons and normalized usage;
- finalized tool calls and assistant messages;
- normalized stream events;
- `ModelProvider` trait. [CODE-PROTO]

The provider trait is intentionally small:

```rust
#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn id(&self) -> ProviderId;
    fn capabilities(&self, model: &ModelId) -> ModelCapabilities;
    async fn stream(
        &self,
        request: ModelRequest,
        context: ProviderContext,
    ) -> Result<ModelEventStream, ProviderError>;
}
```

The `ResponseAssembler` is a pure state machine separate from transport. It rejects:

- events before `Started`;
- duplicate start;
- events after terminalization;
- stream end without terminal event;
- duplicate/unknown/completed tool-call misuse;
- invalid JSON arguments;
- inconsistent completion reasons;
- usage regression;
- length-truncated tool-call responses even if partial JSON happens to parse. [CODE-ASM]

### 18.2 Verification

The dedicated verification run passed:

- provider-boundary check: no `ww-runtime`, `ww-store`, `rusqlite`, `reqwest`, `ww-flow`, or OWS dependency in the provider crate;
- clippy with warnings denied;
- full workspace tests;
- **15/15 provider assembler/conformance tests**. [CI-T002]

T002 is recorded complete. [G003-TASKS][G003-VERIFY]

---

## 19. T003 — immutable Agent entries, operational records, recovery reducer

### 19.1 Why T003 preceded persistence

The project wanted recovery semantics to be a pure deterministic model before adding SQLite. This follows the Pi Harness lesson: durable state should be reducible from immutable facts/records, and impossible histories should fail closed. [ADR3]

### 19.2 Model introduced

`ww-agent-core` owns:

- immutable Agent context entries;
- operational records for model/tool attempts and terminal settlement;
- run/entry/attempt identities;
- `AgentRecoveryState` reducer;
- storage port definitions. [CODE-CORE]

The durable vocabulary separates model-facing context from execution history. That is a deliberate ownership distinction, not just schema style. [ADR3]

### 19.3 Corruption rules

The reducer rejects histories such as:

- non-contiguous entry ordinals;
- non-contiguous record sequences;
- generated entries referencing unknown attempts;
- records after terminal result;
- tool attempts that violate provider source order;
- unknown assistant-entry references;
- duplicate committed model-visible result for one logical call. [G003-EVAL][CODE-REC-TEST]

### 19.4 Verification

CI run `33704393611` passed the dependency boundary, clippy, and full workspace suite. The `ww-agent-core` recovery suite passed **11/11** tests. [CI-T003]

T003 is recorded complete. [G003-TASKS]

---

## 20. T004 — Agent-owned SQLite persistence and process-restart reconstruction

### 20.1 Ownership rule

T004 implemented `ww-agent-store-sqlite` without pushing Agent DTOs into shared `ww-store`. Agent persistence is logically separate even when it shares the same SQLite file in embedded mode. [ADR2][ADR3]

### 20.2 Durable behaviors proved

The T004 store tests cover:

- create + append + reopen produces identical history and recovery projection;
- stale Agent expected-version conflicts reject without partial mutation;
- a failure midway through an append batch rolls back inserted entries, records, and version;
- non-contiguous ordinals reject before mutation;
- a test-only helper process writes Agent history and a second OS process reopens the same DB and reconstructs the same state. [CODE-STORE-TEST][G003-VERIFY]

### 20.3 Ownership correction during T004

The first persistence fixture imported `CompletionReason` directly from `ww-agent-provider`, which would have made the store test depend directly on the provider crate instead of the Agent-owned normalized durable model. The fix moved the ownership seam in the correct direction: `ww-agent-core` re-exports/owns the normalized types already embedded in its durable messages, and the store fixture depends on core. This preserved the intended dependency boundary. [CODE-OWNERSHIP]

### 20.4 Verification

After the ownership correction, T004's verification run passed clippy and the complete workspace test suite, including rollback/reopen/version-conflict and real process-restart reconstruction. [CI-T004][G003-VERIFY]

T004 is recorded complete. [G003-TASKS]

---

## 21. T005 — common execution + Agent creation transaction seam

### 21.1 Why T005 exists

G002 owns common execution state. G003 owns Agent-specific state. Embedded mode wants one physical SQLite database, but physical co-location must not become shared semantic ownership. [ADR2][ADR3]

T005 therefore narrows the cross-model transaction proof to:

```text
one SQLite transaction
        |
        +--> create common ExecutionRecord + initial event
        |
        +--> create AgentRun + initial Agent entry
        |
        +--> create durable one-to-one AgentRun <-> Execution link
```

If any step fails, neither side should be half-created. Terminal repair remains T009 rather than being duplicated here. [G003-TASKS]

### 21.2 Implementation shape

`ww-store-sqlite` exposes only a backend-level insertion seam that can participate in an existing SQLite transaction. It does **not** gain Agent types.

`ww-agent-store-sqlite::SqliteAgentCoordinator` owns the coordination. It:

- requires runtime and Agent stores to point at the same DB path;
- runs both migrations;
- hashes Agent configuration into the common execution configuration digest;
- creates a common `agent` execution;
- creates the Agent run;
- inserts `agent_execution_links`;
- commits all three in one immediate SQLite transaction. [CODE-COORD]

The intended acceptance tests already exist:

1. common execution, Agent run, and link commit together;
2. a pre-existing Agent primary-key conflict rolls back the preceding common execution creation;
3. different DB paths are rejected. [CODE-COORD-TEST]

### 21.3 Issues encountered in T005

T005 has not yet reached a successful verification run.

**First gate failure:** `thiserror` was used in the coordinator error enum but omitted from `ww-agent-store-sqlite/Cargo.toml`. That dependency was added. [CI-T005-A][CODE-STORE-MANIFEST]

**Current gate failure:** clippy rejects:

```rust
deadline: new.deadline.clone(),
```

because `Option<DateTime<Utc>>` is `Copy` in this context. The correct mechanical fix is:

```rust
deadline: new.deadline,
```

The current local file still contains the `.clone()` at `crates/ww-agent-store-sqlite/src/coordinator.rs:112`. No T005 acceptance tests have run on the latest code state after that clippy point. [R01][CODE-COORD][CI-T005-B]

T005 therefore remains **active, implemented but unverified**. [G003-TASKS][G003-VERIFY]

---

# Part VII — sandbox/GitHub operational issues and resynchronization

## 22. Why project state diverged

The development environment introduced non-product complications that materially affected source state management.

### 22.1 Shell/network/tooling constraints

At multiple points, shell Git/network access was unavailable while the GitHub connector/API plane remained available. The project's operating rules explicitly allow using the connector or verified artifacts when direct Git network access is blocked. [R07]

Rust was also unavailable in the local sandbox during the first G002 implementation, so GitHub Actions became the authoritative compilation/clippy/test environment. That pattern continued for G003 verification.

### 22.2 Harness remount behavior

Between turns, the harness repeatedly remounted only surfaced Markdown files into `/mnt/data/ww-engine`, temporarily replacing the full checkout from the model's point of view. This caused local `.git`, Cargo files, and crate directories to appear missing even though durable snapshots and GitHub state still existed.

This is why several explicit full-checkout artifacts were created and why the project has a shared-workspace rule rather than assuming the mount is permanently stable. [R07][OPS2]

### 22.3 Three diverging source states

Before the current resync there were three relevant states:

1. **GitHub `main`** — durable G002 evidence, but behind the later deep G003/G004 split and implementation.
2. **local deep-plan snapshot / local `main`** — contained the narrowed G003/G004 plan and ADR updates.
3. **temporary verifier branch** — contained G003 T002–T005 implementation and CI-driven fixes.

This was operational divergence, not a desired branch strategy. The current resync checkpoint combines the latest product/bookkeeping state and excludes temporary verifier/export workflow files. [R07][OPS2]

---

## 23. 2026-09-03 — local resync checkpoint

The current local checkpoint is:

`3a3fb1e8f63f3098ca42fd1509b4afe5a53ed1c5` on branch `g003-resync`. [OPS2]

The checkpoint includes:

- G001/G002 accepted records;
- accepted ADR-0003;
- proposed ADR-0004;
- narrowed G003/G004 plans;
- `ww-agent-provider`;
- `ww-agent-core`;
- `ww-agent-store-sqlite` including T005 coordinator code;
- regenerated `Cargo.lock` including all three Agent crates;
- updated Project State and verification evidence;
- no temporary verifier/export workflow or trigger files in the product tree. [CODE-WS][OPS2]

The current workspace has nine crates total: the six G002 crates plus three Agent crates. [CODE-WS]

### Remote/local relation at the checkpoint

The local repository currently records:

- local `main`: `4953a4ec41f9ca2ad959506a74c14b025b511326` — deep G003/G004 split plan;
- remote `origin/main`: `b03a1278f127ecd4c65a57bd0c4130890f0da916` — frozen final G002 evidence;
- current local `g003-resync`: `3a3fb1e...` — combined local checkpoint through the T005 blocker. [OPS2]

The GitHub verifier branch separately reached T005 implementation/fix attempts; the current local checkpoint is intentionally treated as the working authority until the next clean publication step.

---

# Part VIII — current architecture and proof status

## 24. Current crate topology

```text
WorkWeave Engine workspace

shared substrate — G002
  ww-types
  ww-store
  ww-store-sqlite
  ww-runtime
  ww-sdk
  ww-cli

Agent — G003 so far
  ww-agent-provider
  ww-agent-core
  ww-agent-store-sqlite

not yet implemented
  ww-agent-tools
  RecordedProvider implementation
  functional Agent loop
  Agent lifecycle/cancellation binding
  Agent durable limits
  full crash/restart matrix

proposed G004
  concrete OpenAI-compatible adapter
  bounded fs.read
  Agent SDK projection
  ww agent CLI

future
  deterministic OWS Flow kernel
  Flow → Agent integration
```

The workspace membership and Rust 1.98 requirement are current at the resync checkpoint. [CODE-WS]

---

## 25. Proven invariants so far

### G002 proven

- common runtime can remain semantically neutral;
- state + corresponding event mutate atomically;
- optimistic conflict does not partially commit;
- current execution state can be reconstructed from durable events;
- process restart preserves execution inspection;
- cancellation request and cancellation settlement are separate durable phases;
- CLI uses SDK rather than DB coupling;
- common runtime has no Agent or Flow semantic leakage. [G002-2][ADR2]

### G003 T002 proven

- provider transport can be normalized behind a small provider-neutral contract;
- stream assembly can fail closed before tool execution;
- malformed/truncated/duplicate provider streams are rejected deterministically;
- the provider contract can exist without runtime/store/HTTP/Flow dependencies. [CI-T002][CODE-PROTO][CODE-ASM]

### G003 T003 proven

- Agent model-facing context can be separated from operational execution records;
- recovery can be a pure deterministic reduction;
- impossible durable histories can be detected explicitly instead of guessed through. [CI-T003][ADR3]

### G003 T004 proven

- Agent-specific persistence can remain logically Agent-owned;
- Agent history survives SQLite reopen and OS process restart;
- stale writers and partial append batches fail without partial Agent mutation. [CI-T004][CODE-STORE-TEST]

### Not yet proven

- atomic common+Agent creation transaction under T005, despite implementation/tests being present;
- recorded provider implementation and conformance fixtures;
- tool schema/policy/replay model;
- actual model → tool → model Agent kernel;
- cancellation propagation to active provider/tool work;
- durable Agent budgets;
- ambiguous safe/non-replayable effect restart matrix;
- final G003 EvaluationRuns and architecture review. [G003-VERIFY]

---

# Part IX — current Goal state and next decisions

## 26. Goal status

| Goal | State | Evidence/result |
|---|---|---|
| G001 WorkWeave Execution Architecture | achieved | accepted source-pinned C1–C4 architecture and implementation dossier |
| G002 Shared Runtime Walking Skeleton | achieved | final CI + owner review accepted |
| G003 Thin/Durable Agent Kernel | active | T002–T004 verified; T005 active, one clippy blocker |
| G004 Agent Provider and Surface | proposed | ADR-0004 drafted, activation depends on G003 terminal acceptance |
| G005 Deterministic OWS Flow Kernel | future | not activated |
| G006 Flow → Agent integration | future | not activated |

Current authoritative Goal status is recorded in `PROJECT_STATE.md`. [R01]

---

## 27. Immediate next engineering step

The next engineering action should remain narrow:

1. fix the T005 `clone_on_copy` line;
2. rerun clippy;
3. run the existing coordinator tests proving atomic creation/link and rollback;
4. only if T005 passes, mark T005 complete;
5. consolidate/publish the resynced G003 state onto a durable GitHub development branch before beginning T006.

The project should **not** jump to OpenAI or filesystem tooling. Those remain G004 by design. [ADR3][ADR4][R05]

---

## 28. Known bookkeeping residue discovered while preparing this dossier

One non-code inconsistency remains in the resynced checkpoint:

`QUESTIONS.md` still lists Q004 — whether Agent conversational persistence and the common execution journal share one physical SQLite database — as `open — proposed answer in ADR-0003`, and its explanatory text still says the question remains open until ADR-0003 is accepted. ADR-0003 is now accepted and T004/T005 are already implementing that direction. [R08][ADR3]

The effective architecture is therefore already decided:

- one physical SQLite database is permitted in embedded mode;
- common runtime and Agent maintain separate logical ownership/tables/repos;
- cross-model operations that require atomicity coordinate at the SQLite backend transaction seam;
- Agent DTOs stay out of `ww-store`. [ADR2][ADR3]

Q004 should be marked resolved in the next bookkeeping cleanup.

---

# Part X — decision history in compact form

## 29. Accepted project-level decisions

The current `DECISIONS.md` records fifteen accepted directions. The highest-impact ones are:

- Rust implementation;
- sibling Agent/Flow kernels;
- distinct probabilistic vs deterministic semantics;
- Orchestration above execution;
- Pi primary Agent reference;
- Pi Harness qualified as future/incomplete reference;
- OWS as Flow authority;
- LangGraph runtime-only reference;
- A2A-shaped local/remote Agent seam;
- first-class SDK/CLI/TUI direction;
- durable audit distinct from OTel;
- SQLite-first physical storage;
- thin common execution abstraction;
- ADR required before Goal activation;
- G003/G004 split. [R03]

---

## 30. Rejected architectural shortcuts

Across G001–G003, the project has repeatedly rejected shortcuts that would reduce short-term implementation friction but create long-term ambiguity:

- port Pi package-for-package;
- treat Pi Harness scaffolding as production behavior;
- adopt LangGraph's graph DSL under OWS;
- make one shared Agent/Flow state machine;
- treat model/workflow context or audit as WorkWeave Domain truth;
- make OTel the only audit record;
- push Agent DTOs into shared `ww-store` to simplify transactions;
- expose write/process/network tools before replay/intervention semantics are proven;
- pull OpenAI/fs.read/SDK/CLI back into G003. [R05][ADR1][ADR3][ADR4]

These rejections are as important as the positive architecture because they define the project's drift boundaries.

---

# Part XI — evidence ledger

## 31. Current local repository records

**[R01] Current project state**  
`PROJECT_STATE.md:L5-L18`, `L20-L30`, `L32-L65` at local checkpoint `3a3fb1e8f63f3098ca42fd1509b4afe5a53ed1c5`.

**[R02] Product architecture and reference mapping**  
`README.md:L3-L10`, `L36-L43`, `L57-L63`.

**[R03] Durable project decisions**  
`DECISIONS.md:L3-L19`.

**[R04] Project learnings**  
`LEARNINGS.md:L3-L16`.

**[R05] Drift/safety warnings**  
`WARNINGS.md:L3-L15`.

**[R06] Agent operating and ADR rules**  
`AGENTS.md:L5-L24`, `L26-L38`.

**[R07] GitHub/sandbox operating rules**  
`GITHUB-SANDBOX-RULES.md:L3-L18`, `L20-L43`.

**[R08] Current questions / stale Q004 residue**  
`QUESTIONS.md:L3-L19`.

**[STARTER] Starter adoption boundary**  
`docs/STARTER-ADOPTION.md:L3-L30`.

**[ORCH-REF] Canonical orchestration reference**  
`docs/orchestration/README.md:L3-L31`.

---

## 32. Goal evidence

**[G001-1] G001 Goal**  
`goals/G001-workweave-execution-architecture/GOAL.md:L1-L30`.

**[G001-2] G001 Verification**  
`goals/G001-workweave-execution-architecture/VERIFICATION.md:L1-L16`.

**[G002-1] G002 Goal**  
`goals/G002-shared-runtime-walking-skeleton/GOAL.md:L1-L47`.

**[G002-TASKS] G002 Tasks**  
`goals/G002-shared-runtime-walking-skeleton/TASKS.md:L1-L14`.

**[G002-REV] G002 review and owner approval**  
`goals/G002-shared-runtime-walking-skeleton/REVIEWS.md:L3-L51`.

**[G002-2] G002 Verification**  
`goals/G002-shared-runtime-walking-skeleton/VERIFICATION.md:L3-L44`.

**[G003-1] G003 Goal**  
`goals/G003-thin-agent-kernel/GOAL.md:L1-L59`.

**[G003-TASKS] G003 Tasks**  
`goals/G003-thin-agent-kernel/TASKS.md:L1-L16`.

**[G003-VERIFY] G003 Verification**  
`goals/G003-thin-agent-kernel/VERIFICATION.md:L1-L74`.

**[G003-EVAL] G003 Evaluation contracts**  
`goals/G003-thin-agent-kernel/EVALUATIONS.md:L1-L77`.

---

## 33. ADR evidence

**[ADR1] ADR-0001 — WorkWeave execution architecture baseline**  
`docs/adr/ADR-0001-g001-execution-architecture.md:L1-L44`.

**[ADR2] ADR-0002 — Shared runtime walking skeleton**  
`docs/adr/ADR-0002-g002-shared-runtime.md:L1-L43`.

**[ADR3] ADR-0003 — Durable provider-neutral Agent kernel**  
`docs/adr/ADR-0003-g003-thin-agent-kernel.md:L1-L100`.

**[ADR4] ADR-0004 — First concrete Agent provider and SDK/CLI surface**  
`docs/adr/ADR-0004-g004-agent-provider-surface.md:L1-L46`.

**[D015] G003/G004 split**  
`DECISIONS.md:L18-L19`; see also `ADR-0003:L8-L15` and `ADR-0004:L8-L13`.

---

## 34. Architecture/research evidence

**[ARCH] Integrated engine architecture dossier**  
`docs/architecture/WORKWEAVE-ENGINE-ARCHITECTURE-DOSSIER.md`, especially sections 1–17 and the implementation sequence.

**[PI-REF] Pi reference architecture**  
`docs/architecture/PI-REFERENCE-ARCHITECTURE.md`, especially C3 provider/loop/tool/session sections and future Harness qualification.

**[FLOW-REF] Flow reference architecture**  
`docs/architecture/FLOW-REFERENCE-ARCHITECTURE.md`, especially OWS boundary, LangGraph runtime lessons, deterministic stepping, and Flow-to-Agent seam.

**[SRC] Immutable source register**  
`docs/architecture/SOURCE-REGISTER.md:L3-L56`.

---

## 35. External immutable source pins

**[S-PI-TYPES] Pi Agent provider/tool seam**  
`earendil-works/pi@6c87d9a026677b601e8278030dcf1ad97fe0bd86`  
`packages/agent/src/types.ts#L18-L32`, `#L149-L210`  
https://github.com/earendil-works/pi/blob/6c87d9a026677b601e8278030dcf1ad97fe0bd86/packages/agent/src/types.ts

**[S-PI-LOOP] Pi low-level Agent loop**  
`packages/agent/src/agent-loop.ts#L32-L102`, `#L156-L360`  
https://github.com/earendil-works/pi/blob/6c87d9a026677b601e8278030dcf1ad97fe0bd86/packages/agent/src/agent-loop.ts

**[S-PI-AGENT] Pi Agent façade**  
`packages/agent/src/agent.ts#L98-L124`, `#L173-L214`  
https://github.com/earendil-works/pi/blob/6c87d9a026677b601e8278030dcf1ad97fe0bd86/packages/agent/src/agent.ts

**[S-PI-HARNESS] Pi future Harness**  
`packages/agent/src/harness/agent-harness.ts#L134-L198`, `#L305-L520`  
https://github.com/earendil-works/pi/blob/6c87d9a026677b601e8278030dcf1ad97fe0bd86/packages/agent/src/harness/agent-harness.ts

**[S-PI-REDUCER] Pi Harness reducer**  
`packages/agent/src/harness/reducer.ts#L79-L126`, `#L506-L620`  
https://github.com/earendil-works/pi/blob/6c87d9a026677b601e8278030dcf1ad97fe0bd86/packages/agent/src/harness/reducer.ts

**[S-OWS] OWS source pin**  
`open-workflow-specification/specification@2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`.

**[S-LG-PREGEL] LangGraph Pregel runtime**  
`langchain-ai/langgraph@11ee185999b86bfea2d8c0e69cef9a5e37acf686`  
`libs/langgraph/langgraph/pregel/main.py`  
https://github.com/langchain-ai/langgraph/blob/11ee185999b86bfea2d8c0e69cef9a5e37acf686/libs/langgraph/langgraph/pregel/main.py

**[S-LG-CKPT] LangGraph checkpoint contract**  
`libs/checkpoint/langgraph/checkpoint/base/__init__.py`  
https://github.com/langchain-ai/langgraph/blob/11ee185999b86bfea2d8c0e69cef9a5e37acf686/libs/checkpoint/langgraph/checkpoint/base/__init__.py

---

## 36. WorkWeave commits

**[C-G001-BASE]**  
`5b1e99025e5c374fa53c16b79035d706469c5cd5` — Establish WorkWeave Engine architecture baseline  
https://github.com/misawsneto/ww-engine/commit/5b1e99025e5c374fa53c16b79035d706469c5cd5

**[C-G001-DEEP]**  
`7e688b58fa4b715e24b83463165b190061948786` — deepen implementation dossier  
https://github.com/misawsneto/ww-engine/commit/7e688b58fa4b715e24b83463165b190061948786

**[C-G002]**  
`6b9e14ae13d96de9c8ed6cf0c2bfd9bc24b5bebc` — G002 shared runtime walking skeleton  
https://github.com/misawsneto/ww-engine/commit/6b9e14ae13d96de9c8ed6cf0c2bfd9bc24b5bebc

**[C-G002-FINAL]**  
`b03a1278f127ecd4c65a57bd0c4130890f0da916` — freeze final G002 executable evidence  
https://github.com/misawsneto/ww-engine/commit/b03a1278f127ecd4c65a57bd0c4130890f0da916

**[OPS2] Current local resync checkpoint**  
`3a3fb1e8f63f3098ca42fd1509b4afe5a53ed1c5` — `wip: resync G003 through T005 verification blocker`; local `g003-resync` branch. This checkpoint is not yet represented as one coherent commit on remote `main`.

---

## 37. CI evidence

**[CI-G002] Final G002 CI**  
Run `33646651848` — success  
https://github.com/misawsneto/ww-engine/actions/runs/33646651848

**[CI-T002] G003 T002 provider protocol verification**  
Run `33703827542` — provider-boundary check, clippy, full workspace tests; 15 provider assembler/conformance tests passed.  
https://github.com/misawsneto/ww-engine/actions/runs/33703827542

**[CI-T003] G003 T003 recovery reducer verification**  
Run `33704393611` — clippy and full workspace suite; 11 recovery/corruption tests passed.  
https://github.com/misawsneto/ww-engine/actions/runs/33704393611

**[CI-T004] G003 T004 Agent SQLite verification**  
Run `33705447856` — clippy and full workspace suite passed after fixture ownership correction; includes SQLite rollback/reopen/version-conflict and process-restart reconstruction.  
https://github.com/misawsneto/ww-engine/actions/runs/33705447856

**[CI-T005-A] G003 T005 first coordinator gate**  
Run `33706060545` — failed before tests because `ww-agent-store-sqlite` omitted `thiserror` dependency.  
https://github.com/misawsneto/ww-engine/actions/runs/33706060545

**[CI-T005-B] G003 T005 current gate**  
Run `33706225636` — dependency issue resolved; clippy failed on `clone_on_copy` at coordinator deadline assignment; tests skipped.  
https://github.com/misawsneto/ww-engine/actions/runs/33706225636

---

## 38. Current code evidence

**[CODE-WS] Cargo workspace**  
`Cargo.toml:L1-L37` — nine current crates and Rust 1.98 workspace baseline.

**[CODE-PROTO] Provider protocol**  
`crates/ww-agent-provider/src/protocol.rs:L37-L45`, `L55-L87`, `L99-L142`, `L170-L213`.

**[CODE-ASM] Stream assembler**  
`crates/ww-agent-provider/src/assembler.rs`, especially typed `AssemblyError` and finalization logic.

**[CODE-CORE] Agent durable core**  
`crates/ww-agent-core/src/history.rs`, `src/reducer.rs`, `src/store.rs`.

**[CODE-REC-TEST] Agent recovery tests**  
`crates/ww-agent-core/tests/recovery.rs`.

**[CODE-STORE-TEST] Agent SQLite persistence tests**  
`crates/ww-agent-store-sqlite/tests/store.rs:L85-L274`; `tests/process_restart.rs`.

**[CODE-OWNERSHIP] Agent persistence fixture uses core-owned normalized durable types**  
`crates/ww-agent-store-sqlite/src/bin/agent-store-fixture.rs:L4-L9`; `crates/ww-agent-core/src/lib.rs:L5-L18`.

**[CODE-COORD] T005 coordinator**  
`crates/ww-agent-store-sqlite/src/coordinator.rs:L16-L21`, `L23-L50`, `L60-L147`, `L149-L203`. Current clippy issue is line 112.

**[CODE-COORD-TEST] T005 coordinator tests**  
`crates/ww-agent-store-sqlite/tests/coordinator.rs:L37-L149`.

**[CODE-STORE-MANIFEST] Agent SQLite crate dependencies**  
`crates/ww-agent-store-sqlite/Cargo.toml:L1-L22`, including corrected `thiserror.workspace = true`.

---

# 39. Final current-state statement

As of this dossier:

- the **architecture direction is stable**;
- G001 and G002 are achieved;
- G003 is the active recovery-first Agent kernel proof;
- the provider protocol, recovery reducer, and Agent SQLite store are verified;
- common/Agent atomic creation coordination is implemented but awaiting verification after one mechanical clippy fix;
- G004 remains deliberately deferred;
- Flow work has not started, preserving OWS as future Flow-definition authority rather than allowing Agent work to accrete workflow semantics;
- the local `g003-resync` checkpoint is currently the most coherent working snapshot and should be consolidated to a durable GitHub development branch before the next implementation slice.

This is the handoff point for continuing WorkWeave Engine development.
