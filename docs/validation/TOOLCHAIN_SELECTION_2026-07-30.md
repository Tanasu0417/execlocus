# Toolchain selection validation — 2026-07-30

## Scope

This validation exercised one Windows Documents checkout from both Windows PowerShell and Ubuntu-24.04 on WSL2. It tested the bounded shell-session wrappers, schema `0.6.0`, candidate rendering, finding guidance, redaction, and temporary-file cleanup.

No paid service, hosted API, telemetry, package installation, or machine mutation was used. Generated reports remain under the Git-ignored `target/user-validation/` directory.

## Commands

Windows PowerShell, from the repository root:

```powershell
& .\scripts\try-execlocus.ps1 -Profile balanced
```

WSL bash, after opening the same Windows Documents checkout:

```bash
source ./scripts/try-execlocus.sh balanced
```

## Sanitized result

| Context | Runtime / project | Selected | Not found | Findings |
|---|---|---|---|---|
| Windows | Windows native / Windows project | Codex, Git | Claude Code, Node, npm | none |
| WSL | Ubuntu-24.04 WSL / Windows-mounted project | Codex, Claude Code, Git, npm script | Node | `TOOL001`, `FS001` |

The WSL result exposed a concrete inconsistency: an npm launcher was selected, but `node` was not resolvable in the same complete bash snapshot. ExecLocus reports that wrapper-specific npm behavior may still work, while direct Node commands and tools requiring `node` on PATH may fail. It recommends independent `type -a` checks and rerunning the diagnosis after any intentional change.

## State and output checks

- `Selected` and `Not found` were observed in real runs.
- `Candidates found / selection unconfirmed` and `Probe failed` have deterministic model tests.
- Markdown and redacted JSON include state, selected binding kind, candidate count, origin, PE/ELF/script format, selection reason, and verification command.
- Findings include impact, recommended actions, and reverification steps.
- PowerShell captures bounded `Get-Command -All` alias/function/cmdlet evidence.
- bash captures bounded alias/function/builtin evidence while the Rust resolver inspects ordered PATH candidates.
- No alias expansion, function body, token, credential, or unrestricted environment value is serialized.

## Privacy and cleanup checks

The four generated Windows/WSL Markdown and redacted JSON reports were scanned for the local username, machine name, Windows home prefix, Linux home prefix, mounted Windows user path, and the local project directory name. No match remained. The temporary PowerShell and bash snapshot files were absent after each wrapper completed.

## GUI gate

The GUI diagnostic button remains intentionally unimplemented. Before starting it, the evaluator should confirm both statements:

1. The paired report revealed at least one problem or boundary that was not already clear.
2. The state, impact, recommendation, and verification command shortened the time needed to decide what to inspect next.

If either answer is no, improve the CLI result and explanation first. If both are yes, connect the existing local GUI concept to the read-only diagnostic command without adding network access or automatic fixes.
