//! `avatar` — a user portrait, in the fallbacks, sizes and chrome the catalog
//! shares with GPUI and Slint.
//!
//! Run it with `cargo run --example avatar`, or in the browser as
//! `?story=avatar` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The six sections are the Slint example's `GroupBox`es, which are the page's
//! own usage headings: Image, Fallback, Group, Custom shape, Custom style and
//! Avatar Sizes. The page's later examples — Team Display, User Profile Header,
//! Anonymous User and the `AvatarGroup` headings — are GPUI compositions with no
//! iced counterpart here, so they are left to GPUI rather than showing a
//! stand-in.
//!
//! iced ships no image and no icon set, and this example embeds neither: the
//! portrait is a handle built from RGBA bytes the file generates at startup, and
//! the icon fallback is an inline SVG string. Both arrive through the same
//! public API an application uses for its own assets.
//!
//! iced has no negative spacing, so the Slint `AvatarGroup` — overlapping
//! avatars with an overflow chip — is a `Stack` this file builds in the Group
//! section rather than a component.

use iced::widget::{column, container, mouse_area, row, scrollable, stack, svg, text};
use iced::{Alignment, Color, Element, Length, Padding, Task, Theme};

use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — avatar")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Fallback {
    Image,
    #[default]
    Initials,
    Icon,
}

#[derive(Debug, Default)]
struct App {
    /// Which of the three things the live avatar holds.
    fallback: Fallback,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// The content the live avatar shows.
    Fallback(Fallback),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Fallback(fallback) => app.fallback = fallback,
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

/// The portrait, as RGBA bytes: a 64-pixel diagonal wash that reads as a photo
/// at every size the example shows. An application loads its own file here.
fn portrait() -> iced::widget::image::Handle {
    const SIDE: u32 = 64;
    let mut pixels = Vec::with_capacity((SIDE * SIDE * 4) as usize);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let t = (x + y) as f32 / (2 * SIDE - 2) as f32;
            pixels.extend_from_slice(&[
                (0x2f as f32 + (0x8f as f32 - 0x2f as f32) * t) as u8,
                (0x6f as f32 + (0xb8 as f32 - 0x6f as f32) * t) as u8,
                (0xd8 as f32 + (0x6a as f32 - 0xd8 as f32) * t) as u8,
                0xff,
            ]);
        }
    }
    iced::widget::image::Handle::from_rgba(SIDE, SIDE, pixels)
}

/// The icon fallback: the Lucide `user` glyph. The caller owns its icons, so
/// the component never sees a name — only the element this makes.
const USER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;

/// The icon as an element at the size the component centres: 45% of the edge.
fn user_icon(edge: f32) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(USER))
        .width(Length::Fixed(edge))
        .height(Length::Fixed(edge))
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
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

    let icon = mouse_area(
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

/// The box under a heading: the same `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// The 3px edge and the drop shadow the Slint example's "Custom style" box draws.
/// Slint sets the edge to `Theme.foreground`; here it stays the theme's own border
/// token at three pixels, which is the same proportion of the foreground and
/// keeps an edge in both appearances instead of a fixed dark that vanishes on the
/// dark page.
fn chrome<'a>(avatar: Avatar<'a, Message>) -> Avatar<'a, Message> {
    avatar
        .border_width(3.0)
        .shadow_blur(4.0)
        .shadow_offset_y(2.0)
}

/// An overlapping row, the `AvatarGroup` iced has no negative spacing for: each
/// avatar sits one step further right inside a `Stack`, so the ring overlaps
/// instead of sitting in a `row`. Slint reaches the same shape with
/// `spacing: -8px`, and its overflow chip follows the shown avatars.
fn avatar_group<'a>(avatars: Vec<Avatar<'a, Message>>, chip: bool) -> Element<'a, Message> {
    const EDGE: f32 = 32.0;
    const STEP: f32 = 24.0;

    let count = avatars.len() + usize::from(chip);
    let width = EDGE + STEP * (count.saturating_sub(1)) as f32;

    let layers = avatars
        .into_iter()
        .enumerate()
        .map(|(index, avatar)| layer(avatar.into(), index))
        .chain(chip.then(|| {
            // Slint writes the midline ellipsis `⋯`, which the embedded face has
            // no glyph for; the baseline ellipsis is the face's own character.
            layer(Avatar::new().initials("…").into(), count - 1)
        }));

    // A `Stack` takes its size from its first child, so the group states its own
    // width and height rather than letting the first layer decide.
    stack(layers)
        .width(Length::Fixed(width))
        .height(Length::Fixed(EDGE))
        .into()
}

