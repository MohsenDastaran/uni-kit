#!/usr/bin/env bash
# Shrinks a wasm-bindgen output in place, best effort.
#
# The browser has to compile the module before it shows anything, so a smaller
# one is a faster preview, and wasm-opt is the standard way to get there.
#
# The feature flags are the ones rustc enables by default for
# wasm32-unknown-unknown — stable's set and nightly's, which adds atomics. It
# matters because wasm-opt keeps every feature switched off unless asked: handed
# a module it was not told about, it rejects the module instead of optimizing it
# ("Bulk memory operation (bulk memory is disabled)", then the same for
# atomics). Names come from Binaryen's own feature list, so they exist in the
# version Debian ships.
#
# Best effort on purpose. rustc's default feature set has grown twice already,
# and a toolchain that starts using one more feature must not fail the image: a
# refused optimization says so and leaves the module as rustc built it.
WASM_OPT_FEATURES=(
  --enable-bulk-memory
  --enable-mutable-globals
  --enable-nontrapping-float-to-int
  --enable-sign-ext
  --enable-reference-types
  --enable-multivalue
  --enable-threads
  --enable-simd
)

optimize_wasm() {
  local file="$1"
  if [[ ! -f "$file" ]]; then
    echo "optimize_wasm: no such file: $file" >&2
    return 1
  fi
  if ! command -v wasm-opt >/dev/null 2>&1; then
    echo "wasm-opt not found (apt install binaryen): leaving $(basename "$file") as built" >&2
    return 0
  fi

  local before after
  before=$(wc -c <"$file")
  if wasm-opt -O3 --strip-debug "${WASM_OPT_FEATURES[@]}" -o "$file.tmp" "$file"; then
    mv "$file.tmp" "$file"
    after=$(wc -c <"$file")
    printf 'wasm-opt: %s %.1f MB -> %.1f MB\n' \
      "$(basename "$file")" \
      "$(echo "$before" | awk '{print $1/1048576}')" \
      "$(echo "$after" | awk '{print $1/1048576}')"
  else
    # A half-written `.tmp` must not be left beside the module.
    rm -f "$file.tmp"
    echo "wasm-opt: refused $(basename "$file") — leaving it as rustc built it" >&2
  fi
  return 0
}
