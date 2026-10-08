dev-web:
	$(MAKE) -C crates/story-web build-wasm-dev build-web
	$(MAKE) dev\:website

dev-gallery:
	$(MAKE) -C crates/story-web dev

# `--cwd` moves only the spawned process, so the site's own package.json and
# astro.config stay inside website/ and the repository root stays Rust-only.
dev\:website:
	bun run --cwd website dev

# The one command for working on a Slint component: rebuild the gallery, then
# start the site. Use `dev:website` instead once www/dist is already current and
# only site files changed.
#
# The rebuild is capped in a cgroup (scripts/guard.sh) because a single rustc
# peaks near 13 GiB, which freezes the desktop before the kernel kills it. With
# the cap the build dies on its own and the machine stays responsive.
#
# Name the page under work to compile only that page. Every page is a separate
# component type in one generated tree, so the full set is a ~97 MB Rust file
# that takes a wasm rustc tens of minutes; one page is a couple of MB and
# finishes in seconds. SLUGS=dock,dropdown_button takes several.
#
#   make dev SLUGS=dock      the pages you are working on, then the site
#   make dev                 the pages changed since the last build
dev:
	@SLUGS="$(SLUGS)" $(MAKE) build\:wasm-slint-dev
	bun run --cwd website dev

# The site serves crates/story-web/www/dist at `/gallery`, so this rebuilds the
# GPUI story gallery (WASM + Vite bundle) before starting the same dev server.
dev\:website-gpui:
	$(MAKE) -C crates/story-web build
	bun run --cwd website dev

# The site serves crates/slint-component/www/dist as-is, so this rebuilds the
# Slint gallery before starting the same dev server. Same as `make dev`; takes
# SLUGS too.
dev\:website-slint:
	@SLUGS="$(SLUGS)" $(MAKE) build\:wasm-slint-dev
	bun run --cwd website dev

# Debug WASM for one page of the Slint gallery, memory-capped.
#
# The site serves each page from its own folder under `/slint-gallery/pages`,
# and this writes the pages SLUGS names. Building every page in one catalog is
# what a single `rustc` cannot do on a 14 GiB laptop: the generated file holds
# every example and needs more memory than the machine can give. One page at a
# time keeps the peak in the low gigabytes and never risks the desktop.
#
#   make build:wasm-slint-dev SLUGS=dock
#   make build:wasm-slint-dev SLUGS=dock,menu
#   make build:wasm-slint-dev            the changed pages, or a starter set
build\:wasm-slint-dev:
	@set -e; \
	cd crates/slint-component; \
	slugs="$(SLUGS)"; \
	if [ -z "$$slugs" ]; then slugs="$$(./scripts/changed-pages.sh)"; fi; \
	if [ -z "$$slugs" ]; then echo "slint: nothing changed since the last build"; exit 0; fi; \
	for slug in $$(echo "$$slugs" | tr ',' ' '); do \
	  echo "slint: building $$slug"; \
	  SLINT_GALLERY_ONLY="$$slug" ./scripts/guard.sh ./scripts/build.sh --out "www/dist/pages/$$slug" >/dev/null; \
	done; \
	echo "slint: built $$(echo "$$slugs" | tr ',' ' ' | wc -w) page(s)"

# Release WASM for the GPUI component story gallery (`/gallery`).
build\:wasm-gpui:
	$(MAKE) -C crates/story-web build-wasm

# Every page of the Slint gallery, one bounded build each, into
# `www/dist/pages/*`. This is what a fresh clone or CI runs before the site can
# show the Slint source of every component.
build\:wasm-slint-pages:
	cd crates/slint-component && for slug in $$(./scripts/pages.sh); do \
	  echo "slint: $$slug"; \
	  SLINT_GALLERY_ONLY="$$slug" ./scripts/guard.sh ./scripts/build.sh --out "www/dist/pages/$$slug" >/dev/null || exit 1; \
	done

# Release WASM gallery bundle for Slint (`/slint-gallery`), every page in one
# module. Needs more memory than a 14 GiB machine can spare; prefer
# `build:wasm-slint-pages`.
build\:wasm-slint:
	$(MAKE) -C crates/slint-component build

# Release WASM for GPUI Base examples (`/examples/base`). Uses nightly Rust.
build\:wasm-base:
	$(MAKE) -C crates/base/examples/wasm build-wasm

# All website WASM targets (same three as release-docs CI).
build\:wasms: build\:wasm-gpui build\:wasm-slint build\:wasm-base

build\:website:
	bun run --cwd website build
