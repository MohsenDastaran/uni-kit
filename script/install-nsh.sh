#!/usr/bin/env sh
# Install the pinned Native Shell (NSH) CLI and its runtimes.
#
#   script/install-nsh.sh
#   NSH_VERSION=v0.1.1 script/install-nsh.sh
#   NSH_INSTALL_DIR=/opt/nsh script/install-nsh.sh
#
# The gallery compiler is pinned so a checkout of this repository builds the
# same bytecode as CI and the website image. The NSH distribution is a single
# directory (the CLI plus the prebuilt native and web runtimes), so it is
# installed whole and `nsh` is linked from it, never copied alone.
set -eu

repository="longbridge/nsh"
version="${NSH_VERSION:-v0.1.1}"
dir="${NSH_INSTALL_DIR:-$HOME/.nsh}"
bin_dir="${NSH_BIN_DIR:-$HOME/.local/bin}"

fail() {
  echo "install-nsh: $*" >&2
  exit 1
}

case "$(uname -s)/$(uname -m)" in
  Darwin/arm64) platform="macos-arm64" ;;
  Linux/x86_64 | Linux/amd64) platform="linux-x86_64" ;;
  Darwin/*) fail "no prebuilt NSH for Intel Macs; build it from source: https://nsh.dev/docs/installation" ;;
  *) fail "no prebuilt NSH for $(uname -s) $(uname -m); see https://nsh.dev/docs/installation" ;;
esac

command -v curl > /dev/null 2>&1 || fail "curl is required"

asset="nsh-$platform.tar.gz"
url="https://github.com/$repository/releases/download/$version/$asset"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT INT TERM

echo "Downloading $url"
curl -fsSL "$url" -o "$work/$asset"
curl -fsSL "$url.sha256" -o "$work/$asset.sha256"

(
  cd "$work"
  if command -v shasum > /dev/null 2>&1; then
    shasum -a 256 -c "$asset.sha256" > /dev/null
  else
    sha256sum -c "$asset.sha256" > /dev/null
  fi
) || fail "the checksum of $asset does not match"

tar -xzf "$work/$asset" -C "$work"
[ -x "$work/nsh/bin/nsh" ] || fail "$asset holds no nsh/bin/nsh"

# Replace the whole directory: runtimes of an older release must not stay.
mkdir -p "$(dirname "$dir")" "$bin_dir"
rm -rf "$dir.old"
[ ! -e "$dir" ] || mv "$dir" "$dir.old"
mv "$work/nsh" "$dir"
rm -rf "$dir.old"
ln -sf "$dir/bin/nsh" "$bin_dir/nsh"

installed="$("$dir/bin/nsh" --version)"
echo "Installed $installed to $dir"
echo "  nsh -> $bin_dir/nsh"
case ":$PATH:" in
  *":$bin_dir:"*) ;;
  *) echo "  Add $bin_dir to your PATH: export PATH=\"$bin_dir:\$PATH\"" ;;
esac
