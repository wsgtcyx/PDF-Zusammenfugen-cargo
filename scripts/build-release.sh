#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="pdf-zusammenfugen"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
HOST_TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"
DIST_DIR="$ROOT_DIR/dist"
ARCHIVE_PATH="$DIST_DIR/${BIN_NAME}-v${VERSION}-${HOST_TRIPLE}.tar.gz"

mkdir -p "$DIST_DIR"

cargo build --release
cp "target/release/$BIN_NAME" "$DIST_DIR/$BIN_NAME"
tar -C "$DIST_DIR" -czf "$ARCHIVE_PATH" "$BIN_NAME"
rm -f "$DIST_DIR/$BIN_NAME"

echo "Build abgeschlossen: $ARCHIVE_PATH"
