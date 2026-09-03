# Specification

## Title

- First concrete Agent provider, bounded read tool, and SDK/CLI surface

## State

- draft

## Interfaces

### `ww-agent-openai`

- Implements G003 `ModelProvider` only.
- Owns OpenAI request/response/SSE/stream parser types.
- Accepts endpoint/base URL, credential reference/resolver, model, request options, and cancellation context without exposing transport types to `ww-agent-core`.
- Converts OpenAI streaming/tool-call fragments into the G003 normalized event vocabulary.
- Recorded transport fixtures and a local mock HTTP/SSE server are mandatory conformance inputs.
- Live credentialed smoke is optional and excluded from required CI.

### `ww-agent-tools-local`

G004 adds only `fs.read`.

Proposed input:

```json
{
  "path": "relative/or/absolute/path",
  "offset_line": 1,
  "limit_lines": 200
}
```

Policy/validation rules:

- workspace root is explicit and canonicalized at run start;
- candidate path is resolved/canonicalized before file open and must remain within the canonical workspace root;
- symlink escapes reject;
- directories reject;
- non-UTF-8 content rejects in G004 rather than silently decoding;
- line offset/limit are bounded positive integers;
- total bytes returned are capped by a configured maximum even when line count is small;
- read result includes normalized path relative to workspace root plus content/truncation metadata;
- no filesystem write occurs.

### `ww-sdk`

Add Agent façade methods around G003/G004 services:

```text
start_agent_run
request_agent_cancel
inspect_agent_run
agent_events
watch_agent_events
```

Inspection returns a projection containing:

```text
common execution summary
Agent run/provider/model/tool pins
committed ordered transcript entries
usage summary
current recovery/terminal disposition
committed tool attempt summaries
artifacts/warnings
```

No raw database handle or raw provider payload is exposed.

### `ww-cli`

Required commands:

```text
ww agent run
ww agent inspect <execution-id>
ww agent events <execution-id> --after <seq>
```

`--json` / JSONL machine-readable behavior must remain stable enough for automated tests. CLI composes/configures SDK; it does not query SQLite directly.

## OpenAI fixture conformance

Required deterministic fixture cases:

- text-only success;
- streamed tool call with fragmented JSON arguments;
- multiple sequential tool calls in one assistant response;
- provider usage reporting;
- provider error;
- stream disconnect before finalization;
- cancellation during stream;
- provider length/truncation stop with incomplete tool call;
- unknown/unsupported event shape fails closed.

## Credential and audit boundary

- credentials resolve at request time from environment/secret resolver reference;
- API keys and authorization headers never enter normalized `ModelRequest`, durable Agent records, audit events, error display, or CLI JSON;
- provider request ID may be retained;
- raw response capture remains disabled by default.

## CLI end-to-end proof

Mandatory CI starts a local OpenAI-compatible mock server that emits recorded SSE responses. The `ww` binary is launched as a separate process against a temporary SQLite database and workspace fixture.

The fixture sequence must cause:

```text
user prompt
→ OpenAI adapter streamed tool call: fs.read
→ bounded fs.read result
→ second OpenAI adapter request
→ terminal text result
→ process exits successfully
```

A new CLI process then inspects the same run and event cursor from the durable database.

## Explicit exclusions

- second concrete provider;
- provider fallback/routing;
- filesystem write/patch;
- bash/process execution;
- arbitrary network/MCP/plugin tools;
- TUI/server/local daemon;
- sessions/steering/follow-up/compaction;
- Flow/OWS and WorkWeave Orchestration semantics.
