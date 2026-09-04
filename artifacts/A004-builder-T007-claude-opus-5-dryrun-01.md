# T007 Execution Dry Run

- Actor: `A004-builder`
- Task: `G003 / T007 — tool contract, schema validation, policy, replay fixtures`
- Model: `claude-opus-5`
- Sequence: `01`
- Basis head: `9f0a02f`
- Status: dry run only. No repository code changed.

This document is a reference walkthrough. It shows the snippets the builder writes.
It does not add scope. It follows SPEC v2 §5.1, §6, §7 and TASKS T007.

---

## 1. Authority and scope

Authority order comes from `HANDOFF.md`.

```text
Decisions + ADR-0003 → GOAL → SPEC → PLAN → TASKS → V&V → HANDOFF → code
```

T007 delivers three work units.

```text
A. ww-agent-tools identity + offline Draft 2020-12 schema contract
B. registry + digest + effect/replay + Allow/Deny policy + fixtures
C. Agent-owned durable preparation vocabulary + reducer rules
```

T007 does not build the model→tool→model loop. T008 owns that loop.

---

## 2. Preconditions checked before code

All checks below ran against head `9f0a02f`.

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| five CI boundary greps | pass |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | pass |
| `cargo test --workspace --all-features --locked` | 58 pass, 0 fail |
| `jsonschema@0.52.1` exists | yes |
| `--no-default-features` removes `resolve-http`, `resolve-file`, `reqwest`, TLS | yes |
| `sha2` in workspace deps | yes, `0.10` |
| `serde_json` `preserve_order` enabled | no |
| `ww-agent-core` already depends on `ww-agent-provider` | yes |

The last two rows change the design. Section 6 and section 17 give the reason.

---

## 3. Step A — crate skeleton

Add one crate. Do not add more.

```toml
# crates/ww-agent-tools/Cargo.toml
[package]
name = "ww-agent-tools"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[dependencies]
async-trait.workspace = true
jsonschema = { workspace = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
thiserror.workspace = true
tokio-util.workspace = true

[dev-dependencies]
tokio.workspace = true
```

Add the pin to the workspace.

```toml
# Cargo.toml  [workspace.dependencies]
jsonschema = { version = "0.52.1", default-features = false }
```

`default-features = false` removes network and file resolution. This satisfies SPEC §6.4.

Do not add `schemars`. Schemas are fixture data.

---

## 4. Step A — identity types

Copy the SPEC §6.1 shapes. Do not add fields.

```rust
// crates/ww-agent-tools/src/identity.rs
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolId(String);

impl ToolId {
    pub fn new(value: impl Into<String>) -> Result<Self, ToolDefinitionError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ToolDefinitionError::EmptyIdentity);
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}
```

`ToolVersion` uses the same rule. A macro removes the duplication.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolIdentity {
    pub id: ToolId,
    pub version: ToolVersion,
    pub implementation_digest: Option<String>,
}
```

`implementation_digest` stays `None` for both fixtures. The field belongs to the contract.

---

## 5. Step A — schema profile

Compile one validator per tool at registration.

```rust
// crates/ww-agent-tools/src/schema.rs
use jsonschema::{Draft, Validator};

pub struct CompiledSchema {
    validator: Validator,
}

impl CompiledSchema {
    pub fn compile(schema: &Value) -> Result<Self, ToolDefinitionError> {
        reject_external_refs(schema)?;                 // see below
        let validator = jsonschema::options()
            .with_draft(Draft::Draft202012)
            .build(schema)
            .map_err(|error| ToolDefinitionError::InvalidSchema {
                message: error.to_string(),
            })?;
        Ok(Self { validator })
    }

