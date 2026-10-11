.PHONY: build check run build-wasm clean-wasm install-iced-bindgen

# The components and the gallery, then every usage example. Both must be clean:
# an application installs this source, so a warning here is a warning there.
build:
	cargo build
	cargo build --examples

check:
	./scripts/check.sh

run:
	cargo run --bin iced-gallery

# The browser gallery the website serves at `/iced-gallery`.
#
# Release is the only profile offered. A debug iced module is around 94 MB, which
# a preview frame cannot ask anybody to download, and the whole gallery is one
# build that takes a couple of minutes either way.
build-wasm:
	./scripts/build-wasm.sh --release

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
