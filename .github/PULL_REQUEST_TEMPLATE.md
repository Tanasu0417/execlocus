## Summary

Describe the focused change and why it belongs in ExecLocus.

## User question

Which question in `docs/USE_CASES.md` does this change answer? For documentation or maintenance work, write `N/A` and explain the repository outcome.

## Evidence contract

- Evidence added or changed:
- Missing-evidence behavior:
- Legitimate non-triggering setup:
- Terminal/JSON/Markdown behavior affected:
- Privacy impact:

## Validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-targets --all-features`
- [ ] Rust 1.85 compatibility is preserved
- [ ] Positive, non-triggering, and missing-evidence cases are covered or marked `N/A`
- [ ] `docs/SUPPORT_MATRIX.md` and user-facing examples remain accurate
- [ ] No credentials, personal paths, or identifying information are included

## Cost impact

- [ ] This change follows the [cost policy](https://github.com/Tanasu0417/execlocus/blob/main/COST_POLICY.md).
- [ ] This change does not add paid or metered services, billing credentials,
      paid runners, or storage that could create an incremental charge.
- Approved exception and maximum charge (write `N/A` when none):
