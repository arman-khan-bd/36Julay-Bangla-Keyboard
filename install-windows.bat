@echo off
chcp 65001 >nul
title Bijoy 36July Bangla Keyboard Installer
echo ============================================================
echo     Bijoy 36July Bangla Keyboard Installer (Windows)
echo ============================================================
echo.
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install-windows.ps1"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Installation failed! Please check if Rust/Cargo is installed.
    pause
) else (
    echo.
    echo Press any key to finish...
    pause >nul
)
