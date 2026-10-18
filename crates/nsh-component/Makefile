.PHONY: build check gallery gallery-page run stories clean

NSH ?= nsh
STORY ?=

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

# Run one demo in the NSH web host.
#
#   make run STORY=button
run:
	@test -n "$(STORY)" || (echo "usage: make run STORY=<component>" >&2; exit 2)
	$(NSH) run examples/$(STORY).tsx --target web

# Every story `?story=` accepts, one per line.
stories:
	@ls examples/*.tsx | sed -e 's|examples/||' -e 's|\.tsx$$||' -e 's|_|-|g' | sort

clean:
	rm -rf www/dist .tmp examples/.nsh examples/out
