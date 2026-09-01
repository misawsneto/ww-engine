# Markdown record authoring

Markdown is an authoring surface, not a serialized relational database.

## Rules

- Keep semantic information that an agent needs to understand, decide, or act.
- Infer ownership and grouping from directory location and nesting when possible.
- Embed local relationships: Success Criteria and Requirements in Goal; Acceptance Criteria, Gates, and Outcome in Task; Gates and Outcome in Chore; Resolution in Question; Evaluation Runs in Evaluation; Findings in Review.
- Keep explicit references only when the relationship carries information that locality cannot infer, such as Dependencies, Releases, Blocks Completion Of, Required For Closure Of, Supports, or Disposed By.
- Use meaningful record names or headings for references. Do not introduce opaque persistence IDs for Markdown bookkeeping.
- Omit optional sections when they contain no information. Do not write `none`, `N/A`, or empty placeholder collections into live records.
- Do not add `RecordHeader`, `id`, `revision`, generic `Relations`, or persistence-oriented Relation/Event serialization.
- Do not manually reproduce large Domain unions. Use canonical semantic categories such as `Addressable`, `Checkable`, and `Releasable` where a type boundary matters.
- Derived facts are not editable fields.
- Keep examples outside live record structure; do not place concrete example records beside template placeholders at the same heading level.

## Goal-local layout

```text
goals/<goalid>/
├── GOAL.md
├── SPECIFICATION.md   # optional
├── PLAN.md            # optional
├── TASKS.md
├── DECISIONS.md
├── QUESTIONS.md
├── EVALUATIONS.md
└── REVIEWS.md
```

The enclosing Goal directory establishes Goal-local context. Do not repeat the Goal reference in sibling Plan, Task, Decision, Question, Evaluation, or Review records unless the relationship itself carries additional meaning.

## Workspace-level records

`ACTORS.md` and `CHORES.md` normally live at workspace level. Chores are standalone work outside Goal debt; do not place them under a Goal merely to simplify navigation.