/// One avatar in the group, shifted right by its place so the ring overlaps.
fn layer<'a>(avatar: Element<'a, Message>, index: usize) -> Element<'a, Message> {
    const STEP: f32 = 24.0;
    container(avatar)
        .padding(Padding {
            left: index as f32 * STEP,
            ..Padding::ZERO
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let portrait = portrait();
    // The portrait at one size, so the rows below read as variants rather than
    // as repetition.
    let shown = || Avatar::new().image(portrait.clone());

    // The content: exactly one of the three things an avatar holds, which is how
    // Slint decides between them too. The switches under it choose which.
    let switchable: Element<'_, Message> = match app.fallback {
        Fallback::Image => shown().into(),
        Fallback::Initials => Avatar::new().initials("JL").into(),
        Fallback::Icon => Avatar::new().icon(user_icon(32.0 * 0.45)).into(),
    };

    // section: Image
    // The image wins when there is one; the initials behind it are the fallback
    // Slint also writes into its first box.
    let image = row![shown().initials("JL"), shown()]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Center);

    // section: Fallback
    // Neither image nor initials: the fill alone. The live avatar repeats the
    // three fallbacks and takes the switches below.
    let fallback = column![
        row![
            Avatar::new().initials("JL"),
            Avatar::new(),
            Avatar::new().icon(user_icon(32.0 * 0.45)),
        ]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Center),
        clipped(switchable),
        row![
            switch(
                "image",
                app.fallback == Fallback::Image,
                Message::Fallback(Fallback::Image)
            ),
            switch(
                "initials",
                app.fallback == Fallback::Initials,
                Message::Fallback(Fallback::Initials)
            ),
            switch(
                "icon",
                app.fallback == Fallback::Icon,
                Message::Fallback(Fallback::Icon)
            ),
        ]
        .spacing(theme::SPACING_SM),
    ]
    .spacing(theme::SPACING_MD);

    // section: Group
    // Three overlapping avatars, and the overflow row: five shown behind a chip
    // because eleven names is more than the limit Slint sets.
    let group = column![
        avatar_group(vec![shown(), shown(), shown()], false),
        avatar_group((0..5).map(|_| shown()).collect(), true),
    ]
    .spacing(theme::SPACING_LG);

    // section: Custom shape
    // A custom edge overrides the size ladder, and the radius is the caller's,
    // so this is a rounded rectangle rather than the default circle.
    let custom_shape = shown().edge(100.0).radius(20.0);

    // section: Custom style
    // The 3px edge and the drop shadow are the chrome Slint draws; the fill and
    // the initials colour are the two colours a caller can override.
    let custom_style = row![
        labelled("edge 100, chrome", chrome(shown().edge(100.0).radius(20.0))),
        labelled(
            "custom fill",
            Avatar::new()
                .initials("JL")
                .fill(Color::from_rgb(0.13, 0.13, 0.13))
                .initials_color(Color::WHITE),
        ),
    ]
    .spacing(theme::SPACING_XL)
    .align_y(Alignment::Start);

    // section: Avatar Sizes
    // The Slint ladder is 20 / 24 / 32 / 40 / 56.
    let sizes = row![
        labelled(
            "xs 20",
            Avatar::new().initials("XS").size(AvatarSize::XSmall)
        ),
        labelled(
            "sm 24",
            Avatar::new().initials("SM").size(AvatarSize::Small)
        ),
        labelled(
            "md 32",
            Avatar::new().initials("MD").size(AvatarSize::Medium)
        ),
        labelled(
            "lg 40",
            Avatar::new().initials("LG").size(AvatarSize::Large)
        ),
        labelled(
            "xl 56",
            Avatar::new().initials("XL").size(AvatarSize::XLarge)
        ),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(Alignment::Start);

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Image"),
        body_box(image),
        heading(app, 1, "Fallback"),
        body_box(fallback),
        heading(app, 2, "Group"),
        body_box(group),
        heading(app, 3, "Custom shape"),
        body_box(custom_shape),
        heading(app, 4, "Custom style"),
        body_box(custom_style),
        heading(app, 5, "Avatar Sizes"),
        body_box(sizes),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

/// Puts an avatar on one line so a row of them does not stretch it.
fn clipped<'a>(avatar: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(avatar).width(Length::Shrink).into()
}

/// An avatar with its caption beneath it.
fn labelled<'a>(caption: &str, avatar: Avatar<'a, Message>) -> Element<'a, Message> {
    column![
        clipped(avatar),
        text(caption.to_owned()).size(theme::FONT_SIZE_XS),
    ]
    .spacing(theme::SPACING_XS)
    .align_x(iced::Alignment::Center)
    .into()
}

/// A control that flips one flag, which is the state the live avatar reads.
fn switch(label: &str, on: bool, message: Message) -> Element<'static, Message> {
    iced::widget::button(text(label.to_owned()).size(theme::FONT_SIZE_SM))
        .padding([4.0, 8.0])
        .on_press(message)
        .style(move |theme: &Theme, _status| {
            let tokens = Tokens::of(theme);
            iced::widget::button::Style {
                background: Some(if on {
                    tokens.primary.into()
                } else {
                    tokens.secondary.into()
                }),
                text_color: if on {
                    tokens.primary_foreground
                } else {
                    tokens.secondary_foreground
                },
                border: iced::Border {
                    radius: theme::RADIUS_MD.into(),
                    ..iced::Border::default()
                },
                ..iced::widget::button::Style::default()
            }
        })
        .into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
