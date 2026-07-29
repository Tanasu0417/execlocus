# Changelog

All notable changes to ExecLocus will be documented in this file.

The project is pre-alpha and has not published a versioned release. Entries remain under `Unreleased` until release artifacts and notes are verified.

## [Unreleased]

### Added

- Evidence-backed Windows, WSL, project-path, and executable-origin prototype
- Terminal and JSON report foundations
- Initial `ENV002`, `PATH001`, and `GIT001` rules
- Windows and Ubuntu CI, Dependabot, CodeQL, and security-reporting foundations
- English and Japanese project documentation
- Thirty-project OSS benchmark and adoption/launch planning documents
- Public use-case contracts and an explicit support matrix
- One-page product overviews, staged demo plan, alternatives analysis, and demand-validation plan
- Production-ready storyboard, synthetic recording scenario contract, and staged X research copy
- Claude Design handoff, timed narration, and original faceless otter motion specification for the Concept MV
- Privacy-first structured field-report template without a raw-output field
- OS-backed current-user observation and process-ancestry shell detection on Windows and Linux/WSL
- Provenance fields for distribution, user, and shell values in terminal and JSON reports
- JSON schema version `0.2.0` for runtime identity provenance fields
- Automatically redacted Markdown reports and explicit redacted JSON output
- Conservative Codex and Claude Code adapters based on exact names in bounded process ancestry
- JSON schema version `0.3.0` with separate agent-product and agent-runtime provenance
- Isolated real-environment validation of Claude Code launching ExecLocus inside Ubuntu-24.04 WSL

### Security

- Normal execution is designed to remain local-only and read-only
- Public examples, fixtures, and reports must exclude credentials and personal identity
- Shareable rendering removes usernames, home directories, machine names, and absolute paths
- CSV cells that begin with formula-significant characters are escaped for spreadsheet import

[Unreleased]: https://github.com/Tanasu0417/execlocus/commits/main
