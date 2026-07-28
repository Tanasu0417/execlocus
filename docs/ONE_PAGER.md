# ExecLocus in one page

> See what your agent context resolves—and why.

ExecLocus is a local, read-only CLI for developers who use AI coding agents across Windows and WSL. It explains the current execution context, which executable would be selected, where the project is stored, and the evidence behind each conclusion.

**Status:** pre-alpha source prototype. No public binary is available yet. Current-context resolution does not prove what an agent previously executed; that claim requires separate invocation or process evidence.

## The problem

A WSL-looking terminal does not guarantee that Git, Node, npm, the agent executable, its configuration, and the project all belong to Linux. A valid setup can also intentionally combine `/mnt/c` sharing with WSL tooling.

Today, users often combine `which`, `where.exe`, `Get-Command`, PATH inspection, WSL checks, and manual path comparison. The result is fragmented and easy to misinterpret.

## The three questions

1. Where is ExecLocus running, and is there evidence for the agent runtime?
2. Which executable would the current shell context select, which candidates lost, and why?
3. Is `/mnt/c` intentional sharing or a mismatch with the selected workflow profile?

## What the result should contain

| Result | Meaning |
|---|---|
| Runtime and shell | Current ExecLocus runtime is kept separate from terminal appearance and agent-runtime evidence |
| Command resolution | Selected candidate, losing candidates, Windows/Linux/script origin, and evidence |
| Filesystem boundary | Windows-native, Windows-mounted, WSL-native, or WSL UNC classification |
| Finding | Deterministic explanation such as a cross-layer executable selection |
| Evidence state | `observed`, `inferred`, `unavailable`, or `failed` |

## Safety contract

- Read-only: never rewrites PATH, WSL, shell, agent, or project configuration.
- Local-only during normal execution: no diagnostic upload or telemetry.
- Secret-avoiding: no tokens or unrestricted environment-variable collection.
- Public reports: command output must not be posted until redaction-before-rendering is implemented and tested.

## Current and planned

| Available in source | Required before v0.1 |
|---|---|
| Runtime and filesystem classification | Codex and Claude invocation/process adapters |
| Executable-origin and PATH candidate foundation | Shell-accurate resolution contracts and scenario fixtures |
| Terminal and pre-alpha JSON output | Redacted Markdown output and golden privacy tests |
| Initial deterministic rules | Signed/checksummed Windows and Linux release artifacts |

See the [support matrix](SUPPORT_MATRIX.md) for the current source of truth and the [alternatives analysis](research/ALTERNATIVES.md) for how this differs from manual commands.
