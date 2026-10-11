//! `avatar` — a user portrait, in the fallbacks, sizes and chrome the catalog
//! shares with GPUI and Slint.
//!
//! Run it with `cargo run --example avatar`, or in the browser as
//! `?story=avatar` in the gallery the website embeds.
//!
//! iced ships no image and no icon set, and this example embeds neither: the
//! portrait is a handle built from RGBA bytes the file generates at startup, and
//! the icon fallback is an inline SVG string. Both arrive through the same
//! public API an application uses for its own assets.

use iced::widget::{column, container, row, svg, text};
use iced::{Color, Element, Length, Task};

use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::theme::{self, Tokens};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — avatar")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fallback {
    Image,
    Initials,
    Icon,
}

#[derive(Debug, Clone)]
enum Message {
    /// Which of the three fallbacks the switchable avatar holds.
    Fallback(Fallback),
    /// A large avatar with a custom edge, the size the Slint example shows at
    /// 100 pixels.
    Large(bool),
}

#[derive(Debug)]
struct App {
    fallback: Fallback,
    large: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            fallback: Fallback::Initials,
            large: false,
        }
    }
}

impl Default for Fallback {
    fn default() -> Self {
        Self::Initials
    }
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Fallback(fallback) => app.fallback = fallback,
        Message::Large(large) => app.large = large,
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

/// The dark border and the shadow the Slint example's "Custom style" box draws.
fn chrome<'a>(avatar: Avatar<'a, Message>) -> Avatar<'a, Message> {
    avatar
        .border_width(3.0)
        .border_color(Color::from_rgb(0.04, 0.04, 0.04))
        .shadow_blur(4.0)
        .shadow_offset_y(2.0)
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

    // The custom edge, which overrides the size ladder when it is set.
    let custom = || shown().edge(56.0).radius(20.0);

    column![
        section("Image"),
        row![shown().initials("JL"), shown()].spacing(theme::SPACING_MD),
        section("Fallback"),
        row![
            Avatar::new().initials("JL"),
            // Neither image nor initials: the fill alone.
            Avatar::new(),
            Avatar::new().icon(user_icon(32.0 * 0.45)),
        ]
        .spacing(theme::SPACING_MD),
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
        section("Sizes"),
        row![
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
        .align_y(iced::Alignment::Start),
        section("States"),
        row![
            // Rest: the circle, the 1-pixel border, initials on the fill.
            labelled("rest", Avatar::new().initials("JL")),
            // The fill and the initials colour are both overridable, which is
            // what a caller does when it darkens the fill.
            labelled(
                "custom fill",
                Avatar::new()
                    .initials("JL")
                    .fill(Color::from_rgb(0.13, 0.13, 0.13))
                    .initials_color(Color::WHITE),
            ),
            // No border: the fill and the initials alone.
            labelled("borderless", Avatar::new().initials("JL").border_width(0.0)),
            labelled("chrome", chrome(shown())),
            // A rounded rectangle: the radius is the caller's, not the circle.
            labelled("radius 12", shown().radius(12.0)),
        ]
        .spacing(theme::SPACING_MD)
        .align_y(iced::Alignment::Start),
        section("Custom edge"),
        row![
            labelled("edge 56, radius 20", custom()),
            labelled("edge 56, chrome", chrome(custom())),
            // The switch below grows this one, which is what a custom edge is
            // for: an avatar that is not on the size ladder.
            labelled(
                "edge 56 or 100",
                if app.large {
                    shown().edge(100.0).radius(20.0)
                } else {
                    custom()
                },
            ),
        ]
        .spacing(theme::SPACING_MD)
        .align_y(iced::Alignment::Start),
        switch(
            if app.large {
                "edge: 100 px"
            } else {
                "edge: 56 px"
            },
            app.large,
            Message::Large(!app.large),
        ),
    ]
    .spacing(theme::SPACING_MD)
    .padding(theme::SPACING_LG)
    .into()
}

/// A heading for one group of avatars, drawn in the muted foreground the way
/// the rest of the catalog labels a section.
fn section(title: &str) -> Element<'static, Message> {
    text(title.to_owned())
        .size(theme::FONT_SIZE_SM)
        .font(iced::Font {
            weight: iced::font::Weight::Semibold,
            ..iced::Font::DEFAULT
        })
        .style(|theme: &iced::Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
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

/// A control that flips one flag, which is the state the switchable avatar reads.
fn switch(label: &str, on: bool, message: Message) -> Element<'static, Message> {
    iced::widget::button(text(label.to_owned()).size(theme::FONT_SIZE_SM))
        .padding([4.0, 8.0])
        .on_press(message)
        .style(move |theme: &iced::Theme, _status| {
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
