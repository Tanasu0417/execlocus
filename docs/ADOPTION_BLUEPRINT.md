# ExecLocus OSS adoption blueprint

- Status: approved direction before the next feature slice
- Based on: [30-project OSS benchmark](research/OSS_BENCHMARK_30.md)
- Updated: 2026-07-28

## Purpose

This document turns recurring patterns from successful individually owned OSS projects into decisions for ExecLocus. It does not copy another project's wording, visual identity, code, or repository layout. Each pattern is adapted to the specific Windows/WSL runtime problem and must be validated with ExecLocus users.

## Decisions derived from the benchmark

| Observed pattern | ExecLocus adaptation | Repository evidence | Gate |
|---|---|---|---|
| Connect to a familiar problem in one sentence | Lead with “which runtime and executable actually handled the command?” | README and product specification | Keep the first screen understandable without learning a new category |
| Show value in about ten seconds | Record one real Windows/WSL executable-resolution case | Real terminal capture, never a staged result presented as real | Capture only after the scenario passes on both platforms |
| Support humans and automation | Keep terminal and versioned JSON as equal contracts | Terminal renderer, JSON renderer, schema tests | Both outputs describe the same report |
| Explain evidence and limitations | Attach provenance and confidence; make `unknown` legitimate | Evidence model, rule contract, Non-goals | No warning from missing evidence alone |
| Treat platforms and shells as first-class | Publish an honest capability matrix | [Support matrix](SUPPORT_MATRIX.md) and CI | “Planned” is never presented as supported |
| Treat distribution as a product feature | Start with signed/checksummed binaries, then add package managers selectively | Release checklist and package allowlist | No release until artifacts are reproducible |
| Make output safe to share | Redact before rendering and collect sanitized field reports | Privacy tests and field-report Issue form | Zero personal paths, credentials, or machine identity in shared output |

## Current repository audit

| Area | Current state | Decision before more feature work |
|---|---|---|
| Positioning | Strong problem-first English and Japanese README | Keep; replace illustrative output with a real capture when available |
| Product scope | Product, MVP, rules, privacy, and Non-goals documented | Keep these as the scope authority |
| Community intake | Bug and feature forms exist | Add a privacy-first real-environment field report |
| Cross-platform confidence | Windows and Ubuntu CI exist | Add the declared Rust 1.85 MSRV check |
| Public claims | Implemented and planned items appear in several documents | Use one support matrix as the current truth |
| Release history | No public release and no changelog | Start an Unreleased changelog now |
| Package contents | Cargo package contents were implicit | Allowlist only source, manifest, lockfile, README, and license |
| Growth evidence | Launch plan exists | Tie outreach to three proven use cases and measurable field reports |

## Target repository structure

The target is intentionally small. A directory is introduced only when it has a real implementation or fixture; empty architecture is not progress.

```text
.
├── .github/
│   ├── ISSUE_TEMPLATE/       bug, feature, sanitized field report
│   └── workflows/            CI, security analysis, later release
├── docs/
│   ├── research/             source benchmark and launch analysis
│   ├── ADOPTION_BLUEPRINT.md product, engineering, and adoption decisions
│   ├── USE_CASES.md          three public scenario contracts
│   ├── SUPPORT_MATRIX.md     implemented versus planned capabilities
│   └── MVP_SCOPE.md          v0.1 boundary and milestones
├── src/
│   ├── probes/               read-only observations
│   ├── renderers/            terminal, JSON, later Markdown
│   ├── model.rs              normalized report contract; split when it grows
│   ├── rules.rs              deterministic findings; split when it grows
│   ├── lib.rs                orchestration
│   └── main.rs               CLI boundary
├── tests/                    add with the first cross-module fixture slice
│   ├── fixtures/             synthetic Windows/WSL scenarios
│   └── golden/               terminal, JSON, Markdown, and redaction outputs
├── CHANGELOG.md
├── PRODUCT_SPEC.md
├── RULES.md
└── README.md / README.ja.md
```

Planned `adapters/` and `privacy/` modules are created with their first tested feature, not as empty folders. The existing flat model and rule modules remain until splitting reduces real complexity.

## Development model: scenario-first vertical slices

Every feature must complete one user question across the stack instead of adding many disconnected probes.

