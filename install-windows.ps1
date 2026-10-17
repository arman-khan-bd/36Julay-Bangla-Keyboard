# Windows Installation Script for Bijoy 36July Bangla Keyboard
# Installs application, creates desktop and start menu shortcuts, adds startup registry, and launches app

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "     Bijoy 36July Bangla Keyboard - Windows Installer       " -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# 1. Compile Release Binaries
Write-Host "[1/5] Building high-performance release binaries in Rust..." -ForegroundColor Green
cargo build --release --workspace

# Stop any running instances before updating files
Get-Process -Name "bijoy-app" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 500

$TargetDir = "$env:LOCALAPPDATA\Bijoy36July"
if (!(Test-Path -Path $TargetDir)) {
    New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null
}

# 2. Copy Executables
Write-Host "[2/5] Installing binaries to $TargetDir..." -ForegroundColor Green
Copy-Item "target\release\bijoy-app.exe" "$TargetDir\bijoy-app.exe" -Force
Copy-Item "target\release\bijoy-cli.exe" "$TargetDir\bijoy-cli.exe" -Force

# 3. Create Desktop Shortcut
Write-Host "[3/5] Creating Desktop Shortcut..." -ForegroundColor Green
$WshShell = New-Object -ComObject WScript.Shell
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$Shortcut = $WshShell.CreateShortcut("$DesktopPath\Bijoy 36July Bangla Keyboard.lnk")
$Shortcut.TargetPath = "$TargetDir\bijoy-app.exe"
$Shortcut.WorkingDirectory = "$TargetDir"
$Shortcut.Description = "Bijoy 36July Bangla Keyboard (BDS 1738:2018)"
$Shortcut.Save()

# 4. Create Start Menu Shortcut
$StartMenuPath = [Environment]::GetFolderPath("Programs") + "\Bijoy 36July"
if (!(Test-Path -Path $StartMenuPath)) {
    New-Item -ItemType Directory -Path $StartMenuPath -Force | Out-Null
}
$StartMenuShortcut = $WshShell.CreateShortcut("$StartMenuPath\Bijoy 36July Bangla Keyboard.lnk")
$StartMenuShortcut.TargetPath = "$TargetDir\bijoy-app.exe"
$StartMenuShortcut.WorkingDirectory = "$TargetDir"
$StartMenuShortcut.Save()

# 5. Enable Autostart on Windows Boot
Write-Host "[4/5] Adding to Windows Startup..." -ForegroundColor Green
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "Bijoy36July" -Value "`"$TargetDir\bijoy-app.exe`"" -Force

# 6. Launch Application
Write-Host "[5/5] Launching Bijoy 36July Top Keyboard Bar and Preview Window..." -ForegroundColor Yellow
Start-Process "$TargetDir\bijoy-app.exe"

Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host "  Installation Successful! (Shofolbhabe Install Shompurno)  " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
Write-Host "Top Floating Keyboard Bar & Preview Window are now running!" -ForegroundColor Cyan
Write-Host "Hotkeys:" -ForegroundColor White
Write-Host "  F12            : Quick Toggle English <-> Bijoy" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + V : Bijoy 36July Unicode Mode" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + B : Bijoy Classic (SutonnyMJ) Mode" -ForegroundColor Yellow
Write-Host "  Ctrl + Alt + E : English Mode" -ForegroundColor Yellow
Write-Host ""
