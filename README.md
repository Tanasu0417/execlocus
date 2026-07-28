# ExecLocus

> See what your agent context resolves—and why.

ExecLocus is a read-only CLI that maps the current Windows/WSL execution context, command resolution, filesystems, and supporting evidence. When invocation or process evidence is available, it can also identify the agent runtime.

It answers a deceptively simple question:

> In this context, would the command resolve to Windows, WSL, or a mixture of both—and what evidence supports that result?

> **Project status:** pre-alpha prototype. Runtime, path, executable-origin, terminal, JSON, and initial rule foundations are implemented; no release is available yet.

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
  Product       Claude Code                    inferred · high confidence
  Executable    /usr/local/bin/claude          observed · Linux

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
execlocus report --format markdown --redact
```

The zero-argument command is the primary workflow. No configuration should be required for the first useful result.

## Run the prototype from source

```console
cargo run --
cargo run -- check
cargo run -- report --format json
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

## Initial diagnostic rules

| Rule | Detects |
|---|---|
| `ENV001` | Visible terminal and observed agent runtime differ |
| `ENV002` | WSL resolves a Windows executable for a core tool |
| `ENV003` | The same agent is installed in Windows and WSL |
| `ENV004` | Agent state or configuration crosses OS layers |
| `FS001` | A WSL workflow uses a Windows-mounted path |
| `FS002` | A share-first workflow uses a WSL-native project |
| `PATH001` | PATH chooses a cross-layer executable over a native candidate |
| `GIT001` | Git and the project use incompatible OS layers |

Rules are read-only. Suggestions explain options but never modify the machine.

## Planned v0.1 support

| Area | Initial support |
|---|---|
| Host | Windows 11 |
| Linux layer | WSL2, with Ubuntu 24.04 as the primary validation target |
| Shells | PowerShell, cmd, bash, zsh |
| Agents | Codex CLI, Claude Code; evidence-limited Codex Desktop detection |
| Tools | Agent executable, Git, Node.js, npm, shell |
| Output | Terminal, JSON, Markdown, redacted Markdown |

## Safety and privacy

ExecLocus v0.1 is designed to be:

- **Read-only** — no PATH, profile, WSL, agent, or project changes
- **Local-only** — no network calls during normal execution
- **Secret-avoiding** — no token or unrestricted environment-variable collection
- **Shareable by design** — planned redacted reports remove usernames, home paths, machine names, and personal absolute paths before serialization

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

- [Product specification](https://github.com/Tanasu0417/execlocus/blob/main/PRODUCT_SPEC.md)
- [Diagnostic rule specification](https://github.com/Tanasu0417/execlocus/blob/main/RULES.md)
- [MVP scope](https://github.com/Tanasu0417/execlocus/blob/main/docs/MVP_SCOPE.md)
- [Initial use-case contracts](https://github.com/Tanasu0417/execlocus/blob/main/docs/USE_CASES.md)
- [Current support matrix](https://github.com/Tanasu0417/execlocus/blob/main/docs/SUPPORT_MATRIX.md)
- [Sanitized runtime identity validation](https://github.com/Tanasu0417/execlocus/blob/main/docs/validation/RUNTIME_IDENTITY_2026-07-29.md)
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
- [ ] Codex and Claude adapters
- [ ] Redacted Markdown reports
- [x] Windows and Linux CI workflow
- [ ] v0.1.0 release artifacts

## Help validate the product hypothesis

Before v0.1.0, the project is seeking categorical feedback from at least ten unique collaborators across ten Windows/WSL environments and at least three confirmed useful cases. A useful case must produce a verified conclusion or decision that was difficult to obtain manually; a difference alone does not count.

Use the [field-report form](https://github.com/Tanasu0417/execlocus/issues/new?template=field_report.yml) if you test the prototype. Automatic redaction is not implemented yet, so the form does not request command output. Do not paste raw diagnostics, credentials, usernames, machine names, personal paths, or private project information into any public Issue.

## Contributing

The project is still defining its first implementation. A new diagnostic rule must include a reproducible failure or material tradeoff, minimum evidence, a legitimate non-triggering setup, and fixtures.

See [RULES.md](https://github.com/Tanasu0417/execlocus/blob/main/RULES.md) for the contribution requirements that will govern rule additions.

## License

ExecLocus is licensed under the [MIT License](https://github.com/Tanasu0417/execlocus/blob/main/LICENSE).
