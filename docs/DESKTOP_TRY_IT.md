# Try the ExecLocus Windows desktop development build

This flow opens the same read-only diagnostic GUI in a native Windows window instead of a browser tab. It is a pre-alpha, **unsigned development build**, not an installer or a public release artifact.

## Cost and network boundary

- Tauri, Rust, WebView2, and Microsoft C++ Build Tools add no usage fee to this local build.
- The first build connects to crates.io only to fetch free open-source dependencies.
- Normal diagnostics call no external API, hosted AI model, telemetry endpoint, or cloud service.
- The UI and diagnostic API use only a random `127.0.0.1` port owned by the same process.

## Build and run

Use Windows 10 1803 or newer, Rust 1.88.0 or newer, Microsoft C++ Build Tools, the Windows SDK, and WebView2 Runtime. The CLI-only MSRV remains 1.85.0. From the repository root:

```powershell
git rev-parse --show-toplevel
Test-Path .\scripts\build-desktop.ps1
& .\scripts\build-desktop.ps1 -Configuration Debug
$desktop = Resolve-Path .\src-tauri\target\debug\execlocus-desktop.exe
Get-AuthenticodeSignature -LiteralPath $desktop | Select-Object Status
& $desktop
```

`NotSigned` is expected for this development build. Run it only when you built it yourself from a source revision you reviewed. Code signing and Store distribution remain separate release decisions that may involve identity verification and cost.

## Validation checklist

1. Select `Run diagnostic` and confirm `complete · 0 values uploaded`.
2. Review the five-tool summary under Compare.
3. Expand only the rows that need candidate, origin, format, reason, and verification details.
4. Test `Expand all` and `Collapse all`.
5. Review impact, actions, and reverification under Explain.
6. Toggle English/Japanese and confirm the real result changes language.
7. Confirm Share contains only the redacted Markdown.
8. Close the window and confirm the process-local loopback listener exits too.

Inspect, Compare, and Explain can contain local absolute paths and are not publication surfaces. Review even the redacted Share output before posting it.

The desktop process cannot reconstruct aliases or functions from an already-open PowerShell session. Use the session wrapper when exact current-shell precedence matters:

```powershell
& .\scripts\try-execlocus.ps1 -Gui -Language en -Profile balanced
```

The next desktop candidate is a side-by-side Windows/WSL launch flow, not automatic fixes or hosted AI. It should proceed only after this local interaction proves useful.
