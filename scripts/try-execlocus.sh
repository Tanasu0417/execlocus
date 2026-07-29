#!/usr/bin/env bash

_execlocus_try_main() (
  set -uo pipefail

  profile="${1:-balanced}"
  case "$profile" in
    share-first|balanced|linux-first) ;;
    *)
      printf 'Profile must be share-first, balanced, or linux-first.\n' >&2
      return 2
      ;;
  esac
  mode="${2:-report}"
  language="${3:-ja}"
  case "$mode" in
    report|gui) ;;
    *)
      printf 'Mode must be report or gui.\n' >&2
      return 2
      ;;
  esac
  case "$language" in
    en|ja) ;;
    *)
      printf 'Language must be en or ja.\n' >&2
      return 2
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
    return 2
  fi

  cd "$repository_root" || return 2
  printf '[1/2] Building ExecLocus from the checked-out source...\n'
  if ! "$cargo_command" build --locked; then
    printf 'cargo build failed.\n' >&2
    return 2
  fi

  output_directory="$repository_root/target/user-validation"
  mkdir -p "$output_directory"
  report_path="$output_directory/wsl-$profile.md"
  json_path="$output_directory/wsl-$profile.redacted.json"
  snapshot_path="$output_directory/.bash-session-$$.json"
  trap 'rm -f -- "$snapshot_path"' EXIT

  binding_items=()
  for command_name in codex claude git node npm; do
    command_kind="$(type -t -- "$command_name" 2>/dev/null || true)"
    case "$command_kind" in
      alias|function|builtin)
        # Do not serialize alias expansions or function bodies.
        binding_items+=("{\"kind\":\"$command_kind\",\"name\":\"$command_name\",\"source\":\"$command_kind:$command_name\"}")
        ;;
    esac
  done
  binding_json=""
  if (( ${#binding_items[@]} > 0 )); then
    binding_json="$(IFS=,; printf '%s' "${binding_items[*]}")"
  fi
  printf '{"shell":"bash","complete":true,"bindings":[%s]}\n' "$binding_json" >"$snapshot_path"

  if [[ "$mode" == "gui" ]]; then
    printf '[2/2] Starting the loopback-only, read-only GUI...\n'
    "$cargo_command" run --quiet --locked -- --shell-snapshot "$snapshot_path" --profile "$profile" --lang "$language" gui --open
    return $?
  fi

  printf '[2/2] Creating an automatically redacted Markdown report...\n'
  "$cargo_command" run --quiet --locked -- --shell-snapshot "$snapshot_path" --profile "$profile" --lang "$language" report --format markdown >"$report_path"
  report_exit_code=$?
  if (( report_exit_code >= 2 )); then
    printf 'ExecLocus could not create the report (exit code %s).\n' "$report_exit_code" >&2
    return "$report_exit_code"
  fi
  "$cargo_command" run --quiet --locked -- --shell-snapshot "$snapshot_path" --profile "$profile" report --format json --redact >"$json_path"
  json_exit_code=$?
  if (( json_exit_code >= 2 )); then
    printf 'ExecLocus could not create the redacted JSON report (exit code %s).\n' "$json_exit_code" >&2
    return "$json_exit_code"
  fi

  printf '\nDone. Review the redacted reports locally:\n  %s\n  %s\n' "$report_path" "$json_path"
  printf 'The target directory is ignored by Git. Do not publish raw terminal or raw JSON output.\n'

  if [[ "${SHOW_LOCAL_DETAILS:-0}" == "1" ]]; then
    printf '\nWARNING: The following terminal output may contain local absolute paths. Keep it on this machine.\n' >&2
    "$cargo_command" run --quiet --locked -- --shell-snapshot "$snapshot_path" --profile "$profile" --lang "$language" check || true
    "$cargo_command" run --quiet --locked -- --shell-snapshot "$snapshot_path" --profile "$profile" --lang "$language" explain FS001
  fi
)

if _execlocus_try_main "$@"; then
  _execlocus_try_status=0
else
  _execlocus_try_status=$?
fi
unset -f _execlocus_try_main
return "$_execlocus_try_status" 2>/dev/null || exit "$_execlocus_try_status"
