#!/usr/bin/env bash
# Linux Installation Script for Bijoy 36July Bangla Keyboard
# Supports Ubuntu, Debian, Fedora, Arch Linux, openSUSE, Pop!_OS, etc.

set -e

echo "============================================================"
echo "    বিজয় ৩৬জুলাই বাংলা কীবোর্ড (Bijoy 36July) Linux Installer"
echo "============================================================"

# 1. Build release binaries
echo "[1/5] Building release binaries in Rust..."
cargo build --release --workspace

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "${INSTALL_DIR}"

# 2. Copy binaries
echo "[2/5] Installing binaries to ${INSTALL_DIR}..."
cp "target/release/bijoy-app" "${INSTALL_DIR}/bijoy-app"
cp "target/release/bijoy-cli" "${INSTALL_DIR}/bijoy-cli"
chmod +x "${INSTALL_DIR}/bijoy-app" "${INSTALL_DIR}/bijoy-cli"

# 3. Create .desktop launcher
echo "[3/5] Creating Desktop & Application Launcher..."
DESKTOP_DIR="${HOME}/.local/share/applications"
mkdir -p "${DESKTOP_DIR}"

cat <<EOF > "${DESKTOP_DIR}/bijoy-36july.desktop"
[Desktop Entry]
Name=বিজয় ৩৬জুলাই বাংলা কীবোর্ড
Comment=Bijoy 36July Bangla Keyboard (BDS 1738:2018)
Exec=${INSTALL_DIR}/bijoy-app
Icon=accessories-character-map
Terminal=false
Type=Application
Categories=Utility;InputMethod;
Keywords=bangla;bijoy;typing;keyboard;
EOF

chmod +x "${DESKTOP_DIR}/bijoy-36july.desktop"

# 4. Add to Autostart
echo "[4/5] Adding to Desktop Autostart..."
AUTOSTART_DIR="${HOME}/.config/autostart"
mkdir -p "${AUTOSTART_DIR}"
cp "${DESKTOP_DIR}/bijoy-36july.desktop" "${AUTOSTART_DIR}/"

# 5. IBus Registration
echo "[5/5] Registering IBus component for standard desktop IME..."
IBUS_DIR="${HOME}/.local/share/ibus/component"
mkdir -p "${IBUS_DIR}"
"${INSTALL_DIR}/bijoy-cli" linux install-ibus || true

if command -v ibus >/dev/null 2>&1; then
    ibus restart || true
fi

echo ""
echo "✓ সফলভাবে ইনস্টল সম্পন্ন হয়েছে! (Installation Successful!)"
echo "শীর্ষে বিজয় কীবোর্ড বার এবং প্রিভিউ উইন্ডো চালু করতে চালান:"
echo "    ${INSTALL_DIR}/bijoy-app &"
echo ""
echo "GNOME/KDE কীবোর্ড সেটিংসে গিয়ে 'Bangla (Bijoy Keyboard BDS 1738)' সিলেক্ট করতে পারবেন।"

# Launch app in background
"${INSTALL_DIR}/bijoy-app" &
