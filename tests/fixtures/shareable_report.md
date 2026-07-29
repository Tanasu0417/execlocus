# ExecLocus shareable report

> Automatically redacted before rendering. Do not use raw JSON as a public attachment.

- Schema: `0.4.0`
- Profile: `balanced`

## Current execution

| Field | Value | Source |
|---|---|---|
| Runtime | Wsl | KernelRelease |
| Distribution | Ubuntu-24.04 | Environment |
| User | [redacted-user] | OsAccount |
| Shell | [redacted-home]\bin\bash.exe | Environment |
| Terminal | [redacted-machine] terminal | environment hint |
| Session layer | Unknown | unavailable |
| Project | [windows-mounted-project] | WindowsMounted |

## Agent execution evidence

| Field | Value | Evidence |
|---|---|---|
| Product | Codex | Inferred / High / ProcessAncestry |
| Runtime | Wsl | Observed / Certain |
| Codex | 2 candidate(s) | Observed / Certain |
| Primary config root | [redacted-agent-state] | WindowsNative / High |

## Toolchain

| Role | Selected | Origin |
|---|---|---|
| git | [windows-executable:git:1] | Windows |

## Findings

- **ENV002** (Warning): git resolves to a Windows executable while ExecLocus runs in WSL.
- **ENV004** (Warning): The active agent runs in Wsl, while its primary-config root is classified as WindowsNative.
- **PATH001** (Warning): PATH selects [windows-executable:git:1] even though a native candidate is also available.
- **ENV003** (Info): Certain executable evidence found Codex candidates in both Windows and WSL layers.
- **FS001** (Info): The project is stored on a Windows filesystem mounted into WSL. This is a supported interoperability choice with filesystem tradeoffs.

## Probe status

1 optional probe failure(s). Details are omitted from this shareable report.

_Redacted fields include usernames, home directories, machine names, and absolute paths._