    pub fn validate(&self, instance: &Value) -> Result<(), Vec<ArgumentViolation>> {
        let mut violations: Vec<ArgumentViolation> = self
            .validator
            .iter_errors(instance)
            .map(|error| ArgumentViolation {
                instance_path: error.instance_path.to_string(),
                message: error.to_string(),
            })
            .collect();
        if violations.is_empty() {
            return Ok(());
        }
        violations.sort_by(|a, b| {
            a.instance_path.cmp(&b.instance_path).then(a.message.cmp(&b.message))
        });
        Err(violations)
    }
}
```

The sort gives deterministic order. SPEC §6.4 requires that order.

`ArgumentViolation` is WorkWeave-owned. No `jsonschema` type leaves this module.

Reject external references before compilation.

```rust
fn reject_external_refs(schema: &Value) -> Result<(), ToolDefinitionError> {
    match schema {
        Value::Object(map) => {
            if let Some(Value::String(reference)) = map.get("$ref")
                && !reference.starts_with('#')
            {
                return Err(ToolDefinitionError::ExternalReference {
                    reference: reference.clone(),
                });
            }
            map.values().try_for_each(reject_external_refs)
        }
        Value::Array(items) => items.iter().try_for_each(reject_external_refs),
        _ => Ok(()),
    }
}
```

This check runs before `build`. No retrieval attempt occurs. This satisfies `V-T007-07`.

Disabled default features give a second guarantee. The crate cannot retrieve a remote schema.

---

## 6. Step B — canonical arguments digest

The digest uses SHA-256 over a deterministic serialization.

```rust
// crates/ww-agent-tools/src/digest.rs
pub fn arguments_digest(arguments: &Value) -> String {
    let canonical = canonical_value(arguments);
    let bytes = serde_json::to_vec(&canonical).expect("Value always serializes");
    format!("{:x}", Sha256::digest(&bytes))
}

fn canonical_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted.insert(key.clone(), canonical_value(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonical_value).collect()),
        other => other.clone(),
    }
}
```

Warning. `serde_json::Map` is a `BTreeMap` today. Keys already sort on serialize.
The explicit sort looks redundant. It is not redundant. Section 17 gives the reason.

The digest never mutates the input. The caller keeps the original `Value`.

---

## 7. Step B — effect, replay, policy

Copy the SPEC §6.1 enums. Add no variant.

```rust
pub enum ReplayPolicy { Safe, Never }

pub enum EffectDescriptor {
    Pure { kind: String },
    Synthetic { kind: String, attributes: Value },
}

pub enum PolicyDecision {
    Allow,
    Deny { code: String, message: String },
}
```

The policy seam is synchronous. It returns one decision.

```rust
pub struct ToolPolicyInput<'a> {
    pub identity: &'a ToolIdentity,
    pub arguments: &'a Value,
    pub arguments_digest: &'a str,
    pub effect: &'a EffectDescriptor,
    pub replay: ReplayPolicy,
}

pub trait ToolPolicy: Send + Sync {
    fn evaluate(&self, input: &ToolPolicyInput<'_>) -> PolicyDecision;
}
```

`ToolPolicyInput` carries no Agent identity. SPEC §5.1 forbids that dependency.

G003 needs one allow-list policy only. Do not build a policy language.

```rust
pub struct AllowListPolicy { allowed: BTreeSet<ToolId> }
```

---

## 8. Step B — registry

The registry is immutable for one run.

```rust
pub struct ToolRegistry {
    order: Vec<ToolId>,                              // configured order
    entries: BTreeMap<ToolId, RegisteredTool>,
}

struct RegisteredTool {
    tool: Arc<dyn Tool>,
    spec: ToolSpec,
    schema: CompiledSchema,
}

impl ToolRegistry {
    pub fn build(tools: Vec<Arc<dyn Tool>>) -> Result<Self, ToolDefinitionError> {
        let mut order = Vec::new();
        let mut entries = BTreeMap::new();
        for tool in tools {
            let spec = tool.spec();
            let id = spec.identity.id.clone();
            if entries.contains_key(&id) {
                return Err(ToolDefinitionError::DuplicateId { id });
            }
            let schema = CompiledSchema::compile(&spec.input_schema)?;
            order.push(id.clone());
            entries.insert(id, RegisteredTool { tool, spec, schema });
        }
        Ok(Self { order, entries })
    }

