# syntax=docker/dockerfile:1

# The component previews are WebAssembly, and the site only serves them when the
# built galleries sit inside its own output: `/gallery`, `/examples/base` and
# `/slint-gallery` are Vite dev-server middleware in development, so a production
# build has to carry the files. Building them needs the wasm toolchain, and the
# Slint catalog's own script says a full compile wants more than a 14 GiB
# machine. That is why this image is built in CI and Coolify only pulls it.

# ---- galleries and site ----------------------------------------------------
FROM rust:1-bookworm AS build

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential pkg-config libssl-dev ca-certificates curl git unzip \
 && rm -rf /var/lib/apt/lists/*

# The base examples build with `cargo +nightly`; story-web and the Slint gallery
# use the default stable toolchain. CI installs both, so both get the target.
RUN rustup toolchain install nightly --profile minimal \
 && rustup target add wasm32-unknown-unknown \
 && rustup target add wasm32-unknown-unknown --toolchain nightly

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
 && cargo install -f wasm-bindgen-cli --version "$version"

WORKDIR /src
COPY . .

# `GITHUB_TOKEN` only lifts the rate limit for the navbar's star count, which the
# build falls back to 0 without. It is an ARG and is passed inline, so the value
# never becomes part of a layer.
ARG GITHUB_TOKEN

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/src/crates/slint-component/target \
    make -C crates/story-web build-prod \
 && make -C crates/base/examples/wasm build \
 && make -C crates/slint-component build

# The site reads the Story and Slint sources from the repository root to count
# examples and to render the Slint samples, so it has to build from here rather
# than from `website/` alone. The copies mirror release-docs.yml.
RUN cd website \
 && bun install --frozen-lockfile \
 && GITHUB_TOKEN="$GITHUB_TOKEN" bun run build \
 && mkdir -p dist/gallery dist/examples/base dist/slint-gallery \
 && cp -r ../crates/story-web/www/dist/. dist/gallery/ \
 && cp -r ../crates/base/examples/wasm/www/dist/. dist/examples/base/ \
 && cp -r ../crates/slint-component/www/dist/. dist/slint-gallery/

# ---- server ----------------------------------------------------------------
FROM nginx:alpine AS runtime

COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /src/website/dist /usr/share/nginx/html

EXPOSE 80
