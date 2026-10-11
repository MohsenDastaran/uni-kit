#!/usr/bin/env bash
# Everything that has to be true before the iced integration is done.
#
#   scripts/check.sh            # the crate, the examples and the manifest
#   scripts/check.sh --install  # also install into a scratch project and build
#
# The manifest builder is the registry's own validator: it refuses an entry whose
# source does not exist, so a component that is registered but missing, or
# present but unregistered, fails here rather than at install time.
set -uo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
root="$(cd "$crate_dir/../.." && pwd)"
failed=0

step() { printf '\n== %s\n' "$1"; }
ok()   { printf '   ok: %s\n' "$1"; }
bad()  { printf '   FAIL: %s\n' "$1"; failed=1; }

step "cargo build"
if cargo build --quiet --manifest-path "$crate_dir/Cargo.toml"; then ok "components and gallery"; else bad "cargo build"; fi

step "cargo build --examples"
if cargo build --quiet --examples --manifest-path "$crate_dir/Cargo.toml"; then ok "usage examples"; else bad "cargo build --examples"; fi

step "registry manifest"
if node "$root/registry/build-manifest.mjs"; then ok "manifest rebuilt from the sources"; else bad "manifest builder"; fi

step "component checklist"
if node "$crate_dir/scripts/status.mjs" --check >/dev/null 2>&1; then
  ok "every component has a source, an example and a registry entry"
else
  started="$(node "$crate_dir/scripts/status.mjs" 2>/dev/null | grep -oE '\*\*[0-9]+ / 79\*\*' | head -1)"
  bad "components are missing — ${started:-none} complete"
fi

if [[ "${1:-}" == "--install" ]]; then
  step "install into a scratch project"
  scratch="$(mktemp -d)"
  if cargo new --bin "$scratch/app" --quiet >/dev/null 2>&1 \
    && (cd "$scratch/app" && cargo add iced --features svg,canvas,image,qr_code,markdown --quiet >/dev/null 2>&1) \
    && (cd "$scratch/app" && UNI_KIT_ROOT="$root" node "$root/registry/index.js" add iced button >/dev/null) \
    && printf 'mod components;\n' > "$scratch/app/src/main.rs" \
    && (cd "$scratch/app" && CARGO_TARGET_DIR="$crate_dir/target" cargo build --quiet 2>/dev/null); then
    ok "uni-kit add iced button installs a project that compiles"
  else
    bad "install and compile in a scratch project"
  fi
  rm -rf "$scratch"
fi

if [[ $failed -eq 0 ]]; then printf '\niced-component: all checks passed\n'; else printf '\niced-component: checks FAILED\n'; fi
exit $failed
