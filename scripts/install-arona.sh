#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARONA_DIR="$PROJECT_ROOT/apps/bluearch-arona"

echo "Building Arona..."
cd "$ARONA_DIR"
cargo build --release

echo "Installing Arona..."
sudo install -Dm755 target/release/arona /usr/local/bin/arona

echo "Arona installed successfully."
arona version
