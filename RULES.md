# ExecLocus Diagnostic Rules

- Status: Draft for v0.1
- Last updated: 2026-07-28
- Applies to: ExecLocus v0.1

## 1. Rule principles

Rules explain conflicts in an observed execution topology. They are not a checklist of personal preferences.

Every warning or error must satisfy all of the following:

1. The required evidence was actually observed
2. The condition is deterministic for that evidence
3. The impact is concrete and relevant to the active profile
4. The suggestion does not modify the system automatically
5. The output distinguishes fact from inference

Missing evidence must produce `not_evaluated`, not a warning.

## 2. Rule result states

| State | Meaning |
|---|---|
| `pass` | Required evidence exists and the conflict is absent |
| `finding` | Required evidence exists and the condition matched |
| `not_applicable` | The rule does not apply to this runtime or profile |
| `not_evaluated` | Required evidence is missing or a probe failed |
| `suppressed` | The user explicitly suppressed the rule |

`not_evaluated` is visible in verbose and machine-readable reports. It must not be presented as a passed check.

## 3. Severity

| Severity | Meaning | Default exit-code effect |
|---|---|---:|
| `info` | Relevant topology or tradeoff; no conflict proven | none |
| `warning` | Likely failure, inconsistency, or material tradeoff | none |
| `error` | Strong evidence of a broken or unsafe execution path | exit 1 |

Severity may depend on the filesystem profile, but facts and rule IDs remain unchanged.

## 4. Rule schema

Each rule definition must contain:

```text
id
version
title
category
default_severity
applicability
required_evidence
condition
confidence_requirement
profile_adjustments
summary
impact
suggested_actions
false_positive_notes
references
```

Rule IDs are stable public API. A materially changed meaning requires a new ID or rule version.

## 5. Evidence constraints

- A warning requires observed evidence or a high-confidence inference supported by at least two independent facts
- An error requires observed evidence; low- or medium-confidence inference alone is insufficient
- PATH rules must use actual resolution order, not only the raw PATH string
- Executable-layer rules should prefer file-format evidence over filename extension
- Agent-runtime claims require process or invocation evidence; installation presence alone is insufficient
- Secret-bearing raw values may not be attached to findings

## 6. Initial rules

### ENV001 — Visible terminal and agent runtime differ

- Category: environment
- Default severity: info
- Applies when: both visible-terminal layer and agent-runtime layer are available
- Required evidence: terminal layer, agent runtime, relationship evidence
- Confidence requirement: agent runtime `high` or `certain`

Condition:

```text
terminal.os_layer != agent.os_layer
```

Summary:

> The visible terminal and the agent execute in different OS layers.

Impact:

Commands manually executed in the visible terminal may resolve different paths, tools, configuration, and permissions from commands executed by the agent.

Suggested actions:

- Compare resolved Git, Node, shell, and project paths before changing configuration
- Use `execlocus report --format json` to inspect the relationship evidence

False-positive control:

- A Windows terminal hosting a WSL shell is not automatically a conflict
- Do not emit a warning based only on terminal brand or environment-variable names

### ENV002 — WSL execution resolves a Windows executable

- Category: environment
- Default severity: warning
- Applies when: ExecLocus or observed agent runtime is WSL
- Required evidence: runtime layer, resolved executable path, executable format or origin
- Confidence requirement: certain

Condition:

```text
runtime.kind == wsl
and executable.origin == windows
and executable.role in [agent, node, npm, git, shell]
```

Summary:

> WSL execution resolves a Windows executable.

Impact:

Path syntax, permissions, subprocess behavior, configuration directories, and package installation targets may differ from Linux-native expectations.

Suggested actions:

- If Linux behavior is intended, install and prioritize the Linux-native executable in the distribution
- If Windows interoperability is intentional, keep the setup and document the boundary
- Inspect PATH001 before changing PATH order

False-positive control:

- WSL interoperability is supported behavior; this rule is not an error by default
- Windows tools intentionally invoked with `.exe` remain warnings only when they occupy a core toolchain role

### ENV003 — Agent is installed in both Windows and WSL

- Category: environment
- Default severity: info
- Applies when: a supported agent is detected in both layers
- Required evidence: installation paths in Windows and WSL
- Confidence requirement: certain for each installation

Condition:

```text
agent.windows_installation.exists
and agent.wsl_installation.exists
```

Summary:

> The same agent has Windows and WSL installations.

Impact:

The selected installation can vary with terminal, PATH, launcher, or desktop integration. Configuration and update versions may diverge.

Suggested actions:

- Compare versions and resolved paths
- Keep both if both workflows are intentional
- Remove or deprioritize one only after confirming the active workflow

False-positive control:

- Duplicate installation is not inherently a problem
- Upgrade to warning only when PATH ambiguity or version divergence is also observed

### ENV004 — Agent state or configuration crosses OS layers

- Category: environment
- Default severity: warning
- Applies when: an agent process reads its primary writable state from another OS layer
- Required evidence: agent runtime, normalized config/state path, mount classification
- Confidence requirement: high or certain

Condition:

```text
agent.runtime.os_layer != config.storage_os_layer
and config.kind in [writable_state, database, primary_config]
```

Summary:

> Agent configuration or writable state crosses the Windows/WSL boundary.

Impact:

File locking, permissions, line endings, performance, and concurrent access may become inconsistent.

Suggested actions:

- Keep writable databases and caches native to the executor when practical
- Share only documented portable configuration files
- Back up state before manually relocating it

False-positive control:

- Read-only shared configuration is not enough to trigger the rule
- Do not inspect secret content; path and file role are sufficient

