.PHONY: build check run

# The components and the gallery, then every usage example. Both must be clean:
# an application installs this source, so a warning here is a warning there.
build:
	cargo build
	cargo build --examples

check:
	./scripts/check.sh

run:
	cargo run --bin iced-gallery
