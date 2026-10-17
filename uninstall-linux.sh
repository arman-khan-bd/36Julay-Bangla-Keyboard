#!/usr/bin/env bash
# Uninstallation Script for Bijoy 36July Bangla Keyboard
echo "Uninstalling Bijoy 36July..."
killall bijoy-app 2>/dev/null || true
rm -f "${HOME}/.local/bin/bijoy-app"
rm -f "${HOME}/.local/bin/bijoy-cli"
rm -f "${HOME}/.local/share/applications/bijoy-36july.desktop"
rm -f "${HOME}/.config/autostart/bijoy-36july.desktop"
rm -f "${HOME}/.local/share/ibus/component/bijoy.xml"
if command -v ibus >/dev/null 2>&1; then
    ibus restart || true
fi
echo "Bijoy 36July uninstalled successfully."
