# ExecLocus initial use cases

- Status: design contracts for v0.1
- Updated: 2026-07-28

These are the three questions ExecLocus must answer before its scope expands. Each use case defines minimum evidence, a legitimate setup that must not be misreported, privacy boundaries, and acceptance tests.

## UC-01 — Where is the coding agent actually executing?

### User question

> Is this Codex or Claude session executing on Windows, inside WSL, or across both layers?

### Minimum evidence

- Current ExecLocus process runtime
- WSL kernel and distribution evidence when applicable
- Agent invocation, executable, or process relationship evidence
- Shell and terminal hints kept separate from process evidence

Terminal appearance and installation presence alone are insufficient to claim an observed agent runtime.

### Expected result

- Report the current runtime as observed when available.
- Report the agent runtime as observed, inferred with confidence, or unknown.
- Name the evidence source supporting the conclusion.
- Explain when the visible terminal and agent runtime are different without declaring every difference broken.

### Legitimate non-triggering setup

Windows Terminal hosting a WSL shell is normal. It must not produce a warning unless independent evidence shows a material execution-layer conflict.

### Privacy boundary

Do not read agent tokens or unrestricted private state. Redacted output must not contain the username, home directory, machine name, or personal absolute paths.

### Acceptance tests

- Windows-native invocation fixture
- WSL-native invocation fixture
- Windows terminal hosting a WSL shell fixture
- Agent installation present without runtime evidence returns `unknown`
- Missing optional process evidence produces a partial report, not a panic

## UC-02 — Which executable would win in the current context, and why?

### User question

> If Git, Node, npm, or an agent command were resolved in the current ExecLocus context, which executable would be selected and which alternatives would lose?

This use case models current-context resolution. It must not claim that the agent already executed the selected file unless separate invocation or process evidence establishes agent-observed execution.

### Minimum evidence

- Active runtime layer
- Ordered command-resolution candidates
- Selected path
- Executable format or other origin evidence
- Native alternative presence when reporting PATH precedence

### Expected result

- Show the executable that would be selected and its Windows/Linux/script origin.
- Show that selection order, not filename alone, determined the result.
- Emit `ENV002` when the modeled WSL context selects a Windows core tool.
- Emit `PATH001` only when a native candidate was actually observed and lost precedence.
- Keep actionable suggestions read-only.

### Legitimate non-triggering setup

An intentionally invoked Windows executable from WSL, with no observed native alternative, must not be described as a PATH collision.

### Privacy boundary

Terminal output may show normalized paths needed to explain the result. Shareable output must replace personal path segments before serialization.

### Acceptance tests

- WSL selects Linux Git and Node
- WSL selects Windows Node with no native alternative
- WSL selects Windows Node while a Linux candidate exists
- Windows selects a Windows executable
- Missing command returns unavailable evidence without failing the report

## UC-03 — Is `/mnt/c` intentional sharing or a harmful mismatch?

### User question

> Is this project on `/mnt/c` because I want Windows/Cowork access, or is the filesystem boundary conflicting with the active toolchain?

### Minimum evidence

- Current runtime layer
- Project path classification
- Selected `share-first`, `balanced`, or `linux-first` profile
- Tool origins relevant to any stronger finding

Filesystem location alone is not evidence of measured slowness or breakage.

### Expected result

- Classify `/mnt/<drive>` as Windows-mounted storage.
- Preserve the same observed fact under every profile.
- Change only severity and advice according to user intent.
- Under `share-first`, explain interoperability benefits and tradeoffs.
- Report a stronger warning only when separate evidence proves a conflicting tool or workflow.

### Legitimate non-triggering setup

A shared source tree on `/mnt/c` with WSL-native build/cache output and intentional Windows application access is a supported `balanced` or `share-first` workflow.

### Privacy boundary

The report may classify the filesystem without retaining the complete personal path. Redacted output should preserve a shape such as `/mnt/c/<redacted>/project` only when the shape is relevant.

### Acceptance tests

- `/mnt/c` under `share-first`, `balanced`, and `linux-first`
- WSL-native project under `share-first`
- Windows-native and WSL UNC path classification
- `/mnt/c` with a cross-layer executable finding
- No unmeasured performance claim in any result

## Contract shared by all three use cases

- Default execution is local-only and read-only.
- A missing fact remains unavailable; it never becomes a passing check.
- Findings reference stable evidence identifiers.
- Terminal and JSON renderers consume the same report.
- Warnings do not modify PATH, files, WSL configuration, or agent configuration.
- Public examples and fixtures use synthetic identities and paths.
