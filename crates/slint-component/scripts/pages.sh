#!/usr/bin/env bash
# Prints the gallery page slugs, one per line, in the order the catalog uses.
#
# This mirrors build.rs: a page is an `ui/examples/*.slint` that exports a
# component whose name ends in `Example`. Keeping the rule in one place matters,
# because a slug that build.rs knows and this script does not would never be
# built into its own folder.
set -euo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

for file in "$crate_dir"/ui/examples/*.slint; do
  if grep -qE '^export component [A-Za-z0-9_]*Example' "$file"; then
    basename "$file" .slint
  fi
done | sort
