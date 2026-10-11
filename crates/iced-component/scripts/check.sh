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

step "module list matches the directory"
if node "$crate_dir/scripts/mod-rs.mjs" --check >/dev/null 2>&1; then
  ok "$(node "$crate_dir/scripts/mod-rs.mjs" 2>/dev/null)"
else
  bad "src/components/mod.rs is stale — a component would not be compiled: run node crates/iced-component/scripts/mod-rs.mjs"
fi

step "cargo build"
if cargo build --quiet --manifest-path "$crate_dir/Cargo.toml"; then ok "components and gallery"; else bad "cargo build"; fi

step "cargo build --examples"
if cargo build --quiet --examples --manifest-path "$crate_dir/Cargo.toml"; then ok "usage examples"; else bad "cargo build --examples"; fi

step "theme parity with the Slint tokens"
if node "$crate_dir/scripts/theme-parity.mjs" | tail -1 | grep -q "agree, colour for colour"; then
  ok "every token matches crates/slint-component/ui/theme.slint"
else
  bad "theme.rs and theme.slint disagree"
  node "$crate_dir/scripts/theme-parity.mjs" | grep -A 40 "MISMATCHED" | head -20
fi

step "features agree between the crate and the install hint"
crate_features="$(grep -oE '^iced = \{[^}]*\}' "$crate_dir/Cargo.toml" | grep -oE '"[a-z_]+"' | tr -d '"' | sort | tr '\n' ' ')"
hint_features="$(grep -oE 'const ICED_FEATURES = \[[^]]*\]' "$root/registry/index.js" | grep -oE "'[a-z_]+'" | tr -d "'" | sort | tr '\n' ' ')"
if [[ -n "$crate_features" && "$crate_features" == "$hint_features" ]]; then
  ok "both name: $crate_features"
else
  bad "crate has [$crate_features] and the install hint says [$hint_features]"
fi

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
