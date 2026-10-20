# syntax=docker/dockerfile:1

# The component previews are WebAssembly, and the site only serves them when the
# built galleries sit inside its own output: `/gallery`, `/examples/base`,
# `/slint-gallery`, `/iced-gallery` and `/nsh-gallery` are Vite dev-server middleware in
# development, so a production build has to carry the files. Building them needs
# the wasm toolchain, and the Slint catalog's own script says a full compile
# wants more than a 14 GiB machine. That is why this image is built in CI and
# Coolify only pulls it.

# ---- NSH gallery -----------------------------------------------------------
# The prebuilt `nsh` CLI needs glibc 2.39 and the main stage is Debian bookworm
# (2.36), so the TSX gallery is compiled in an Ubuntu 24.04 stage and copied in.
# Only the output crosses the stage boundary; the 85 MB distribution does not
# reach the runtime image.
FROM ubuntu:24.04 AS nsh-gallery
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl nodejs \
 && rm -rf /var/lib/apt/lists/*
ENV NSH_VERSION=v0.1.1
RUN sh -c "$(curl -fsSL https://nsh.dev/install.sh)"
ENV PATH="/root/.local/bin:${PATH}"
WORKDIR /src
COPY crates/nsh-component crates/nsh-component
RUN node crates/nsh-component/scripts/build-gallery.mjs

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
      build-essential pkg-config libssl-dev ca-certificates curl git unzip gnupg binaryen \
 && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && node --version \
 && rm -rf /var/lib/apt/lists/*

# `OPTIMIZE_WASM=0` ships the gallery modules exactly as rustc builds them,
# without wasm-opt. They are the only artifact that switch affects, so a preview
# that misbehaves after an optimization has a one-variable way back.
ARG OPTIMIZE_WASM=1
ENV OPTIMIZE_WASM=${OPTIMIZE_WASM}

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

# The iced crate keeps its own lockfile, and its `wasm-bindgen` is a different
# version from the workspace's, so it gets its own CLI rather than replacing the
# one the other galleries need. Two versions cannot share one binary, and the
# crate's build script looks in this prefix first. Its lock file is copied just
# as early, for the same caching reason.
COPY crates/iced-component/Cargo.lock crates/iced-component/Cargo.lock
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    version=$(grep -A 1 '^name = "wasm-bindgen"$' crates/iced-component/Cargo.lock | grep '^version' | cut -d '"' -f 2) \
 && cargo install -f wasm-bindgen-cli --version "$version" --root /root/.wasm-bindgen-iced \
 && /root/.wasm-bindgen-iced/bin/wasm-bindgen --version

WORKDIR /src
COPY . .
COPY --from=nsh-gallery /src/crates/nsh-component/www/dist crates/nsh-component/www/dist

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

# One module answers for every iced component -- `?story=<registry key>` picks
# the demo -- so unlike the Slint catalog this is a single bounded build. The CLI
# is named outright rather than left to `$HOME` or `PATH`, so the wrong
# wasm-bindgen cannot be picked up by accident.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/crates/iced-component/target \
    WASM_BINDGEN=/root/.wasm-bindgen-iced/bin/wasm-bindgen \
    make -C crates/iced-component build-wasm

# The galleries are tens of megabytes each, and nginx deliberately does not
# compress wasm on the fly. Compress it once here instead: `gzip_static on`
# serves the `.wasm.gz` beside each module as-is, so the transfer shrinks ~2.5x
# with no per-request cost, and the module itself is untouched. This runs before
# the site build, which embeds the compressed sizes in the pages that explain
# each download.
RUN node website/scripts/precompress-wasm.mjs \
      crates/story-web/www/dist \
      crates/base/examples/wasm/www/dist \
      crates/slint-component/www/dist \
      crates/iced-component/www/dist \
      crates/nsh-component/www/dist

# The site reads the Story, Slint, iced and NSH sources from the repository root
# to count examples and to render their samples, so it has to build
# from here rather than from `website/` alone. The copies mirror
# release-website.yml.
#
# Each gallery is checked before it is copied: a missing or half-built gallery
# used to be copied as an empty directory, which deploys as a page whose iframe
# answers 404. Failing the image build is the only way that mistake is visible.
#
# The check accepts both layouts the galleries use: Story, the base examples and
# iced are one page at the root of their `www/dist`, so they have an
# `index.html`; the Slint catalog is a folder per page under `pages/` and has no
# root page at all, because the image never runs the memory-hungry whole-catalog
# build that would write one. A gallery must match one of the two and hold at
# least one module; an empty or half-written folder still fails.
#
# `GITHUB_TOKEN` only lifts the rate limit for the navbar's star count, which the
# build falls back to 0 without. It arrives as a secret mount rather than an ARG so
# the value is not recorded in the image metadata.
RUN --mount=type=secret,id=github_token \
    cd website \
 && bun install --frozen-lockfile \
 && GITHUB_TOKEN="$(cat /run/secrets/github_token 2>/dev/null || true)" bun run build \
 && for gallery in \
      ../crates/story-web/www/dist:gallery \
      ../crates/base/examples/wasm/www/dist:examples/base \
      ../crates/slint-component/www/dist:slint-gallery \
      ../crates/iced-component/www/dist:iced-gallery \
      ../crates/nsh-component/www/dist:nsh-gallery; do \
      source_dir="${gallery%%:*}"; target_dir="${gallery#*:}"; \
      if { [ ! -f "$source_dir/index.html" ] && [ ! -d "$source_dir/pages" ]; } || \
         ! find "$source_dir" -name '*.wasm' -print -quit | grep -q .; then \
        echo "::error::$source_dir holds no built gallery; dist/$target_dir would deploy as a 404." >&2; \
        exit 1; \
      fi; \
      mkdir -p "dist/$target_dir" \
      && cp -r "$source_dir/." "dist/$target_dir/"; \
    done

# ---- server ----------------------------------------------------------------
FROM nginx:alpine AS runtime

COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /src/website/dist /usr/share/nginx/html

EXPOSE 80
