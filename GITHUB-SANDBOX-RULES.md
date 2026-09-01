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

## Verification

Local verification is sufficient when it directly establishes the requested behavior. CI is not a mandatory second opinion. Record genuinely unavailable checks as unverified instead of creating ceremony around them.

## Publication

Prefer one coherent commit or the simplest equivalent GitHub write. Avoid force pushes, history repair, synthetic synchronization commits, and Git Data plumbing unless repository state actually requires them.

## Project bookkeeping

- `PROJECT_STATE.md` describes current project state.
- `DECISIONS.md` owns durable project-level architectural direction.
- `QUESTIONS.md` owns material open uncertainty.
- `goals/<goal>/` owns Goal-specific scope, plan, tasks, verification, and reviews.
- `docs/templates/` are authoring supports, not persistence models.
- Canonical WorkWeave Orchestration semantics remain upstream in `misawsneto/ww-orchestration`.
