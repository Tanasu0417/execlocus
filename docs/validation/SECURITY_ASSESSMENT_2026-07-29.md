# v0.1 security assessment — 2026-07-29

This assessment retains categorical results and synthetic test values only. No raw environment dump, raw terminal/JSON report, username, machine name, personal absolute path, credential, token, billing identifier, or private repository detail is included.

## Result

No release-blocking finding was detected in the reviewed source, dependency graph, current GitHub security alerts, or automated security regression suite. Release integrity remains a planned gate because binaries do not exist yet.

## Dependency and advisory gate

| Check | Tool/policy | Result |
|---|---|---|
| RustSec advisories | `cargo-deny 0.19.4`, current advisory fetch, committed lockfile | pass |
| Unsound advisories | all dependency levels | pass |
| Yanked releases | deny | pass |
| Licenses | MIT, Apache-2.0, Unicode-3.0 allowlist | pass |
| Sources | crates.io registry only; unknown registry/Git denied | pass |
| Wildcards | denied | pass |
| Duplicate lines | warning by default; exact reasoned exceptions for current transitive `syn` and `windows-link` lines | pass with accepted risk |

The fixed tool version is installed from crates.io with its published lockfile in CI. The weekly job runs on a standard `ubuntu-latest` public-repository runner, requests `contents: read`, persists no checkout credential, uploads no artifact/cache, and requires no API key or billing account.

At the assessment snapshot, the GitHub API reported 0 open Dependabot alerts, 0 open code-scanning alerts, and 0 open secret-scanning alerts.

## Source and behavior review

| Area | Evidence | Result |
|---|---|---|
| Unsafe Rust | workspace lint `unsafe_code = "forbid"` | pass |
| Command execution | no production `Command::new`, shell-string execution, profile sourcing, or executable launch | pass |
| Network/telemetry | no network client dependency or normal-runtime upload path | pass |
| Bounded reads | executable prefix 512 bytes; kernel and OS identity files 64 KiB | pass |
| Agent secrets | configuration locations classified without content reads | pass |
| Terminal injection | newline, tab, ESC, and other controls escaped | pass |
| Markdown injection | table, Markdown image/link prefix, raw HTML delimiters, and controls neutralized | pass |
| Redaction bypass | mixed-case secrets, modeled paths, unmodeled absolute paths, and quoted `key=path` values | pass |
| Path edges | `/mnt/c`, Windows verbatim drive paths, regular and verbatim WSL UNC paths | pass |
| Malformed headers | truncated PE/ELF markers and invalid bytes remain unknown without panic | pass |
| Partial failures | candidate inspection failure preserves a remaining result at reduced confidence and records the failure | pass |

## GitHub automation review

- Workflow default permission is `contents: read`.
- `actions/checkout` and `dtolnay/rust-toolchain` are pinned to full commit SHAs.
- Checkout uses `persist-credentials: false`.
- Jobs have explicit timeouts and concurrency cancellation.
- CodeQL default setup, Dependabot, secret scanning, push protection, and private vulnerability reporting are enabled.
- The dependency workflow uses no unapproved third-party Action; `cargo-deny` runs as a pinned Rust binary.

GitHub recommends full-length Action commit SHAs and least-privilege workflow tokens: <https://docs.github.com/en/actions/reference/security/secure-use>.

## Release review decision

The release workflow is intentionally not created in this assessment. The v0.1 release PR must provide Windows/Linux binaries, SHA-256 checksums, an SPDX SBOM, artifact attestations, least-privilege release-only write permissions, and clean-machine verification. GitHub artifact attestation is available for public repositories and binds an artifact to its repository, workflow, and commit; it does not certify vulnerability absence.

## Accepted risks

Accepted v0.1 risks and the conditions that reopen them are maintained in [`../SECURITY_MODEL.md`](../SECURITY_MODEL.md). No advisory or dependency ignore is present. The two exact duplicate-line exceptions are not vulnerability exceptions and will become stale warnings when upstream versions converge.
