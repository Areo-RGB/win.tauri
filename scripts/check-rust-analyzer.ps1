$ErrorActionPreference = 'Stop'

$Root = Split-Path -Parent $PSScriptRoot
$RustAnalyzer = Get-Command rust-analyzer -ErrorAction SilentlyContinue

if (-not $RustAnalyzer) {
  Write-Error "rust-analyzer is not in PATH. Put rust-analyzer.exe in PATH or install rustup/rust-analyzer."
}

Push-Location (Join-Path $Root 'src-tauri')
try {
  Write-Host "rust-analyzer version:" -ForegroundColor Cyan
  rust-analyzer --version

  Write-Host "Running cargo check for full Rust diagnostics..." -ForegroundColor Cyan
  cargo check

  Write-Host "Done." -ForegroundColor Green
} finally {
  Pop-Location
}
