#!/usr/bin/env bash
# Runs a command inside a memory-capped cgroup, and sizes that cap from the
# machine rather than a fixed number.
#
# Compiling the whole Slint gallery is one `rustc` over one generated file that
# holds every example. It peaks well above 9 GiB, more than a 14 GiB laptop can
# give while the desktop is running: the kernel evicts everything else to swap
# before it considers killing the build, and the machine stops responding. A
# cgroup makes the build the first thing to die instead.
#
#   scripts/guard.sh <command> [args...]
#   scripts/guard.sh -- <command> [args...]
#
# The cap defaults to the machine's RAM minus a reserve for the desktop
# (GUARD_DESKTOP_RESERVE_MIB, 5 GiB), so the build can never eat the memory the
# session needs. Override it outright with GUARD_MEMORY_MAX (a systemd size,
# e.g. `6G`). GUARD_MEMORY_HIGH defaults to the same value: a lower soft ceiling
# only throttles the build without adding protection the hard cap does not
# already give.
#
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

# The reserve is what keeps the session alive, so the cap is derived from it on
# every machine instead of being a number that only suits one of them.
total_kib="$(awk '/^MemTotal:/ {print $2}' /proc/meminfo)"
reserve_kib=$(( ${GUARD_DESKTOP_RESERVE_MIB:-5120} * 1024 ))
derived_kib=$(( total_kib - reserve_kib ))
# A machine with very little RAM still needs a workable floor; the reserve then
# wins and the build is expected to fail rather than the desktop.
(( derived_kib < 3145728 )) && derived_kib=3145728

memory_max="${GUARD_MEMORY_MAX:-${derived_kib}K}"
memory_kib="${GUARD_MEMORY_ULIMIT_KIB:-9000000}"
# `MemoryHigh` is a soft ceiling that makes the kernel reclaim continuously, and
# reclaiming is where the freeze's real damage lands. With 6G high / 9G max the
# build hit the high mark 2 million times and never the max, and rustc ran at
# 37% CPU because it spent most of its time waiting on reclaim. It therefore
# defaults to the hard cap: only the hard cap does anything useful here.
memory_high="${GUARD_MEMORY_HIGH:-$memory_max}"

# `MemorySwapMax=0` is the part that stops the freeze: with swap available the
# kernel throttles to disk instead of failing the allocation.
#
# The probe chooses the wrapper once, so a scope that cannot be created falls
# back to `ulimit` instead of running the command twice.
#
# GUARD_FORCE_ULIMIT=1 exercises the fallback on a machine that has systemd-run.
if [[ "${GUARD_FORCE_ULIMIT:-}" != "1" ]] &&
  command -v systemd-run >/dev/null 2>&1 &&
  systemd-run --user --scope --collect --quiet \
    -p "MemoryHigh=$memory_high" -p "MemoryMax=$memory_max" -p MemorySwapMax=0 \
    -- /bin/true 2>/dev/null; then
  systemd-run --user --scope --collect --quiet \
    -p "MemoryHigh=$memory_high" -p "MemoryMax=$memory_max" -p MemorySwapMax=0 \
    -- "$@"
  status=$?
  # 137 is SIGKILL and 143 is the SIGTERM systemd sends when it tears the scope
  # down. Both mean the same thing here, and without this note the caller only
  # sees an exit code with no hint that memory was the reason.
  if [[ $status -eq 137 || $status -eq 143 ]]; then
    echo "guard: the command was stopped after reaching ${memory_max}." >&2
    echo "guard: it needs more memory than this machine can spare (MemTotal ${total_kib} KiB, desktop reserve ${reserve_kib} KiB)." >&2
    echo "guard: build fewer pages at once, or raise GUARD_MEMORY_MAX if you know the machine can take it." >&2
  fi
  exit "$status"
fi

echo "guard: using ulimit -v ${memory_kib} KiB (systemd-run scope unavailable)" >&2
ulimit -v "$memory_kib"
exec "$@"
