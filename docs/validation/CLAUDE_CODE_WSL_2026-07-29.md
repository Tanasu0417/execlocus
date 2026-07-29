# Real Claude Code on WSL validation — 2026-07-29

## Outcome

Claude Code 2.1.212 launched the release-built ExecLocus binary as a child process inside Ubuntu-24.04 WSL. ExecLocus identified both the agent product and its execution layer from the bounded process ancestry:

| Field | Sanitized result | Provenance |
|---|---|---|
| ExecLocus runtime | `wsl` | observed, certain, kernel release |
| Distribution | `Ubuntu-24.04` | environment hint |
| Launching shell | `bash` | process ancestry |
| Agent product | `claude_code` | inferred, high confidence, process ancestry |
| Agent runtime | `wsl` | observed, certain |
| Project class | `wsl_native` | observed; rendered as `[wsl-project]` |
| Agent topology | `agent.current --runs-in--> runtime.current` | normalized report |
| Findings | 0 | rule evaluation |
| Optional probe failures | 0 | probe status |

This validates the previously synthetic positive Claude Code/WSL case. It also confirms that a separately launched WSL process returning `Unknown` is the correct negative behavior when no supported agent exists in the Linux parent-process chain.

## Privacy-preserving setup

1. The Anthropic installer was downloaded from `https://claude.ai/install.sh` instead of being piped directly to a shell.
2. The 217-line installer was inspected before execution. Its downloaded SHA-256 was `cde4f1702d3b1695f92b73d26888364e17bca476e17f0fd676484c951d36c125`.
3. The installer selected the stable native build, verified the release binary against Anthropic's manifest checksum, and installed without `sudo` into the WSL user's local application directory.
4. ExecLocus was built with Rust 1.85 in release mode. Only the resulting binary was copied into a newly created mode-`0700` directory under `/tmp`; no repository source, Git metadata, private document, or project configuration was copied.
5. Claude Code was restricted to the Bash tool and pre-approved for exactly `./execlocus report --format json --redact`. The run used `dontAsk`, disabled slash commands, an empty strict MCP configuration, disabled prompt history, and disabled session persistence.
6. The prompt requested one command execution and no file read or modification. The only diagnostic content returned to Claude was ExecLocus's automatically redacted JSON.
7. The temporary installer, isolated binary, and validation directory were removed after the run. Claude Code itself remains installed in the WSL user's local application directory for future validation.

## Data exposure review

The returned report contained placeholders rather than the current username or absolute paths. It did not contain a home directory, machine name, repository path, token, credential, process ID, command line, or unrestricted environment block. The recorded evidence values were limited to categorical runtime, distribution, shell, agent, executable-origin, topology, and probe-status facts.

No raw JSON response is committed. This document retains only categorical results and the public installer checksum.

## Reproduction boundary

- The non-interactive Claude Code call may consume Anthropic Agent SDK or API usage according to the user's plan. The successful validation used one minimal call.
- The result covers Claude Code 2.1.212 on Ubuntu-24.04 WSL with a WSL-native temporary directory.
- It does not verify Windows-native Claude Code, Codex launched inside WSL, every wrapper process shape, or future Claude Code releases.
- Exact agent product remains a high-confidence inference because public command documentation does not guarantee every internal process shape.
