//! Composed GPUI screens that `uni-kit add gpui <name>` copies into an
//! application.
//!
//! A GPUI application already gets every control from the `gpui-kit` crate, so
//! what is worth copying for this framework is a *screen*: the controls arranged
//! for a purpose, with the spacing, grouping and keyboard path already decided.
//! Each module here is one such screen, written against the crate's public API
//! and nothing else, so the file the command writes into `src/components/`
//! compiles in the application that installed it.
//!
//! ```text
//! npx @dastaran/uni-kit@latest add gpui sidebar
//! ```
//!
//! A block is one file. It imports `gpui_kit` and `std`, uses the built-in
//! `IconName` set, and never reaches for a gallery helper or another block, so
//! installing one of them alone is enough.
//!
//! Run one to look at it:
//!
//! ```text
//! cargo run -p blocks -- sidebar
//! ```

pub mod dock;
pub mod settings;
pub mod sidebar;
