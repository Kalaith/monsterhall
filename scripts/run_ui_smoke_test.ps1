param(
    [string]$Scenario = "smoke",
    [switch]$UpdateBaseline,
    [switch]$SkipBuild,
    [string]$Python = "python"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$runnerScript = Join-Path $repoRoot "tmp_screens\play_ui_test.py"

if (-not (Test-Path $runnerScript)) {
    throw "UI test runner not found at $runnerScript"
}

if (-not $SkipBuild) {
    Write-Host "Building debug game executable..."
    cargo build
}

$pythonArgs = @(
    $runnerScript
    "--scenario"
    $Scenario
)

if ($UpdateBaseline) {
    $pythonArgs += "--update-baseline"
}

Write-Host "Running UI smoke harness..."
& $Python @pythonArgs

if ($LASTEXITCODE -ne 0) {
    throw "UI smoke harness failed with exit code $LASTEXITCODE"
}