1. Start from one contract in [USE_CASES.md](USE_CASES.md).
2. Add a sanitized positive fixture, legitimate non-triggering fixture, and missing-evidence fixture.
3. Collect the minimum read-only evidence.
4. Normalize evidence without storing unrestricted raw output.
5. Evaluate deterministic findings separately from probes.
6. Render the same report in terminal and JSON; include Markdown when implemented.
7. Apply redaction before any shareable serialization.
8. Update the support matrix and user-facing example in the same pull request.

### Definition of ready

- The user question is specific and belongs to one of the three initial use cases.
- Minimum evidence and unavailable behavior are defined.
- At least one intentional setup that must not warn is documented.
- Privacy impact and platform scope are known.

### Definition of done

- Positive, negative, and missing-evidence tests pass.
- Windows and Ubuntu CI pass; Rust 1.85 compatibility remains valid.
- Terminal and JSON do not contradict each other.
- Personal identity and secret-bearing values are absent from fixtures and logs.
- README or support-matrix claims match the implemented behavior.

## Development order

### Gate A — repository and evidence contracts

- Publish the benchmark, blueprint, use cases, and support matrix.
- Add MSRV and package-content checks.
- Collect field reports using only sanitized data.

### Gate B — prove the three questions

Required outcomes:

- Observe or safely infer the active Codex/Claude execution layer.
- Explain which Git, Node, npm, or agent executable wins and why.
- Explain `/mnt/c` according to user intent without treating it as inherently wrong.

Recommended implementation sequence:

1. Complete UC-02 executable resolution first because the prototype already has most of the evidence path and can produce the first honest demo.
2. Complete UC-03 profile-aware `/mnt/c` behavior by reusing the same fixture and finding contracts.
3. Implement UC-01 agent adapters after invocation/process evidence and `unknown` behavior are proven by fixtures.

Each question becomes a separate focused pull request when possible.

### Gate C — make evidence shareable

- Freeze the JSON schema version.
- Implement Markdown output and redaction-before-rendering.
- Add golden tests for usernames, home paths, machine names, and absolute paths.
- Validate ten real Windows/WSL environments and record only sanitized outcomes.

### Gate D — release and distribution

- Produce Windows x86_64 and Linux x86_64 binaries and SHA-256 checksums.
- Verify package contents and release notes.
- Publish a prerelease before crates.io or a Windows package manager.
- Promote only after the three scenarios are reproducible from the README.

## How to present and grow the project

### Message hierarchy

1. Problem: a terminal appearance does not prove which runtime or executable an agent used.
2. Proof: show the selected executable, alternatives, filesystem boundary, and evidence.
3. Safety: local-only, read-only, and redacted before sharing.
4. Scope: Windows and WSL first; no automatic repair and no generic system monitoring.

Avoid leading with “written in Rust,” “AI-powered,” or a long feature list. Those describe implementation or category, not the user's outcome.

### Proof assets

Create these from real, repeatable fixtures:

- one ten-second terminal capture showing Windows Node selected from WSL;
- one before/after comparison against manual `which`/`where` investigation;
- one sanitized Markdown report suitable for an Issue or agent conversation;
- one platform support matrix with explicit limitations;
- one release page containing binaries, checksums, known limitations, and three copyable scenarios.

### Channel order

1. GitHub prerelease for the first ten testers.
2. Problem-focused Japanese article and English project post after the demo is reproducible.
3. crates.io after package contents and the public contract are stable.
4. WinGet or Scoop after the Windows binary update path is repeatable.
5. Show HN only when a visitor can download and reproduce the main result immediately.

### Measures for the first validation cycle

| Measure | Gate | What it validates |
|---|---:|---|
| Sanitized real-environment reports | 10 | The setup exists outside the maintainer's machine |
| Previously unknown runtime/tool differences found | 3 or more | The core diagnosis creates new information |
| Reports that can be reproduced from supplied evidence | 90% | The evidence contract is useful |
| Confirmed privacy leaks | 0 | Sharing is safe enough to continue |
| Repeat users after the first run | Track before setting a target | The tool is more than a novelty |

Stars and downloads are secondary signals. They do not replace evidence that the product found a real, previously unclear boundary.

## Patterns deliberately not copied

- No TUI before the one-command report is trustworthy.
- No plugin ecosystem before the rule and privacy contracts stabilize.
- No broad system-information feature set.
- No simultaneous support for every package manager.
- No huge README that duplicates product and rule specifications.
- No fabricated benchmark, testimonial, terminal capture, or unsupported safety claim.
- No telemetry by default to manufacture usage metrics.

The reusable lesson from the 30 projects is focus, proof, portability, and disciplined distribution—not their exact code or presentation.
