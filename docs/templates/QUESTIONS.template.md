# Questions

## `<Question prompt>`

### Subject
- `<Addressable subject>`

### Blocks Completion Of
- `<Goal | Task | Chore>`

### Temporal Constraints
- `<not_before | due_at | expires_at | review_at>` — `<datetime>` — `<reason>`

### Resolution
- Kind: `<answer | work_outcome | decision | inconclusive | obsolete>`
- Statement: `<Immutable terminal semantic closure>`
- Evidence:
  - `<Evidence>`
- Based On:
  - `<Outcome | Decision | EvaluationRun>`

<!-- Omit Subject, Blocks Completion Of, and Temporal Constraints when unnecessary. Author Resolution only once the Question is resolved. Omit Resolution Evidence or Based On when absent. Question closure is derived and is never authored. -->
