# GitHub and sandbox rules

## Primary rule

Use the simplest working approach that completes the task correctly. Do not add branch, worktree, CI, synchronization, recovery, or publication ceremony unless the task needs it.

## Shared workspace

Use `/mnt/data/ww-engine` as the default mutable workspace when a local checkout is available.

- Reuse the workspace across tasks.
- Edit, build, test, inspect, and run experiments there.
- Do not create worktrees or disposable repository copies by default.
- Treat GitHub as durable publication; local work does not need a remote write after every edit.

## Repository access

Use normal Git first when direct network access works. When the sandbox blocks Git network access, use the GitHub connector/API or a verified repository artifact rather than reconstructing repository history unnecessarily.

## Development loop

1. Inspect the relevant source and active Goal packet.
2. Make the smallest coherent change.
3. Run the smallest sufficient verification.
4. Review the resulting diff/state.
5. Publish only coherent verified work.

## Branch policy (D016)

`main` is the continuously integrated, always-green engineering line. A Goal does not need to be terminally accepted before its intermediate work lands on `main`.

Keep four states distinct; none implies another:

- **Merged to `main`** — the implementation passed the complete engineering gate and is part of the integrated codebase.
- **Task complete** — that Task's declared acceptance evidence is satisfied.
- **Goal accepted/achieved** — the Goal's Verification/Evaluation/Review obligations are complete.
- **Architecture accepted** — governed by ADR state, never by branch placement.

A Goal therefore stays active while its completed Tasks are already on `main`. Long-lived divergent feature branches are not the model; land verified work and keep the Goal records synchronized with what has actually been proven.

## Verification

Local verification is sufficient when it directly establishes the requested behavior. Record genuinely unavailable checks as unverified instead of creating ceremony around them.

Any task-specific or temporary verification path must execute the **complete merge-target CI gate** (D017). It may add checks; it may never replace or omit target-branch checks. Prefer the permanent `ci.yml` over creating a temporary workflow. If a temporary verifier is genuinely needed, it must run at least the full gate below.

The current minimum gate for `main` is:

```bash
cargo fmt --all -- --check
# architecture-boundary checks from .github/workflows/ci.yml
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

This rule exists because a temporary G003 verification workflow omitted the format step that `main` enforces. Three Tasks were recorded verified against a weaker gate than their merge target, and the omission surfaced only at consolidation.

## Publication

Prefer one coherent commit or the simplest equivalent GitHub write. Avoid force pushes, history repair, synthetic synchronization commits, and Git Data plumbing unless repository state actually requires them.

Stale branches are housekeeping. They do not block Goal work and are not worth engineering time.

## Project bookkeeping

- `PROJECT_STATE.md` describes current project state.
- `DECISIONS.md` owns durable project-level architectural direction.
- `QUESTIONS.md` owns material open uncertainty.
- `goals/<goal>/` owns Goal-specific scope, plan, tasks, verification, and reviews.
- `docs/templates/` are authoring supports, not persistence models.
- Canonical WorkWeave Orchestration semantics remain upstream in `misawsneto/ww-orchestration`.
