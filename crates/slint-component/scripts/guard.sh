#!/usr/bin/env bash
# Runs a command inside a memory-capped cgroup.
#
# Compiling the Slint gallery peaks around 13 GiB in a single `rustc`, which is
# the whole laptop on a 14 GiB machine: the kernel evicts everything else to a
# disk swapfile before it ever considers killing the build, and the desktop
# stops responding. A cgroup makes the build the first thing to die instead, so
# a build that is too big fails on its own and the machine stays usable.
#
#   scripts/guard.sh <command> [args...]
#   scripts/guard.sh -- <command> [args...]
#
# Tune with GUARD_MEMORY_MAX (systemd size, default 9G). GUARD_MEMORY_HIGH
# defaults to that same value: a lower soft ceiling only throttles the build
# without adding any protection the hard cap does not already give.
# Falls back to `ulimit -v` when systemd-run is not usable, which is harsher:
# the limit then counts reserved address space, so the build may fail earlier.
set -uo pipefail

if [[ "${1:-}" == "--" ]]; then
  shift
fi
if [[ $# -eq 0 ]]; then
  echo "usage: guard.sh <command> [args...]" >&2
  exit 2
fi

memory_max="${GUARD_MEMORY_MAX:-9G}"
memory_kib="${GUARD_MEMORY_ULIMIT_KIB:-9000000}"
# `MemoryHigh` is a soft ceiling that makes the kernel reclaim continuously, and
# reclaiming is where the freeze's real damage lands. With 6G high / 9G max the
# build hit the high mark 2 million times and never the max, and rustc ran at
# 37% CPU because it spent most of its time waiting on reclaim. It therefore
# defaults to the hard cap: only the hard cap does anything useful here.
# `MemoryMax` plus no swap is what contains the problem — the allocation fails
# and the build dies, with the desktop intact.
memory_high="${GUARD_MEMORY_HIGH:-$memory_max}"

# The wrapper is chosen once and then exec'd, so the command runs exactly once
# and its own exit status is this script's. An OOM-killed build must fail the
# caller rather than send it down a fallback path to try again.
#
# `MemorySwapMax=0` is the part that stops the freeze: with swap available the
# kernel throttles to disk instead of failing the allocation.
#
# GUARD_FORCE_ULIMIT=1 exercises the fallback on a machine that has systemd-run.
if [[ "${GUARD_FORCE_ULIMIT:-}" != "1" ]] &&
  command -v systemd-run >/dev/null 2>&1 &&
  systemd-run --user --scope --collect --quiet \
    -p "MemoryHigh=$memory_high" -p "MemoryMax=$memory_max" -p MemorySwapMax=0 \
    -- /bin/true 2>/dev/null; then
  exec systemd-run --user --scope --collect --quiet \
    -p "MemoryHigh=$memory_high" -p "MemoryMax=$memory_max" -p MemorySwapMax=0 \
    -- "$@"
fi

echo "guard: using ulimit -v ${memory_kib} KiB (systemd-run scope unavailable)" >&2
ulimit -v "$memory_kib"
exec "$@"
