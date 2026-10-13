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
# Hyphens are part of a feature name — `image-without-codecs` — and a pattern
# without them drops it from both sides, which would hide the very drift this
# step exists to catch.
crate_features="$(grep -oE '^iced = \{[^}]*\}' "$crate_dir/Cargo.toml" | grep -oE '"[a-z_-]+"' | tr -d '"' | sort | tr '\n' ' ')"
hint_features="$(grep -oE 'const ICED_FEATURES = \[[^]]*\]' "$root/registry/index.js" | grep -oE "'[a-z_-]+'" | tr -d "'" | sort | tr '\n' ' ')"
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
  step "install every iced component into a scratch project"

  # One scratch project per component. Installing the family into a single
  # project would let `input` supply the file `number-input` is supposed to
  # carry, which is the hole this check exists to find: the entry that omitted
  # `input.rs` and `button.rs` compiled here only because `button` was the one
  # component installed.
  #
  # The scratch is a bin crate and keeps a `main`, because a bin without one
  # fails to build for that reason alone and the check would then pass or fail
  # without ever compiling the installed component.
  #
  # The list is the manifest's own keys rather than a list kept here, so every
  # component the registry ships is installed and compiled — including the
  # compositions whose closure is large enough to have dropped a file, such as
  # `command`, `combobox`, `date-picker` and `data-table`. The extra crates are
  # the ones the manifest records a component reaching for beyond iced
  # (`text-view` and `iced_core`), at the version it recorded, which is the same
  # statement `uni-kit add` prints for an application.
  install_list="$(node -e '
    const fs = require("fs");
    const registry = JSON.parse(fs.readFileSync(process.argv[1], "utf8"));
    const iced = registry.iced;
    for (const name of Object.keys(iced.components).sort()) {
      const extra = (iced.needs_extra?.[name] ?? []).map(
        (crate) => `${crate}@${iced.extra_versions[crate]}`,
      );
      process.stdout.write(`${name}\t${extra.join(" ")}\n`);
    }
  ' "$root/registry/components.json")"

  # The features the crate and the install hint agreed on above, as `cargo add`
  # spells them. Asking for the same set is what lets the scratch reuse the iced
  # the crate build already compiled in the shared target directory.
  install_features="${crate_features% }"
  install_features="${install_features// /,}"

  if [[ -z "$install_list" || -z "$install_features" ]]; then
    bad "could not read the iced components and features from the registry"
  else
    install_total=0
    install_passed=0
    while IFS=$'\t' read -r component extras; do
      [[ -z "$component" ]] && continue
      install_total=$((install_total + 1))
      scratch="$(mktemp -d)"
      log="$scratch/build.log"
      # `--offline` and the shared target directory keep iced compiled once; the
      # crate build above has already put every crate the scratch resolves into
      # the local cargo cache, so each iteration only rebuilds the copied source.
      if (
        cd "$scratch" &&
        cargo new --bin app --quiet &&
        cd app &&
        cargo add iced --features "$install_features" --quiet --offline &&
        for extra in $extras; do cargo add "$extra" --quiet --offline || exit 1; done &&
        UNI_KIT_ROOT="$root" node "$root/registry/index.js" add iced "$component" >/dev/null &&
        printf 'mod components;\nfn main() {}\n' > src/main.rs &&
        CARGO_TARGET_DIR="$crate_dir/target" cargo build --quiet --offline
      ) >/dev/null 2>"$log"; then
        install_passed=$((install_passed + 1))
        ok "uni-kit add iced $component installs a project that compiles"
      else
        bad "uni-kit add iced $component does not compile in a scratch project"
        tail -20 "$log" | sed 's/^/      /'
      fi
      rm -rf "$scratch"
    done <<< "$install_list"
    if [[ $install_passed -eq $install_total ]]; then
      ok "$install_passed of $install_total iced components install and compile"
    else
      bad "$install_passed of $install_total iced components install and compile"
    fi
  fi
fi

if [[ $failed -eq 0 ]]; then printf '\niced-component: all checks passed\n'; else printf '\niced-component: checks FAILED\n'; fi
exit $failed
