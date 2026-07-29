# ExecLocus support matrix

- Snapshot: 2026-07-29
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
| Windows 11, x86_64 MSVC | Verified | Windows CI plus sanitized local runtime-identity validation | No public binary is available |
| WSL2, Ubuntu 24.04 | Verified | MSRV 1.85 test suite plus sanitized WSL runtime-identity validation | Validation covers one WSL distribution and no public binary |
| Linux native, x86_64 | Partial | Ubuntu CI exercises the Linux code path | Product positioning and real-world testing focus on Windows/WSL |
| PowerShell and cmd | Partial | PowerShell 7 ancestry and a Windows Codex-family adapter run are verified; cmd resolution has synthetic contract tests | Real cmd ancestry and cmd-launched agent invocation remain unverified |
| bash | Verified | bash ancestry is observed in Ubuntu-24.04 WSL | A real WSL agent ancestor has not yet been exercised |
| zsh | Partial | Identity recognition and resolution contract tests exist | No real zsh validation yet |
| macOS | Planned beyond v0.1 | Generic code may compile | No support guarantee or CI |
| ARM | Planned beyond v0.1 | None | No release or verification target |

## Capability coverage

| Capability | Status | Notes |
|---|---|---|
| Current Windows, WSL, or Linux runtime | Verified | Target-platform and kernel-release provenance are explicit; a WSL environment-only fallback is labeled inferred |
| Distribution, current user, and launching shell | Verified | WSL registration/OS release, OS account, and bounded process ancestry carry explicit provenance; environment values are labeled as hints |
| Project filesystem classification | Implemented | Windows-native, Windows-mounted, WSL-native, WSL UNC, and Linux-native shapes are represented |
| Git, Node, and npm resolution | Implemented | Selected executable and observed candidates are collected |
| PE, ELF, and script classification | Implemented | Unknown remains a valid outcome |
| `ENV002`, `PATH001`, and `GIT001` | Implemented | Deterministic rules use the normalized report |
| Terminal output | Implemented | Human-readable pre-alpha output |
| JSON output | Implemented | Schema `0.3.0` is pre-alpha and not frozen for compatibility yet |
| Codex process adapter | Verified on Windows | Exact `codex`/`codex.exe` ancestor names infer the Codex family; installation presence alone is ignored |
| Claude Code process adapter | Implemented | Exact `claude`/`claude.exe` ancestor names are covered by Windows/WSL-shaped fixtures; a real Claude Code run remains unverified |
| Codex Desktop surface detection | Partial, evidence-limited | A `codex` ancestor proves the process family but does not distinguish CLI from a Desktop backend; insufficient evidence remains `Unknown` |
| Markdown report | Implemented | Shareable Markdown is always redacted before rendering |
| Redaction-before-rendering | Verified | Synthetic golden tests plus sanitized Windows and WSL executions cover username, home, machine, and absolute-path removal |
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

The sanitized Windows/WSL evidence is recorded in [`validation/RUNTIME_IDENTITY_2026-07-29.md`](validation/RUNTIME_IDENTITY_2026-07-29.md).
Shareable output validation is recorded in [`validation/SHAREABLE_REDACTION_2026-07-29.md`](validation/SHAREABLE_REDACTION_2026-07-29.md).
Agent adapter validation is recorded in [`validation/AGENT_RUNTIME_ADAPTERS_2026-07-29.md`](validation/AGENT_RUNTIME_ADAPTERS_2026-07-29.md).
