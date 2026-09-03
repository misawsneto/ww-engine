# G004 Evaluations

## OpenAI adapter conformance
- State: `draft`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G004 — Agent Provider and Surface`

### Checks

#### protocol-translation
- Covers: `OpenAI adapter normalized provider behavior`
- Subjects: `ww-agent-openai transport and stream translation`
- Criteria: `Recorded/local HTTP fixtures map text, fragmented tool calls, usage, completion, provider failure, disconnect, cancellation, and truncation to the G003 normalized protocol without vendor-type leakage.`
- Procedure: `Run provider contract tests against a local mock OpenAI-compatible HTTP/SSE server.`
- Expected: `All supported fixtures produce expected normalized events; malformed/unsupported/incomplete streams fail closed.`

#### credential-redaction
- Covers: `Provider secret boundary`
- Subjects: `credential resolver, model requests, durable Agent audit, CLI output`
- Criteria: `API keys and authorization headers never appear in normalized durable state or user-visible machine output.`
- Procedure: `Use sentinel credentials and scan durable records, event JSON, error snapshots, and CLI JSON/JSONL.`
- Expected: `No sentinel secret value or raw authorization header is present.`

## Bounded filesystem read safety
- State: `draft`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G004 — Agent Provider and Surface`

### Checks

#### workspace-containment
- Covers: `fs.read authorization boundary`
- Subjects: `ww-agent-tools-local fs.read`
- Criteria: `Only canonical regular files inside the configured workspace root can be read; path traversal and symlink escapes reject before content return.`
- Procedure: `Run temporary-filesystem fixtures for in-root, outside-root, traversal, symlink, directory, and missing paths.`
- Expected: `Only valid in-root regular files return content.`

#### output-bounds
- Covers: `fs.read bounded effect/result`
- Subjects: `fs.read line/range/byte and encoding behavior`
- Criteria: `Reads obey configured line/byte bounds, reject non-UTF-8 in G004, and expose deterministic truncation metadata.`
- Procedure: `Run large-file, range, UTF-8, non-UTF-8, and cancellation fixtures.`
- Expected: `Returned content never exceeds configured bounds and invalid encoding/cancellation has typed behavior.`

## Agent SDK and CLI surface conformance
- State: `draft`
- Mode: `deterministic`
- Evaluator Mode: `deterministic`
- Required For Closure Of:
  - `G004 — Agent Provider and Surface`

### Checks

#### sdk-projection
- Covers: `Rust Agent SDK surface`
- Subjects: `ww-sdk Agent methods and projectors`
- Criteria: `Caller can start/cancel/inspect/stream a run through normalized projections without direct database access.`
- Procedure: `Run SDK integration tests against temporary SQLite and local mock provider.`
- Expected: `Projected transcript, attempts, usage, terminal disposition, and events match durable state.`

#### cli-process-boundary
- Covers: `ww agent CLI surface`
- Subjects: `ww binary, SDK, local mock provider, temporary workspace/database`
- Criteria: `Separate CLI processes execute and later inspect one durable OpenAI-protocol → fs.read → OpenAI-protocol run with stable machine-readable output.`
- Procedure: `Launch local mock server and CLI processes, then reconnect with a new CLI process using the same database.`
- Expected: `Run succeeds once, inspection matches durable state, event cursor reconnects without duplicates, and CLI has no direct store dependency.`
