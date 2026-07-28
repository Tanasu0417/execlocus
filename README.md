# ExecLocus

> See where your agent actually executes.

ExecLocus is a read-only CLI that maps the runtime behind AI coding agents across Windows, WSL, shells, filesystems, and toolchains.

It answers a deceptively simple question:

> Is the agent using Windows, WSL, or a mixture of both—and what evidence proves it?

> **Project status:** pre-alpha prototype. Runtime, path, executable-origin, terminal, JSON, and initial rule foundations are implemented; no release is available yet.

## What the first screen will show

```text
ExecLocus
See where your agent actually executes.

CURRENT EXECUTION
  Runtime       WSL2 / Ubuntu 24.04            observed
  Shell         /usr/bin/bash                  environment hint
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
| Linux layer | WSL2, with Ubuntu as the primary verified distribution |
| Shells | PowerShell, cmd, bash, zsh |
| Agents | Codex CLI, Claude Code; evidence-limited Codex Desktop detection |
| Tools | Agent executable, Git, Node.js, npm, shell |
| Output | Terminal, JSON, Markdown, redacted Markdown |

## Safety and privacy

ExecLocus v0.1 is designed to be:

- **Read-only** — no PATH, profile, WSL, agent, or project changes
- **Local-only** — no network calls during normal execution
- **Secret-avoiding** — no token or unrestricted environment-variable collection
- **Shareable** — usernames, home paths, machine names, and personal absolute paths are removed from redacted reports

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

- [Product specification](PRODUCT_SPEC.md)
- [Diagnostic rule specification](RULES.md)
- [MVP scope](docs/MVP_SCOPE.md)
- [Contributing guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

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

## Contributing

The project is still defining its first implementation. A new diagnostic rule must include a reproducible failure or material tradeoff, minimum evidence, a legitimate non-triggering setup, and fixtures.

See [RULES.md](RULES.md) for the contribution requirements that will govern rule additions.

## License

ExecLocus is licensed under the [MIT License](LICENSE).
