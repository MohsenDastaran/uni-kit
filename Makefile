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
# The rebuild is capped in a cgroup (script/guard.sh, shared with the iced
# gallery) because a single rustc
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

# The site serves crates/iced-component/www/dist at `/iced-gallery`, the folder
# the iced build writes. One module answers for every component — `?story=<key>`
# picks the demo — so unlike Slint there is nothing to build per page, and the
# whole gallery is one bounded build.
# Release, not debug: a debug iced module is about 90 MB against 7.8 MB, which is
# not a preview anybody waits for. One build covers every component here, so the
# release build is affordable — unlike Slint, whose catalogue has to be split by
# page to fit in memory at all.
#
#   make dev:website-iced        after changing a demo or a component
#   make dev:website-iced-page   after changing only the page or the sources
dev\:website-iced:
	$(MAKE) -C crates/iced-component build-wasm
	bun run --cwd website dev

# The same gallery without the module. The page, its source view and its themes
# need no Rust at all — `scripts/build-page.sh` only copies files — so this is the
# loop for working on them, and it is seconds rather than minutes. The module is
# whatever the last build left in www/dist.
dev\:website-iced-page:
	$(MAKE) -C crates/iced-component build-page
	bun run --cwd website dev

# Release WASM for the iced gallery (`/iced-gallery`).
build\:wasm-iced:
	$(MAKE) -C crates/iced-component build-wasm

# The page and the demo sources the gallery serves, without the module. The
# release build writes them too; this writes them on their own.
build\:page-iced:
	$(MAKE) -C crates/iced-component build-page

# NSH (Native Shell): one TSX app per component, compiled by the `nsh` CLI into
# `/nsh-gallery?story=<key>`. The crate keeps its own app root under examples/;
# the site serves `crates/nsh-component/www/dist` through the same middleware.
dev\:website-nsh:
	$(MAKE) -C crates/nsh-component gallery
	bun run --cwd website dev

# The player and its runtime without compiling a demo.
dev\:website-nsh-page:
	$(MAKE) -C crates/nsh-component gallery-page
	bun run --cwd website dev

# Release gallery for `/nsh-gallery`.
build\:wasm-nsh:
	$(MAKE) -C crates/nsh-component gallery

build\:page-nsh:
	$(MAKE) -C crates/nsh-component gallery-page

# The NSH crate gate: generated icons/theme, theme parity, type check, every
# demo rendered headlessly, the gallery build, the manifest and the checklist.
check\:nsh:
	$(MAKE) -C crates/nsh-component check

# One NSH demo in a development host.
#
#   make run:nsh STORY=button                  web
#   make run:nsh STORY=button TARGET=linux     GTK 4
#   make run:nsh STORY=button TARGET=macos     AppKit
#   make run:nsh STORY=button TARGET=windows   Win32 / Direct2D
#   make run:nsh STORY=button TARGET=android   device or emulator
run\:nsh:
	$(MAKE) -C crates/nsh-component run STORY="$(STORY)" TARGET="$(or $(TARGET),web)"

# Every story `?story=` takes, one per line.
stories\:nsh:
	@$(MAKE) -C crates/nsh-component stories

# Remove the built gallery.
clean\:nsh:
	$(MAKE) -C crates/nsh-component clean

# The pinned NSH CLI and its runtimes.
install\:nsh:
	script/install-nsh.sh

# The iced crate's own gate: the module lists, the library and every example,
# theme parity with the Slint tokens, the feature list against the install hint,
# the generated manifest, and the component checklist. The checklist is the one
# step that fails while components are missing, which is the point of it.
check\:iced:
	$(MAKE) -C crates/iced-component check

# The gallery natively, through the same dispatcher the browser module calls.
# With no story it lists what there is.
#
#   make run:iced                  the list
#   make run:iced STORY=button     one story
#   make run:iced STORY=button DARK=1
run\:iced:
	$(MAKE) -C crates/iced-component run STORY="$(STORY)" DARK="$(DARK)"

# Every story `?story=` takes, one per line, read from the demos themselves.
stories\:iced:
	@$(MAKE) -C crates/iced-component stories

# The wasm-bindgen CLI the iced lockfile resolves, installed into a prefix of its
# own. The workspace's galleries use a different version, and one binary cannot
# serve both, so this leaves theirs alone.
install\:iced-bindgen:
	$(MAKE) -C crates/iced-component install-iced-bindgen

# Remove the built gallery. Nothing outside crates/iced-component/www/dist.
clean\:iced:
	$(MAKE) -C crates/iced-component clean-wasm

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
	  SLINT_GALLERY_ONLY="$$slug" ../../script/guard.sh ./scripts/build.sh --out "www/dist/pages/$$slug" >/dev/null; \
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
	  SLINT_GALLERY_ONLY="$$slug" ../../script/guard.sh ./scripts/build.sh --out "www/dist/pages/$$slug" >/dev/null || exit 1; \
	done

# Release WASM gallery bundle for Slint (`/slint-gallery`), every page in one
# module. Needs more memory than a 14 GiB machine can spare; prefer
# `build:wasm-slint-pages`.
build\:wasm-slint:
	$(MAKE) -C crates/slint-component build

# Release WASM for GPUI Base examples (`/examples/base`). Uses nightly Rust.
build\:wasm-base:
	$(MAKE) -C crates/base/examples/wasm build-wasm

# All website WASM targets (the same five as release-website CI).
build\:wasms: build\:wasm-gpui build\:wasm-slint build\:wasm-base build\:wasm-iced build\:wasm-nsh

build\:website:
	bun run --cwd website build
