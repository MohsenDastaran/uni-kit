//! `settings-block` — the Blocks page's preferences screen: the heading that
//! says what saving will do, then the groups the rows belong to.
//!
//! A block is not a component sample. This is the finished application screen the
//! site's Blocks page shows for iced, composed from the same files
//! `uni-kit add iced` writes: two [`Card`]s carrying the groups, the [`Settings`]
//! rows inside them, a [`Switch`] on every row, and the [`Button`]s that answer
//! the heading.
//!
//! The rows are one state. Every switch reports the value it wants, the block
//! stores it, and the heading reads whether anything has changed since the last
//! save — the subtitle says so and `Save changes` is only pressable while it is
//! true, which is GPUI's block. `Reset` puts every field back to the value the
//! screen opened with.
//!
//! The `Dark mode` row is a value like the rest. This screen is shown in the
//! palette the embedding page asked for, and switching that palette from inside a
//! demo would take the page's own theme away, so the row records the preference
//! without applying it — the existing `settings` demo makes the same choice.
//!
//! Run it with `cargo run --bin iced-gallery -- settings-block`, or in the
//! browser as `?story=settings-block` in the gallery the website embeds. The
//! Blocks page asks for that story and reads this file for its usage prompt.

use iced::font::Weight;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Border, Element, Font, Length, Task, Theme};

use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::card::Card;
use crate::components::settings::{SettingRow, Settings};
use crate::components::switch::Switch;
use crate::components::theme::{self, Tokens};

/// The screen's inset, the Slint block's `padding: 24px`, and the gap between
/// the heading and the cards under it.
const PAGE: f32 = 24.0;
const GAP: f32 = 20.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — settings")
        .theme(theme)
        .run()
}

/// One row's preference. The screen has six, and the two groups are the two
/// cards; an enum keeps a switch's reader and its writer on the same field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Setting {
    DarkMode,
    ReduceMotion,
    CompactDensity,
    DesktopAlerts,
    Sounds,
    WeeklySummary,
}

impl Setting {
    /// What the row is currently set to.
    fn value(self, app: &App) -> bool {
        match self {
            Self::DarkMode => app.dark_mode,
            Self::ReduceMotion => app.reduce_motion,
            Self::CompactDensity => app.compact_density,
            Self::DesktopAlerts => app.desktop_alerts,
            Self::Sounds => app.sounds,
            Self::WeeklySummary => app.weekly_summary,
        }
    }

    /// Where a switch's report lands.
    fn set(self, app: &mut App, value: bool) {
        match self {
            Self::DarkMode => app.dark_mode = value,
            Self::ReduceMotion => app.reduce_motion = value,
            Self::CompactDensity => app.compact_density = value,
            Self::DesktopAlerts => app.desktop_alerts = value,
            Self::Sounds => app.sounds = value,
            Self::WeeklySummary => app.weekly_summary = value,
        }
    }
}

/// The screen's whole state: the six preferences, and whether anything has
/// changed since the last save. Every part is the caller's, which is the
/// controlled shape the settings rows keep.
#[derive(Debug, Clone)]
struct App {
    dark_mode: bool,
    reduce_motion: bool,
    compact_density: bool,
    desktop_alerts: bool,
    sounds: bool,
    weekly_summary: bool,
    /// Whether a field has moved since the last save. The heading reads it.
    dirty: bool,
}

impl Default for App {
    fn default() -> Self {
        // The values the Slint block opens with: the dark palette on, motion
        // kept, the compact rows on, desktop alerts on and the quieter two off.
        Self {
            dark_mode: true,
            reduce_motion: false,
            compact_density: true,
            desktop_alerts: true,
            sounds: false,
            weekly_summary: false,
            dirty: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A switch reported the value it wants.
    Toggle(Setting, bool),
    /// `Save changes`: where an application would write the preferences.
    Save,
    /// `Reset`: every field goes back to the value the screen opened with.
    Reset,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Toggle(setting, value) => {
            setting.set(app, value);
            app.dirty = true;
        }
        Message::Save => app.dirty = false,
        Message::Reset => *app = App::default(),
    }
    Task::none()
}

/// One row: the title, the description, and the switch at the trailing edge.
/// The switch reports the value it wants, and the enum says which field it is.
fn switch_row<'a>(
    app: &App,
    setting: Setting,
    title: &'static str,
    description: &'static str,
) -> SettingRow<'a, Message> {
    SettingRow::new(title).description(description).field(
        Switch::new(setting.value(app)).on_toggle(move |value| Message::Toggle(setting, value)),
    )
}

/// A group's card: its title and description above the rows. The surface is the
/// card's, so the rows draw none of their own.
fn group<'a>(
    title: &'static str,
    description: &'static str,
    rows: impl IntoIterator<Item = SettingRow<'a, Message>>,
) -> Element<'a, Message> {
    Card::new()
        .title(title)
        .description(description)
        .body(Settings::new().rows(rows))
        .into()
}

/// The heading: what this screen is for, whether anything is waiting to be
/// written, and the two actions that answer it. `Save changes` is pressable only
/// while something has changed, which is iced's spelling of GPUI's disabled
/// button: no message means no press.
fn heading<'a>(app: &App) -> Element<'a, Message> {
    let lines = column![
        text("Preferences")
            .size(theme::FONT_SIZE_XL)
            .font(Font {
                weight: Weight::Semibold,
                ..Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        text(if app.dirty {
            "Changes apply to this device as soon as you save."
        } else {
            "All changes are saved."
        })
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        }),
    ]
    .spacing(theme::SPACING_XS)
    .width(Length::Fill);

    let save = Button::new("Save changes").variant(ButtonVariant::Primary);
    let save = if app.dirty {
        save.on_press(Message::Save)
    } else {
        save
    };

    row![lines, Button::new("Reset").on_press(Message::Reset), save,]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let appearance = group(
        "Appearance",
        "How the application looks on this machine.",
        [
            switch_row(app, Setting::DarkMode, "Dark mode", "Use the dark palette."),
            switch_row(
                app,
                Setting::ReduceMotion,
                "Reduce motion",
                "Skip non-essential animation.",
            ),
            switch_row(
                app,
                Setting::CompactDensity,
                "Compact density",
                "Fit more rows on screen.",
            ),
        ],
    );

    let notifications = group(
        "Notifications",
        "What the application is allowed to interrupt you with.",
        [
            switch_row(
                app,
                Setting::DesktopAlerts,
                "Desktop alerts",
                "Show a banner for new activity.",
            ),
            switch_row(
                app,
                Setting::Sounds,
                "Sounds",
                "Play a sound for new messages.",
            ),
            switch_row(
                app,
                Setting::WeeklySummary,
                "Weekly summary",
                "Email a digest every Monday.",
            ),
        ],
    );

    // The inset belongs to the page the scroll owner carries, not to a
    // container around the scroll owner: the bar then resolves against the
    // frame's own trailing edge, in the page padding, instead of drawing over
    // the heading's buttons and the cards.
    let page = column![heading(app), appearance, notifications]
        .spacing(GAP)
        .width(Length::Fill)
        .padding(PAGE);

    // The screen scrolls rather than clipping: at a short frame the second group
    // falls below the bottom edge, and a preferences page that cannot reach its
    // lower half is not a page. The bar keeps its own lane in the page padding.
    container(scrollable(page).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme::alpha(Tokens::of(theme).muted, 0.25).into()),
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        })
        .into()
}
