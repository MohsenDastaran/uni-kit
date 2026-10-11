#!/usr/bin/env bash
# Builds one component, and its example, on its own.
#
# The crate compiles as one unit, so a file somebody else is halfway through
# editing hides whether yours compiles at all. This copies the theme, the
# components you name and their examples into a scratch crate and builds that, so
# the answer is about your files only.
#
#   scripts/check-one.sh checkbox
#   scripts/check-one.sh checkbox radio slider
#   scripts/check-one.sh --all          # every component, ignoring nothing
#
# It shares this crate's target directory, so the dependencies stay warm.
set -uo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
names=("$@")
if [[ "${1:-}" == "--all" ]]; then
  names=()
  for file in "$crate_dir"/src/components/*.rs; do
    name="$(basename "$file" .rs)"
    [[ "$name" == "theme" || "$name" == "mod" ]] && continue
    names+=("$name")
  done
fi
if [[ ${#names[@]} -eq 0 ]]; then
  echo "usage: scripts/check-one.sh <component>... | --all" >&2
  exit 2
fi

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/src/components" "$scratch/examples"

# The manifest, the lockfile and the crate root, so the scratch resolves to the
# same dependency versions with nothing to download.
#
# The scratch keeps the library NAME `iced_component` — the examples import it —
# but takes a different PACKAGE name, because it shares this crate's target
# directory. Two packages called `iced-component` 0.1.0 building into one target
# directory collide in cargo's fingerprint cache, and the scratch's library, which
# holds only the components it was given, then answers for the real crate: a
# full-crate build starts reporting "could not find `button` in `components`" for
# modules that are plainly declared. A distinct package name keeps the units
# distinct while the library name stays importable.
cp "$crate_dir/Cargo.toml" "$crate_dir/Cargo.lock" "$scratch/"
sed -i.bak 's/^name = "iced-component"$/name = "iced-component-scratch"/' "$scratch/Cargo.toml" && rm -f "$scratch/Cargo.toml.bak"
printf 'pub mod components;\n' > "$scratch/src/lib.rs"
printf 'fn main() {}\n' > "$scratch/src/main.rs"
cp "$crate_dir/src/components/theme.rs" "$scratch/src/components/"
printf 'pub mod theme;\n' > "$scratch/src/components/mod.rs"

missing=0
for name in "${names[@]}"; do
  module="${name//-/_}"
  if [[ ! -f "$crate_dir/src/components/$module.rs" ]]; then
    echo "no such component: $module" >&2
    missing=1
    continue
  fi
  cp "$crate_dir/src/components/$module.rs" "$scratch/src/components/"
  printf 'pub mod %s;\n' "$module" >> "$scratch/src/components/mod.rs"
  [[ -f "$crate_dir/examples/$module.rs" ]] && cp "$crate_dir/examples/$module.rs" "$scratch/examples/"
done
[[ $missing -eq 1 ]] && exit 1

echo "building ${#names[@]} component(s): ${names[*]}"
CARGO_TARGET_DIR="$crate_dir/target" cargo build --quiet --manifest-path "$scratch/Cargo.toml"
lib=$?
CARGO_TARGET_DIR="$crate_dir/target" cargo build --quiet --examples --manifest-path "$scratch/Cargo.toml"
examples=$?
if [[ $lib -eq 0 && $examples -eq 0 ]]; then
  echo "clean: no errors and no warnings"
  exit 0
fi
echo "failed: lib=$lib examples=$examples" >&2
exit 1
