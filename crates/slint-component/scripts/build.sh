#!/usr/bin/env bash
# Builds the Slint gallery into a folder the website serves under
# `/slint-gallery`.
#
#   scripts/build.sh                     every example -> www/dist
#   scripts/build.sh --out <dir>         every example -> <dir>
#   SLINT_GALLERY_ONLY=dock scripts/build.sh --out <dir>   just `dock`
#
# `SLINT_GALLERY_ONLY` is read by build.rs, which then generates a catalog with
# only those pages. That is what makes a development build small: the whole
# catalog is one generated Rust file, and a single `rustc` compiling all of it
# needs more memory than a 14 GiB laptop can spare. One page needs a fraction of
# that.
set -euo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
profile="debug"
cargo_args=(build --manifest-path "$crate_dir/Cargo.toml" --target wasm32-unknown-unknown --lib)
out_dir="$crate_dir/www/dist"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release)
      profile="release"
      cargo_args+=(--release)
      ;;
    --out)
      out_dir="$2"
      shift
      ;;
    *)
      echo "build.sh: unknown argument '$1'" >&2
      exit 2
      ;;
  esac
  shift
done

# Absolute, so the removals below cannot follow a relative path twice.
out_dir="$(cd "$crate_dir" && mkdir -p "$out_dir" && cd "$out_dir" && pwd)"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$crate_dir/target}"
cargo "${cargo_args[@]}"

target_dir="$CARGO_TARGET_DIR"

mkdir -p "$out_dir"
# Remove only the three files this build owns. `rm -rf www/dist` would take the
# per-page folders and their stamps with it, and those are what the site serves.
rm -f "$out_dir/slint_component.js" "$out_dir/slint_component_bg.wasm" "$out_dir/index.html"
wasm-bindgen "$target_dir/wasm32-unknown-unknown/$profile/slint_component.wasm" \
  --out-dir "$out_dir" --target web --no-typescript
# A smaller module compiles faster in the browser and ships fewer bytes. wasm-opt
# is optional: a machine without Binaryen still builds, just larger.
if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -O3 --strip-debug -o "$out_dir/slint_component_bg.wasm.tmp" "$out_dir/slint_component_bg.wasm" \
    && mv "$out_dir/slint_component_bg.wasm.tmp" "$out_dir/slint_component_bg.wasm"
else
  echo "build.sh: wasm-opt not found (apt install binaryen) — leaving the module unoptimized" >&2
fi
cp "$crate_dir/www/index.html" "$out_dir/index.html"

# Record what this page was built from, so the next `make dev` can tell whether
# it needs rebuilding. Only a single-page build has a page to stamp.
only="${SLINT_GALLERY_ONLY:-}"
if [[ "$only" != *","* && -n "$only" && "$only" != "all" ]]; then
  "$crate_dir/scripts/changed-pages.sh" --write "$only"
fi
