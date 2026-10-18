#!/usr/bin/env bash
# Builds one component, its demo and its example, on their own.
#
# The crate compiles as one unit, so a file somebody else is halfway through
# editing hides whether yours compiles at all. This copies the theme, the
# components you name, their demos and their examples into a scratch crate and
# builds that, so the answer is about your files only.
#
#   scripts/check-one.sh checkbox
#   scripts/check-one.sh checkbox radio slider
#   scripts/check-one.sh --all          # every component, ignoring nothing
#
# A demo is part of the component's contract — the gallery shows it — so it is
# built here too, together with every component it imports: several demos show
# more than one component, and a demo that cannot find one of them is not clean.
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
mkdir -p "$scratch/src/components" "$scratch/src/demos" "$scratch/examples"

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
copied=()
democount=0
# Copies a component into the scratch, and says whether it was already there, so
# a demo's imports can pull in what they need without copying anything twice.
copy_component() {
  local module="$1"
  for done_module in "${copied[@]:-}"; do
    [[ "$done_module" == "$module" ]] && return 0
  done
  if [[ ! -f "$crate_dir/src/components/$module.rs" ]]; then
    echo "no such component: $module" >&2
    return 1
  fi
  cp "$crate_dir/src/components/$module.rs" "$scratch/src/components/"
  # `theme` is written first, by hand, so it is never declared twice.
  [[ "$module" != "theme" ]] && printf 'pub mod %s;\n' "$module" >> "$scratch/src/components/mod.rs"
  copied+=("$module")
  return 0
}

for name in "${names[@]}"; do
  module="${name//-/_}"
  copy_component "$module" || { missing=1; continue; }

  # The demo the gallery would show, and everything it imports. Reading the
  # imports rather than listing them keeps this honest when a demo grows a
  # second component.
  if [[ -f "$crate_dir/src/demos/$module.rs" ]]; then
    cp "$crate_dir/src/demos/$module.rs" "$scratch/src/demos/"
    printf 'pub mod %s;\n' "$module" >> "$scratch/src/demos/mod.rs"
    democount=$((democount + 1))
    for used in $(grep -oE 'crate::components::[a-z_]+' "$crate_dir/src/demos/$module.rs" | sort -u | sed 's/.*:://'); do
      copy_component "$used" || missing=1
    done
  fi

  [[ -f "$crate_dir/examples/$module.rs" ]] && cp "$crate_dir/examples/$module.rs" "$scratch/examples/"
done
[[ $missing -eq 1 ]] && exit 1
# The examples call into `demos`, so the scratch declares it whenever it holds one.
[[ $democount -gt 0 ]] && printf 'pub mod demos;\n' >> "$scratch/src/lib.rs"

echo "building ${#names[@]} component(s) and $democount demo(s): ${names[*]}"
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
