#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
crate_dir="$(cd "$script_dir/.." && pwd)"
workspace_dir="$(cd "$crate_dir/../../../.." && pwd)"
profile="debug"
# Dated, so a moving channel cannot make rustup replace the installed wasm32 std
# mid-build. Keep in step with `crates/story-web/rust-toolchain.toml`.
cargo_args=(+nightly-2026-10-10 build --manifest-path "$crate_dir/Cargo.toml" --target wasm32-unknown-unknown)
# The workspace's `wasm-release` profile: release, tuned for a module the browser
# has to compile. Its artifacts land under `target/.../wasm-release`.
if [[ "${1:-}" == "--release" ]]; then profile="wasm-release"; cargo_args+=(--profile wasm-release); fi
cargo "${cargo_args[@]}"
wasm-bindgen "$workspace_dir/target/wasm32-unknown-unknown/$profile/gpui_base_examples_wasm.wasm" --out-dir "$crate_dir/www/src/wasm" --target web --no-typescript
# A smaller module compiles faster in the browser. The optimizer knows which
# features rustc is allowed to use; see script/optimize-wasm.sh.
source "$workspace_dir/script/optimize-wasm.sh"
optimize_wasm "$crate_dir/www/src/wasm/gpui_base_examples_wasm_bg.wasm"
