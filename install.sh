#!/bin/sh

set -eu

REPO="h2depot/clipls"
BIN_NAME="clipls"
INSTALL_DIR="${HOME}/.local/bin"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        OS="linux"
        ;;
    Darwin)
        OS="macos"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

ASSET="${BIN_NAME}-${OS}-${ARCH}"

echo "Installing ${BIN_NAME}..."

mkdir -p "$INSTALL_DIR"

TMP_DIR="$(mktemp -d "${INSTALL_DIR}/.clipls-install.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' 0
trap 'exit 1' HUP INT TERM

curl -fsSL \
    "https://github.com/${REPO}/releases/latest/download/${ASSET}.tar.gz" \
    -o "${TMP_DIR}/${ASSET}.tar.gz"

tar -xzf "${TMP_DIR}/${ASSET}.tar.gz" -C "$TMP_DIR" "$BIN_NAME"
chmod +x "${TMP_DIR}/${BIN_NAME}"
mv -f "${TMP_DIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"

echo "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"
echo
echo "Make sure ${INSTALL_DIR} is in your PATH."
