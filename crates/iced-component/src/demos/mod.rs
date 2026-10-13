//! The demos the gallery shows: one module each, named after the registry key
//! with hyphens as underscores, and this list is generated from the directory by
//! `scripts/mod-rs.mjs` — never edited by hand.
//!
//! A demo is the whole application, not a view: each one calls
//! `iced::application` itself, so the native binary in `examples/` and the
//! browser gallery run the same code rather than two copies that drift apart.

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
pub mod combobox;
pub mod date_picker;
pub mod description_list;
pub mod dialog;
pub mod dropdown_button;
pub mod empty;
pub mod form;
pub mod group_box;
pub mod icon;
pub mod input;
pub mod kbd;
pub mod label;
pub mod list;
pub mod marker;
pub mod menu;
pub mod message_role;
pub mod message_scroller;
pub mod number_input;
pub mod otp_input;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
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
pub mod tag;
pub mod text_view;
pub mod textarea;
pub mod toggle;
pub mod tooltip;
pub mod tree;

mod index;

/// Every story the gallery can show, named the way the website names it: the
/// registry key, hyphenated.
pub const STORIES: &[&str] = &[
    "accordion",
    "alert",
    "alert-dialog",
    "attachment",
    "avatar",
    "badge",
    "button",
    "calendar",
    "card",
    "checkbox",
    "combobox",
    "date-picker",
    "description-list",
    "dialog",
    "dropdown-button",
    "empty",
    "form",
    "group-box",
    "icon",
    "input",
    "kbd",
    "label",
    "list",
    "marker",
    "menu",
    "message-role",
    "message-scroller",
    "number-input",
    "otp-input",
    "pagination",
    "popover",
    "progress",
    "radio",
    "scrollable",
    "select",
    "sheet",
    "shimmer",
    "skeleton",
    "slider",
    "spinner",
    "stepper",
    "switch",
    "table",
    "tag",
    "text-view",
    "textarea",
    "toggle",
    "tooltip",
    "tree",
];

/// Run the demo a story names, with the theme the page asked for.
///
/// An unknown story is not an error. The gallery is also how somebody finds out
/// what exists, so a name that matches nothing shows the list rather than a blank
/// window or a panic.
pub fn run(story: &str, theme: iced::Theme) -> iced::Result {
    match story {
        "accordion" => accordion::run(theme),
        "alert" => alert::run(theme),
        "alert-dialog" => alert_dialog::run(theme),
        "attachment" => attachment::run(theme),
        "avatar" => avatar::run(theme),
        "badge" => badge::run(theme),
        "button" => button::run(theme),
        "calendar" => calendar::run(theme),
        "card" => card::run(theme),
        "checkbox" => checkbox::run(theme),
        "combobox" => combobox::run(theme),
        "date-picker" => date_picker::run(theme),
        "description-list" => description_list::run(theme),
        "dialog" => dialog::run(theme),
        "dropdown-button" => dropdown_button::run(theme),
        "empty" => empty::run(theme),
        "form" => form::run(theme),
        "group-box" => group_box::run(theme),
        "icon" => icon::run(theme),
        "input" => input::run(theme),
        "kbd" => kbd::run(theme),
        "label" => label::run(theme),
        "list" => list::run(theme),
        "marker" => marker::run(theme),
        "menu" => menu::run(theme),
        "message-role" => message_role::run(theme),
        "message-scroller" => message_scroller::run(theme),
        "number-input" => number_input::run(theme),
        "otp-input" => otp_input::run(theme),
        "pagination" => pagination::run(theme),
        "popover" => popover::run(theme),
        "progress" => progress::run(theme),
        "radio" => radio::run(theme),
        "scrollable" => scrollable::run(theme),
        "select" => select::run(theme),
        "sheet" => sheet::run(theme),
        "shimmer" => shimmer::run(theme),
        "skeleton" => skeleton::run(theme),
        "slider" => slider::run(theme),
        "spinner" => spinner::run(theme),
        "stepper" => stepper::run(theme),
        "switch" => switch::run(theme),
        "table" => table::run(theme),
        "tag" => tag::run(theme),
        "text-view" => text_view::run(theme),
        "textarea" => textarea::run(theme),
        "toggle" => toggle::run(theme),
        "tooltip" => tooltip::run(theme),
        "tree" => tree::run(theme),
        other => index::run(theme, other.to_string()),
    }
}
