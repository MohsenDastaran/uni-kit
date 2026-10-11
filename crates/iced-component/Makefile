.PHONY: build check run build-wasm clean-wasm

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
