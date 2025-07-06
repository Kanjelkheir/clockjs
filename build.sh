#!/bin/bash
set -e

# Build script for compiling clock-timer to WebAssembly

echo "Building clock-timer WebAssembly package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the package with different targets
echo "Building for web..."
wasm-pack build --target web --out-dir dist/web

echo "Building for bundlers (webpack, rollup, etc.)..."
wasm-pack build --target bundler --out-dir dist/bundler

echo "Building for Node.js..."
wasm-pack build --target nodejs --out-dir dist/node

# Copy the README.md to the dist directory
echo "Copying README.md to dist directories..."
cp README.md dist/web/
cp README.md dist/bundler/
cp README.md dist/node/

echo "Build complete! The WebAssembly modules are available in the dist directory."
echo ""
echo "Usage instructions:"
echo "- For web: Use dist/web/clock_timer.js"
echo "- For bundlers: Use dist/bundler/clock_timer.js"
echo "- For Node.js: Use dist/node/clock_timer.js"
echo ""
echo "To publish to npm, run: cd dist/bundler && npm publish"
