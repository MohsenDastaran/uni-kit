#!/usr/bin/env bash
# Copies the gallery page into `www/dist`.
#
# This is the half of the gallery that is not a wasm build, which matters twice:
# the release build runs it after the module is built, and `make build-page`
# runs it alone so the page and its themes can be worked on and checked without
# compiling anything.
#
#   scripts/build-page.sh
#
# No source is served: a section's code icon asks the *page* to show the sample,
# and the page has that code from the same file the gallery runs — cut out of it
# at build time by `website/src/lib/remark-iced-source.js`.
set -euo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out="$crate_dir/www/dist"

mkdir -p "$out"
cp "$crate_dir/www/index.html" "$out/index.html"

echo "iced: page in www/dist"
