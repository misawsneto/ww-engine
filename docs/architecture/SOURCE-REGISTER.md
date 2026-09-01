# Source Register

G001 uses immutable source pins. Reference repositories inform design; they do not become WorkWeave authority.

| Source | Revision | Role |
| --- | --- | --- |
| `earendil-works/pi` | `6c87d9a026677b601e8278030dcf1ad97fe0bd86` | Agent execution, provider/tool seams, sessions, extensions, future Harness |
| `misawsneto/ww-orchestration` | `21aac374d28e6ad39944214866780a74b39f8e24` | canonical WorkWeave Domain/Flow v0.5 boundary |
| `open-workflow-specification/specification` | `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a` | OWS 1.0.3 schema/profile reference |
| `langchain-ai/langgraph` | `11ee185999b86bfea2d8c0e69cef9a5e37acf686` | deterministic runtime, checkpoints, interrupts, streams |

## Pi critical evidence

- `packages/agent/src/types.ts#L18-L32` — `StreamFn` provider-stream contract.
- `packages/agent/src/types.ts#L34-L50` — tool execution and queue modes.
- `packages/agent/src/types.ts#L149-L210` — agent-loop configuration and context/provider seams.
- `packages/agent/src/agent.ts#L98-L124` — Agent construction contract.
- `packages/agent/src/agent.ts#L173-L214` — stateful Agent façade and injected collaborators.
- `packages/agent/src/agent-loop.ts#L32-L102` — prompt/continue loop entry points.
- `packages/agent/src/agent-loop.ts#L156-L360` — low-level loop orchestration.
- `packages/coding-agent/src/core/agent-session.ts#L311-L520` — coding-agent session composition layer.
- `packages/coding-agent/src/core/session-manager.ts#L856-L1040` — JSONL session persistence behavior.
- `packages/coding-agent/src/core/extensions/types.ts#L451-L500` — tool registration/execution contract.
- `packages/server/src/types.ts#L41-L60` — durable session runtime/service boundary.
- `packages/agent/src/harness/agent-harness.ts#L134-L198` — suspended operations, snapshots and actions.
- `packages/agent/src/harness/agent-harness.ts#L305-L520` — future Harness façade; inspect implementation status before treating any method as production behavior.
- `packages/agent/src/harness/reducer.ts#L79-L126` and `#L506-L620` — reducible lane execution state and corruption checking.

Immutable source prefix:

`https://github.com/earendil-works/pi/blob/6c87d9a026677b601e8278030dcf1ad97fe0bd86/`

## WorkWeave v0.5 critical evidence

- `docs/orchestration/flow/model.yaml` — two durable Flow entities, OWS ownership, runtime/observability boundaries.
- `docs/orchestration/ows/profile.yaml` — OWS 1.0.3, strict jq, native A2A/MCP/function/run mechanisms.
- `docs/orchestration/WORKWEAVE-ORCHESTRATION-DOSSIER.md` — lineage and architectural separation.

Immutable source prefix:

`https://github.com/misawsneto/ww-orchestration/blob/21aac374d28e6ad39944214866780a74b39f8e24/`

## LangGraph critical evidence

- `libs/langgraph/langgraph/graph/state.py` — `StateGraph`: nodes read state and emit partial state updates.
- `libs/langgraph/langgraph/pregel/main.py` — Pregel runtime; plan/execute/update supersteps and streaming.
- `libs/checkpoint/langgraph/checkpoint/base/__init__.py` — checkpoint persistence contract keyed by thread identity.
- `libs/langgraph/langgraph/types.py` — commands, sends, interrupts and execution types.

Immutable source prefix:

`https://github.com/langchain-ai/langgraph/blob/11ee185999b86bfea2d8c0e69cef9a5e37acf686/`

## OWS critical evidence

WorkWeave pins the OWS schema source to commit `2dd2c84170d5f3e05d58e913e9ca298dcf8d543a`; the qualified profile currently supports native `call`, `for`, `fork`, `listen`, `run.workflow`, `set`, and `switch`, plus A2A and MCP calls.
