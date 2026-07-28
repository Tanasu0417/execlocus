# Sanitized runtime identity validation — 2026-07-29

This record validates the current-process runtime, distribution, OS account, launching shell, and project filesystem classification without storing raw usernames, machine names, home directories, or personal absolute paths.

## Implementation under test

- Branch: `codex/runtime-environment-detection`
- Rust MSRV: 1.85.0
- Network use by the runtime probe: none
- Shell commands started by the runtime probe: none
- Process inspection: local OS snapshot through a pinned Rust dependency; requests only process names, parent IDs, and user IDs, then resolves the current ID through the local account catalog

## Windows 11

| Field | Sanitized result | Provenance |
|---|---|---|
| Runtime | `windows_native` | target platform |
| User | present; value withheld | `os_account` |
| Shell | `PowerShell 7` | `process_ancestry` |
| Distribution | unavailable, as expected | not applicable |
| Probe failures | 0 | report summary |

The Windows binary was built with the MSVC toolchain and executed from a PowerShell 7 command chain.
One warmed local invocation of `report --format json`, with output discarded, completed in approximately 730 ms. This is a smoke measurement, not a benchmark claim.

## WSL2 / Ubuntu-24.04

| Field | Sanitized result | Provenance |
|---|---|---|
| Runtime | `wsl` | kernel release |
| Distribution | `Ubuntu-24.04` | WSL registration environment |
| User | present; value withheld | `os_account` |
| Shell | `bash` | `process_ancestry` |
| Project filesystem | `windows_mounted` | path classification; absolute path withheld |
| Probe failures | 0 | report summary |

The complete MSRV 1.85 test suite passed inside Ubuntu-24.04 WSL: 18 unit tests, 6 shell-resolution contract tests, and 4 synthetic-resolution tests.
One warmed `/mnt/c` invocation of `report --format json`, with output discarded, completed in approximately 0.69 seconds. This is a smoke measurement, not a benchmark claim.

## Evidence boundary

- If an interactive shell remains in the parent chain, the shell is reported as `process_ancestry`.
- If a non-interactive shell replaces itself with the launched process, that shell is no longer observable in ancestry. ExecLocus then reports only an allowlisted `SHELL` or `ComSpec` value as `environment` rather than upgrading the hint to an observed process fact.
- This record does not validate real cmd or zsh sessions, agent-process adapters, report redaction, or release binaries.
- A host-side unrelated PATH translation warning was excluded from product claims and no raw environment dump was retained.
