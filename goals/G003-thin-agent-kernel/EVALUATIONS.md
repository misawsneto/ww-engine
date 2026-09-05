# G003 Evaluations

- Version: `v3`
- State: `approved`
- Approval: `approved by requester 2026-09-05 under resumed D022`
- Specification basis: `G003 SPEC v3`
- All closure EvaluationRuns execute on the exact final reviewed commit.

## EvaluationRun record

Append each run under its check:

```markdown
### EvaluationRun: <stable-id>
- Check: <check id/name>
- Commit: <full SHA>
- Date: <ISO date/time>
- Mode: deterministic | review
- Command/fixture: <exact command and fixture>
- Result: pass | fail
- Evidence: <test/log/review location>
- Notes: <material interpretation only>
```

A passing run becomes stale when relevant code or the check contract changes.

## Agent protocol conformance

### `stream-ordering`

**Covers:** normalized provider protocol and mandatory kernel finalization.

Procedure:

1. run valid text/tool/usage/failure/cancellation fixtures;
2. run delta-before-start, duplicate terminal, post-terminal, unexpected EOF, invalid JSON, incomplete tool, and length-truncated tool fixtures;
3. assert invalid streams create no assistant entry or effect.

Expected:

- valid streams finalize exactly once;
- production consumer drains through EOF and calls `finish`;
- outer dispatch failure and in-stream failure create typed attempt failure with no assistant/effect;
- invalid streams execute zero tools.

### `provider-boundary`

**Covers:** provider neutrality and crate boundaries.

Expected:

- no vendor transport/request/response type enters Agent core;
- no HTTP/credential dependency exists in G003;
- tools public contracts contain no Agent operational identity/core dependency;
- RecordedProvider uses the same provider contract as the kernel.

## Tool preparation and policy conformance

### `configured-tool-order`

**Covers:** TOOL-01/02 and KERN-02 ordering basis.

Procedure:

1. register available tools in order `B, A, C`;
2. configure one run with exact pins `C@1, A@1`;
3. resolve/project model-visible specs;
4. later run the real T008 request builder with the same arrangement.

Expected:

- pure registry projection returns `C, A` only;
- real provider request exposes `C, A` only;
- registration/availability order never becomes model-visible authority.

### `schema-profile`

**Covers:** TOOL-03/04/05.

Procedure:

1. compile/validate a self-contained Draft 2020-12 fixture repeatedly;
2. reject malformed schema;
3. reject non-fragment `$ref`;
4. reject non-fragment `$dynamicRef`;
5. validate local `$ref` and local `$dynamicRef`/`$dynamicAnchor` fixture;
6. include `$id` and prove it neither causes rejection nor external retrieval;
7. assert non-coercion and deterministic WorkWeave-owned errors.

Expected:

- no external resolution occurs;
- external references fail before compile;
- local self-contained references work;
- `$id` is not treated as a retrieval request.

### `preparation-seam`

**Covers:** TOOL-06…TOOL-10, TOOL-14, TOOL-16.

Procedure:

1. inspect public ownership: `ToolPreparationDisposition` and `ToolPreparationStage` are defined in `ww-agent-tools`, while core only embeds them in Agent-owned durable records;
2. exercise the single production preparation seam through valid/invalid/unknown/classification-failure/allow/deny calls;
3. instrument resolve, validate, canonicalize, classify, policy, execute;
4. exercise nested objects with reordered keys;
5. use an effect/replay-aware policy whose decision changes when classification metadata is substituted.

Expected:

- exactly one preparation taxonomy exists and there is no tools→core dependency;
- exact order is resolve → validate → canonical bytes/digest → effect/replay → policy;
- earlier failure prevents later stages;
- invalid args invoke classification/policy/execute 0;
- nested canonical bytes are identical across key insertion order and different for different values;
- policy input structurally requires effect/replay metadata;
- policy observes the exact classified effect/replay values before decision and substitution can change the decision;
- policy denial returns stable `NoEffect(Policy)` and executes zero effects;
- no second preparation pipeline exists in core.

### `tool-execution-contract`

**Covers:** TOOL-11/12/15.

Procedure:

1. directly execute echo and unsafe fixture tests;
2. exercise ordinary failure fixture;
3. exercise cooperative cancellation fixture;
4. exercise test-only panic/invariant path outside normal outcome matching.

Expected:

- echo is deterministic/Safe;
- unsafe probe runs once per direct execute/Never;
- Output, OrdinaryError, and Cancelled are machine-distinguishable;
- cancellation is not `ToolExecutionError`;
- panic/invariant is outside normal outcomes.

