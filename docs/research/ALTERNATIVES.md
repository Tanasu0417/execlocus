# ExecLocus alternatives and current workarounds

- Snapshot: 2026-07-28
- Scope: built-in commands and common manual workflows, not an exhaustive competitor census

ExecLocus overlaps with several command-resolution and environment-inspection tools, but its proposed product boundary is the combined evidence model across Windows, WSL, shells, executables, and project storage. Absence of an identical product is not proof of demand; this document makes the current substitutes explicit so prototype tests can compare against them.

## Comparison

| Alternative | Useful answer | Current-context selection | Candidate enumeration | Windows/WSL boundary | Agent-observed execution | Evidence and privacy contract |
|---|---|---:|---:|---:|---:|---:|
| POSIX `command -v` | What the current shell resolves | Yes | Usually no | No combined model | No | No report contract |
| Shell `type -a` | Command type and multiple matches | Yes | Shell-dependent | No combined model | No | No report contract |
| `which` | A PATH match | Partial and implementation-dependent | Often limited | No combined model | No | No report contract |
| PowerShell `Get-Command -All` | Commands visible to PowerShell, including command types | Yes for PowerShell | Yes | No combined model | No | No report contract |
| Windows `where.exe` | Matching files from current directory/PATH | Yes for its lookup rules | Yes | No combined model | No | No report contract |
| `wsl.exe -l -v`, `uname`, `/proc/version` | Installed WSL distributions or current kernel context | No command selection | No | Partial runtime facts | No | No combined report |
| Manual PATH and file-format inspection | Potential candidates and PE/ELF/script origin | Possible | Possible | Possible with expertise | No | High manual effort |
| ExecLocus target | One normalized topology with selected/losing candidates, origin, project boundary, and provenance | Yes, under an explicit shell contract | Yes | Yes | Only with independent invocation/process evidence | Local/read-only; redaction before shareable output |

## Important distinctions

- ExecLocus must not replace shell semantics with a generic PATH scan. PowerShell aliases/functions/scripts, cmd current-directory and `PATHEXT` behavior, and bash/zsh resolution need explicit contracts and tests.
- Current-context resolution predicts what would be selected now. It does not prove which executable an agent already ran.
- The product earns its place only if the combined report reaches a verified conclusion faster or more reliably than the user's existing command sequence.
- Existing commands remain valuable as independent checks in field validation and troubleshooting.

## Product test against substitutes

For each prototype session, record:

1. Which manual commands the collaborator would otherwise use.
2. Whether those commands produced a verified answer.
3. Manual and ExecLocus time-to-conclusion.
4. Whether ExecLocus added a decision-relevant relationship rather than merely restating one command.
5. Any false positive or shell-semantic disagreement.

This comparison is the direct-alternative test. The separate [30-project benchmark](OSS_BENCHMARK_30.md) studies successful OSS presentation and distribution patterns; it is not a substitute for competitor or workaround analysis.
