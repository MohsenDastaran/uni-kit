#!/usr/bin/env bash
# Prints the gallery pages whose sources changed since they were last built.
set -euo pipefail
exec node "$(dirname "${BASH_SOURCE[0]}")/changed-pages.mjs" "$@"
