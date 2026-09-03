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