    pub fn model_visible_specs(&self) -> Vec<&ToolSpec> {
        self.order.iter().map(|id| &self.entries[id].spec).collect()
    }
}
```

`order` keeps configured order. `entries` gives deterministic lookup. This satisfies `V-T007-04`.

Version resolution rejects a mismatch. It does not substitute.

```rust
pub fn resolve(&self, id: &ToolId, version: &ToolVersion)
    -> Result<&RegisteredTool, ToolResolutionError>
```

---

## 9. Step B — fixtures

`test.echo` is pure and replay-safe.

```rust
pub struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            identity: ToolIdentity { id: ToolId::new("test.echo").unwrap(), version: v1(), implementation_digest: None },
            description: "Return the input value.".to_owned(),
            input_schema: json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": { "value": {} },
                "required": ["value"],
                "additionalProperties": false
            }),
        }
    }
    fn effect(&self, _a: &Value) -> Result<EffectDescriptor, ToolExecutionError> {
        Ok(EffectDescriptor::Pure { kind: "test.echo".to_owned() })
    }
    fn replay_policy(&self, _a: &Value) -> ReplayPolicy { ReplayPolicy::Safe }
    async fn execute(&self, request: ToolRequest, _c: ToolContext)
        -> Result<ToolOutput, ToolExecutionError> {
        Ok(ToolOutput { content: json!({ "value": request.arguments["value"] }) })
    }
}
```

`test.unsafe_once` is synthetic and never replay-safe. It calls an injected probe.

```rust
pub trait EffectProbe: Send + Sync {
    fn observe(&self, key: &str);
}

pub struct UnsafeOnceTool { probe: Arc<dyn EffectProbe> }

// replay_policy() always returns ReplayPolicy::Never.
// execute() calls probe.observe(key) exactly once, then returns
// { "applied": true, "key": <key> }.
```

The probe stays test-only. The tool exposes no file, process or network capability.

`CountingProbe` in unit tests holds an `AtomicUsize`. T011 may use a durable probe later.

---

## 10. Step C — durable vocabulary

`ww-agent-core` gains the SPEC §7.2 records. `ww-agent-core` already depends on
`ww-agent-provider`. The new `ww-agent-tools` dependency follows the same direction.

```rust
// crates/ww-agent-core/src/history.rs  (additions)
pub enum ToolPreparationStage { Resolve, Validate, Classify, Policy }

pub enum ToolPreparationDisposition {
    Executable {
        identity: ToolIdentity,
        arguments_digest: String,
        effect: EffectDescriptor,
        replay: ReplayPolicy,
        policy: PolicyDecision,          // Allow only
    },
    NoEffect {
        failed_at: ToolPreparationStage,
        code: String,
        message: String,
        identity: Option<ToolIdentity>,
        arguments_digest: Option<String>,
        effect: Option<EffectDescriptor>,
        replay: Option<ReplayPolicy>,
        policy: Option<PolicyDecision>,
    },
}

pub enum ToolEffectResult {
    Output { content: Value },
    Error { code: String, message: String },
}
```

Add five variants to `AgentRecordData`. Keep the existing variants unchanged.

```rust
ToolCallPrepared { attempt_id, logical_call_id, assistant_entry_id,
                   source_index, provider_call_id, requested_tool_name,
                   result_entry_id, disposition },
