#!/usr/bin/env bash
# Copies the gallery page and the sources it can show into `www/dist`.
#
# This is the half of the gallery that is not a wasm build, which matters twice:
# the release build runs it after the module is built, and `make build-page`
# runs it alone so the page and its source view can be worked on and checked
# without compiling anything.
#
#   scripts/build-page.sh
#
# The page fetches `src/<story>.rs` when its source button is used, so the demo
# Rust the gallery is running is served beside the module rather than compiled
# into it: a browser that never opens the source never downloads it.
set -euo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out="$crate_dir/www/dist"

mkdir -p "$out/src"
cp "$crate_dir/www/index.html" "$out/index.html"

# One file per story, named the way the page asks for it: the registry key, so
# `message_role.rs` is served as `message-role.rs`.
count=0
for file in "$crate_dir"/src/demos/*.rs; do
  name="$(basename "$file" .rs)"
  [[ "$name" == "index" || "$name" == "mod" ]] && continue
  cp "$file" "$out/src/${name//_/-}.rs"
  count=$((count + 1))
done

echo "iced: page and $count source file(s) in www/dist"
