# Try ExecLocus on Windows and WSL

This guide lets you run the implemented Rust CLI in your own environment and evaluate correctness and usefulness. The helper scripts always save an automatically redacted Markdown report under `target/user-validation/`, which Git ignores.

> ExecLocus is pre-alpha and has no binary release yet, so this flow builds from source. Rust, the dependencies, and Microsoft Build Tools are available at no charge. The first build may access the network to download free open-source crates; normal ExecLocus diagnostics do not use the network.

## Concept UI versus the real CLI

| Surface | Purpose | Data |
|---|---|---|
| [Interactive concept demo](demo/prototype/index.html) | Review the layout, Japanese/English switch, and Inspect → Compare → Explain → Share flow | Synthetic only |
| Rust CLI | Observe Windows/WSL, distribution, user, shell, executable candidates, and filesystem boundaries | Your local environment |

The concept demo does not run probes. Use the CLI flow below when evaluating real usefulness.

## Review the interaction first

After cloning the repository, open `docs/demo/prototype/index.html` in a browser. Use the `EN` / `日本語` control to switch the complete explanatory interface.

1. Read the evidence categories on Inspect.
2. Press `Run diagnostic` or `R`; confirm that the otter changes to its swimming pose before the transition to Compare.
3. Switch among `share-first`, `balanced`, and `linux-first`.
4. Open Explain and confirm that `/mnt/c` advice changes with intent.
5. Open Share and review which identifiers are removed.
6. Open `docs/demo/prototype/mv.html` and test playback, frame selection, language switching, and the land/swim pose changes.

Clipboard access may be blocked when the HTML file is opened directly. This does not affect the visual review. If Python is available, serve it locally from the repository root:

```console
python -m http.server 8765 --directory docs/demo
```

Then open `http://127.0.0.1:8765/prototype/index.html`. If you cannot explain the product within 30 seconds, record that as presentation feedback rather than a CLI detection failure.

## Prerequisites

- Git
- Rust 1.85 or newer, installed at no charge with [rustup](https://rustup.rs/)
- Windows: Microsoft C++ Build Tools and the Windows SDK
- WSL: Ubuntu 24.04 is the primary target; a C linker such as `cc` is required

Clone into a normal user-writable working directory, not `C:\WINDOWS\system32`. Because `git clone` creates the `execlocus` directory, enter it **after** cloning.

Windows PowerShell:

```powershell
Set-Location ([Environment]::GetFolderPath("MyDocuments"))
git clone https://github.com/Tanasu0417/execlocus.git
Set-Location .\execlocus
git rev-parse --show-toplevel
if (-not (Test-Path .\scripts\try-execlocus.ps1)) { throw "Not at the repository root, or the checkout is outdated" }
```

WSL:

```bash
cd ~
git clone https://github.com/Tanasu0417/execlocus.git
cd execlocus
git rev-parse --show-toplevel
test -f scripts/try-execlocus.sh || { pwd; echo "Not at the repository root, or the checkout is outdated"; exit 1; }
```

For an existing clone, enter its repository root and run `git pull --ff-only` instead of cloning again. Confirm that `git rev-parse --show-toplevel` identifies the current directory and that the `scripts` directory is present.

Record the commit ID if you need a reproducible pre-alpha test.

## Run from Windows PowerShell

```powershell
& .\scripts\try-execlocus.ps1
& .\scripts\try-execlocus.ps1 -Profile share-first
```

Running with `&` in the current PowerShell session lets the wrapper capture bounded `Get-Command -All` evidence for aliases, functions, and cmdlets. Reports are written to `target/user-validation/windows-<profile>.md` and `windows-<profile>.redacted.json`. Both formats contain redacted candidate details. Add `-ShowLocalDetails` only when you want to inspect raw terminal output locally. That output may contain personal absolute paths and must not be published.

## Run from WSL

```bash
source ./scripts/try-execlocus.sh
source ./scripts/try-execlocus.sh share-first
```

The reports are written to `target/user-validation/wsl-<profile>.md` and `wsl-<profile>.redacted.json`. To inspect raw details locally:

```bash
(SHOW_LOCAL_DETAILS=1 source ./scripts/try-execlocus.sh balanced)
```

Sourcing the wrapper captures alias, function, builtin, and PATH resolution evidence from the same bash session. It never stores alias expansions or function bodies, and deletes its bounded temporary JSON when the run ends.

If PowerShell reports `not recognized as the name of a script file`, or bash reports `No such file or directory`, the diagnostic has not started. Confirm that:

- `git rev-parse --show-toplevel` identifies the intended ExecLocus checkout;
- `Test-Path .\scripts\try-execlocus.ps1` returns `True` on Windows;
- `test -f scripts/try-execlocus.sh && echo OK` prints `OK` in WSL; and
- an existing checkout has been updated with `git pull --ff-only`.

To open the same Windows Documents checkout from WSL without hard-coding a personal path:

```bash
documents_win="$(powershell.exe -NoProfile -Command "[Environment]::GetFolderPath('MyDocuments')" | tr -d '\r')"
cd "$(wslpath "$documents_win")/execlocus"
source ./scripts/try-execlocus.sh balanced
```

Compare `windows-balanced.*` and `wsl-balanced.*` in the shared `target/user-validation/` directory. Use a separate non-sensitive test copy under the WSL filesystem if you also want to compare WSL-native placement.

## Review checklist

- Windows reports Windows and WSL reports WSL.
- The WSL distribution and shell evidence are reasonable.
- Codex or Claude Code is inferred only when process evidence exists; `Unknown` is valid when launched from an ordinary terminal.
- `Not found`, `Candidates found / selection unconfirmed`, `Selected`, and `Probe failed` match the available evidence.
- Git, Node, npm, and agent candidate counts, Windows/Linux origins, and PE/ELF/script formats match independently observed command resolution.
- Project storage is classified as Windows-mounted or WSL-native correctly.
- `share-first`, `balanced`, and `linux-first` explanations match their stated intent.
- The shareable Markdown contains no username, machine name, home directory, or personal absolute path.

The CLI alone cannot safely reconstruct aliases or functions from its parent. The wrappers pass only bounded binding kinds and fixed command names from the current session. Without that snapshot, ExecLocus lists external candidates as `Candidates found / selection unconfirmed` to avoid a false claim.

## Evaluate usefulness

Ask four questions:

1. Did it reveal a Windows/WSL boundary you did not already know?
2. Could you independently verify the conclusion?
3. Did it help you decide to change or keep the setup?
4. Would you use it again for the same uncertainty?

Submit only categorical results and a non-sensitive conclusion through the [field report](https://github.com/Tanasu0417/execlocus/issues/new?template=field_report.yml). Never post raw terminal/JSON output, credentials, usernames, machine names, personal paths, or private repository details. Recheck even an automatically redacted report before publishing it.
