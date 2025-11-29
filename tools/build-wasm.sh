#!/bin/bash
set -e

echo "Building WASM module..."

# Install wasm-pack if not present
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Navigate to WASM crate
cd "$(dirname "$0")/../src-wasm"

# Build WASM with release optimization
echo "Running wasm-pack build..."
wasm-pack build --target web --release

# Copy to frontend node_modules for Vite
echo "Linking WASM to frontend..."
mkdir -p ../src-ui/node_modules/code-warrior-wasm
cp -r pkg/* ../src-ui/node_modules/code-warrior-wasm/

echo "WASM build complete!"
echo "Output: src-wasm/pkg/"
echo "Linked to: src-ui/node_modules/code-warrior-wasm/"
