#!/usr/bin/env bash
set -e

REPO="VLOD-ZDOV/RMouseButtonControl"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"

echo "=== Installing RMouseButtonControl ==="

mkdir -p "$INSTALL_DIR" "$DESKTOP_DIR" "$ICON_DIR"

TARBALL_URL="https://github.com/$REPO/releases/latest/download/RMouseButtonControl-Linux-Qt.tar.gz"
TMP_DIR=$(mktemp -d)

echo "Downloading latest release from $TARBALL_URL..."
curl -sSL -o "$TMP_DIR/rmouse.tar.gz" "$TARBALL_URL"

echo "Extracting..."
tar -xzf "$TMP_DIR/rmouse.tar.gz" -C "$TMP_DIR"

echo "Installing binaries to $INSTALL_DIR..."
cp "$TMP_DIR/rmousebuttoncontrol" "$INSTALL_DIR/"
cp "$TMP_DIR/rmouse-gui" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/rmousebuttoncontrol" "$INSTALL_DIR/rmouse-gui"

# Create Desktop entry
cat <<EOF > "$DESKTOP_DIR/RMouseButtonControl.desktop"
[Desktop Entry]
Name=RMouseButtonControl
Comment=Mouse button remapping & macro utility
Exec=$INSTALL_DIR/rmouse-gui
Icon=input-mouse
Terminal=false
Type=Application
Categories=Utility;Settings;
EOF

chmod +x "$DESKTOP_DIR/RMouseButtonControl.desktop"

rm -rf "$TMP_DIR"

echo ""
echo "=== Installation Successful! ==="
echo "Binaries installed to: $INSTALL_DIR"
echo "You can launch RMouseButtonControl from your application menu or run:"
echo "  $INSTALL_DIR/rmousebuttoncontrol &   # background daemon"
echo "  $INSTALL_DIR/rmouse-gui              # configuration GUI"
