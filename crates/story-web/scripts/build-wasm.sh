#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building GPUI Component Story Web...${NC}"

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."

# Parse arguments
RELEASE_FLAG=""
# The workspace's `wasm-release` profile: release, tuned for a module the browser
# has to compile. Its artifacts land under `target/.../wasm-release`.
if [[ "$1" == "--release" ]]; then
    RELEASE_FLAG="--profile wasm-release"
    echo -e "${YELLOW}Building in release mode${NC}"
fi

# Step 1: Build WASM
echo -e "${GREEN}Step 1: Building WASM...${NC}"
cd "$PROJECT_ROOT"
# ButtonStory's large render tree needs more than wasm-ld's default 1 MiB
# stack in debug builds. Apply this at the final link, for both build modes.
cargo rustc -p gpui-component-story-web --target wasm32-unknown-unknown $RELEASE_FLAG -- \
    -C link-arg=-zstack-size=8388608

# Determine the build directory
if [[ "$RELEASE_FLAG" == "--profile wasm-release" ]]; then
    BUILD_MODE="wasm-release"
else
    BUILD_MODE="debug"
fi

# WASM file is in workspace target directory
WORKSPACE_ROOT="$PROJECT_ROOT/../.."
WASM_PATH="$WORKSPACE_ROOT/target/wasm32-unknown-unknown/$BUILD_MODE/gpui_component_story_web.wasm"

# Check if WASM file exists
if [[ ! -f "$WASM_PATH" ]]; then
    echo -e "${RED}Error: WASM file not found at: $WASM_PATH${NC}"
    exit 1
fi

# Step 2: Generate JavaScript bindings
echo -e "${GREEN}Step 2: Generating JavaScript bindings...${NC}"
wasm-bindgen "$WASM_PATH" \
    --out-dir "$PROJECT_ROOT/www/src/wasm" \
    --target web \
    --no-typescript

# A smaller module compiles faster in the browser. wasm-opt is optional: a
# machine without Binaryen still builds, just larger.
#
# The feature flags are the ones Rust's wasm32-unknown-unknown target enables by
# default. wasm-opt has them off unless asked, and a module using bulk memory —
# `memory.copy`, which rustc does emit — fails its validator without
# `--enable-bulk-memory`.
if command -v wasm-opt >/dev/null 2>&1; then
    wasm-opt -O3 --strip-debug \
        --enable-bulk-memory --enable-mutable-globals \
        --enable-nontrapping-float-to-int --enable-sign-ext \
        -o "$PROJECT_ROOT/www/src/wasm/gpui_component_story_web_bg.wasm.tmp" \
        "$PROJECT_ROOT/www/src/wasm/gpui_component_story_web_bg.wasm" \
        && mv "$PROJECT_ROOT/www/src/wasm/gpui_component_story_web_bg.wasm.tmp" \
              "$PROJECT_ROOT/www/src/wasm/gpui_component_story_web_bg.wasm"
else
    echo "wasm-opt not found (apt install binaryen): leaving the module unoptimized" >&2
fi

echo -e "${GREEN}✓ Build completed successfully!${NC}"
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  cd www"
echo -e "  bun install"
echo -e "  bun run dev"
