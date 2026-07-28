# ExecLocus Product Specification

- Status: Draft for v0.1
- Product name: ExecLocus
- CLI command: `execlocus`
- Tagline: `See where your agent actually executes.`
- Last updated: 2026-07-28

## 1. Product definition

ExecLocus is a read-only CLI that shows where an AI coding agent and its commands actually execute across Windows, WSL, filesystems, shells, and toolchains.

It does not merely print environment variables. It builds an evidence-backed execution topology, distinguishes observed facts from inference, and explains important cross-layer conflicts.

### One-sentence promise

> ExecLocus tells Windows and WSL users where their coding agent actually runs, which toolchain it resolves, and which filesystem boundary it crosses—within one screen.

### Three-second outcome

The first screen must answer these questions in this order:

1. Where is ExecLocus running now?
2. Which agent was observed or inferred?
3. Where is the project stored?
4. Which OS layer supplies Git, Node, and the agent executable?
5. Is there a conflict that needs attention?

## 2. Target users

Primary users:

- Developers using Windows 11 and WSL2 together
- Codex Desktop or Codex CLI users
- Claude Code users
- Developers sharing projects between Windows applications and Linux tooling
- Developers unsure whether commands resolve to Windows or Linux executables

Initial user problem:

> “The terminal looks like WSL, but I cannot prove whether the agent, Git, Node, configuration, and project files are all using the same runtime layer.”

## 3. Product position

ExecLocus is not a general WSL doctor and not a system-information clone.

| Existing category | Typical answer | ExecLocus difference |
|---|---|---|
| System information | What machine and OS exist? | Which layers participate in this execution? |
| WSL doctor | Is WSL configured correctly? | Are agent, toolchain, config, and project crossing layers? |
| Agent doctor | Is one agent installed correctly? | How do multiple agents and OS layers resolve together? |
| Environment dump | Which variables are set? | Which observed evidence supports each conclusion? |

## 4. Terminology

These terms must remain separate in code and output.

### Visible terminal

The terminal application or shell UI visible to the user, such as Windows Terminal, PowerShell, cmd, bash, or zsh. A visible terminal is not proof of the agent executor.

### ExecLocus runtime

The OS layer in which the current `execlocus` process is executing. Examples: Windows native, WSL2 Ubuntu, Linux native, or unknown.

### Agent runtime

The OS layer in which a detected Codex or Claude process executes. It may only be reported as observed when process, parent-process, adapter, or invocation evidence supports it.

### Project location

The filesystem class of the current project:

- Windows-native path, such as `C:\Users\me\project`
- Windows filesystem mounted into WSL, such as `/mnt/c/Users/me/project`
- WSL-native filesystem, such as `/home/me/project`
- WSL UNC path visible from Windows, such as `\\wsl.localhost\Ubuntu\home\me\project`
- Unknown or remote

### Tool origin

The OS layer of the resolved executable, determined from the resolved path and executable format where possible. Examples: Linux ELF, Windows PE/`.exe`, shell shim, or unknown.

### Fact

A directly observed value with a named probe and evidence source.

### Inference

A conclusion derived from one or more facts. Inference must include a confidence level and may never be rendered as an unqualified fact.

## 5. v0.1 scope

### Supported environments

- Windows 11
- WSL2
- Ubuntu on WSL2 as the primary verified distribution
- PowerShell, cmd, bash, and zsh
- Windows-native project paths
- `/mnt/c` project paths
- WSL-native project paths

### Supported agents

- Codex CLI
- Claude Code
- Codex Desktop only where evidence is available from the current invocation or process relationship

Agent detection is best-effort. Absence of evidence must produce `Unknown`, not `Not installed` or a guessed runtime.

### Supported executable probes

- Agent executable
- Git
- Node.js
- npm
- Shell

### Supported outputs

- Human-readable terminal view
- JSON report
- Markdown report
- Redacted shareable report

## 6. Non-goals for v0.1

- Automatic repair or configuration changes
- GUI or background daemon
- Generic hardware, GPU, network, or Docker diagnostics
- Secret scanning
- Reading tokens or secret environment-variable values
- Supporting every WSL distribution
- Supporting every AI coding agent
- Proving information that the operating system does not expose
- Treating `/mnt/c` as inherently wrong

## 7. Core user experience

### Commands

```text
execlocus
execlocus check
execlocus report --format json
execlocus report --format markdown
execlocus report --format markdown --redact
execlocus explain ENV002
execlocus --version
execlocus --help
```

`execlocus` with no arguments is the primary path. It must complete without configuration and print the topology plus the highest-priority findings.

### Example terminal output

```text
ExecLocus
See where your agent actually executes.

CURRENT EXECUTION
  Runtime       WSL2 / Ubuntu 24.04            observed
  User          dev                            observed
  Shell         /usr/bin/bash                  observed
  Project       /mnt/c/Users/dev/project       observed · Windows-mounted

AGENT
  Product       Claude Code                    inferred · high confidence
  Executable    /usr/local/bin/claude          observed · Linux
  Config        /home/dev/.claude              observed · redacted when shared

TOOLCHAIN
  Git           /usr/bin/git                   Linux
  Node          /mnt/c/Program Files/node.exe  Windows

1 finding
  ENV002  WSL execution resolves Windows Node                   warning

Run `execlocus explain ENV002` for evidence and suggested actions.
```

