# ExecLocus shareable report

> Automatically redacted before rendering. Do not use raw JSON as a public attachment.

- Schema: `0.2.0`
- Profile: `Balanced`

## Current execution

| Field | Value | Source |
|---|---|---|
| Runtime | Wsl | KernelRelease |
| Distribution | Ubuntu-24.04 | Environment |
| User | [redacted-user] | OsAccount |
| Shell | [redacted-home]\bin\bash.exe | Environment |
| Terminal | [redacted-machine] terminal | environment hint |
| Project | [windows-mounted-project] | WindowsMounted |

## Toolchain

| Role | Selected | Origin |
|---|---|---|
| git | [windows-executable:git:1] | Windows |

## Findings

- **ENV002** (Warning): git resolves to a Windows executable while ExecLocus runs in WSL.
- **PATH001** (Warning): PATH selects [windows-executable:git:1] even though a native candidate is also available.

## Probe status

1 optional probe failure(s). Details are omitted from this shareable report.

_Redacted fields include usernames, home directories, machine names, and absolute paths._
