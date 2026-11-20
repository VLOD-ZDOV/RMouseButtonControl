# RMouseButtonControl Installer for Windows
$ErrorActionPreference = "Stop"

$Repo = "VLOD-ZDOV/RMouseButtonControl"
$SetupUrl = "https://github.com/$Repo/releases/latest/download/RMouseButtonControl-Windows-Setup.exe"
$TempFile = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "RMouseButtonControl-Setup.exe")

Write-Host "=== RMouseButtonControl Windows Installer ===" -ForegroundColor Cyan
Write-Host "Downloading installer from $SetupUrl..." -ForegroundColor Gray

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $SetupUrl -OutFile $TempFile -UseBasicParsing
    Write-Host "Launching Setup wizard..." -ForegroundColor Green
    Start-Process -FilePath $TempFile -Wait
    Write-Host "Setup finished successfully!" -ForegroundColor Green
}
catch {
    Write-Error "Failed to download or run setup: $_"
}
finally {
    if (Test-Path $TempFile) {
        Remove-Item -Force $TempFile -ErrorAction SilentlyContinue
    }
}
