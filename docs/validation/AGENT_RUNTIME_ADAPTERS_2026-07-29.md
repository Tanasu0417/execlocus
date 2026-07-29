# Agent runtime adapter validation — 2026-07-29

## Scope

This record validates the first read-only Codex and Claude Code process adapters. It contains categorical results only: no username, machine name, home directory, process ID, command line, or absolute project path is recorded.

The public command names are grounded in the official [Codex CLI documentation](https://developers.openai.com/codex/cli) and [Claude Code installation documentation](https://code.claude.com/docs/en/installation). Those documents establish the user-facing command names; they do not guarantee every internal OS process shape. ExecLocus therefore treats an exact process-name match as a high-confidence product inference, not as certain product identity.

## Detection contract

- Inspect at most 24 records in the local parent-process chain.
- Skip the first record because it is the current ExecLocus process.
- Match only `codex`, `codex.exe`, `claude`, or `claude.exe`, ASCII-case-insensitively.
- Prefer the nearest supported ancestor if both names occur.
- Treat installation presence, `node` alone, prefixes, suffixes, and Desktop-like names as insufficient.
- Reuse the already observed ExecLocus OS layer only after a supported ancestor is found.
- Collect process names, parent IDs, and the OS user ID only. Do not collect command lines, process environments, executable paths, working directories, agent configuration, or tokens.

## Automated results

The Rust 1.85 suite passed on both Windows and Ubuntu-24.04 WSL:

| Contract | Result |
|---|---|
| Exact Codex Windows name, including case and `.exe` | Pass |
| Exact Claude Code Linux/WSL name | Pass |
| Nearest supported ancestor wins | Pass |
| `node`, helper, Desktop-like, and prefixed names remain `Unknown` | Pass |
| Renaming the current ExecLocus executable cannot self-identify an agent | Pass |
| Missing process evidence returns `Unknown` without panic | Pass |
| Shareable Markdown privacy golden file | Pass |
| Product inference can coexist with an unavailable runtime without guessing | Pass |
| Process names beyond the depth limit are ignored | Pass |
| Full test suite | 41 passed on Windows; 41 passed on WSL |

## Sanitized real-environment results

| Execution | Product result | Runtime result | Optional failures | Privacy check |
|---|---|---|---:|---|
| Windows validation launched from the active Codex environment | `Codex`, inferred, high confidence, process ancestry | Windows native, observed | 0 | Redacted JSON contained none of the checked user, home, or machine values |
| Ubuntu-24.04 WSL launched as a separate validation command | `Unknown` | Agent runtime `Unknown`; ExecLocus runtime remained observed WSL | 0 | Only redacted JSON was retained |

The WSL `Unknown` result is expected: launching a separate WSL command from Windows does not create a supported Codex or Claude ancestor inside the Linux process chain. The adapter does not copy the visible Windows caller assumption across the OS boundary.

## Known limits

- A `codex` ancestor establishes the Codex process family but cannot distinguish Codex CLI from a Codex Desktop backend with the same visible name.
- A package-manager wrapper that exposes only `node` remains `Unknown`; reading the command line solely to force a match is intentionally out of scope for this privacy-first adapter.
- Real Claude Code ancestry and a real agent-launched WSL session are not yet verified.
- Absence of process evidence means `Unknown`, not “not installed”.
