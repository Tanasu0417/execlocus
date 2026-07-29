# ExecLocus security and threat model

- Scope: v0.1 local CLI, loopback GUI, Windows desktop development shell, CI, and release artifacts
- Last reviewed: 2026-07-30
- Security contact: [private vulnerability reporting](https://github.com/Tanasu0417/execlocus/security/advisories/new)

ExecLocus observes an execution context that may already contain attacker-controlled names, paths, files, and environment hints. It must report partial facts without turning those inputs into commands, trusted conclusions, or shareable identity leaks.

## Security objectives

1. Normal execution remains local and read-only. Single-environment probes do not execute candidates; the opt-in paired Windows/WSL flow runs only a fixed, allowlisted companion command.
2. Missing, conflicting, or inaccessible evidence produces an explicit partial or unknown result, not a fabricated pass.
3. A shareable renderer removes modeled identity and absolute paths before serialization.
4. Terminal and Markdown renderers neutralize their own output-control syntax.
5. Dependencies and release artifacts have reproducible integrity checks with least-privilege automation.

ExecLocus is a diagnostic aid, not a sandbox, endpoint-security product, secret scanner, malware scanner, or proof that an observed executable is trustworthy.

## Assets

- username, machine name, home directory, project path, agent-state path, and executable paths;
- correctness of runtime, shell, agent, path, and executable-origin conclusions;
- integrity of the source, lockfile, CI workflows, binaries, checksums, SBOM, and provenance;
- user trust that a read-only diagnostic will not modify PATH, profiles, projects, WSL, or agent state.

## Trust boundaries and bounded inputs

| Boundary | Input used | Bound or provenance | Not collected / not performed |
|---|---|---|---|
| CLI | profile, command, rule ID | clap parser; known rule catalog | no configuration rewrite |
| Target and kernel | compile target; `/proc/sys/kernel/osrelease` and `/proc/version` | kernel reads capped at 64 KiB; source recorded | no WSL mutation |
| OS identity | process names, parent IDs, user ID/account lookup | bounded ancestry depth | no process command lines, process environment blocks, working directories, or executable paths |
| Environment hints | explicit allowlist such as `WSL_DISTRO_NAME`, `SHELL`, `ComSpec`, homes, and agent markers | hints are labeled; the Codex marker value is shape-checked and never retained | no unrestricted environment dump; no token variables |
| Linux identity files | `/etc/os-release` | read capped at 64 KiB | no arbitrary config traversal |
| Project | current working directory and normalized path class | one path observation | no project-content read |
| Agent state | allowlisted primary-root location and path class | active high-confidence agent only | no database, config, cache, credential, or token content read |
| Executables | ordered search directories, metadata, canonical path, first 512 bytes | PE/ELF/shebang classification; failures retained | no execution of candidates; no signature or malware claim |
| Shell | supported name in bounded ancestry; explicit resolution contracts | parent session is incomplete unless supplied as a bounded snapshot | no profile sourcing, function-body capture, command-line scraping, or shell-string execution |
| Rendering | normalized report object | redaction occurs before shareable serialization; context-specific escaping | raw terminal/JSON is not advertised as shareable |
| Local GUI | same-process HTTP on a random `127.0.0.1` port | Host, Origin, custom header, request-size, method, CSP, no-cache, and loopback binding checks | no LAN bind, CORS, telemetry, or hosted upload |
| Windows desktop shell | Tauri WebView loading the assigned loopback URL | navigation allowlist requires the exact `http://127.0.0.1:<assigned-port>` origin; no Tauri command or plugin capability is exposed | no remote navigation, installer, automatic update, or code-signing key |
| Windows/WSL paired diagnostic | `wsl.exe` starts `bash -lc` in the same launch directory and executes only `execlocus --profile <enum> --lang <enum> report --format <enum>` | shell text is a fixed literal; profile, language, format, and redaction are passed as separate arguments from closed enums; stdin is closed; stderr is never returned to the GUI | no arbitrary user command, project-content read, package install, automatic fix, or remote upload |
| Network | RustSec and source metadata in dedicated development/CI checks | normal CLI has no network path | no telemetry or hosted report upload |

## Primary threats and controls

| Threat | Control |
|---|---|
| Spoofed WSL or agent environment marker | target/kernel and exact ancestry outrank environment hints; confidence and source remain explicit |
| Alias/function state hidden from a child process | effective shell selection remains unknown; external candidates are not promoted to a proven winner |
| Malicious filename or observed name emits ANSI/control sequences | terminal renderer escapes control characters before display |
| Markdown table, raw HTML, or image-link injection | shareable cells escape table delimiters, Markdown punctuation, HTML delimiters, and controls |
| Personal path appears in a non-path field or quoted assignment | shareable redaction scans modeled secrets plus absolute-path tokens, including quoted `key=path` forms |
| Symlink or candidate changes between metadata and prefix read | failures are partial; no candidate is executed; consumers must not treat the result as a security verdict |
| Oversized or malformed evidence file/header | bounded reads, lossy UTF-8 handling, malformed-header tests, and no panic contract |
| One optional probe fails | successful evidence remains available with reduced confidence and an explicit probe failure |
| Malicious or compromised dependency | committed lockfile; weekly RustSec, license, registry/source, wildcard, and duplicate checks; Dependabot and CodeQL |
| Workflow supply-chain substitution | workflow token defaults to `contents: read`; checkout credentials are not persisted; external Actions are full-SHA pinned |
| Desktop WebView navigates away from the local diagnostic | native navigation policy accepts only the assigned loopback host and port; the served page also uses a restrictive CSP and contains no remote asset |
| Query input is injected into the WSL companion command | unknown query values fall back to typed enums; only enum labels are inserted into the fixed companion template; the working directory is passed as a separate `wsl.exe --cd` argument |
| Tampered release binary | v0.1 gate requires SHA-256 checksums, immutable source tag/SHA, clean-machine verification, and GitHub artifact attestation |

## Dependency policy

`cargo-deny 0.19.4` runs against the committed CLI lockfile and its Windows/Linux target graph. A second committed policy and lockfile cover the Tauri Windows target. The policies:

The desktop shell has a separate Rust 1.88 MSRV because the Rust 1.85-compatible transitive selection contained known XML/time denial-of-service advisories. No advisory exception is accepted for that compatibility issue; the CLI-only MSRV remains 1.85.

The reviewed desktop lockfile uses patched `quick-xml 0.41.0` and `time 0.3.54`; the desktop cargo-deny gate reports advisories, bans, licenses, and sources as clean. Duplicate transitive lines remain visible warnings rather than hidden exceptions.

- fails known RustSec vulnerabilities and unsound advisories;
- fails yanked dependencies;
- permits only the explicitly reviewed permissive SPDX licenses in `deny.toml`;
- fails unknown registries, unknown Git sources, and wildcard dependency requirements;
- reports new duplicate dependency lines; and
- contains only exact-version, reasoned duplicate exceptions.

The scheduled check uses a standard public-repository GitHub runner, no credential, no paid API, no cache/artifact storage, and no third-party Action beyond the already allowed pinned toolchain Action. RustSec describes `cargo-deny` as checking advisories, licenses, bans, and sources: <https://rustsec.org/> and <https://embarkstudios.github.io/cargo-deny/checks/>.

## Release integrity decision

The release PR must add, verify, and document:

- deterministic Windows x86_64 and Linux x86_64 builds from a protected tag and exact commit;
- SHA-256 files generated in the same release workflow and verified on clean systems;
- an SPDX SBOM exported from the dependency graph or generated for the exact artifacts;
- a GitHub artifact attestation for each binary/checksum bundle; and
- install instructions that verify checksum and optionally `gh attestation verify` before execution.

GitHub documents that public repositories can export SPDX SBOMs and use artifact attestations, with the release job temporarily receiving only `contents: read`, `id-token: write`, and `attestations: write`: <https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations> and <https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/export-dependencies-as-sbom>.

Those write permissions belong only in the release job after owner approval. They are not added to pull-request CI.

## Accepted risks for v0.1

- Process names and allowlisted environment hints can be spoofed by the same local user; they are provenance, not authentication.
- The parent shell session is intentionally opaque, so some effective command selections remain unknown.
- The unsigned desktop executable is a local development artifact only. SmartScreen reputation, publisher identity, installer integrity, and automatic-update safety are not claimed.
- Automatic paired diagnosis depends on a separately installed WSL companion found by the WSL login-shell PATH. A modified same-name executable controlled by the local user can return deceptive observations; the result is diagnostic provenance, not authentication.
- Reading metadata and a 512-byte prefix cannot establish publisher identity, code safety, or absence of malware.
- Files can change between observations; ExecLocus does not lock project or executable files.
- Redaction covers modeled identity/path classes and is not a general secret detector for arbitrary pasted content. Users must not paste raw terminal or raw JSON into public reports.
- License checks trust package metadata and recognized license files; they are not legal advice or an exhaustive source-file audit.
- RustSec detects publicly known advisories, not unknown vulnerabilities.
- Exact duplicate exceptions for `syn 2.0.119` and `windows-link 0.1.3` remain while upstream Windows/proc-macro dependency lines differ. New duplicates still surface.
- Release attestation proves build provenance, not that the source or binary is vulnerability-free.

Any accepted risk that changes impact, becomes remotely exploitable, causes a shareable-output leak, or receives a relevant advisory blocks v0.1 until reassessed.