### FS001 — Project or heavy artifacts are on `/mnt/*`

- Category: filesystem
- Default severity: profile-dependent
- Applies when: WSL runtime uses a Windows-mounted project or artifact path
- Required evidence: path classification and selected profile
- Confidence requirement: certain

Condition:

```text
runtime.kind == wsl
and path.class == windows_mounted
```

Profile behavior:

| Profile | Severity | Message focus |
|---|---|---|
| `share-first` | info | Sharing is intentional; explain tradeoffs only |
| `balanced` | info | Keep source shared; consider native cache/build output |
| `linux-first` | warning | Linux-native project or artifacts may perform better |

Summary:

> The project is stored on a Windows filesystem mounted into WSL.

Impact:

This improves interoperability with Windows apps but may affect metadata-heavy I/O, permissions, symlinks, watchers, or case sensitivity.

Suggested actions:

- `share-first`: keep `/mnt/c` when Windows/Cowork/Explorer access is the priority
- `balanced`: place build, cache, and dependency-heavy artifacts in WSL-native storage where supported
- `linux-first`: consider a WSL-native checkout and expose it to Windows through `\\wsl.localhost`

False-positive control:

- Never label `/mnt/c` itself as broken
- Do not claim a measured performance problem without measurement evidence

### FS002 — WSL-native project may be inconvenient for a Windows-first workflow

- Category: filesystem
- Default severity: info
- Applies when: project is WSL-native and profile is `share-first`
- Required evidence: path classification and selected profile
- Confidence requirement: certain

Condition:

```text
project.path.class == wsl_native
and profile == share_first
```

Summary:

> The project is WSL-native while the selected profile prioritizes Windows sharing.

Impact:

Windows tools can usually reach the project through WSL UNC paths, but some applications, file dialogs, watchers, or integrations may be less convenient.

Suggested actions:

- Keep the WSL-native project if Linux compatibility and performance matter more
- Use the WSL UNC path from Windows
- Move only if the Windows application in question cannot work reliably with the UNC path

### PATH001 — PATH precedence selects an executable from another layer

- Category: path
- Default severity: warning
- Applies when: multiple candidates exist and the selected executable crosses the active runtime layer
- Required evidence: ordered candidates, selected path, runtime layer, executable origins
- Confidence requirement: certain

Condition:

```text
candidates.count > 1
and selected.origin != runtime.os_layer
and candidates contains origin == runtime.os_layer
```

Summary:

> PATH selects a cross-layer executable even though a native candidate exists.

Impact:

The invoked version may use different configuration, packages, path rules, or subprocess semantics.

Suggested actions:

- Review PATH order for the active shell only
- Confirm version and behavior before modifying shell profiles
- Prefer explicit paths in automation where reproducibility matters

False-positive control:

- Do not trigger if no native alternative was observed
- Shell aliases and functions must be represented separately from file executables

### GIT001 — Git and project reside in different OS layers

- Category: toolchain
- Default severity: warning
- Applies when: Git operates on a project stored in another layer
- Required evidence: resolved Git origin and project path classification
- Confidence requirement: certain

Condition:

```text
git.origin == windows and project.path.class == wsl_native
or git.origin == linux and project.path.class == windows_native_direct
```

`windows_mounted` is evaluated according to profile and is not enough by itself to trigger the rule.

Summary:

> Git and the project are using different OS layers.

Impact:

Credentials, file modes, case sensitivity, hooks, line endings, and path handling may differ.

Suggested actions:

- Prefer Git native to the runtime that owns the project workflow
- Review `core.autocrlf`, file-mode behavior, credential helper, and hooks before switching

## 7. Rule ordering

Default output order:

1. error before warning before info
2. environment before path before toolchain before filesystem
3. stable lexical rule ID within the same category and severity

The default terminal view shows at most three findings. The full set remains available through `execlocus check` and machine-readable reports.

## 8. Suggested-action rules

Suggested actions must:

- Be reversible or informational
- Avoid shell commands that overwrite configuration
- State the intended workflow before recommending a change
- Offer a “keep current setup” interpretation when the boundary may be intentional
- Avoid claiming that one filesystem layout is universally best

v0.1 must not execute suggested actions.

## 9. Suppression

Suppression is not required for the first prototype. When added, it must use stable rule IDs and a project-local or user-level configuration file with explicit scope.

Suppression must not hide evidence from JSON reports. It changes the result state to `suppressed` and records the suppression scope without including sensitive paths.

## 10. Testing requirements

Every rule requires:

- Positive fixture
- Negative fixture
- Missing-evidence fixture
- Cross-platform path fixture
- Redaction golden test for finding evidence
- Profile-specific tests where applicable

Minimum v0.1 fixture matrix:

| Runtime | Project | Tool | Expected focus |
|---|---|---|---|
| Windows | Windows | Windows | no cross-layer finding |
| WSL | `/mnt/c` | Linux | FS001 profile behavior |
| WSL | `/mnt/c` | Windows Node | ENV002 |
| WSL | WSL-native | Linux | no cross-layer finding |
| WSL | WSL-native | Windows Git | GIT001 |
| WSL | any | both native and Windows candidates | PATH001 |

## 11. Contribution policy for new rules

A proposed rule must include:

1. A reproducible real-world failure or material tradeoff
2. The minimum evidence required
3. At least one legitimate setup that must not trigger
4. Fixtures and tests
5. Read-only suggested actions

Rules based only on style preferences, unverifiable assumptions, or one vendor's undocumented private files should not be accepted.