### `durable-tool-grammar`

**Covers:** DUR-01…DUR-10 without claiming real execution sequencing.

Procedure:

1. construct canonical executable, no-effect, effect-in-flight, completed-awaiting-result, settled, interrupted, and intervention histories using the tools-owned preparation disposition/stage inside Agent-owned records;
2. reopen SQLite and reduce;
3. run all T007 corruption cases;
4. verify Q008 placement.

Expected:

- projections are identical after reopen;
- `ToolEffectStarted` is reduced as ambiguity boundary only;
- no-effect histories contain no effect-start/completion;
- policy denial records Policy stage only in preparation disposition and ends as Denied;
- core has not introduced a duplicate preparation taxonomy;
- invalid histories fail closed.

## Agent kernel execution conformance

### `commit-before-effect`

**Covers:** KERN-06…KERN-09 and `V-T008-23…27`.

Procedure:

1. run an allowed fixture with a probe and instrument AgentStore append commit;
2. inject append failure/conflict before effect-start authorization;
3. run a successful authorization path;
4. run tool OrdinaryError and Cancelled paths.

Expected:

- kernel uses the single T007 preparation seam and tools-owned preparation disposition;
- executor/probe count remains zero until `ToolEffectStarted` append commits;
- failed/conflicted authorization invokes zero effects;
- successful path executes exactly once after commit;
- OrdinaryError becomes one model-visible error result;
- Cancelled enters control/interruption flow and is not model-visible ordinary error;
- panic/invariant remains unnormalized.

### `no-effect-settlement`

**Covers:** real invalid/unknown/classification/denial persistence.

Procedure:

1. run each no-effect case through the real kernel;
2. inspect durable records/result entries and effect counters;
3. issue next provider request where continuation is permitted.

Expected:

- exactly one reserved model-visible error result per call;
- zero execution/effect;
- Rejected versus Denied taxonomy matches Q008;
- source order is preserved.

### `text-only`

Expected:

- one model request/assistant entry;
- successful Agent terminal result;
- no tool attempt;
- identical reopened state.

### `model-tool-model`

Procedure:

1. first recorded response requests echo;
2. kernel prepares, commits pre-effect state, executes, commits result;
3. second request contains ordered tool result;
4. final response stops;
5. reopen and inspect.

Expected:

- one logical call/one model-visible result;
- two model requests in order;
- all durable boundaries present;
- expected final Agent result.

### `cancellation-limits`

**Covers:** T009/T010 bounded execution.

Expected:

- durable cancel precedes live token;
- pre-effect cancellation prevents marker/invocation;
- post-start Cancelled follows replay-sensitive control semantics;
- common deadline is authoritative;
- over-budget tool batch executes zero calls;
- exactly fitting batch admits as a whole;
- token limits require usage capability and missing promised usage fails closed;
- same conflicting conditions choose same disposition before/after reopen.

## Agent durable recovery safety

### `recovery-reduction`

Expected:

- valid histories project identically after reopen/process restart;
- impossible histories fail closed;
- optimistic loser performs no external work;
- no next action depends on process-local state.

### `replay-safety`

Expected:

- Safe ambiguity creates a distinct retry attempt and one logical result;
- Never effect count never increases on restart and settles RequiresIntervention;
- second restart creates no additional effect/result.

### `settlement-repair`

Expected:

- Agent terminal/common nonterminal repairs common once;
- provider/tool are not invoked;
- dispositions remain consistent.

## Recovery fault matrix

### `fault-boundaries-F1-F8`

Procedure:

1. execute F1–F8 in distinct process restarts, including F2 pre-event and transient-partial-delta cases;
2. capture durable histories and provider/effect counts;
3. run competing-resumer case;
4. restart a second time.

Expected:

- exact matrix action at every boundary;
- no duplicate logical result;
- no Never replay;
- no partial stream becomes durable assistant state;
- stale resumer launches no external work;
- second restart is idempotent.

## Terminal architecture review

### `architecture-and-scope`

Procedure:

1. inspect Cargo graph/source imports/public APIs/durable records;
2. trace T007–T012 to SPEC reference evidence and WorkWeave adaptations;
3. map every requirement family to Verification/Evaluation evidence;
4. run permanent gate on exact commit.

Expected:

- Agent/Flow kernels remain separate;
- common runtime remains semantically neutral;
- no concrete transport/capability/product/Orchestration scope entered;
- no contradictory normative instruction remains;
- no blocking Stop Condition or unverified normative requirement remains.
