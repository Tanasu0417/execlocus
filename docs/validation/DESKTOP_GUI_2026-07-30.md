# Windows desktop GUI validation — 2026-07-30

This record contains no account name, machine name, home directory, or personal absolute path.

## Build boundary

- Tauri `2.11.5` and Tauri Build `2.6.3` use a separate committed lockfile and a desktop-only Rust `1.88.0` MSRV. The CLI-only MSRV remains `1.85.0`.
- The final source-built debug executable was `13,330,944` bytes in this run and Windows reported `NotSigned`, as expected for an unsigned development build.
- The process created a visible window titled `ExecLocus` and listened on one random `127.0.0.1` port only.
- The native WebView navigation policy accepted only the assigned HTTP loopback origin and port.
- No hosted service, metered API, external AI model, credential, or paid runner was used.

## Interaction boundary

- Live mode hides the synthetic hero layer, removing the overlap seen in the prior Inspect screenshot.
- One diagnostic action moved to Compare and rendered five summary rows plus five native disclosure rows.
- All detail rows were collapsed initially; Expand all opened five and Collapse all returned to zero.
- English/Japanese switching re-ran and localized the real diagnostic result.
- The Share view contained the modeled `redacted-user` and `redacted-path` markers and no Windows user-profile or WSL home absolute-path token.
- The status remained explicit that zero values were uploaded.

## GUI value review

- The first decision now requires one action and one compact five-row scan; candidate detail no longer dominates the page before it is requested.
- The summary immediately distinguishes selected tools from missing or unresolved tools, while each disclosure retains the independent verification command.
- This reduces lookup time for a known question such as “which Git/Node/Codex is active,” but it does not yet create the strongest Windows/WSL value moment by itself.
- The next highest-value GUI experiment is one button that launches paired Windows and WSL observations and shows only meaningful differences. It must remain read-only and explicit about which side supplied each fact.

## Accepted development limitations

- There is no installer, public download, code signature, automatic update, or release provenance for the desktop executable yet.
- A standalone GUI process cannot reconstruct aliases and functions in an already-running terminal. The bounded PowerShell/bash wrappers remain the exact-current-session path.
- Tauri adds a materially larger dependency graph. The desktop lockfile therefore has a separate RustSec, license, registry/source, wildcard, and duplicate-version policy gate.
- The initial Rust `1.85.0` desktop resolution selected vulnerable transitive XML/time releases. The development build was blocked, the desktop MSRV was raised independently, and the lockfile was regenerated rather than adding advisory exceptions.
- The regenerated graph selected `quick-xml 0.41.0` and `time 0.3.54`. `cargo-deny 0.19.4` then reported advisories, bans, licenses, and sources all `ok`; upstream duplicate dependency lines remain warnings.
- The generated development icon is a project-native geometric locus mark, not the final product logo.
