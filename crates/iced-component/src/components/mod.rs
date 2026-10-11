//! The components. One file each, named after the registry key with hyphens as
//! underscores, and this list is generated from the directory by
//! `scripts/mod-rs.mjs` — never edited by hand, because a module missing from it
//! is a component the crate silently does not compile.
//!
//! `theme` is shared: the CLI writes it once however many components are
//! installed, and a component reads it as `super::theme`, which keeps the
//! installed directory movable.

pub mod theme;

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod calendar;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod description_list;
pub mod empty;
pub mod group_box;
pub mod kbd;
pub mod label;
pub mod marker;
pub mod progress;
pub mod shimmer;
pub mod skeleton;
pub mod spinner;
pub mod tabs;
pub mod tag;
pub mod tree;
