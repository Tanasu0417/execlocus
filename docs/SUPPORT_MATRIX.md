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
| PowerShell and cmd | Partial | PowerShell/cmd contracts feed reports when ancestry proves the shell; paired Windows contract tests cover selected and losing external candidates | Parent aliases/functions/macros cannot be reconstructed; real cmd ancestry remains unverified |
| bash | Verified | bash ancestry is observed under Claude Code on Windows and in Ubuntu-24.04 WSL; an incomplete WSL session remains explicitly unknown | Validation covers Git Bash on Windows and one WSL distribution |
| zsh | Partial | Identity recognition and resolution contract tests exist | No real zsh validation yet |
| macOS | Planned beyond v0.1 | Generic code may compile | No support guarantee or CI |
| ARM | Planned beyond v0.1 | None | No release or verification target |

## Capability coverage

| Capability | Status | Notes |
|---|---|---|
| Current Windows, WSL, or Linux runtime | Verified | Target-platform and kernel-release provenance are explicit; a WSL environment-only fallback is labeled inferred |
| Distribution, current user, and launching shell | Verified | WSL registration/OS release, OS account, and bounded process ancestry carry explicit provenance; environment values are labeled as hints |
| Project filesystem classification | Implemented | Windows-native, Windows-mounted, WSL-native, WSL UNC, and Linux-native shapes are represented |
| Git, Node, and npm resolution | Implemented | Shell-contract provenance or a labeled PATH fallback is recorded; terminal output shows selected, losing, or unproven external candidates with origin and evidence IDs |
| PE, ELF, and script classification | Implemented | Scripts remain a neutral origin when their shebang cannot establish an OS layer |
| `ENV002`, `PATH001`, and `GIT001` | Implemented | Deterministic rules use the normalized report; neutral scripts do not trigger cross-layer PATH warnings |
| `FS001`, `FS002`, and profiles | Implemented | Certain filesystem evidence produces profile-specific read-only guidance; `/mnt/c` remains a supported interoperability choice |
| `ENV001` rule evaluation | Partial, evidence-limited | The deterministic rule requires a session layer inferred from process evidence plus high-confidence agent runtime; current local adapters stay silent when a cross-layer launcher hides one side of that relationship |
| `ENV003` and `ENV004` | Implemented | Duplicate agents require certain candidates in both layers; config checks classify the active agent root without reading its contents |
| Terminal output | Implemented | Human-readable pre-alpha output escapes observed control characters before display |
| JSON output | Implemented | Schema `0.5.0` adds resolution method, shell, and session-completeness provenance; it is not frozen yet |
| Codex evidence adapter | Verified on Windows and WSL | Exact ancestry is high confidence; a UUID-shaped Codex-injected child marker is a medium-confidence fallback when the WSL sandbox hides ancestors |
| Claude Code process adapter | Verified on Windows and WSL | Claude Code 2.1.212 launched packaged ExecLocus binaries in both OS layers; exact ancestry produced high-confidence product evidence |
| Codex Desktop surface detection | Partial, evidence-limited | A `codex` ancestor proves the process family but does not distinguish CLI from a Desktop backend; insufficient evidence remains `Unknown` |
| Markdown report | Implemented | Shareable Markdown is always redacted before rendering |
| Redaction-before-rendering | Verified | Synthetic golden tests plus sanitized Windows and WSL executions cover username, home, machine, and absolute-path removal |
| Output-injection safety | Verified | Terminal controls and Markdown table/HTML/image-link syntax are neutralized with regression tests |
| Dependency security policy | Verified | Pinned cargo-deny checks RustSec, yanked releases, licenses, registries/Git sources, wildcards, and duplicates on PRs and weekly |
| Threat model | Implemented | Trust boundaries, controls, release requirements, and accepted v0.1 risks are documented |
| `explain <RULE_ID>` | Implemented | Case-insensitive IDs show current trigger state, rationale, referenced evidence, and read-only suggestions; unknown IDs exit with code 2 |
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
The isolated real Claude Code/WSL run is recorded in [`validation/CLAUDE_CODE_WSL_2026-07-29.md`](validation/CLAUDE_CODE_WSL_2026-07-29.md).
Windows-native Claude Code and WSL-native Codex CLI are recorded in [`validation/WINDOWS_CLAUDE_WSL_CODEX_2026-07-29.md`](validation/WINDOWS_CLAUDE_WSL_CODEX_2026-07-29.md).
Production shell-resolution provenance, paired Windows/WSL scenarios, candidate display, and terminal control escaping are recorded in [`validation/SHELL_RESOLUTION_2026-07-29.md`](validation/SHELL_RESOLUTION_2026-07-29.md).
The free dependency gate, source review, injection/redaction/path tests, GitHub alert snapshot, and accepted risks are recorded in [`validation/SECURITY_ASSESSMENT_2026-07-29.md`](validation/SECURITY_ASSESSMENT_2026-07-29.md) and [`SECURITY_MODEL.md`](SECURITY_MODEL.md).
