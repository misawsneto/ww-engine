# G004 Reviews

No implementation review exists because G004 is proposed and depends on terminal G003 acceptance.

## Planned review focus

- OpenAI protocol normalization and vendor-type containment;
- credential/redaction boundaries;
- `fs.read` canonical path containment, symlink handling, and output bounds;
- SDK projection ownership and absence of raw persistence leakage;
- CLI store-boundary discipline and machine-readable output;
- preservation of G003 recovery/replay semantics;
- absence of Flow/OWS, write/process/network/MCP/plugin, TUI/server/session scope creep.
