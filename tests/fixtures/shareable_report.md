# ExecLocus shareable report

> Automatically redacted before rendering. Do not use raw JSON as a public attachment.

- Schema: `0.6.0`
- Profile: `balanced`

## Current execution

| Field | Value | Source |
|---|---|---|
| Runtime | Wsl | KernelRelease |
| Distribution | Ubuntu-24.04 | Environment |
| User | \[redacted-user\] | OsAccount |
| Shell | \[redacted-home\]\\bin\\bash.exe | Environment |
| Terminal | \[redacted-machine\] terminal | environment hint |
| Session layer | Unknown | unavailable |
| Project | \[windows-mounted-project\] | WindowsMounted |

## Agent execution evidence

| Field | Value | Evidence |
|---|---|---|
| Product | Codex | Inferred / High / ProcessAncestry |
| Runtime | Wsl | Observed / Certain |
| Codex | 2 candidate(s) | Observed / Certain |
| Primary config root | \[redacted-agent-state\] | WindowsNative / High |

## Toolchain

| Role | State | Selected | Kind | Candidates | Why |
|---|---|---|---|---:|---|
| git | Selected | \[windows-executable:git:1\] | application | 2 | PATH selected \[redacted-path\] |

### Candidate details

| Role | # | Disposition | Origin | Format | Candidate |
|---|---:|---|---|---|---|
| git | 1 | selected | Windows | PE | \[windows-executable:git:1\] |
| git | 2 | not selected | Linux | ELF | \[linux-executable:git:2\] |

### Independent verification

| Role | Command | Context |
|---|---|---|
| git | Get-Command -All \[redacted-path\] | run in the same shell session |

## Findings

- **ENV002** (Warning): git resolves to a Windows executable while ExecLocus runs in WSL.
  - Recommended: Keep the setup if Windows interoperability is intentional.
  - Recommended: If Linux behavior is intended, install and prioritize Linux-native git.
  - Verify: Run \`Get-Command -All \[redacted-path\]\` in the same shell, then rerun ExecLocus.
- **ENV004** (Warning): The active agent runs in Wsl, while its primary-config root is classified as WindowsNative.
  - Recommended: Keep writable databases, caches, and primary configuration native to the executor when practical.
  - Recommended: Share only configuration files documented as portable by the agent vendor.
  - Recommended: Back up state before manually relocating it.
  - Verify: Rerun ExecLocus after changing the configuration location and confirm ENV004 is absent.
- **PATH001** (Warning): PATH selects \[windows-executable:git:1\] even though a native candidate is also available.
  - Recommended: Review PATH order for the active shell before changing configuration.
  - Recommended: Use an explicit executable path in reproducible automation.
  - Verify: Run \`Get-Command -All \[redacted-path\]\` in the same shell, then rerun ExecLocus.
- **ENV003** (Info): Certain executable evidence found Codex candidates in both Windows and WSL layers.
  - Recommended: Compare versions and resolved paths before changing either installation.
  - Recommended: Keep both installations when both workflows are intentional.
  - Recommended: Remove or deprioritize one only after confirming the active workflow.
  - Verify: Run the Codex verification command shown in Toolchain from both Windows and WSL, then rerun ExecLocus.
- **FS001** (Info): The project is stored on a Windows filesystem mounted into WSL. This is a supported interoperability choice with filesystem tradeoffs.
  - Recommended: Keep shared source on the Windows mount when interoperability is useful.
  - Recommended: Where supported, place dependency caches and build output in WSL-native storage.
  - Verify: Rerun ExecLocus with the same profile after changing project or cache placement.

## Probe status

1 optional probe failure(s). Details are omitted from this shareable report.

_Redacted fields include usernames, home directories, machine names, and absolute paths._
