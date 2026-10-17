@echo off
title Uninstall 36Julay Bangla Keyboard
echo Uninstalling 36Julay Bangla Keyboard...
taskkill /f /im 36Julay.exe >nul 2>&1
taskkill /f /im July36.exe >nul 2>&1
taskkill /f /im bijoy-app.exe >nul 2>&1
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "36Julay" /f >nul 2>&1
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "July36" /f >nul 2>&1
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "Bijoy36July" /f >nul 2>&1
del /f /q "%USERPROFILE%\Desktop\36Julay Bangla Keyboard.lnk" >nul 2>&1
del /f /q "%USERPROFILE%\Desktop\July36 Keyboard.lnk" >nul 2>&1
del /f /q "%USERPROFILE%\Desktop\বিজয় ৩৬জুলাই বাংলা কীবোর্ড.lnk" >nul 2>&1
rmdir /s /q "%APPDATA%\Microsoft\Windows\Start Menu\Programs\36Julay" >nul 2>&1
rmdir /s /q "%APPDATA%\Microsoft\Windows\Start Menu\Programs\July36" >nul 2>&1
rmdir /s /q "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Bijoy 36July" >nul 2>&1
rmdir /s /q "%LOCALAPPDATA%\36Julay" >nul 2>&1
rmdir /s /q "%LOCALAPPDATA%\July36" >nul 2>&1
rmdir /s /q "%LOCALAPPDATA%\Bijoy36July" >nul 2>&1
echo 36Julay Bangla Keyboard uninstalled successfully.
pause

