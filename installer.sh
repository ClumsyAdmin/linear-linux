#!/usr/bin/env bash

set -euo pipefail

VERSION="${VERSION:-0.2.3}"
DEB_URL="${DEB_URL:-https://github.com/kleo-dev/linear-linux/releases/download/v${VERSION}/linear_${VERSION}_amd64.deb}"
BINARY_NAME="linear"
INSTALL_DIR="/opt/linear-linux-${VERSION}"
WRAPPER_PATH="/usr/local/bin/${BINARY_NAME}"
DESKTOP_PATH="/usr/share/applications/linear.desktop"
APP_NAME="Linear"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ICON_SVG="${SCRIPT_DIR}/assets/linear-icon.svg"
ICON_PNG="${SCRIPT_DIR}/assets/linear-app-icon.png"

if [[ ! -f "${ICON_SVG}" || ! -f "${ICON_PNG}" ]]; then
  echo "Icon assets missing. Expected ${ICON_SVG} and ${ICON_PNG}." >&2
  exit 1
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

echo "Downloading Linear ${VERSION}..."
curl -fsSL "${DEB_URL}" -o "${tmpdir}/linear.deb"

echo "Installing .deb package..."
sudo dpkg -i "${tmpdir}/linear.deb" || sudo apt-get install -f -y

echo "Installing icons..."
sudo install -Dm644 "${ICON_SVG}" /usr/share/icons/hicolor/scalable/apps/linear.svg
sudo install -Dm644 "${ICON_PNG}" /usr/share/icons/hicolor/512x512/apps/linear.png
sudo gtk-update-icon-cache -f /usr/share/icons/hicolor || true

echo "Linear ${VERSION} installed."
