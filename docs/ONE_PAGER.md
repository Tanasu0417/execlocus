# ExecLocus in one page

> See what your agent context resolves—and why.

ExecLocus is a local, read-only CLI for developers who use AI coding agents across Windows and WSL. It explains the current execution context, which executable would be selected, where the project is stored, and the evidence behind each conclusion.

**Status:** pre-alpha source prototype. No public binary is available yet. Exact Codex/Claude ancestor-process evidence can identify the agent family and runtime; current-context command resolution alone does not prove what an agent previously executed.

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
- Public reports: Markdown is automatically redacted and JSON requires `--redact`; raw terminal and raw JSON output remain local-only.

## Current and planned

| Available in source | Required before v0.1 |
|---|---|
| Runtime, filesystem, and conservative Codex/Claude process adapters | Broader invocation/wrapper evidence without collecting command lines |
| PowerShell/cmd/bash/zsh contracts plus selected, losing, and unproven external candidates | Real cmd/zsh validation without reconstructing private parent-session state |
| Terminal, pre-alpha JSON, and automatically redacted Markdown | Signed/checksummed Windows and Linux release artifacts |
| All eight v0.1 rules, `explain`, and privacy golden tests | Real demo and external prototype validation |

See the [support matrix](SUPPORT_MATRIX.md) for the current source of truth and the [alternatives analysis](research/ALTERNATIVES.md) for how this differs from manual commands.
