# Windows Claude Code and WSL Codex validation — 2026-07-29

## Outcome

Release-built ExecLocus binaries were launched by Windows-native Claude Code and WSL-native Codex CLI in isolated directories. The final sanitized results were:

| Launching agent | ExecLocus runtime | Product evidence | Agent runtime | Findings | Probe failures |
|---|---|---|---|---:|---:|
| Claude Code 2.1.212 on Windows | `windows_native`, observed, certain | `claude_code`, inferred, high confidence, process ancestry | `windows_native`, observed, certain | 0 | 0 |
| Codex CLI 0.146.0 in Ubuntu-24.04 WSL | `wsl`, observed, certain | `codex`, inferred, medium confidence, environment marker | `wsl`, observed, certain | 0 | 0 |

The Windows run observed `bash` in process ancestry because native Claude Code used Git Bash for its shell tool. Git and Node resolved to Windows PE executables. The selected npm candidate was a script and remained neutral rather than being mislabeled as a Linux executable.

## Codex PID-namespace finding

The first Codex run correctly observed WSL but returned `Unknown` for the agent. A bounded diagnostic emitted process names and environment-variable names only—no values, process IDs, paths, or command lines. It confirmed that the sandbox PID namespace hid the Codex ancestor and exposed only a short internal process chain.

The child environment contained an allowlisted `CODEX_THREAD_ID` name. OpenAI's source at the tested revision defines that marker and injects it into shell-tool environments even when an include-only environment policy is active:

- [`codex-rs/core/src/exec_env.rs`](https://github.com/openai/codex/blob/28f3f1f9ef4e9578a5f023f6b6eba018914a5342/codex-rs/core/src/exec_env.rs)
- [`codex-rs/protocol/src/shell_environment.rs`](https://github.com/openai/codex/blob/28f3f1f9ef4e9578a5f023f6b6eba018914a5342/codex-rs/protocol/src/shell_environment.rs)

ExecLocus now uses a UUID-shaped value as medium-confidence fallback evidence only when stronger process evidence is absent. It never stores or renders the marker value. Exact process ancestry remains high confidence and always wins. A missing or malformed marker remains insufficient.

## Windows script-origin finding

The initial Windows run labeled an npm script with a POSIX shebang as Linux-origin and produced a false `PATH001` warning. Git Bash can execute such scripts while the surrounding runtime and resolved Node binary remain Windows-native, so a shebang alone cannot establish the script's OS layer on Windows.

The executable probe now keeps that script origin neutral on Windows, and `PATH001` evaluates only concrete Windows or Linux origins. A focused regression test and the final real run both produced no finding.

## Isolation and privacy controls

- Only release binaries were copied into newly created generic temporary directories; repository source and Git metadata were not exposed to either agent.
- Both agents were instructed to execute exactly one automatically redacted report command and no file discovery or modification command.
- Claude Code used safe mode, an empty strict MCP configuration, disabled skills and slash commands, disabled browser integration, no session persistence, one allowlisted Bash command, and low effort.
- Codex used an ephemeral session, a fresh temporary `CODEX_HOME`, no project instructions, no MCP configuration, no plugins, `never` approval mode, and a workspace-limited sandbox.
- The temporary Codex home contained only a mode-`0600` copy of existing ChatGPT OAuth state. It was not displayed, sent in the prompt, or committed.
- All retained reports were automatically redacted. No raw report, username, machine name, home directory, personal absolute path, token, credential, thread identifier, session identifier, or billing detail is committed.

## Cost boundary

The repository owner explicitly approved bounded use of existing Claude and ChatGPT subscription entitlements for this validation. API-key, cloud-provider, usage-credit, and pay-as-you-go authentication routes were not used. CLI-reported dollar values are local usage estimates rather than proof of a subscription charge, so no estimate is retained here.

The first Claude capture returned only a success summary, requiring a stream-output rerun to retain the redacted tool result. Both products were then rerun once after the fixes to verify the final evidence. No further model-backed validation is authorized by this record.

## Automated verification

Rust 1.85 formatting, clippy with warnings denied, and all 46 tests passed on Windows and Ubuntu-24.04 WSL. The new contracts cover:

- valid, invalid, and absent Codex child markers;
- rejection of the marker outside Linux and WSL;
- marker non-retention;
- process-ancestry precedence;
- Windows-neutral and WSL/Linux script origins; and
- suppression of cross-layer `PATH001` for neutral scripts.
