//! `badge` — a count, a dot or an icon anchored to the corner of its child.
//!
//! Run it with `cargo run --example badge`, or in the browser as
//! `?story=badge` in the gallery the website embeds.
//!
//! The Slint example's six outlined sections — Icon, Count, Badge icon, Dot,
//! Color and Sizes — are the six here, in the same order, each under the heading
//! the component page documents it at. The `// section: <title>` comments below
//! are the contract the website reads to cut this file into the usage panels a
//! component page shows, so the code on the page is the code that runs here.
//! Keep them, one per section, in order, and keep the closing `// section: end`
//! after the last one.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`],
//! the same bridge the Slint and GPUI galleries use.
//!
//! The child is the application's: Slint resolves `Icon { name: "bell" }` and
//! `Avatar { initials: "AL" }` from its own folders, while iced's
//! [`Icon`](crate::components::icon::Icon) takes an SVG the caller supplies and
//! [`Avatar`](crate::components::avatar::Avatar) takes initials directly, so the
//! demo embeds the same Lucide marks the Slint example names and the same
//! stand-in avatars. The badge's own icon is the caller's element for the same
//! reason, and the demo inks it with the chip's solid-fill foreground.

use iced::widget::{column, container, row, scrollable, svg, text};
use iced::{Alignment, Color, Element, Length, Task, Theme};

use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::badge::Badge;
use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 6] = ["Icon", "Count", "Badge icon", "Dot", "Color", "Sizes"];

pub fn run(theme: Theme) -> iced::Result {
    // The palette in force at boot is the one the view draws explicit colours
    // from. Embedded, the gallery has already parked the page's own palette, so
    // this is the page's set; native, and standalone in a browser, it is the
    // built-in light or dark table.
    let tokens = Tokens::of(&theme);
    iced::application(move || boot(tokens), update, view)
        .title("uni-kit — badge")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The live count the control under the sections drives.
    count: u32,
    /// The tokens the view names a colour from explicitly — the chip fills the
    /// badge does not take from its own default.
    tokens: Tokens,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Clear,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot(tokens: Tokens) -> App {
    App {
        count: 3,
        tokens,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Increment => app.count += 1,
        Message::Clear => app.count = 0,
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// The Lucide `bell` mark, from the set the Slint example names.
const BELL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/></svg>"#;

/// The Lucide `inbox` mark.
const INBOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"#;

/// The Lucide `check` mark.
const CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>"#;

/// The Lucide `star` mark.
const STAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/></svg>"#;

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// An icon the application supplies, drawn in the theme's foreground, which is
/// what the Slint `Icon` defaults its colour to.
fn icon_mark<'a>(handle: &'static [u8], pixels: f32) -> Element<'a, Message> {
    Icon::new(svg::Handle::from_memory(handle))
        .size(pixels)
        .into()
}

/// An avatar stand-in, at the Slint example's medium edge.
fn avatar_mark<'a>(initials: &'static str) -> Element<'a, Message> {
    Avatar::new()
        .initials(initials)
        .size(AvatarSize::Medium)
        .into()
}

/// The glyph inside the badge's own round chip. The chip's fill is a solid
/// colour, so the caller inks it with the solid-fill foreground, the same token
/// the component draws its count in.
fn chip_mark<'a>(handle: &'static [u8], ink: Color) -> Element<'a, Message> {
    Icon::new(svg::Handle::from_memory(handle))
        .size(10.0)
        .color(ink)
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = iced::widget::mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| container::Style {
            text_color: Some(Tokens::of(theme).muted_foreground),
            ..container::Style::default()
        }),
    )
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    let icon = iced::widget::mouse_area(
        container(
            svg(svg::Handle::from_memory(CODE_XML))
                .width(16.0)
                .height(16.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).primary),
                }),
        )
        .width(28.0)
        .height(28.0)
        .center_x(28.0)
        .center_y(28.0)
        .style(move |theme: &Theme| {
            let primary = Tokens::of(theme).primary;
            let (fill, line) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(line))
                    .width(1.0),
                ..container::Style::default()
            }
        }),
    )
    .on_enter(Message::HoverSource(Some(title)))
    .on_exit(Message::HoverSource(None))
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    row![label, icon]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Icon
    // A count on an icon: 3 reads as it is, 103 rolls over the default maximum
    // of 99 and reads `99+`.
    let icon = body_box(
        row![
            Badge::new(icon_mark(BELL, 24.0)).count(3),
            Badge::new(icon_mark(INBOX, 24.0)).count(103),
        ]
        .spacing(24.0)
        .align_y(Alignment::Center),
    );

    // section: Count
    // The same count on an avatar, which is the shape the component page
    // documents this sample with.
    let count = body_box(
        row![
            Badge::new(avatar_mark("AL")).count(3),
            Badge::new(avatar_mark("MK")).count(103),
        ]
        .spacing(24.0)
        .align_y(Alignment::Center),
    );

    // section: Badge icon
    // A status icon in a round chip instead of a number, in the semantic colours
    // the Slint example names: info for the check, warning for the star.
    let badge_icon = body_box(
        row![
            Badge::new(avatar_mark("AL"))
                .icon(chip_mark(CHECK, app.tokens.primary_foreground))
                .color(app.tokens.info),
            Badge::new(avatar_mark("JR"))
                .icon(chip_mark(STAR, app.tokens.primary_foreground))
                .color(app.tokens.warning),
        ]
        .spacing(24.0)
        .align_y(Alignment::Center),
    );

    // section: Dot
    // A six-pixel dot in place of the count, for a status rather than a number.
    let dot = body_box(
        row![Badge::new(avatar_mark("AL")).dot().count(1)]
            .spacing(24.0)
            .align_y(Alignment::Center),
    );

    // section: Color
    // A caller-supplied fill: `selection` for the count, `success` for the dot.
    let color = body_box(
        row![
            Badge::new(avatar_mark("AL"))
                .count(3)
                .color(app.tokens.selection),
            Badge::new(avatar_mark("AL"))
                .dot()
                .count(1)
                .color(app.tokens.success),
        ]
        .spacing(24.0)
        .align_y(Alignment::Center),
    );

    // section: Sizes
    // The size ladder, with the icons the Slint example pairs it with: small at
    // 16 pixels, the default at 24, large at 32.
    let sizes = body_box(
        row![
            Badge::new(icon_mark(BELL, 16.0))
                .count(2)
                .size(ControlSize::Small),
            Badge::new(icon_mark(BELL, 24.0))
                .count(2)
                .size(ControlSize::Medium),
            Badge::new(icon_mark(BELL, 32.0))
                .count(2)
                .size(ControlSize::Large),
        ]
        .spacing(24.0)
        .align_y(Alignment::Center),
    );

    // section: end
    // The live control the demo is driven by: one badge the count buttons move,
    // so the `99+` rollover and the hidden zero can be seen rather than read.
    let live = row![
        Badge::new(avatar_mark("UN")).count(app.count),
        Button::new("Unread +1")
            .variant(Variant::Primary)
            .on_press(Message::Increment),
        Button::new("Clear").on_press(Message::Clear),
    ]
    .spacing(16.0)
    .align_y(Alignment::Center);

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        icon,
        heading(app, 1, SECTIONS[1]),
        count,
        heading(app, 2, SECTIONS[2]),
        badge_icon,
        heading(app, 3, SECTIONS[3]),
        dot,
        heading(app, 4, SECTIONS[4]),
        color,
        heading(app, 5, SECTIONS[5]),
        sizes,
        live,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
