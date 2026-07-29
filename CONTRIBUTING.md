# Contributing to ExecLocus

ExecLocus welcomes focused bug reports, diagnostic-rule proposals, documentation
improvements, and portable test fixtures.

## Before contributing

- Search existing issues before opening a new one.
- Use synthetic paths and identities in examples and fixtures.
- Never submit credentials, tokens, private keys, machine names, personal absolute
  paths, or unrestricted environment dumps.
- Report suspected vulnerabilities through private vulnerability reporting as
  described in [SECURITY.md](SECURITY.md).

## Development checks

Install the stable Rust toolchain with `rustfmt` and `clippy`, then run:

```console
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Windows development with the MSVC Rust target also requires the Microsoft C++
linker and Windows SDK. WSL/Linux requires a C linker such as `cc`.

## Cost policy

Development has a default incremental budget of 0 JPY. Do not add or invoke a
metered API, hosted AI model, paid runner, cloud resource, paid external
service, or paid-subscription quota without explicit approval before use.
Trials and promotional credits are not treated as free. See
[COST_POLICY.md](COST_POLICY.md) for the approval and CI requirements.

## Diagnostic rule requirements

A new rule must include:

1. a reproducible failure or material tradeoff;
2. the minimum evidence required to trigger;
3. a legitimate setup that must not trigger;
4. deterministic fixtures for triggering and non-triggering cases;
5. suggestions that remain read-only and profile-aware.

See [RULES.md](RULES.md) for the complete rule contract.

## Pull requests

Keep each pull request focused. Explain the observed evidence, user impact, privacy
impact, and validation performed. By contributing, you agree that your contribution
is provided under the repository's [MIT License](LICENSE).
