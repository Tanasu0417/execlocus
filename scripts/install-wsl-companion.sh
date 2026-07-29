#!/usr/bin/env bash

set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"

cargo_command="$(command -v cargo 2>/dev/null || true)"
if [[ -z "$cargo_command" && -n "${HOME:-}" && -x "$HOME/.cargo/bin/cargo" ]]; then
  cargo_command="$HOME/.cargo/bin/cargo"
fi
if [[ -z "$cargo_command" ]]; then
  printf 'Rust/Cargo was not found in WSL. Install the free Rust toolchain from https://rustup.rs/ first.\n' >&2
  exit 2
fi
if [[ -z "${HOME:-}" ]]; then
  printf 'HOME is unavailable in WSL, so the local companion cannot be installed safely.\n' >&2
  exit 2
fi

target_directory="${XDG_CACHE_HOME:-$HOME/.cache}/execlocus-target"
mkdir -p "$target_directory"
printf 'Building or updating the free local ExecLocus WSL companion from this checkout...\n'
CARGO_TARGET_DIR="$target_directory" "$cargo_command" install --path "$repository_root" --locked --force

installed="$(command -v execlocus 2>/dev/null || true)"
if [[ -z "$installed" && -x "${HOME:-}/.cargo/bin/execlocus" ]]; then
  installed="$HOME/.cargo/bin/execlocus"
fi
if [[ -z "$installed" ]]; then
  printf 'Installation finished, but execlocus is not on the WSL PATH. Add $HOME/.cargo/bin to PATH and retry.\n' >&2
  exit 2
fi

printf 'WSL companion ready: %s\n' "$installed"
printf 'No hosted service, API key, or paid resource was used.\n'
