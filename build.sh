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

# Get version from root package.json
if command -v node &> /dev/null; then
    ROOT_VERSION=$(node -e "console.log(JSON.parse(require('fs').readFileSync('package.json', 'utf8')).version)")
    echo "Found version $ROOT_VERSION in root package.json"
else
    ROOT_VERSION=$(grep -oP '"version": "\K[^"]+' package.json)
    echo "Found version $ROOT_VERSION in root package.json (using grep)"
fi

# Update package.json files using a more reliable approach with jq if available
echo "Updating package.json files..."
if command -v jq &> /dev/null; then
    # Use jq for reliable JSON manipulation
    for dir in dist/bundler dist/web dist/node; do
        if [ -f "$dir/package.json" ]; then
            # Create a temporary file with the updated JSON
            jq --arg version "$ROOT_VERSION" '.name = "rust-ticker" | .version = $version | .main = "clockjs.js" | .files += ["clockjs.js"] | .files = (.files | unique)' "$dir/package.json" > "$dir/package.json.tmp"
            mv "$dir/package.json.tmp" "$dir/package.json"
            echo "Updated $dir/package.json with jq (version: $ROOT_VERSION)"
        fi
    done
else
    # Fallback to sed if jq is not available
    echo "Warning: jq not found, using sed for basic replacements (less reliable)"

    # Update package name
    sed -i 's/"name": "clock-timer"/"name": "rust-ticker"/g' dist/bundler/package.json
    sed -i 's/"name": "clock-timer"/"name": "rust-ticker"/g' dist/web/package.json
    sed -i 's/"name": "clock-timer"/"name": "rust-ticker"/g' dist/node/package.json

    # Update version
    sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"$ROOT_VERSION\"/g" dist/bundler/package.json
    sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"$ROOT_VERSION\"/g" dist/web/package.json
    sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"$ROOT_VERSION\"/g" dist/node/package.json

    # Update main entry point
    sed -i 's/"main": "clock_timer.js"/"main": "clockjs.js"/g' dist/bundler/package.json
    sed -i 's/"main": "clock_timer.js"/"main": "clockjs.js"/g' dist/web/package.json
    sed -i 's/"main": "clock_timer.js"/"main": "clockjs.js"/g' dist/node/package.json

    # Check if clockjs.js is already in files array to avoid duplicate entries
    for dir in dist/bundler dist/web dist/node; do
        if [ -f "$dir/package.json" ]; then
            if ! grep -q '"clockjs.js"' "$dir/package.json"; then
                # Add clockjs.js to files list if not present
                sed -i '/"clock_timer.d.ts"/a \ \ \ \ "clockjs.js",' "$dir/package.json"
            fi
        fi
    done
fi

# Validate JSON syntax
for dir in dist/bundler dist/web dist/node; do
    if [ -f "$dir/package.json" ]; then
        if command -v node &> /dev/null; then
            if ! node -e "JSON.parse(require('fs').readFileSync('$dir/package.json', 'utf8'))"; then
                echo "Error: Invalid JSON in $dir/package.json. Please check and fix manually."
                exit 1
            fi
        fi
    fi
done

echo "Build complete! The WebAssembly modules are available in the dist directory."
echo ""
echo "Usage instructions:"
echo "- For web: Use dist/web/clock_timer.js"
echo "- For bundlers: Use dist/bundler/clock_timer.js"
echo "- For Node.js: Use dist/node/clock_timer.js"
echo ""
echo "To publish to npm, run: cd dist/bundler && npm publish"
echo ""
echo "IMPORTANT: Do NOT publish from the root directory! The package must be published from dist/bundler."
