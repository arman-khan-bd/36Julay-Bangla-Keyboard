# Windows Installation Script for July36 Bangla Keyboard
# Installs application, creates desktop and start menu shortcuts, adds startup registry, and launches app

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "         July36 Bangla Keyboard - Windows Installer         " -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# 1. Compile Release Binaries
Write-Host "[1/5] Building high-performance release binaries in Rust..." -ForegroundColor Green
cargo build --release --workspace

# Stop any running instances before updating files
Get-Process -Name "July36", "bijoy-app" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 500

$TargetDir = "$env:LOCALAPPDATA\July36"
if (!(Test-Path -Path $TargetDir)) {
    New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null
}

# 2. Copy Executables
Write-Host "[2/5] Installing binaries to $TargetDir..." -ForegroundColor Green
Copy-Item "target\release\July36.exe" "$TargetDir\July36.exe" -Force
if (Test-Path "target\release\bijoy-cli.exe") {
    Copy-Item "target\release\bijoy-cli.exe" "$TargetDir\July36-cli.exe" -Force
}

# Also keep a copy directly in project root / dist folder for immediate access
$DistDir = "$PSScriptRoot\dist"
if (!(Test-Path -Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
}
Copy-Item "target\release\July36.exe" "$DistDir\July36.exe" -Force
Copy-Item "target\release\July36.exe" "$PSScriptRoot\July36.exe" -Force

# 3. Create Desktop Shortcut
Write-Host "[3/5] Creating Desktop Shortcut..." -ForegroundColor Green
$WshShell = New-Object -ComObject WScript.Shell
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$Shortcut = $WshShell.CreateShortcut("$DesktopPath\July36 Keyboard.lnk")
$Shortcut.TargetPath = "$TargetDir\July36.exe"
$Shortcut.WorkingDirectory = "$TargetDir"
$Shortcut.Description = "July36 Bangla Keyboard (BDS 1738:2018)"
$Shortcut.Save()

# 4. Create Start Menu Shortcut
$StartMenuPath = [Environment]::GetFolderPath("Programs") + "\July36"
if (!(Test-Path -Path $StartMenuPath)) {
    New-Item -ItemType Directory -Path $StartMenuPath -Force | Out-Null
}
$StartMenuShortcut = $WshShell.CreateShortcut("$StartMenuPath\July36 Keyboard.lnk")
$StartMenuShortcut.TargetPath = "$TargetDir\July36.exe"
$StartMenuShortcut.WorkingDirectory = "$TargetDir"
$StartMenuShortcut.Save()

# 5. Enable Autostart on Windows Boot
Write-Host "[4/5] Adding to Windows Startup..." -ForegroundColor Green
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "July36" -Value "`"$TargetDir\July36.exe`"" -Force

# 6. Launch Application
Write-Host "[5/5] Launching July36 Top Keyboard Bar and Preview Window..." -ForegroundColor Yellow
Start-Process "$TargetDir\July36.exe"

Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host "  Installation Successful! (Shofolbhabe Install Shompurno)  " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
Write-Host "July36 Top Floating Keyboard Bar & Preview Window are now running!" -ForegroundColor Cyan
Write-Host "Hotkeys:" -ForegroundColor White
Write-Host "  F12            : Quick Toggle English <-> July36" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + V : July36 Unicode Mode" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + B : Bijoy Classic (SutonnyMJ) Mode" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + E : English Mode" -ForegroundColor Yellow
Write-Host ""
