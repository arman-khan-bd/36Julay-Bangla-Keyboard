# Windows Installation Script for 36Julay Bangla Keyboard
# Installs application, creates desktop and start menu shortcuts, adds startup registry, and launches app

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "     36Julay Bangla Keyboard - Windows Installer            " -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# 1. Compile Release Binaries
Write-Host "[1/5] Building high-performance release binaries in Rust..." -ForegroundColor Green
cargo build --release --workspace

# Stop any running instances before updating files
Get-Process | Where-Object { $_.ProcessName -match "36Julay|July36|bijoy" } | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 700


$TargetDir = "$env:LOCALAPPDATA\36Julay"
if (!(Test-Path -Path $TargetDir)) {
    New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null
}

# 2. Copy Executables & Assets
Write-Host "[2/5] Installing binaries and assets to $TargetDir..." -ForegroundColor Green
Copy-Item "target\release\36Julay.exe" "$TargetDir\36Julay.exe" -Force
if (Test-Path "target\release\bijoy-cli.exe") {
    Copy-Item "target\release\bijoy-cli.exe" "$TargetDir\36Julay-cli.exe" -Force
}
if (Test-Path "assets\app_icon.ico") {
    Copy-Item "assets\app_icon.ico" "$TargetDir\app_icon.ico" -Force
}
if (Test-Path "assets\logo_white.png") {
    Copy-Item "assets\logo_white.png" "$TargetDir\logo_white.png" -Force
}
if (Test-Path "assets\logo_black.png") {
    Copy-Item "assets\logo_black.png" "$TargetDir\logo_black.png" -Force
}
if (Test-Path "assets\logo.png") {
    Copy-Item "assets\logo.png" "$TargetDir\logo.png" -Force
}

# Also keep copies directly in project root / dist folder for immediate access
$DistDir = "$PSScriptRoot\dist"
if (!(Test-Path -Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
}
Copy-Item "target\release\36Julay.exe" "$DistDir\36Julay.exe" -Force
Copy-Item "target\release\36Julay.exe" "$PSScriptRoot\36Julay.exe" -Force
Copy-Item "target\release\36Julay.exe" "$PSScriptRoot\36Julay Bangla Keyboard.exe" -Force
if (Test-Path "assets\app_icon.ico") {
    Copy-Item "assets\app_icon.ico" "$DistDir\app_icon.ico" -Force
}
if (Test-Path "assets\logo_white.png") {
    Copy-Item "assets\logo_white.png" "$DistDir\logo_white.png" -Force
}
if (Test-Path "assets\logo_black.png") {
    Copy-Item "assets\logo_black.png" "$DistDir\logo_black.png" -Force
}

# 3. Create Desktop Shortcut
Write-Host "[3/5] Creating Desktop Shortcut with 36Julay Icon..." -ForegroundColor Green
$WshShell = New-Object -ComObject WScript.Shell
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$Shortcut = $WshShell.CreateShortcut("$DesktopPath\36Julay Bangla Keyboard.lnk")
$Shortcut.TargetPath = "$TargetDir\36Julay.exe"
$Shortcut.WorkingDirectory = "$TargetDir"
if (Test-Path "$TargetDir\app_icon.ico") {
    $Shortcut.IconLocation = "$TargetDir\app_icon.ico, 0"
}
$Shortcut.Description = "36Julay Bangla Keyboard (৩৬জুলাই বাংলা কীবোর্ড)"
$Shortcut.Save()

# 4. Create Start Menu Shortcut
$StartMenuPath = [Environment]::GetFolderPath("Programs") + "\36Julay"
if (!(Test-Path -Path $StartMenuPath)) {
    New-Item -ItemType Directory -Path $StartMenuPath -Force | Out-Null
}
$StartMenuShortcut = $WshShell.CreateShortcut("$StartMenuPath\36Julay Bangla Keyboard.lnk")
$StartMenuShortcut.TargetPath = "$TargetDir\36Julay.exe"
$StartMenuShortcut.WorkingDirectory = "$TargetDir"
if (Test-Path "$TargetDir\app_icon.ico") {
    $StartMenuShortcut.IconLocation = "$TargetDir\app_icon.ico, 0"
}
$StartMenuShortcut.Save()

# 5. Enable Autostart on Windows Boot
Write-Host "[4/5] Adding to Windows Startup..." -ForegroundColor Green
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "36Julay" -Value "`"$TargetDir\36Julay.exe`"" -Force

# 6. Launch Application
Write-Host "[5/5] Launching 36Julay Top Keyboard Bar and Preview Window..." -ForegroundColor Yellow
Start-Process "$TargetDir\36Julay.exe"

Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host "  Installation Successful! (সফলভাবে ইনস্টল সম্পন্ন)          " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
Write-Host "36Julay Top Floating Keyboard Bar & Preview Window are now running!" -ForegroundColor Cyan
Write-Host "Hotkeys:" -ForegroundColor White
Write-Host "  F12            : Quick Toggle English <-> 36Julay" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + V : 36Julay Unicode Mode" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + B : Bijoy Classic (SutonnyMJ) Mode" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + E : English Mode" -ForegroundColor Yellow
Write-Host ""
