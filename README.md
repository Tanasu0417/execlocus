# ExecLocus

> See what your agent context resolves—and why.

ExecLocus is a read-only CLI that maps the current Windows/WSL execution context, command resolution, filesystems, and supporting evidence. When invocation or process evidence is available, it can also identify the agent runtime.

It answers a deceptively simple question:

> In this context, would the command resolve to Windows, WSL, or a mixture of both—and what evidence supports that result?

> **Project status:** pre-alpha prototype. Runtime, conservative Codex/Claude evidence adapters, path, executable-origin, terminal, JSON, shareable-report redaction, and initial rule foundations are implemented; no release is available yet.

## What the first screen will show

The following is an illustrative target for v0.1, not captured output from the current prototype:

```text
ExecLocus
See what your agent context resolves—and why.

CURRENT EXECUTION
  Runtime       WSL2 / Ubuntu 24.04            observed
  User          dev                            OS account
  Shell         bash                           process ancestry
  Terminal      Windows Terminal               environment hint
  Project       /mnt/c/Users/dev/project       observed · Windows-mounted

AGENT
  Product       Claude Code                    inferred · high confidence · process ancestry
  Runtime       Wsl                            observed · certain confidence

TOOLCHAIN
  Git           /usr/bin/git                   Linux
  Node          /mnt/c/Program Files/node.exe  Windows

1 finding
  ENV002  WSL execution resolves Windows Node                   warning

Run `execlocus explain ENV002` for evidence and suggested actions.
```

## Why this exists

A terminal that looks like WSL does not prove that every agent command, executable, configuration file, and project path belongs to Linux.

Common mixed-runtime setups include:

- A Linux shell resolving Windows Node.js through PATH
- Windows and WSL installations of the same agent
- Linux Git operating across a Windows-mounted project
- Agent state stored across an OS boundary
- A project intentionally kept on `/mnt/c` for Windows and Cowork interoperability

ExecLocus models those boundaries instead of treating every mixed setup as broken.

## Planned v0.1 interface

```console
execlocus
execlocus check
execlocus explain ENV002
execlocus report --format json
execlocus report --format markdown
execlocus report --format json --redact
```

The zero-argument command is the primary workflow. No configuration should be required for the first useful result.

## Run the prototype from source

```console
cargo run --
cargo run -- check
cargo run -- report --format json
cargo run -- report --format markdown
cargo run -- report --format json --redact
```

Development checks:

```console
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Windows uses the `x86_64-pc-windows-msvc` toolchain and therefore also requires the Microsoft C++ linker and Windows SDK. WSL/Linux requires a C linker such as `cc`.

## `/mnt/c` is a choice, not an error

ExecLocus evaluates filesystem placement according to intent.

| Profile | Priority | `/mnt/c` behavior |
|---|---|---|
| `share-first` | Windows apps, Explorer, Cowork interoperability | Allowed; tradeoffs are explained |
| `linux-first` | Linux compatibility and I/O performance | WSL-native storage is recommended |
| `balanced` | Shared source and Linux-native heavy artifacts | Shared source is allowed; native caches/builds are suggested |

The default profile is `balanced`.

## Evidence, not guesses

Every reported value has a state:

- `observed` — directly measured by a named probe
- `inferred` — derived from evidence and shown with confidence
- `unavailable` — the system did not expose the value
- `failed` — the probe could not complete

Missing evidence never becomes a passed check, and a visible terminal is never treated as proof of the agent runtime.

Current user identity is read from the local OS process snapshot and resolved through the local OS account catalog. The launching shell is selected only when a supported shell appears in the bounded parent-process chain; otherwise the report labels the allowlisted `SHELL` or `ComSpec` value as an environment hint. The process snapshot requests only process names, parent IDs, and user IDs: command lines, process environments, working directories, roots, and executable paths are not requested. WSL detection prioritizes kernel-release evidence; a WSL environment variable without readable kernel evidence is labeled as an inference. Distribution detection uses the WSL registration name first and `/etc/os-release` as the Linux fallback. Normal execution does not invoke a command shell or access the network for these observations.

The agent adapters inspect that same bounded process snapshot. An exact ancestor name of `codex`/`codex.exe` produces a high-confidence `Codex` inference; `claude`/`claude.exe` does the same for `Claude Code`. When a Codex Linux/WSL sandbox hides every agent ancestor behind its PID namespace, a UUID-shaped `CODEX_THREAD_ID` child-process marker provides a medium-confidence fallback. Codex injects that allowlisted marker into tool processes; ExecLocus checks its shape but never stores or renders its value. Process evidence always wins. Similar names, installation presence, arbitrary environment hints, and a wrapper visible only as `node` remain insufficient. The runtime is reported separately from the product inference. This evidence does not distinguish Codex CLI from a Codex Desktop backend that exposes the same process name, and it does not inspect command lines to force that distinction.

## Diagnostic rule coverage

| Rule | Detects | Current status |
|---|---|---|
| `ENV001` | Visible terminal and observed agent runtime differ | Planned for v0.1 |
| `ENV002` | WSL resolves a Windows executable for a core tool | Implemented |
| `ENV003` | The same agent is installed in Windows and WSL | Planned for v0.1 |
| `ENV004` | Agent state or configuration crosses OS layers | Planned for v0.1 |
| `FS001` | A WSL workflow uses a Windows-mounted path | Planned for v0.1 |
| `FS002` | A share-first workflow uses a WSL-native project | Planned for v0.1 |
| `PATH001` | PATH chooses a cross-layer executable over a native candidate | Implemented |
| `GIT001` | Git and the project use incompatible OS layers | Implemented |

Rules are read-only. Suggestions explain options but never modify the machine.

## Planned v0.1 support

| Area | Initial support |
|---|---|
| Host | Windows 11 |
| Linux layer | WSL2, with Ubuntu 24.04 as the primary validation target |
| Shells | PowerShell, cmd, bash, zsh |
| Agents | Codex through exact ancestry or its sandbox child marker; Claude Code through exact ancestry; otherwise `Unknown` |
| Tools | Agent executable, Git, Node.js, npm, shell |
| Output | Terminal, JSON, Markdown, redacted Markdown |

## Safety and privacy

ExecLocus v0.1 is designed to be:

- **Read-only** — no PATH, profile, WSL, agent, or project changes
- **Local-only** — no network calls during normal execution
- **Secret-avoiding** — no token or unrestricted environment-variable collection
- **Shareable by design** — Markdown reports are always redacted; `report --format json --redact` applies the same transformation before JSON serialization

## How it differs from a doctor CLI

ExecLocus does not ask only whether one installation is healthy. It describes relationships:

```text
terminal → shell → agent → executable
                     ↓
                 project filesystem
                     ↓
               Windows / WSL boundary
