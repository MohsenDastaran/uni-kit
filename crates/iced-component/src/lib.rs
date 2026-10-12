//! The iced component catalog, as the crate the gallery and the compile checks
//! build. An application does not depend on this crate: it copies the component
//! it wants out of `src/components/`, together with `theme.rs`, into its own
//! project — which is what `npx uni-kit add iced <component>` does.
//!
//! Everything here is compiled by `cargo build`, and every usage example in
//! `examples/` by `cargo build --examples`.
//!
//! `demos` holds the same demos as whole applications, so the gallery the
//! website embeds can show one — `?story=<registry key>` — without a second copy
//! of it. The binaries in `examples/` are the thin mains that call them.

pub mod bridge;
pub mod components;
pub mod demos;

// The browser entry point. Native builds use the binary instead, so this is only
// compiled when the module is meant for a page.
#[cfg(target_arch = "wasm32")]
pub mod wasm;
