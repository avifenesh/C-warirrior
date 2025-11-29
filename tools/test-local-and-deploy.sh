#!/bin/bash
# Code Warrior - Local E2E (localhost) then Deploy
# Usage: ./tools/test-local-and-deploy.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   Code Warrior - Local E2E (localhost) → Deploy & Prod    ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

"$SCRIPT_DIR/test-local-e2e.sh"

echo ""
echo "✅ Local tests passed, starting deploy & prod validation..."
echo ""

cd "$ROOT_DIR"
./tools/deploy-and-validate.sh

