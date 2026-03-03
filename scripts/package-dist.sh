#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="pdf-zusammenfugen"
PACKAGE_NAME="$(sed -n 's/^name = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
DIST_DIR="$ROOT_DIR/dist"

mkdir -p "$DIST_DIR"
find "$DIST_DIR" -maxdepth 1 -type f ! -name "checksums.txt" -delete

"$ROOT_DIR/scripts/build-release.sh"

cargo package --allow-dirty
CRATE_PATH="$ROOT_DIR/target/package/${PACKAGE_NAME}-${VERSION}.crate"
cp "$CRATE_PATH" "$DIST_DIR/${PACKAGE_NAME}-v${VERSION}.crate"

(
  cd "$DIST_DIR"
  shasum -a 256 ./*.tar.gz ./*.crate > checksums.txt
)

echo "Dist fertig: $DIST_DIR"
