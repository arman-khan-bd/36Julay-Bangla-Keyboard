@echo off
title Uninstall Bijoy 36July Bangla Keyboard
echo Uninstalling Bijoy 36July...
taskkill /f /im bijoy-app.exe >nul 2>&1
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "Bijoy36July" /f >nul 2>&1
del /f /q "%USERPROFILE%\Desktop\বিজয় ৩৬জুলাই বাংলা কীবোর্ড.lnk" >nul 2>&1
rmdir /s /q "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Bijoy 36July" >nul 2>&1
rmdir /s /q "%LOCALAPPDATA%\Bijoy36July" >nul 2>&1
echo Bijoy 36July uninstalled successfully.
pause
