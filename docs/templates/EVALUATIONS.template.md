# Evaluations

## `<Evaluation title>`
- State: `<draft | active | superseded | deprecated>`
- Mode: `<deterministic | judgment>`
- Evaluator Mode: `<deterministic | agent_judgment | human_judgment | independent_judgment>`

### Required For Closure Of
- `<Goal | Task | Chore>`

### Checks
#### `<check key>`
- Covers:
  - `<Checkable subject>`
- Subjects:
  - Selector: `<subject selector>`
    Required: `<false>`
- Criteria:
  - `<criterion>`
- Procedure:
  - `<procedure>`
- Expected: `<expected result>`

### Evaluation Runs
#### `<run label>`
- Basis: `<exact state evaluated>`
- Check Results:
  - `<check key>` — `<pass | fail | inconclusive>` — `<observed result>`
- Evidence:
  - `<Evidence>`

<!-- Omit Required For Closure Of when the Evaluation is not required for closure. EvaluationSubject.required defaults to true; author Required only when false. Evaluation Runs are nested under their Evaluation, so do not repeat an Evaluation reference. Omit run Evidence when absent. Add per-check rationale/evidence only when useful. -->