ToolEffectStarted { attempt_id },
ToolEffectCompleted { attempt_id, result },
ToolAttemptRejected { attempt_id, result_entry_id, failed_at },
ToolAttemptInterrupted { attempt_id, reason },
```

`ToolAttemptStarted` keeps its meaning. It marks handling start, not effect start.
`ToolEffectStarted` is the only ambiguity marker.

---

## 11. Step C — reducer rules

The reducer tracks one state per logical call.

```rust
enum LogicalCallState {
    Prepared { disposition, attempt_id, result_entry_id },
    EffectInFlight { attempt_id, replay: ReplayPolicy },
    EffectSettled { attempt_id, result: ToolEffectResult },
    Settled { result_entry_id },
    Intervention,
}
```

Reduction rejects each SPEC §7.6 case. The table maps rule to rejection.

| Durable input | Reducer action |
| --- | --- |
| prepare unknown logical call | reject `UnknownLogicalCall` |
| prepare twice for one attempt | reject `DuplicatePreparation` |
| digest/effect/replay/policy differs across attempts | reject `PreparationConflict` |
| `ToolEffectStarted` after `NoEffect` | reject `EffectAfterNoEffect` |
| result entry ID not the reserved ID | reject `ReservedResultMismatch` |
| `ToolEffectCompleted` with no start | reject `EffectCompletionWithoutStart` |
| `ToolAttemptDenied` after `ToolEffectStarted` | reject `DenialAfterEffect` |
| `ToolAttemptRejected` with `failed_at: Policy` | reject `WrongNoEffectRecord` |
| `ToolAttemptDenied` for Resolve/Validate/Classify | reject `WrongNoEffectRecord` |
| second model-visible result for one call | reject `DuplicateLogicalResult` |
| source index out of order | reject `SourceOrderViolation` |
| any record after `AgentResultCommitted` | reject `RecordAfterTerminal` |

Each rejection is a typed variant. The reducer never guesses a repair.

---

## 12. Walkthrough — allowed call

This sequence shows the ordering that SPEC §7.4 requires.

```text
finalized tool call (source_index = 0)
   │
   ├─ resolve identity in registry               ── fail → tool_not_found
   ├─ validate parsed Value against schema       ── fail → invalid_arguments
   ├─ digest = sha256(canonical(arguments))
   ├─ effect  = tool.effect(&arguments)          ── fail → classification_failed
   ├─ replay  = tool.replay_policy(&arguments)
   ├─ policy  = policy.evaluate(&input)          ── Deny → policy_denied
   │
   ├─ APPEND ATOMIC:  ToolAttemptStarted
   │                  ToolCallPrepared::Executable
   │                  ToolEffectStarted
   ├─ COMMIT                                     ◄── ambiguity boundary
   │
   ├─ tool.execute(request, context)             (exactly once)
   │
   ├─ APPEND + COMMIT: ToolEffectCompleted{ result }
   │
   ├─ APPEND ATOMIC:  model-visible result entry (reserved id)
   │                  ToolAttemptCompleted
   └─ COMMIT
```

The effect starts only after the first commit. This is `V-T007-22`.

The gap between `ToolEffectCompleted` and the result entry is deliberate.
T011 repairs that gap.

---

## 13. Walkthrough — denied call

Denial performs zero effect.

```text
policy.evaluate(...) → Deny { code, message }
   │
   ├─ APPEND ATOMIC:  ToolAttemptStarted
   │                  ToolCallPrepared::NoEffect { failed_at: Policy,
   │                                               policy: Some(Deny), .. }
   │                  model-visible error entry { "code": "policy_denied", .. }
   │                  ToolAttemptDenied
   └─ COMMIT
```

No `ToolEffectStarted` appears. No `ToolEffectCompleted` appears.

Resolve, Validate and Classify failures use the same shape. They append
`ToolAttemptRejected` instead of `ToolAttemptDenied`. This is `V-T007-31`.

| Failure | `failed_at` | code | final record |
| --- | --- | --- | --- |
| unknown tool | `Resolve` | `tool_not_found` | `ToolAttemptRejected` |
| bad arguments | `Validate` | `invalid_arguments` | `ToolAttemptRejected` |
| effect error | `Classify` | `classification_failed` | `ToolAttemptRejected` |
| policy deny | `Policy` | `policy_denied` | `ToolAttemptDenied` |

---

## 14. Walkthrough — ambiguity and restart

Restart reads durable truth only. It never reads process memory.

```text
last durable record for the attempt?
   │
   ├─ ToolAttemptStarted, no ToolEffectStarted
   │      → no effect boundary crossed
   │      → preparation may run again
   │
   ├─ ToolEffectStarted, no ToolEffectCompleted     ◄── AMBIGUITY
   │      ├─ replay == Safe  → append ToolAttemptInterrupted, new attempt allowed
   │      └─ replay == Never → append ToolAttemptIntervention
   │                           → terminal RequiresIntervention
   │
   └─ ToolEffectCompleted, no result entry
          → repair: append reserved result entry (T011)
