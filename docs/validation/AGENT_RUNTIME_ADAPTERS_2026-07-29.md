# Agent runtime adapter validation — 2026-07-29

## Scope

This record validates the first read-only Codex and Claude Code evidence adapters. It contains categorical results only: no username, machine name, home directory, process ID, command line, or absolute project path is recorded.

The public command names are grounded in the official [Codex CLI documentation](https://developers.openai.com/codex/cli) and [Claude Code installation documentation](https://code.claude.com/docs/en/installation). Those documents establish the user-facing command names; they do not guarantee every internal OS process shape. ExecLocus therefore treats an exact process-name match as a high-confidence product inference, not as certain product identity.

## Detection contract

- Inspect at most 24 records in the local parent-process chain.
- Skip the first record because it is the current ExecLocus process.
- Match only `codex`, `codex.exe`, `claude`, or `claude.exe`, ASCII-case-insensitively.
- Prefer the nearest supported ancestor if both names occur.
- Treat installation presence, `node` alone, prefixes, suffixes, and Desktop-like names as insufficient.
- Reuse the already observed ExecLocus OS layer only after a supported ancestor is found.
- Collect process names, parent IDs, and the OS user ID only in the process snapshot. Do not collect command lines, process environments, executable paths, working directories, agent configuration, or tokens.
- When a Codex PID sandbox hides the ancestry, inspect only the allowlisted `CODEX_THREAD_ID` value for UUID shape. Never retain or render that value, and assign lower confidence than process evidence.

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
| UUID-shaped Codex child marker is accepted without retaining its value | Pass |
| Missing or malformed Codex child marker remains `Unknown` | Pass |
| Process ancestry wins over a conflicting marker | Pass |
| Neutral scripts do not trigger cross-layer `PATH001` | Pass |
| Full test suite | 46 passed on Windows; 46 passed on WSL |

## Sanitized real-environment results

| Execution | Product result | Runtime result | Optional failures | Privacy check |
|---|---|---|---:|---|
| Windows validation launched from the active Codex environment | `Codex`, inferred, high confidence, process ancestry | Windows native, observed | 0 | Redacted JSON contained none of the checked user, home, or machine values |
| Ubuntu-24.04 WSL launched as a separate validation command | `Unknown` | Agent runtime `Unknown`; ExecLocus runtime remained observed WSL | 0 | Only redacted JSON was retained |
| Ubuntu-24.04 WSL launched by Claude Code 2.1.212 | `Claude Code`, inferred, high confidence, process ancestry | WSL, observed, certain confidence | 0 | Isolated WSL-native directory; only automatically redacted JSON was exposed |
| Windows-native Claude Code 2.1.212 | `Claude Code`, inferred, high confidence, process ancestry | Windows native, observed, certain confidence | 0 | Generic isolated directory; automatically redacted JSON only |
| Ubuntu-24.04 WSL launched by Codex CLI 0.146.0 | `Codex`, inferred, medium confidence, environment marker | WSL, observed, certain confidence | 0 | PID namespace hid ancestry; marker value was shape-checked and discarded |

The first WSL `Unknown` result is expected: launching a separate WSL command from Windows does not create a supported Codex or Claude ancestor inside the Linux process chain. The later Claude Code run establishes the positive case without copying the visible Windows caller assumption across the OS boundary. Full methodology is recorded in [`CLAUDE_CODE_WSL_2026-07-29.md`](CLAUDE_CODE_WSL_2026-07-29.md).

## Known limits

- A `codex` ancestor establishes the Codex process family but cannot distinguish Codex CLI from a Codex Desktop backend with the same visible name.
- A package-manager wrapper that exposes only `node` remains `Unknown`; reading the command line solely to force a match is intentionally out of scope for this privacy-first adapter.
- A UUID-shaped environment marker is spoofable outside Codex and therefore remains medium-confidence inference rather than certain product identity.
- Absence of both process evidence and the allowlisted marker means `Unknown`, not “not installed”.
