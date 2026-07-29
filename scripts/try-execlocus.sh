#!/usr/bin/env bash
set -uo pipefail

profile="${1:-balanced}"
case "$profile" in
  share-first|balanced|linux-first) ;;
  *)
    printf 'Profile must be share-first, balanced, or linux-first.\n' >&2
    exit 2
    ;;
esac

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"

cargo_command="$(command -v cargo 2>/dev/null || true)"
if [[ -z "$cargo_command" && -n "${HOME:-}" && -x "$HOME/.cargo/bin/cargo" ]]; then
  cargo_command="$HOME/.cargo/bin/cargo"
fi
if [[ -z "$cargo_command" ]]; then
  printf 'Rust/Cargo was not found. Install the free Rust toolchain from https://rustup.rs/ and reopen the shell.\n' >&2
  exit 2
fi

cd "$repository_root"
printf '[1/2] Building ExecLocus from the checked-out source...\n'
if ! "$cargo_command" build --locked; then
  printf 'cargo build failed.\n' >&2
  exit 2
fi

output_directory="$repository_root/target/user-validation"
mkdir -p "$output_directory"
report_path="$output_directory/wsl-$profile.md"
json_path="$output_directory/wsl-$profile.redacted.json"

printf '[2/2] Creating an automatically redacted Markdown report...\n'
"$cargo_command" run --quiet --locked -- --profile "$profile" report --format markdown >"$report_path"
report_exit_code=$?
if (( report_exit_code >= 2 )); then
  printf 'ExecLocus could not create the report (exit code %s).\n' "$report_exit_code" >&2
  exit "$report_exit_code"
fi
"$cargo_command" run --quiet --locked -- --profile "$profile" report --format json --redact >"$json_path"
json_exit_code=$?
if (( json_exit_code >= 2 )); then
  printf 'ExecLocus could not create the redacted JSON report (exit code %s).\n' "$json_exit_code" >&2
  exit "$json_exit_code"
fi

printf '\nDone. Review the redacted reports locally:\n  %s\n  %s\n' "$report_path" "$json_path"
printf 'The target directory is ignored by Git. Do not publish raw terminal or raw JSON output.\n'

if [[ "${SHOW_LOCAL_DETAILS:-0}" == "1" ]]; then
  printf '\nWARNING: The following terminal output may contain local absolute paths. Keep it on this machine.\n' >&2
  "$cargo_command" run --quiet --locked -- --profile "$profile" check || true
  "$cargo_command" run --quiet --locked -- --profile "$profile" explain FS001
fi