```

T007 supplies the vocabulary and the reducer rules. T011 proves the restart behaviour.

T007 tests build the histories by hand. No kernel exists yet.

---

## 15. CI boundary extension

`ww-agent-tools` needs a boundary guard. Add one grep to `ci.yml`.

```bash
# ww-agent-tools owns no runtime, storage, transport, or capability dependency
! grep -R -n -E 'ww-agent-core|ww-runtime|ww-store|rusqlite|reqwest|ww-flow|OWS|std::fs|std::process|std::net' \
  crates/ww-agent-tools/Cargo.toml crates/ww-agent-tools/src
```

This proves `V-T007-29` and `V-T007-30` in CI.

Add the grep to the existing block. Do not create a script. D019 removed the script.

---

## 16. Verification map

Each check maps to one test file.

| Checks | Test location |
| --- | --- |
| `V-T007-01..04` identity, registry, order | `crates/ww-agent-tools/tests/registry.rs` |
| `V-T007-05..11` schema profile, no coercion | `crates/ww-agent-tools/tests/schema.rs` |
| `V-T007-12..13` digest stability | `crates/ww-agent-tools/tests/digest.rs` |
| `V-T007-14..18` stage order, policy once, denial | `crates/ww-agent-tools/tests/policy.rs` |
| `V-T007-19..20` fixture behaviour, probe count | `crates/ww-agent-tools/tests/fixtures.rs` |
| `V-T007-21..28`, `31` durable state and reducer | `crates/ww-agent-core/tests/recovery.rs` |
| `V-T007-29..30` boundary | `.github/workflows/ci.yml` |

Order of work follows red-green-refactor per work unit.

```bash
cargo test -p ww-agent-tools --locked
cargo test -p ww-agent-core --test recovery --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

---

## 17. Traps and dead ends

### T1. The digest test can give a false green

`serde_json::Map` is a `BTreeMap` today. Object keys already sort on serialize.
A digest function with no sort passes `V-T007-12` today.
The test proves nothing about the implementation.

Mitigation. Assert the canonical bytes, not only digest equality.

```rust
assert_eq!(
    String::from_utf8(serde_json::to_vec(&canonical_value(&json!({"b":1,"a":2}))).unwrap()).unwrap(),
    r#"{"a":2,"b":1}"#
);
```

A later `preserve_order` feature would break the digest silently. The byte assertion catches it.

### T2. `jsonschema` error text is not a stable contract

`error.to_string()` text can change between crate versions.
A test that asserts the exact message becomes brittle.

Mitigation. Assert the instance path and the violation count. Treat the message as opaque.

### T3. `format` assertion is off by default, but must stay off

SPEC §6.4 disables `format` assertion. Draft 2020-12 treats `format` as annotation only.
Do not enable it for convenience.

### T4. External `$ref` rejection needs its own check

`default-features = false` removes the resolver. A schema with an HTTP `$ref` may then
fail with an obscure build error, or may compile and never resolve.
Neither result is a clear tool-definition error.

Mitigation. Reject external `$ref` explicitly before compile. Section 5 shows the walk.

### T5. `ToolAttemptDenied` has no `failed_at` field

SPEC §7.3 says a policy denial "MUST have `failed_at: Policy`".
The existing `ToolAttemptDenied` record has only `attempt_id` and `result_entry_id`.

Reading. `failed_at` lives in `ToolCallPrepared::NoEffect`, not on `ToolAttemptDenied`.
`V-T007-31` supports this reading.

