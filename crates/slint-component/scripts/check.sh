#!/usr/bin/env bash
# Type-checks component examples one by one, so a broken file is reported on
# its own instead of failing the whole gallery build. Uses the checker in
# `check/`, which compiles with the gallery's exact Slint version.
#
#   scripts/check.sh              # every example
#   scripts/check.sh button tabs  # the named examples
#
# This builds the checker in DEBUG on purpose. The Slint compiler guards some
# assumptions with `debug_assert!`, and a release build compiles those out, so a
# file could pass here and still panic the gallery's build script, which cargo
# always builds in debug. A debug checker fails on exactly what that build does.
set -uo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$crate_dir/check/target/debug/slint-component-check"
if [[ ! -x "$checker" ]]; then
  CARGO_TARGET_DIR="$crate_dir/check/target" \
    cargo build --quiet --manifest-path "$crate_dir/check/Cargo.toml" || exit 1
fi

cd "$crate_dir/ui/examples"
if [[ $# -gt 0 ]]; then
  slugs=("$@")
else
  slugs=()
  for file in *.slint; do slugs+=("${file%.slint}"); done
fi

failed=0
for slug in "${slugs[@]}"; do
  # An example is a component, not a window; that warning is expected.
  output="$("$checker" "$slug.slint" 2>&1 |
    sed -e 's/\x1b\[[0-9;]*m//g' -e 's/^cargo:warning=//' |
    awk '/^warning: Exported component .* doesn.t inherit Window/ { skip = 5 } skip > 0 { skip--; next } { print }')"
  status=${PIPESTATUS[0]}
  if [[ $status -ne 0 || -n "$output" ]]; then
    echo "✗ $slug"
    echo "$output"
    failed=1
  else
    echo "✓ $slug"
  fi
done
exit $failed
