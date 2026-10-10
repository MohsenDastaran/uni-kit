# syntax=docker/dockerfile:1

# The component previews are WebAssembly, and the site only serves them when the
# built galleries sit inside its own output: `/gallery`, `/examples/base` and
# `/slint-gallery` are Vite dev-server middleware in development, so a production
# build has to carry the files. Building them needs the wasm toolchain, and the
# Slint catalog's own script says a full compile wants more than a 14 GiB
# machine. That is why this image is built in CI and Coolify only pulls it.

# ---- galleries and site ----------------------------------------------------
FROM rust:1-bookworm AS build

# `node` is not optional: `crates/slint-component/scripts/changed-pages.sh` execs
# it to write the per-page build stamp, and that runs at the end of every page.
#
# It has to be a *modern* node. Debian bookworm ships 18, whose `import.meta.dirname`
# is `undefined`, and the script reads it on its first line -- `join(undefined, '..')`
# throws. Node 20.11 added that property, so this installs 22 from NodeSource and
# asserts the version here, where a mistake is obvious, rather than forty minutes
# into the wasm build.
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential pkg-config libssl-dev ca-certificates curl git unzip gnupg \
 && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && node --version \
 && rm -rf /var/lib/apt/lists/*

# The base examples build with `cargo +nightly`; story-web and the Slint gallery
# use the default stable toolchain. CI installs both, so both get the target.
#
# The nightly is dated, and installed with everything the story-web toolchain
# file asks for (rustfmt, clippy, the wasm target). Nothing may be installed while
# the wasm build runs: rustup would replace the toolchain's wasm32 std, and that
# replacement renames the old directory into `$RUSTUP_HOME/tmp`, which fails with
# `Invalid cross-device link` because the toolchain sits in a lower overlay layer.
# Keep the date in step with `crates/story-web/rust-toolchain.toml`.
RUN rustup toolchain install nightly-2026-10-10 --profile minimal \
      --component rustfmt --component clippy \
      --target wasm32-unknown-unknown \
 && rustup target add wasm32-unknown-unknown

# The web half of every gallery is a bun build. The version matches CI.
RUN curl -fsSL https://bun.sh/install | bash
ENV BUN_INSTALL="/root/.bun" \
    PATH="/root/.bun/bin:${PATH}"

# wasm-bindgen must match the crate exactly, or the generated glue is rejected by
# the compiled module. The lock file is copied first so this layer caches across
# source changes.
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    version=$(grep -A 1 '^name = "wasm-bindgen"$' Cargo.lock | grep '^version' | cut -d '"' -f 2) \
 && cargo install -f wasm-bindgen-cli --version "$version" \
 && wasm-bindgen --version

WORKDIR /src
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    make -C crates/story-web build-prod \
 && make -C crates/base/examples/wasm build

# The Slint gallery cannot be built as one catalog. `build.rs` folds every example
# into a single generated Rust file, and compiling that file is killed for memory
# on a 16 GiB runner -- the crate's own script says it wants more than a 14 GiB
# machine, and its release profile only tunes `opt-level` to buy a little room.
#
# `SLINT_GALLERY_ONLY` narrows the catalog to one page, which the script documents
# as the way to keep this build inside a memory cap. Each page lands in its own
# folder, which is the layout the site's preview iframe asks for at
# `/slint-gallery/pages/<slug>`. One page takes about twenty seconds once the
# dependencies are compiled, so the 76 pages cost well under an hour.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/src/crates/slint-component/target \
    cd crates/slint-component \
 && mkdir -p www/dist/pages \
 && total=$(./scripts/pages.sh | wc -l) \
 && i=0 \
 && for slug in $(./scripts/pages.sh); do \
      i=$((i + 1)); \
      echo "[slint] $i/$total $slug"; \
      SLINT_GALLERY_ONLY="$slug" ./scripts/build.sh --release --out "www/dist/pages/$slug" || exit 1; \
    done

# The galleries are tens of megabytes each, and nginx deliberately does not
# compress wasm on the fly. Compress it once here instead: `gzip_static on`
# serves the `.wasm.gz` beside each module as-is, so the transfer shrinks ~2.5x
# with no per-request cost, and the module itself is untouched. This runs before
# the site build, which embeds the compressed sizes in the pages that explain
# each download.
RUN node website/scripts/precompress-wasm.mjs \
      crates/story-web/www/dist \
      crates/base/examples/wasm/www/dist \
      crates/slint-component/www/dist

# The site reads the Story and Slint sources from the repository root to count
# examples and to render the Slint samples, so it has to build from here rather
# than from `website/` alone. The copies mirror release-docs.yml.
#
# `GITHUB_TOKEN` only lifts the rate limit for the navbar's star count, which the
# build falls back to 0 without. It arrives as a secret mount rather than an ARG so
# the value is not recorded in the image metadata.
RUN --mount=type=secret,id=github_token \
    cd website \
 && bun install --frozen-lockfile \
 && GITHUB_TOKEN="$(cat /run/secrets/github_token 2>/dev/null || true)" bun run build \
 && mkdir -p dist/gallery dist/examples/base dist/slint-gallery \
 && cp -r ../crates/story-web/www/dist/. dist/gallery/ \
 && cp -r ../crates/base/examples/wasm/www/dist/. dist/examples/base/ \
 && cp -r ../crates/slint-component/www/dist/. dist/slint-gallery/

# ---- server ----------------------------------------------------------------
FROM nginx:alpine AS runtime

COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /src/website/dist /usr/share/nginx/html

EXPOSE 80
