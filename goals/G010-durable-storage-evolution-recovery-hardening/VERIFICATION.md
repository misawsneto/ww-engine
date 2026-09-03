# G010 Verification

## Required checks

- [ ] post-G003 scheduling decision is recorded.
- [ ] governing ADR is accepted before activation.
- [ ] `cargo fmt --all -- --check`
- [ ] storage ownership and dependency-boundary checks
- [ ] `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- [ ] `cargo test --workspace --all-features --locked`

### Durable evolution

- [ ] committed known-old schema fixtures upgrade deterministically
- [ ] committed known-old payload fixtures upgrade deterministically
- [ ] repeated migration is idempotent
- [ ] unknown future schema versions fail closed without mutation
- [ ] unknown future payload versions fail closed without mutation
- [ ] migrated reopen reconstructs the current semantic projection

### Ambiguous acknowledgement

- [ ] retry after committed-but-unacknowledged create returns one logical identity
- [ ] retry remains idempotent after process reopen
- [ ] conflicting idempotency-key reuse rejects explicitly

### Storage conformance

- [ ] stable recovery classes cover retryable, conflict, corruption, unsupported-version, and permanent failures
- [ ] participating adapters pass migration, retry, reopen, concurrency, and injected-failure fixtures
- [ ] reusable SQLite mechanics contain no Agent, Flow, or Orchestration DTOs or schemas
- [ ] failure injection leaves no partial component or cross-component mutation

### Architecture boundaries

- [ ] Agent, Flow, and WorkWeave Orchestration semantics are unchanged
- [ ] component schemas, migrations, row mappings, and durable DTOs remain component-owned
- [ ] no provider, tool, SDK, CLI, TUI, filesystem, or network capability enters G010
- [ ] G010 has not become a prerequisite for G004 without a separate evidence-based decision

## Required Evaluations

All checks in `EVALUATIONS.md` required for G010 closure must have current passing EvaluationRuns on the reviewed final code basis.

## Evidence

- To be recorded only if G010 is activated.