Do not add a field to `ToolAttemptDenied`. Section 18 raises this as a question.

### T6. Tools types enter the core disk format

`ToolCallPrepared` stores `ToolIdentity`, `EffectDescriptor`, `ReplayPolicy` and
`PolicyDecision`. Those types belong to `ww-agent-tools`.
A change in `ww-agent-tools` then changes the `ww-agent-core` on-disk format.

SPEC §7.2 permits this. D018 finding 4 objected to the same shape for the provider crate.
The delta dossier moved that concern to G010.

Action for T007. Follow the SPEC. Record the coupling. Do not redesign.

### T7. The reserved result entry ID has no producer yet

SPEC §7.1 says the kernel allocates the logical ID and the reserved result ID.
T008 builds the kernel. T007 has no producer.

Consequence. T007 tests build durable histories by hand.
Do not build a kernel helper to make the tests easier. That pulls T008 forward.

### T8. `test.unsafe_once` failpoint is not fully testable in T007

The failpoint sits between the probe and the result commit. The kernel commits results.
T007 has no kernel.

Action. T007 proves "probe runs once per execute" only. T011 proves restart behaviour.

### T9. Do not add a `ww-policy` crate

SPEC §5.1 forbids it. One allow-list policy inside `ww-agent-tools` is enough.

### T10. Do not let the registry own run state

The registry is immutable per run. It holds no attempt, call or entry state.
State belongs to `ww-agent-core`.

---

## 18. Self review and criticism

### What this dry run gets right

The work unit order matches PLAN. Contract comes before machinery.
The dependency direction stays one-way: `core → tools`. No cycle exists.
The snippets add no type that SPEC §6.1 and §7.2 do not name.

### Where this dry run is weak

**The reducer state machine is a sketch.** Section 11 lists rejections as a table.
The real implementation needs an explicit state per logical call plus per attempt.
The `EffectSettled` to `Settled` transition needs care. A wrong shape here costs rework.

**The atomic append boundary is proven.** Sections 12 and 13 say "APPEND ATOMIC".
The existing port supports it. `AgentAppend` carries `entries: Vec<AgentEntry>`,
`records: Vec<AgentRecord>` and `expected_version`. One call commits many entries and
records under one optimistic version. T007 needs no port change.

**Test file split may be too fine.** Five test files in `ww-agent-tools` may be more than
the crate needs. Three files would work: `registry.rs`, `validation.rs`, `fixtures.rs`.
Prefer fewer files unless a file exceeds a few hundred lines.

**The `reject_external_refs` walk is naive.** It checks `$ref` string prefixes only.
A schema could hide a reference in `$id` or in a nested `$defs` base URI.
For two fixture schemas this is enough. For general input it is not.
YAGNI applies. Record the limit.

### Estimated size

| Work unit | New files | Rough lines |
| --- | --- | --- |
| A identity + schema | 3 | 250 |
| B registry, digest, policy, fixtures | 4 | 400 |
| C core records + reducer | 2 edits | 350 |
| tests | 4 | 600 |

Total is near 1600 lines. TASKS calls T007 "Large at Task level". The estimate agrees.

### Confidence

| Area | Confidence |
| --- | --- |
| tools crate contracts | high |
| schema profile and offline pin | high |
| digest | high |
| fixtures | high |
| durable record shapes | medium-high |
| reducer invariants | medium |
| atomic multi-record append | high (port verified) |

### Blocking questions

None. Two items need an answer before Step C. Section 19 lists them.

---

## 19. Open questions raised to the requester

1. **`failed_at` placement for policy denial.** Trap T5. Confirm that `failed_at: Policy`
   lives in `ToolCallPrepared::NoEffect` and that `ToolAttemptDenied` keeps its
   current two fields.

This question does not block Step A or Step B. It affects Step C only.

Resolved. The builder actor identifier is `A004-builder`. The requester confirmed the
value on 2026-09-04. This file and its name use that identifier.
