#!/usr/bin/env bash
# Everything that has to be true before the NSH integration is done.
#
#   crates/nsh-component/scripts/check.sh
#   crates/nsh-component/scripts/check.sh --allow-incomplete  # during rollout
#   crates/nsh-component/scripts/check.sh --install           # also install each key
#
# The script does not use `set -e`: it runs every step, reports FAIL for each
# failure, and exits 1 at the end, so one run says everything that is wrong.
set -uo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
root="$(cd "$crate_dir/../.." && pwd)"
failed=0
allow_incomplete=0
install_sweep=0
for argument in "$@"; do
  case "$argument" in
    --allow-incomplete) allow_incomplete=1 ;;
    --install) install_sweep=1 ;;
    *) echo "unknown argument: $argument" >&2; exit 2 ;;
  esac
done

step() { printf '\n== %s\n' "$1"; }
ok()   { printf '   ok: %s\n' "$1"; }
bad()  { printf '   FAIL: %s\n' "$1"; failed=1; }

find_nsh() {
  if [[ -n "${NSH_BIN:-}" && -x "$NSH_BIN" ]]; then
    printf '%s\n' "$NSH_BIN"
    return
  fi
  if command -v nsh >/dev/null 2>&1; then
    command -v nsh
    return
  fi
  if [[ -x "$HOME/.nsh/bin/nsh" ]]; then
    printf '%s\n' "$HOME/.nsh/bin/nsh"
    return
  fi
  return 1
}

nsh="$(find_nsh || true)"
if [[ -z "$nsh" ]]; then
  echo "nsh was not found. Run script/install-nsh.sh or set NSH_BIN." >&2
  exit 1
fi
export NSH_BIN="$nsh"
printf 'nsh: %s (%s)\n' "$("$nsh" --version)" "$nsh"

step "generated icons"
if node "$crate_dir/scripts/gen-icons.mjs" --check >/dev/null 2>&1; then
  ok "$(node "$crate_dir/scripts/gen-icons.mjs" --check 2>/dev/null)"
else
  bad "src/components/icons.ts is stale — run node crates/nsh-component/scripts/gen-icons.mjs"
fi

step "generated mobile theme"
if node "$crate_dir/scripts/gen-theme.mjs" --check >/dev/null 2>&1; then
  ok "$(node "$crate_dir/scripts/gen-theme.mjs" --check 2>/dev/null)"
else
  bad "src/components/theme.json is stale — run node crates/nsh-component/scripts/gen-theme.mjs"
fi

step "theme parity with the Slint tokens"
if node "$crate_dir/scripts/theme-parity.mjs" | tail -1 | grep -q "agree, colour for colour"; then
  ok "every token matches crates/slint-component/ui/theme.slint"
else
  bad "tokens.ts/theme.json and theme.slint disagree"
  node "$crate_dir/scripts/theme-parity.mjs" | grep -A 40 MISMATCHED | head -20
fi

step "theme usage"
if node "$crate_dir/scripts/check-theme-usage.mjs" >/tmp/uni-kit-nsh-theme.log 2>&1; then
  ok "components use the shared tokens; mobile theme keys are complete"
else
  cat /tmp/uni-kit-nsh-theme.log >&2
  fail "theme usage check failed"
fi

step "type check"
if "$nsh" check "$crate_dir/examples/button.tsx" >/dev/null 2>&1; then
  ok "every component and demo type-checks"
else
  bad "nsh check failed"
  "$nsh" check "$crate_dir/examples/button.tsx" 2>&1 | tail -20
fi

step "headless render of every demo"
demos=( "$crate_dir"/examples/*.tsx )
rendered=0
for entry in "${demos[@]}"; do
  name="$(basename "$entry" .tsx)"
  if "$nsh" trace "$entry" --taps 0 >/dev/null 2>&1; then
    rendered=$((rendered + 1))
  else
    bad "nsh trace $name"
  fi
done
if [[ $rendered -eq ${#demos[@]} ]]; then
  ok "$rendered demos render"
fi

step "gallery build"
if STORY="${STORY:-}" node "$crate_dir/scripts/build-gallery.mjs" >/dev/null 2>&1; then
  ok "www/dist is built"
else
  bad "gallery build"
  STORY="${STORY:-}" node "$crate_dir/scripts/build-gallery.mjs" 2>&1 | tail -20
fi

step "registry manifest"
if node "$root/registry/build-manifest.mjs" >/dev/null 2>&1; then
  ok "manifest rebuilt from the sources"
else
  bad "manifest builder"
fi

step "component checklist"
if node "$crate_dir/scripts/status.mjs" --check >/dev/null 2>&1; then
  ok "every component has a source, an example and a registry entry"
else
  progress="$(node "$crate_dir/scripts/status.mjs" 2>/dev/null | grep -oE '\*\*[0-9]+ / [0-9]+\*\*' | head -1)"
  if [[ $allow_incomplete -eq 1 ]]; then
    printf '   pending: %s complete\n' "${progress:-unknown}"
  else
    bad "components are missing — ${progress:-none} complete"
  fi
fi

if [[ $install_sweep -eq 1 ]]; then
  step "install every NSH component into a scratch app"
  install_list="$(node -e '
    const fs = require("fs");
    const registry = JSON.parse(fs.readFileSync(process.argv[1], "utf8"));
    for (const name of Object.keys(registry.nsh.components).sort()) process.stdout.write(`${name}\n`);
  ' "$root/registry/components.json")"
  total=0
  passed=0
  while IFS= read -r component; do
    [[ -z "$component" ]] && continue
    total=$((total + 1))
    scratch="$(mktemp -d)"
    log="$scratch/build.log"
    if (
      cd "$scratch" &&
      "$nsh" init app --name NSHInstall >/dev/null &&
      cd app &&
      UNI_KIT_ROOT="$root" node "$root/registry/index.js" add nsh "$component" >/dev/null &&
      node -e '
        const fs = require("fs");
        const j = JSON.parse(fs.readFileSync("nsh.json", "utf8"));
        j.theme = "src/components/theme.json";
        fs.writeFileSync("nsh.json", JSON.stringify(j, null, 2));
      ' &&
      printf 'import { mount, View } from "nsh";\nimport "./src/components/%s";\nmount(() => <View style={{ flex: 1 }} />);\n' "$(printf '%s' "$component" | tr '-' '_')" > App.tsx &&
      "$nsh" check App.tsx >/dev/null 2>&1
    ) >/dev/null 2>"$log"; then
      passed=$((passed + 1))
    else
      bad "uni-kit add nsh $component does not type-check in a scratch app"
      tail -15 "$log" | sed 's/^/      /'
    fi
    rm -rf "$scratch"
  done <<< "$install_list"
  if [[ $passed -eq $total ]]; then
    ok "$passed of $total NSH components install and type-check"
  else
    bad "$passed of $total NSH components install and type-check"
  fi
fi

if [[ $failed -eq 0 ]]; then printf '\nnsh-component: all checks passed\n'; else printf '\nnsh-component: checks FAILED\n'; fi
exit $failed
