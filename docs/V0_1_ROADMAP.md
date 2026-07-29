# ExecLocus v0.1 delivery roadmap

- Snapshot: 2026-07-29
- Current phase: technical prototype complete; v0.1 delivery in progress
- Milestone target: `v0.1.0`

This roadmap translates the approved MVP scope into independently reviewable work. The [support matrix](SUPPORT_MATRIX.md) remains the source of truth for what works today. A roadmap item is not an available feature until its implementation, tests, documentation, and named validation gate are complete.

## Ordered delivery plan

| Order | Work item | Current state | Tracking | Definition of done |
|---:|---|---|---|---|
| 1 | README alignment and v0.1 tracking | Complete in this change | [v0.1.0 milestone](https://github.com/Tanasu0417/execlocus/milestone/1) | Public status tables distinguish implemented and planned behavior; GitHub issues are attached to the v0.1 milestone |
| 2 | `FS001`, `FS002`, and three profile behaviors | Complete in [#19](https://github.com/Tanasu0417/execlocus/pull/19) | [#11](https://github.com/Tanasu0417/execlocus/issues/11) | `share-first`, `balanced`, and `linux-first` change advice or severity without changing observed filesystem facts; positive, non-triggering, and missing-evidence tests pass |
| 3 | `ENV001`, `ENV003`, and `ENV004` | Complete in [#20](https://github.com/Tanasu0417/execlocus/pull/20) | [#12](https://github.com/Tanasu0417/execlocus/issues/12) | Each rule has minimum evidence, a legitimate non-triggering setup, deterministic fixtures, and no private agent-state reads |
| 4 | `explain <RULE_ID>` | Complete in this change | [#13](https://github.com/Tanasu0417/execlocus/issues/13) | Implemented rules return evidence, rationale, and read-only suggested actions; unknown IDs produce exit code 2 |
| 5 | Production shell resolution and candidate display | Planned | [#14](https://github.com/Tanasu0417/execlocus/issues/14) | PowerShell, cmd, bash, and zsh contracts feed the production report where evidence exists; selected and losing candidates are visible; incomplete shell evidence remains explicit |
| 6 | Real demo and external prototype validation | Planned | [#16](https://github.com/Tanasu0417/execlocus/issues/16) | A privacy-reviewed real demo is recorded; at least 10 collaborators across 10 Windows/WSL environments yield at least 3 independently verified useful cases |
| 7 | Windows/Linux v0.1.0 release | Planned | [#17](https://github.com/Tanasu0417/execlocus/issues/17) | Versioned x86_64 binaries, SHA-256 checksums, release notes, install instructions, and clean-machine smoke tests are published |

## Security workstream

Security is a gate across every item rather than a final audit only.

The dedicated security assessment is tracked in [#15](https://github.com/Tanasu0417/execlocus/issues/15).

| Control | Current state | Required follow-up |
|---|---|---|
| Branch protection, CodeQL, Dependabot, secret scanning | Active | Keep required checks and zero unresolved high-severity alerts |
| Rust formatting, Clippy, MSRV, tests, package verification | Active | Run on every pull request |
| Shareable-report redaction | Verified | Add regression cases whenever a new field can contain identity or a path |
| Terminal and Markdown output safety | Partial | Escape control characters and prevent terminal/Markdown injection from observed names and paths |
| Dependency advisory and license policy | Partial | Add a free RustSec advisory check and a reproducible license/source policy without adding paid services |
| Release integrity | Planned | Publish checksums, immutable source references, least-privilege workflow permissions, and provenance/SBOM where the free public-repository tooling supports it |
| Threat model | Planned | Document trust boundaries for environment variables, process metadata, filesystem paths, executable headers, rendering, and release artifacts |

## Experience and design gates

The user experience is reviewed before the final MV is produced:

1. Implement the final terminal information hierarchy and candidate comparison.
2. Run the CLI on Windows native, WSL on `/mnt/c`, and WSL-native projects.
3. Review comprehension, density, keyboard readability, colors, and copy using privacy-safe captures.
4. Produce the 10-second real demo and 60-second MV only from verified output.
5. Keep the faceless otter as an optional guide layer; the diagnostic result must remain understandable without animation.

Local HTML, CSS, SVG, and static design work does not require a hosted service. Image generation, hosted Sites, subscription-quota model calls, X posting, and public deployment require a separate cost/privacy/publication decision under the [cost policy](../COST_POLICY.md).

## External action gates

The repository owner must explicitly approve or perform these actions when their turn arrives:

- use of subscription quota for generated image or video assets;
- public Sites deployment or another hosted landing page;
- posting the prepared X research sequence;
- inviting and recording third-party prototype feedback; and
- publishing the final GitHub release.

No raw terminal output, raw JSON report, credential, personal path, machine name, private project detail, or respondent-level identity is accepted through public feedback channels.

## v0.1 release gate

The release candidate is ready only when all of the following are true:

- all required v0.1 rules and `explain` behavior are implemented;
- Windows and Ubuntu-24.04 WSL checks pass on Rust 1.85 and stable;
- the vulnerability, dependency, secret, privacy, output-injection, and release-integrity reviews have no unresolved release-blocking finding;
- README examples match the packaged CLI;
- the real demo uses the exact release-candidate behavior;
- external validation meets the published 10-environment／3-useful-case threshold; and
- binaries and checksums pass clean-machine smoke tests before publication.
