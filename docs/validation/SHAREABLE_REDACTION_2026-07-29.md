# Sanitized shareable-output validation — 2026-07-29

This record validates redaction without retaining raw usernames, machine names, home directories, project paths, or executable paths.

## Contract

- `report --format markdown` always redacts before rendering and has no opt-out.
- `report --format json --redact` applies the same transformation before JSON serialization.
- Raw terminal and raw JSON reports remain local diagnostic outputs and must not be posted publicly.
- Redaction removes identity and path data; it is not a general-purpose secret scanner for arbitrary text supplied by another tool.

## Synthetic golden test

The privacy fixture contains synthetic Windows and Linux home directories, a synthetic username and machine name, Windows-mounted project data, Windows and Linux executable paths, an unexpected absolute evidence value, and a probe failure containing private text.

Both redacted JSON and the checked-in Markdown golden file are tested to exclude every synthetic private value. Findings and topology are regenerated from the redacted report so derived text cannot retain raw paths.

## Windows 11

| Check | Sanitized result |
|---|---|
| Markdown generated | pass |
| Redacted JSON generated | pass |
| Current username, home, machine, or project path present | no |
| Windows drive path present | no |
| Complete MSRV test suite | 31 passed |

## WSL2 / Ubuntu-24.04

| Check | Sanitized result |
|---|---|
| Markdown generated | pass |
| Redacted JSON generated | pass |
| Current username, home, machine, or project path present | no |
| `/mnt/<drive>/` path present | no |
| `/home/` path present | no |
| Complete MSRV test suite | 31 passed |

An unrelated host PATH translation warning was excluded from product claims and was not copied into repository fixtures.
