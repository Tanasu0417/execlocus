# ExecLocus GUI guide

ExecLocus is a read-only tool for answering which OS layer and executable will be selected when one project is used from Windows and WSL2.

## What it helps prevent

- WSL work unexpectedly selecting Windows Node or Codex
- Dependencies, caches, settings, or credentials being created on the unintended side
- Git, Node, or agent behavior changing with the launch method
- Spending debugging time before separating code defects from Windows/WSL boundary issues

## Shortest workflow

1. Start the Windows app from the project you want to inspect.
2. Select a workflow priority. Use **Not sure** for the first run.
3. Select **Compare Windows and WSL**.
4. Review the Fix and Review counts under **What we learned**.
5. Open **Win / WSL diff** to see different selections first.
6. Open **Actions** for impact, suggested actions, verification commands, and rerun steps.
7. Use **Share safely** to copy redacted Markdown when asking another person for help.

## Diagnostic profiles

| UI label | Internal name | Use when |
|---|---|---|
| Windows sharing | `share-first` | Explorer, Windows editors, or Cowork need direct access to the same files |
| Not sure | `balanced` | This is the first run or the placement policy is undecided |
| WSL performance | `linux-first` | Linux production parity, build performance, and file watching matter most |

A profile changes how the same observation is explained. It never changes the environment.

## Prepare automatic Windows/WSL comparison

The Windows app calls a free local companion inside WSL. Run this once from the ExecLocus checkout inside WSL:

```bash
bash scripts/install-wsl-companion.sh
```

The script builds ExecLocus from source and installs it into the WSL Cargo binary directory. It does not use a hosted AI service, paid API, or API key.

## Automatic redaction

Before creating the share view, ExecLocus replaces usernames, machine names, home directories, and personal absolute paths. The local diagnostic view may still show real paths because it is intended for local inspection.

Raw JSON, terminal output, and full-screen screenshots are outside this protection. Do not publish them unchanged.

## Current limitations

- The project target is the directory used to launch the app. A native folder picker is a high-priority follow-up.
- Alias and function resolution is most accurate when the wrapper is started from the shell being inspected.
- The development executable is unsigned and does not yet include an installer or automatic update.
