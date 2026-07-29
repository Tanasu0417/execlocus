# Shell-resolution validation — 2026-07-29

This validation records categorical results only. No raw terminal report, username, machine name, home directory, absolute executable path, credential, token, or private repository detail is retained.

## Windows production-path observation

The current Windows process-ancestry run exercised the production shell-contract path after the v0.5 report-model change.

| Check | Sanitized result |
|---|---|
| JSON schema | `0.5.0` |
| Resolution method | `shell_contract` |
| External candidates observed across the standard roles | 3 |
| Candidate evidence IDs visible in terminal output | yes |
| Resolution strategy or unavailable parent-session state visible | yes |
| Raw ESC control character present in captured terminal output | no |

The raw output was held only in process memory long enough to make the categorical assertions above and was not written to the repository.

## Deterministic contract coverage

- PowerShell: alias, external-script, and application precedence; incomplete session does not claim an external winner.
- cmd: current-directory then PATH search with PATHEXT order; report mapping retains selected and losing candidates.
- bash: function precedence and a WSL scenario where incomplete parent-session evidence keeps selection unknown while listing the external candidate.
- zsh: builtin precedence over PATH.
- Generic fallback: reports record `path_fallback` rather than presenting it as shell-session proof.
- Terminal safety: Unicode remains readable while newline, tab, ESC, and other control characters are escaped before display.

## Trust boundary and limitation

ExecLocus does not source shell profiles, inspect function bodies, scrape process command lines, or execute shell command strings. A child process cannot reliably recover parent aliases, functions, cmdlets, builtins, macros, or hash state. When that bounded session evidence is unavailable, the effective command remains unknown and external files are shown only as candidates.
