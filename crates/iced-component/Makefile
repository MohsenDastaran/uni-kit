.PHONY: build check run stories build-wasm build-page clean-wasm install-iced-bindgen

# A story to run or to build, and whether to run it dark. Both are empty by
# default: `make run` with no story shows the list the gallery falls back to.
STORY ?=
DARK ?=

# The components and the gallery, then every usage example. Both must be clean:
# an application installs this source, so a warning here is a warning there.
build:
	cargo build
	cargo build --examples

check:
	./scripts/check.sh

# The gallery natively — the same dispatcher the browser module calls.
#
#   make run                     the list of stories
#   make run STORY=button        one of them
#   make run STORY=button DARK=1 that one, dark
run:
	cargo run --bin iced-gallery -- $(STORY) $(if $(DARK),--dark,)

# Every story the gallery can show, one per line: the names `?story=` takes on the
# website and `make run STORY=<name>` accepts. Read from the demos themselves, so
# it cannot list something that has no demo.
stories:
	@ls src/demos/*.rs | sed -e 's|src/demos/||' -e 's|\.rs$$||' -e 's|_|-|g' \
	  | grep -vE '^(index|mod)$$' | sort

# The browser gallery the website serves at `/iced-gallery`.
#
# Release is the only profile offered. A debug iced module is around 90 MB against
# 7.8 MB, which a preview frame cannot ask anybody to download, and the whole
# gallery is one build that takes a couple of minutes either way.
build-wasm:
	./scripts/build-wasm.sh --release

# The page and the demo sources it serves, without touching the module. Enough
# to work on — or check — the page, its source view and its themes, and the
# release build runs it as its last step.
build-page:
	./scripts/build-page.sh

clean-wasm:
	rm -rf www/dist

# The wasm-bindgen CLI this crate's lockfile resolves, into a prefix of its own.
# The galleries in the workspace use 0.2.121 and the dev server serves whichever
# build ran last, so the two must not overwrite one another: `scripts/build-wasm.sh`
# prefers this prefix and leaves the shared `wasm-bindgen` alone.
install-iced-bindgen:
	@version=$$(grep -A 1 '^name = "wasm-bindgen"$$' Cargo.lock | grep '^version' | cut -d '"' -f 2); \
	echo "installing wasm-bindgen-cli $$version into $$HOME/.wasm-bindgen-iced"; \
	cargo install -f wasm-bindgen-cli --version "$$version" --root "$$HOME/.wasm-bindgen-iced"
