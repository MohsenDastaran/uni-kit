.PHONY: build check gallery gallery-page run stories clean

NSH ?= nsh
STORY ?=
STORY_FILE = $(subst -,_,$(STORY))
# The gallery's run target; override to preview a native backend.
#   make run STORY=button TARGET=macos
TARGET ?= web

# Compile every demo and write the player the website serves.
build gallery:
	node scripts/build-gallery.mjs

# The player, its runtime and the theme, without compiling a demo.
gallery-page:
	node scripts/build-page.mjs

# The crate gate. `make check` is the full gate (the checklist is red until the
# catalog is complete); `make check-incomplete` is the rollout gate.
check:
	./scripts/check.sh

check-incomplete:
	./scripts/check.sh --allow-incomplete

# Run one demo in the NSH host. The website gallery uses `web`; a native
# preview uses the matching host target.
#
#   make run STORY=button                 # web
#   make run STORY=button TARGET=linux    # GTK 4
#   make run STORY=button TARGET=macos    # AppKit
#   make run STORY=button TARGET=windows  # Win32 / Direct2D
#   make run STORY=button TARGET=android  # device or emulator
run:
	@test -n "$(STORY)" || (echo "usage: make run STORY=<component>" >&2; exit 2)
	$(NSH) run examples/$(STORY_FILE).tsx --target $(TARGET)

# Every story `?story=` accepts, one per line.
stories:
	@ls examples/*.tsx | sed -e 's|examples/||' -e 's|\.tsx$$||' -e 's|_|-|g' | sort

clean:
	rm -rf www/dist .tmp examples/.nsh examples/out