```

The output is a runtime topology with evidence, not a list of generic setup checks.

## Project documents

- [Zero-incremental-cost development policy](https://github.com/Tanasu0417/execlocus/blob/main/COST_POLICY.md)
- [追加支出0円の開発ポリシー（日本語）](https://github.com/Tanasu0417/execlocus/blob/main/COST_POLICY.ja.md)
- [Product specification](https://github.com/Tanasu0417/execlocus/blob/main/PRODUCT_SPEC.md)
- [Diagnostic rule specification](https://github.com/Tanasu0417/execlocus/blob/main/RULES.md)
- [MVP scope](https://github.com/Tanasu0417/execlocus/blob/main/docs/MVP_SCOPE.md)
- [v0.1 delivery roadmap](https://github.com/Tanasu0417/execlocus/blob/main/docs/V0_1_ROADMAP.md)
- [Initial use-case contracts](https://github.com/Tanasu0417/execlocus/blob/main/docs/USE_CASES.md)
- [Current support matrix](https://github.com/Tanasu0417/execlocus/blob/main/docs/SUPPORT_MATRIX.md)
- [Sanitized runtime identity validation](https://github.com/Tanasu0417/execlocus/blob/main/docs/validation/RUNTIME_IDENTITY_2026-07-29.md)
- [Shareable redaction validation](https://github.com/Tanasu0417/execlocus/blob/main/docs/validation/SHAREABLE_REDACTION_2026-07-29.md)
- [Agent adapter validation](https://github.com/Tanasu0417/execlocus/blob/main/docs/validation/AGENT_RUNTIME_ADAPTERS_2026-07-29.md)
- [Real Claude Code on WSL validation](https://github.com/Tanasu0417/execlocus/blob/main/docs/validation/CLAUDE_CODE_WSL_2026-07-29.md)
- [Windows Claude Code and WSL Codex validation](https://github.com/Tanasu0417/execlocus/blob/main/docs/validation/WINDOWS_CLAUDE_WSL_CODEX_2026-07-29.md)
- [One-page product overview](https://github.com/Tanasu0417/execlocus/blob/main/docs/ONE_PAGER.md)
- [Demo production plan](https://github.com/Tanasu0417/execlocus/blob/main/docs/DEMO_PLAN.md)
- [Demo storyboard and recording scenario](https://github.com/Tanasu0417/execlocus/blob/main/docs/demo/README.md)
- [OSS pattern adoption blueprint](https://github.com/Tanasu0417/execlocus/blob/main/docs/ADOPTION_BLUEPRINT.md)
- [Alternatives and current workarounds](https://github.com/Tanasu0417/execlocus/blob/main/docs/research/ALTERNATIVES.md)
- [OSS benchmark and launch research](https://github.com/Tanasu0417/execlocus/blob/main/docs/research/README.md)
- [X demand-research post strategy](https://github.com/Tanasu0417/execlocus/blob/main/docs/research/X_POST_STRATEGY.md)
- [Changelog](https://github.com/Tanasu0417/execlocus/blob/main/CHANGELOG.md)
- [Contributing guide](https://github.com/Tanasu0417/execlocus/blob/main/CONTRIBUTING.md)
- [Security policy](https://github.com/Tanasu0417/execlocus/blob/main/SECURITY.md)

## Roadmap

- [x] Rust CLI scaffold
- [x] Normalized topology and evidence model
- [x] Windows and WSL runtime probes
- [x] Executable-origin and filesystem classification
- [x] Terminal and JSON renderers
- [x] Initial deterministic rules
- [x] Conservative Codex and Claude process adapters
- [x] Redacted Markdown reports
- [x] Windows and Linux CI workflow
- [ ] Remaining profile-aware and agent-boundary rules
- [ ] `explain <RULE_ID>` and production shell-specific candidate resolution
- [ ] External prototype validation and real demo capture
- [ ] v0.1.0 release artifacts

## Help validate the product hypothesis

Before v0.1.0, the project is seeking categorical feedback from at least ten unique collaborators across ten Windows/WSL environments and at least three confirmed useful cases. A useful case must produce a verified conclusion or decision that was difficult to obtain manually; a difference alone does not count.

Use the [field-report form](https://github.com/Tanasu0417/execlocus/issues/new?template=field_report.yml) if you test the prototype. The form does not request command output yet. If output is needed for a maintainer-reviewed test, generate Markdown or use JSON with `--redact`; never paste raw terminal or raw JSON diagnostics, credentials, or private project information into a public Issue.

## Contributing

The project is still defining its first implementation. A new diagnostic rule must include a reproducible failure or material tradeoff, minimum evidence, a legitimate non-triggering setup, and fixtures.

See [RULES.md](https://github.com/Tanasu0417/execlocus/blob/main/RULES.md) for the contribution requirements that will govern rule additions.

## License

ExecLocus is licensed under the [MIT License](https://github.com/Tanasu0417/execlocus/blob/main/LICENSE).
