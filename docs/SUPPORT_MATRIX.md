# ExecLocus support matrix

- Snapshot: 2026-07-28
- Project status: pre-alpha; no public release is available

This is the source of truth for current capability claims. “Planned” means design intent, not working support.

## Status legend

| Status | Meaning |
|---|---|
| Implemented | Present in the current source and covered by automated tests |
| Verified | Implemented and exercised in the named real or CI environment |
| Partial | Useful behavior exists, but the public v0.1 contract is incomplete |
| Planned | Not yet supported and must not be advertised as available |

## Environment coverage

| Environment | Status | Current evidence | Limitation |
|---|---|---|---|
| Windows 11, x86_64 MSVC | Verified | GitHub Actions and local development test | Public binary is not released |
| WSL2, Ubuntu 24.04 | Verified | Local WSL test and Ubuntu CI coverage | Other distributions are not yet verified |
| Linux native, x86_64 | Partial | Ubuntu CI exercises the Linux code path | Product positioning and real-world testing focus on Windows/WSL |
| PowerShell and cmd | Partial | Runtime/shell hints and Windows test path exist | Shell-specific agent invocation evidence is incomplete |
| bash | Partial | WSL/Linux execution path exists | Agent adapter evidence is incomplete |
| zsh | Planned | Included in the v0.1 design | No dedicated verification yet |
| macOS | Planned beyond v0.1 | Generic code may compile | No support guarantee or CI |
| ARM | Planned beyond v0.1 | None | No release or verification target |

## Capability coverage

| Capability | Status | Notes |
|---|---|---|
| Current Windows, WSL, or Linux runtime | Implemented | WSL detection uses OS evidence rather than terminal appearance alone |
| WSL distribution, user, and shell hints | Partial | Available evidence is reported; provenance granularity will improve |
| Project filesystem classification | Implemented | Windows-native, Windows-mounted, WSL-native, WSL UNC, and Linux-native shapes are represented |
| Git, Node, and npm resolution | Implemented | Selected executable and observed candidates are collected |
| PE, ELF, and script classification | Implemented | Unknown remains a valid outcome |
| `ENV002`, `PATH001`, and `GIT001` | Implemented | Deterministic rules use the normalized report |
| Terminal output | Implemented | Human-readable pre-alpha output |
| JSON output | Implemented | Schema is pre-alpha and not frozen for compatibility yet |
| Codex CLI adapter | Planned | Installation presence alone will not prove runtime |
| Claude Code adapter | Planned | Must use invocation/process evidence where available |
| Codex Desktop detection | Planned, evidence-limited | Will report unknown when the OS does not expose sufficient evidence |
| Markdown report | Planned | Required before shareable field reports become a product feature |
| Redaction-before-rendering | Planned | Required before v0.1.0; manual sanitization is currently mandatory |
| `explain <RULE_ID>` | Planned | Suggestions will remain read-only |
| Release binaries and checksums | Planned | Required for Windows x86_64 and Linux x86_64 |
| Automatic fixes | Out of scope | ExecLocus does not rewrite PATH or configuration |
| Telemetry or hosted upload | Out of scope | Normal execution remains local-only |

## Claim policy

- README examples marked as planned or illustrative are not user testimonials.
- A platform becomes “Verified” only after automated checks and a recorded real-environment run where appropriate.
- A shareable-report claim requires redaction golden tests before publication.
- Performance is not claimed without a documented measurement on the relevant filesystem and workload.
- When implementation changes, this matrix is updated in the same pull request.
