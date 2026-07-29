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
2. Press `Run diagnostic` or `R` and confirm the transition to Compare.
3. Switch among `share-first`, `balanced`, and `linux-first`.
4. Open Explain and confirm that `/mnt/c` advice changes with intent.
5. Open Share and review which identifiers are removed.
6. Open `docs/demo/prototype/mv.html` and test playback, frame selection, and language switching.

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

```console
git clone https://github.com/Tanasu0417/execlocus.git
cd execlocus
```

Record the commit ID if you need a reproducible pre-alpha test.

## Run from Windows PowerShell

```powershell
pwsh -NoProfile -File .\scripts\try-execlocus.ps1
pwsh -NoProfile -File .\scripts\try-execlocus.ps1 -Profile share-first
```

The reports are written to `target/user-validation/windows-<profile>.md` and `windows-<profile>.redacted.json`. The JSON contains candidate details after redaction. Add `-ShowLocalDetails` only when you want to inspect raw terminal output locally. That output may contain personal absolute paths and must not be published.

## Run from WSL

```bash
bash scripts/try-execlocus.sh
bash scripts/try-execlocus.sh share-first
```

The reports are written to `target/user-validation/wsl-<profile>.md` and `wsl-<profile>.redacted.json`. To inspect raw details locally:

```bash
SHOW_LOCAL_DETAILS=1 bash scripts/try-execlocus.sh balanced
```

Run both scripts against the same repository under `/mnt/c` to compare Windows and WSL execution for one source tree. Use a separate non-sensitive test copy under the WSL filesystem if you also want to compare WSL-native placement.

## Review checklist

- Windows reports Windows and WSL reports WSL.
- The WSL distribution and shell evidence are reasonable.
- Codex or Claude Code is inferred only when process evidence exists; `Unknown` is valid when launched from an ordinary terminal.
- Git, Node, npm, and agent candidates match independently observed command resolution.
- Project storage is classified as Windows-mounted or WSL-native correctly.
- `share-first`, `balanced`, and `linux-first` explanations match their stated intent.
- The shareable Markdown contains no username, machine name, home directory, or personal absolute path.

The child process cannot safely reconstruct aliases, functions, or hash state from its parent shell. When that session evidence is missing, ExecLocus lists external candidates but keeps the effective selection `Unknown` to avoid a false claim.

## Evaluate usefulness

Ask four questions:

1. Did it reveal a Windows/WSL boundary you did not already know?
2. Could you independently verify the conclusion?
3. Did it help you decide to change or keep the setup?
4. Would you use it again for the same uncertainty?

Submit only categorical results and a non-sensitive conclusion through the [field report](https://github.com/Tanasu0417/execlocus/issues/new?template=field_report.yml). Never post raw terminal/JSON output, credentials, usernames, machine names, personal paths, or private repository details. Recheck even an automatically redacted report before publishing it.
