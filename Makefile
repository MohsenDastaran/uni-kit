dev-web:
	cd crates/story-web && make dev

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
#   make dev                 every page, then the site
#   make dev SLUGS=dock      just `dock`, then the site
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

# Debug WASM for the Slint gallery, memory-capped. The dev counterpart of
# `build:wasm-slint`: same output, no release optimization. SLUGS limits it to
# the named example pages; empty means every page.
build\:wasm-slint-dev:
	cd crates/slint-component && SLINT_GALLERY_ONLY="$(SLUGS)" ./scripts/guard.sh ./scripts/build.sh

# Release WASM for the GPUI component story gallery (`/gallery`).
build\:wasm-gpui:
	$(MAKE) -C crates/story-web build-wasm

# Release WASM gallery bundle for Slint (`/slint-gallery`).
build\:wasm-slint:
	$(MAKE) -C crates/slint-component build

# Release WASM for GPUI Base examples (`/examples/base`). Uses nightly Rust.
build\:wasm-base:
	$(MAKE) -C crates/base/examples/wasm build-wasm

# All website WASM targets (same three as release-docs CI).
build\:wasms: build\:wasm-gpui build\:wasm-slint build\:wasm-base

build\:website:
	bun run --cwd website build
