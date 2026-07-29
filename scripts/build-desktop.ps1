[CmdletBinding()]
param(
    [ValidateSet("Debug", "Release")]
    [string]$Configuration = "Debug"
)

$ErrorActionPreference = "Stop"
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $repositoryRoot "src-tauri\Cargo.toml"
$cargoCommand = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"

if (-not (Test-Path -LiteralPath $cargoCommand -PathType Leaf)) {
    throw "Rust is not installed for this Windows account. Install rustup first."
}

$arguments = @("+stable", "build", "--manifest-path", $manifestPath, "--locked")
if ($Configuration -eq "Release") {
    $arguments += "--release"
}

Write-Host "Building the local-only ExecLocus desktop app ($Configuration)..."
& $cargoCommand @arguments
if ($LASTEXITCODE -ne 0) {
    throw "The desktop build failed with exit code $LASTEXITCODE."
}

$profileDirectory = if ($Configuration -eq "Release") { "release" } else { "debug" }
$executablePath = Join-Path $repositoryRoot "src-tauri\target\$profileDirectory\execlocus-desktop.exe"
if (-not (Test-Path -LiteralPath $executablePath -PathType Leaf)) {
    throw "The build completed but the expected executable was not found: $executablePath"
}

Write-Host "Unsigned development executable:"
Write-Host $executablePath