The real renderer may use color and box drawing, but meaning must remain understandable without color.

## 8. Observation model

Every displayed value must have provenance.

```text
ObservedValue<T>
  value: T | null
  status: observed | inferred | unavailable | failed
  confidence: certain | high | medium | low | none
  source: probe identifier
  evidence_ids: list
  observed_at: timestamp
  error_code: optional stable code
```

### Evidence

```text
Evidence
  id: stable report-local identifier
  probe: probe identifier and version
  kind: process | path | executable | filesystem | environment | adapter
  claim: normalized fact
  value: redacted-safe normalized value
  sensitive: boolean
```

Raw command output should not be stored by default. When a probe fails, record a normalized failure code rather than unrestricted stderr whenever possible.

### Confidence

| Level | Meaning |
|---|---|
| certain | Direct OS-level observation with no meaningful ambiguity |
| high | Multiple independent facts support one conclusion |
| medium | One reliable but indirect fact supports the conclusion |
| low | Heuristic only; never eligible for warning/error by itself |
| none | No conclusion can be drawn |

## 9. Runtime topology model

The internal model must support these nodes:

- Terminal
- Shell
- ExecLocus process
- Agent process or installation
- Project filesystem
- Configuration location
- Resolved executable
- Host OS
- WSL distribution

And these relationships:

- launched-by
- executes-in
- resolves-to
- stored-on
- mounted-from
- reads-config-from
- visible-from

The terminal renderer may simplify the graph, but JSON output must retain explicit nodes and relationships.

## 10. Filesystem profiles

The user intent controls `/mnt/c` evaluation.

| Profile | User priority | Expected recommendation |
|---|---|---|
| `share-first` | Windows apps, Explorer, Codex/Cowork interoperability | Allow `/mnt/c`; explain I/O, permission, and symlink tradeoffs |
| `linux-first` | Linux performance and tool compatibility | Prefer WSL-native project and build paths |
| `balanced` | Shared source with Linux-native heavy artifacts | Allow source on `/mnt/c`; suggest WSL-native cache/build directories |

Default profile for v0.1: `balanced`.

The profile must change severity or advice, not rewrite observed facts.

## 11. Privacy and safety

ExecLocus v0.1 is read-only and local-only.

It must not:

- Modify PATH, shell profiles, WSL configuration, agent configuration, or project files
- Read or print authentication tokens
- Collect environment-variable values unless explicitly allowlisted and demonstrably non-secret
- Transmit reports or telemetry
- Include username, home directory, machine name, or absolute personal paths in redacted output

Redaction must occur before rendering and serialization. Redaction after generating an unrestricted report is not sufficient.

## 12. Error handling

A failed optional probe must not prevent the rest of the report.

Exit codes:

| Code | Meaning |
|---:|---|
| 0 | Report produced; no error-severity finding |
| 1 | Report produced; at least one error-severity finding |
| 2 | Invalid CLI usage or report could not be produced |

Warnings do not change the default exit code. A future `--fail-on warning` option may alter this behavior.

## 13. Performance targets

- First terminal output: under 1 second on a typical local environment
- Complete default scan: under 2 seconds
- No network access during default execution
- No probe may wait indefinitely
- Slow cross-layer probes require explicit timeouts and partial-result behavior

## 14. Accessibility and portability

- Output remains understandable without ANSI color
- `NO_COLOR` is respected
- JSON keys and rule IDs remain English and stable
- Paths are serialized with their original form plus a normalized classification
- Windows and Linux binaries produce the same JSON schema

## 15. v0.1 acceptance criteria

The v0.1 milestone is complete when:

1. The same command runs on Windows 11 and inside WSL2
2. Runtime, user, shell, current project, Git, and Node are represented with evidence
3. Windows PE executables and Linux ELF executables are not confused
4. `/mnt/c` is classified without automatically being reported as an error
5. At least ENV002, PATH001, and GIT001 work against fixtures
6. Terminal and JSON renderers describe the same underlying model
7. Redacted Markdown output passes golden tests for personal information
8. Probe failures return partial output rather than a panic

## 16. Success measures

Pre-release validation gate:

- 10 sanitized real-world environment reports
- Reports from Windows-native, WSL2 `/mnt/c`, and WSL-native projects
- At least three previously unclear runtime or executable differences discovered
- At least 90% of submitted outcomes reproducible from the supplied evidence
- Zero confirmed privacy leaks from shared reports

Post-release adoption signals:

- Repeat use after the first diagnostic run
- Reproducible issues from external users
- External documentation or rule contributions
- Downloads and stars, treated as awareness signals rather than release gates

Quality signals:

- False-positive rate below 5% for warning/error rules
- No confirmed secret leakage from redacted reports
- Median default execution under 2 seconds

## 17. Implementation direction

Rust is the implementation language for the initial CLI.

```text
src/
  core/        normalized model, evidence, confidence, findings
  probes/      Windows, WSL, process, path, filesystem, executable
  adapters/    Codex and Claude detection
  rules/       deterministic evaluation
  renderers/   terminal, JSON, Markdown
  privacy/     redaction
```

Adapters may infer product identity, but only probes establish OS and filesystem facts. Rules consume normalized facts and may not execute arbitrary probes themselves.
