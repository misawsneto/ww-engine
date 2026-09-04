# Decisions

| ID | Decision | Status |
| --- | --- | --- |
| D001 | Build WorkWeave Engine in Rust. | accepted |
| D002 | Implement WorkWeave Agent and WorkWeave Flow as sibling kernels on one shared runtime substrate. | accepted |
| D003 | Keep Agent execution probabilistic and Flow execution deterministic; do not force both through one state machine. | accepted |
| D004 | Keep WorkWeave Orchestration above the engine; Goal/Task/Question/Decision/Evaluation/Review semantics do not belong inside the Agent kernel. | accepted |
| D005 | Use Pi Agent as the primary Agent reference architecture. | accepted |
| D006 | Treat Pi future Harness as a useful durability/coordination reference, not as production Pi behavior or as a semantic equivalent of WorkWeave Orchestration. | accepted |
| D007 | Use accepted OWS documents as Flow-definition authority for the qualified WorkWeave profile. | accepted |
| D008 | Use LangGraph as a Flow runtime reference for checkpoints, interrupts, streaming, durable execution and recovery, not as a second workflow DSL. | accepted |
| D009 | Flow may invoke a local WorkWeave Agent through the same logical A2A execution seam used for remote agents. | accepted |
| D010 | SDK, CLI and TUI are first-class surfaces for both Agent and Flow. | accepted |
| D011 | Audit records are durable product data; OpenTelemetry is export/observability and is not the sole canonical execution journal. | accepted |
| D012 | Start embedded with SQLite and preserve storage/transaction boundaries so coordinated PostgreSQL deployment can be added without changing engine semantics. | accepted |
| D013 | Keep the common executor abstraction intentionally thin; engine-specific types remain owned by Agent or Flow. | accepted |
| D014 | Every Goal must have at least one Goal-referenced ADR before it becomes active; architecture changes during a Goal must amend or supersede the governing ADR before reliance. | accepted |
| D015 | Split Agent delivery into G003 durable kernel proof and G004 concrete provider/SDK/CLI surface; future Flow Goal numbering shifts accordingly. | accepted |
| D016 | `main` is the continuously integrated always-green engineering line; intermediate Task work lands on `main` without waiting for terminal Goal acceptance. Merged-to-`main`, Task complete, Goal accepted, and architecture accepted are four separate states. | accepted |
| D017 | Any task-specific or temporary verification path must execute the complete merge-target CI gate; it may add checks but may never replace or omit target-branch checks. | accepted |
| D018 | Before implementing G003's previously planned T007 tool-contract slice, execute a bounded durability/hygiene gate that closes persistence-evolution, idempotent coordination, durable-type ownership, canonical tool-argument, provider-stream finalization, storage-error, SQLite physical-plumbing, architecture-guard, test-fixture, and canonical-document debt. Record the gate in G003 Plan v2 and renumber only the still-open tasks. This strengthens ADR-0003's existing durability and ownership guarantees and does not reopen the accepted Agent/Flow/provider architecture. | superseded by D019 |
| D019 | Supersede D018 and restore G003 to its pre-D018 bounded structure after T006. Revert the D018 implementation and Plan-v2/task-renumbering changes through an ordinary audited revert without rewriting history. Preserve D018's findings, implementation evidence, and lessons as historical review material, and move unresolved durability/hygiene work into a separate proposed hardening Goal rather than inserting prerequisite cleanup tasks into active G003. Existing Task identifiers must remain stable once used; new prerequisite work may be added inside an active Goal only when an existing stop condition makes safe completion impossible. | accepted |
| D020 | Introduce `ww-refine-goal`: an approved requester Decision may place a Goal under `REPLAN_LOCK`, allowing only specification/planning-related mutations until the refined Goal packet is approved and the lock is removed. Completed Task semantics and used Task IDs remain stable. | accepted |
| D021 | Refine G003's specification, plan, open Tasks, and verification/evaluation detail before T007 using the accepted reference implementations and `ww-refine-goal`, without changing the Goal boundary, completed T001–T006 semantics, or used Task IDs. | accepted |
