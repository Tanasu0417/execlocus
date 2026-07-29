[CmdletBinding()]
param(
    [ValidateSet("share-first", "balanced", "linux-first")]
    [string]$Profile = "balanced",

    [switch]$ShowLocalDetails
)

$ErrorActionPreference = "Stop"
$scriptDirectory = Split-Path -Parent $MyInvocation.MyCommand.Path
$repositoryRoot = (Resolve-Path (Join-Path $scriptDirectory "..")).Path
$snapshotPath = $null

function New-ExecLocusPowerShellSnapshot {
    param([Parameter(Mandatory)][string]$Path)

    $bindings = foreach ($commandName in @("codex", "claude", "git", "node", "npm")) {
        foreach ($command in @(Get-Command -All -Name $commandName -ErrorAction SilentlyContinue)) {
            $kind = switch ($command.CommandType.ToString()) {
                "Alias" { "alias" }
                "Function" { "function" }
                "Cmdlet" { "cmdlet" }
                default { $null }
            }
            if ($kind) {
                # Do not serialize alias targets, function bodies, or paths. The
                # independent verification command can show those locally.
                [ordered]@{
                    kind = $kind
                    name = $commandName
                    source = "$kind`:$commandName"
                }
            }
        }
    }

    [ordered]@{
        shell = "power_shell"
        complete = $true
        bindings = @($bindings)
    } | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $Path -Encoding utf8
}

$cargoCommand = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoCommand -and $env:USERPROFILE) {
    $rustupCargo = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    if (Test-Path -LiteralPath $rustupCargo -PathType Leaf) {
        $cargoCommand = $rustupCargo
    }
}
if (-not $cargoCommand) {
    throw "Rust/Cargo was not found. Install the free Rust toolchain from https://rustup.rs/ and reopen PowerShell."
}

Push-Location $repositoryRoot
try {
    Write-Host "[1/2] Building ExecLocus from the checked-out source..."
    & $cargoCommand build --locked
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build failed with exit code $LASTEXITCODE."
    }

    $outputDirectory = Join-Path $repositoryRoot "target\user-validation"
    New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
    $reportPath = Join-Path $outputDirectory "windows-$Profile.md"
    $jsonPath = Join-Path $outputDirectory "windows-$Profile.redacted.json"
    $snapshotPath = Join-Path $outputDirectory ".powershell-session-$PID.json"
    New-ExecLocusPowerShellSnapshot -Path $snapshotPath

    Write-Host "[2/2] Creating an automatically redacted Markdown report..."
    & $cargoCommand run --quiet --locked -- --shell-snapshot $snapshotPath --profile $Profile report --format markdown |
        Set-Content -Encoding utf8 $reportPath
    $reportExitCode = $LASTEXITCODE
    if ($reportExitCode -ge 2) {
        throw "ExecLocus could not create the report (exit code $reportExitCode)."
    }
    & $cargoCommand run --quiet --locked -- --shell-snapshot $snapshotPath --profile $Profile report --format json --redact |
        Set-Content -Encoding utf8 $jsonPath
    $jsonExitCode = $LASTEXITCODE
    if ($jsonExitCode -ge 2) {
        throw "ExecLocus could not create the redacted JSON report (exit code $jsonExitCode)."
    }

    Write-Host ""
    Write-Host "Done. Review the redacted report locally:"
    Write-Host "  $reportPath"
    Write-Host "  $jsonPath"
    Write-Host "The target directory is ignored by Git. Do not publish raw terminal or raw JSON output."

    if ($ShowLocalDetails) {
        Write-Warning "The following terminal output may contain local absolute paths. Keep it on this machine."
        & $cargoCommand run --quiet --locked -- --shell-snapshot $snapshotPath --profile $Profile check
        & $cargoCommand run --quiet --locked -- --shell-snapshot $snapshotPath --profile $Profile explain FS001
    }
}
finally {
    if ($snapshotPath -and (Test-Path -LiteralPath $snapshotPath)) {
        Remove-Item -LiteralPath $snapshotPath -Force
    }
    Pop-Location
}
