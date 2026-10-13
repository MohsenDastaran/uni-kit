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
pub mod alert_dialog;
pub mod attachment;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod calendar;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod color_picker;
pub mod combobox;
pub mod date_picker;
pub mod description_list;
pub mod dialog;
pub mod dropdown_button;
pub mod empty;
pub mod form;
pub mod group_box;
pub mod hover_card;
pub mod icon;
pub mod input;
pub mod kbd;
pub mod label;
pub mod list;
pub mod marker;
pub mod menu;
pub mod message_role;
pub mod message_scroller;
pub mod notification;
pub mod number_input;
pub mod otp_input;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod scrollable;
pub mod select;
pub mod sheet;
pub mod shimmer;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod stepper;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod text_view;
pub mod textarea;
pub mod toggle;
pub mod tooltip;
pub mod tree;
