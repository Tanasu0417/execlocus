# ExecLocus v0.1 MVP Scope

- Status: Approved implementation target after naming
- Last updated: 2026-07-28

## 1. MVP outcome

The MVP is successful when a user can run one local binary on Windows or inside WSL and receive an evidence-backed answer about:

1. The current execution layer
2. The project filesystem layer
3. The origin of Git and Node
4. Any proven cross-layer conflict

Agent-specific detection enriches the topology but must not block the base report.

## 2. Deliverables

### Required for the technical prototype

- Rust workspace and CLI binary named `execlocus`
- Normalized topology, evidence, confidence, and finding types
- Windows and Linux/WSL runtime detection
- User, shell, working-directory, and filesystem classification
- Git and Node executable resolution
- Windows PE versus Linux ELF classification where readable
- Deterministic rule engine
- `ENV002`, `PATH001`, and `GIT001`
- Terminal renderer
- JSON renderer
- Fixture-based tests for Windows and WSL scenarios
- GitHub Actions for Windows and Linux

### Required for v0.1.0 release

- Codex CLI and Claude Code adapters
- `ENV001`, `ENV003`, `ENV004`, `FS001`, and `FS002`
- Markdown renderer
- Redacted reports
- `explain` command
- `share-first`, `balanced`, and `linux-first` profiles
- Release binaries and checksums for Windows x86_64 and Linux x86_64
- English and Japanese README verified against the real CLI

## 3. Explicitly outside v0.1

- Automatic fixes
- GUI or TUI
- Daemon or continuous monitoring
- Telemetry or hosted service
- Docker, GPU, network, package-manager, or general WSL health checks
- MCP server
- Plugin marketplace
- Third-party agent adapter SDK
- macOS release guarantee
- ARM release guarantee
- Reading undocumented secret-bearing agent databases

## 4. CLI contract

### Prototype

```text
execlocus
execlocus check
execlocus report --format json
```

### v0.1.0

```text
execlocus [--profile balanced]
execlocus check [--profile balanced]
execlocus explain <RULE_ID>
execlocus report --format <json|markdown> [--redact]
```

Global behavior:

- `--help` and `--version` use conventional exit code 0
- Invalid arguments use exit code 2
- Default execution performs no network request
- A failed optional probe produces partial output
- Human output goes to stdout; actionable process errors go to stderr

## 5. Normalized data contract

The first internal schema must represent:

```text
Report
  schema_version
  generated_at
  profile
  runtime
  terminal
  shell
  project
  agent
  executables[]
  topology.nodes[]
  topology.edges[]
  evidence[]
  findings[]
  probe_failures[]
```

The renderer does not re-probe the system. Terminal and JSON output consume the same report object.

## 6. Probe order

1. Current process and operating system
2. WSL identity and distribution metadata
3. User, shell, and working directory
4. Project path and filesystem classification
5. Executable resolution for Git and Node
6. Executable format/origin classification
7. Agent adapters
8. Rule evaluation
9. Redaction
10. Rendering

Probe results are independent where possible so a later failure does not discard earlier facts.

## 7. Platform behavior

### Windows binary

Must identify:

- Windows-native execution
- PowerShell or cmd where observable
- Windows path classification
- Available `wsl.exe` distributions without requiring one to be running
- Git and Node resolution on the Windows side

Crossing into WSL for additional evidence is allowed only with a short timeout and clear provenance.

### WSL/Linux binary

Must identify:

- WSL versus non-WSL Linux
- Distribution name and version where available
- Linux user and shell
- `/mnt/<drive>` versus WSL-native paths
- Windows executable interoperability
- Git and Node resolution on the active PATH

Crossing into Windows for additional evidence is allowed only with a short timeout and clear provenance.

## 8. Prototype rule set

| Rule | Prototype requirement |
|---|---|
| `ENV002` | Required |
| `PATH001` | Required |
| `GIT001` | Required |
| `FS001` | Implement data needed; rule may follow in release phase |
| Other rules | Release phase |

The prototype prioritizes rules whose evidence can be obtained without agent-private files.

## 9. Test matrix

| Fixture | Runtime | Project | Git | Node | Expected |
|---|---|---|---|---|---|
| W1 | Windows | Windows | Windows | Windows | no cross-layer finding |
| L1 | Linux | Linux | Linux | Linux | no cross-layer finding |
| WSL1 | WSL | `/mnt/c` | Linux | Linux | profile-aware filesystem fact |
| WSL2 | WSL | `/mnt/c` | Linux | Windows | ENV002 for Node |
| WSL3 | WSL | WSL-native | Windows | Linux | GIT001 |
| WSL4 | WSL | any | Windows selected, Linux available | Linux | PATH001 |
| P1 | any | personal path | any | any | redacted output contains no identity |
| F1 | any | any | probe failure | available | partial report, no panic |

## 10. Milestones

### M0 — Repository foundation

- Cargo project
- Module structure
- CLI parser
- Formatting, linting, unit-test commands
- Windows and Linux CI

### M1 — Observation model

- Core types
- Evidence and confidence
- Runtime and path probes
- JSON fixture format

### M2 — Topology and rules

- Executable origin
- Topology edges
- Rule engine
- ENV002, PATH001, GIT001

### M3 — User-facing output

- Terminal view
- JSON output
- Stable exit codes
- Partial failures

### M4 — Agent and sharing features

- Codex and Claude adapters
- Remaining initial rules
- Profiles
- Markdown and redaction

### M5 — v0.1.0 release

- Real Windows/WSL validation
- English/Japanese README update
- Release binaries and checksums
- Security and privacy review

## 11. Definition of done

The prototype is done when:

- `cargo test` passes on Windows and Linux
- Fixture tests cover W1, L1, WSL1–WSL4, and F1
- The zero-argument command prints a useful report
- `report --format json` represents the same facts and findings
- No production code uses unrestricted shell-string execution for probes
- No rule emits a finding when required evidence is missing
- No panic occurs for missing Git, Node, shell, or WSL metadata

The v0.1.0 release is done only after P1 redaction tests and at least one real Windows plus WSL validation pass.

## 12. Implementation decisions

Decided:

- OSS license: MIT
- Repository name: `execlocus`
- Copyright notice: `Copyright (c) 2026 ExecLocus contributors`
- Declared minimum Rust version: 1.85 (the minimum for Rust 2024 edition)

Still required before publication:

- Initial GitHub owner or organization
- Verify the declared Rust 1.85 minimum in CI
