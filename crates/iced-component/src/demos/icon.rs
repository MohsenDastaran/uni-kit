//! `icon` — an SVG the application supplies, tinted from the theme, drawn at
//! the catalog's four sizes, and turning on its own clock.
//!
//! The artwork below is the example's own: this component never carries an icon
//! set, because the set belongs to the application. A real one embeds Lucide or
//! its own brand marks the same way, through `svg::Handle::from_memory`.
//!
//! Run it with `cargo run --example icon`, or in the browser as
//! `?story=icon` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has. The `// section:
//! <title>` comments below are the contract the website reads to cut this file
//! into the usage panels a component page shows, so the code on the page is the
//! code that runs here. Keep them, one per section, in order, and keep the
//! closing `// section: end` after the last one.

use iced::widget::svg;
use iced::widget::{column, container, row, text};
use iced::{Background, Border, Color, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::icon::Icon;
use crate::components::theme::{ControlSize, DARK, LIGHT, Tokens};

/// The tokens the mode on screen is using. `.theme(..)` above switches the
/// iced theme, and this is how the view names the same set for a colour the
/// caller passes in explicitly.
fn tokens(app: &App) -> Tokens {
    if app.dark { DARK } else { LIGHT }
}

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — icon")
        .theme(|app: &App| {
            Some(if app.dark {
                Theme::Dark
            } else {
                Theme::Light
            })
        })
        .theme(theme)
        .run()
}

/// A heart, the Lucide `heart` glyph.
const HEART: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 0L5 15c-1.5-1.5-3-3.2-3-5.5"/></svg>"#;

/// A star, the Lucide `star` glyph.
const STAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/></svg>"#;

/// A loader ring, the Lucide `loader-circle` glyph — the one the Slint
/// component's `spinning` comment names.
const LOADER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#;

/// A refresh arrow, the Lucide `refresh-cw` glyph the Slint loading section
/// also spins.
const REFRESH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>"#;

/// A tick in a ring, the Lucide `circle-check` glyph.
const CIRCLE_CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;

/// An exclamation in a triangle, the Lucide `triangle-alert` glyph.
const TRIANGLE_ALERT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;

/// A cross in a ring, the Lucide `circle-x` glyph.
const CIRCLE_X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;

#[derive(Debug, Clone)]
struct App {
    /// Whether the loading icons run their own clock. `true` at boot, so the
    /// gallery shows the motion the component owns.
    spinning: bool,
    /// The last control pressed, drawn under the sections.
    pressed: Option<&'static str>,
    dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    /// Start or stop the loading icons' turn.
    ToggleSpin,
    Pressed(&'static str),
    ToggleTheme,
}

fn boot() -> App {
    App {
        spinning: true,
        pressed: None,
        dark: false,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::ToggleSpin => app.spinning = !app.spinning,
        Message::Pressed(label) => app.pressed = Some(label),
        Message::ToggleTheme => app.dark = !app.dark,
    }
    Task::none()
}

/// One icon in a titled cell, so a size or a colour can be read off the screen.
fn sample<'a>(label: &'a str, icon: Icon<Message>) -> Element<'a, Message> {
    column![
        container(Element::<Message>::from(icon))
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(40.0))
            .center(Length::Fixed(40.0)),
        text(label).size(12.0),
    ]
    .spacing(4)
    .align_x(iced::Alignment::Center)
    .into()
}

fn section<'a>(title: &'a str, body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![text(title).size(14.0), body.into()]
        .spacing(8)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let heart = || Icon::<Message>::new(svg::Handle::from_memory(HEART));
    let star = || Icon::<Message>::new(svg::Handle::from_memory(STAR));
    // The colours: a transparent `.color(..)` follows the theme, and an
    // explicit token is the caller's own choice, which is why the theme switch
    // repaints the first and not the second.
    let ink = tokens(app);

    // section: Basic Icon
    let basic = row![sample("heart", heart()), sample("star", star())].spacing(12);

    // section: Icon with Custom Size
    let sizes = row![
        sample("xsmall", heart().size(ControlSize::XSmall)),
        sample("small", heart().size(ControlSize::Small)),
        sample("medium", heart().size(ControlSize::Medium)),
        sample("large", heart().size(ControlSize::Large)),
        sample("32px", heart().size(32.0)),
    ]
    .spacing(12);

    // section: Icon with Custom Color
    let colors = row![
        sample("foreground", heart()),
        sample("primary", heart().color(ink.primary)),
        sample("star", star().color(Color::from_rgb(0.95, 0.75, 0.1))),
        sample("danger", heart().color(ink.danger)),
        sample("muted", heart().color(ink.muted_foreground)),
    ]
    .spacing(12);

    // section: Animated Loading Icon
    // `spinning` is the component's clock, not the caller's value: Slint's
    // `spinning` property, one turn every `SPIN_DURATION`. Stop it with the
    // button under the sections and the icons hold their angle.
    let loading = row![
        sample(
            "loader",
            Icon::new(svg::Handle::from_memory(LOADER))
                .size(ControlSize::Large)
                .spinning(app.spinning),
        ),
        sample(
            "loader primary",
            Icon::new(svg::Handle::from_memory(LOADER))
                .size(ControlSize::Large)
                .color(ink.primary)
                .spinning(app.spinning),
        ),
        sample(
            "refresh success",
            Icon::new(svg::Handle::from_memory(REFRESH))
                .size(ControlSize::Large)
                .color(ink.success)
                .spinning(app.spinning),
        ),
        sample(
            "refresh warning",
            Icon::new(svg::Handle::from_memory(REFRESH))
                .size(ControlSize::Large)
                .color(ink.warning)
                .spinning(app.spinning),
        ),
    ]
    .spacing(12);

    // section: Status Icons
    let status = row![
        sample(
            "success",
            Icon::new(svg::Handle::from_memory(CIRCLE_CHECK)).color(ink.success),
        ),
        sample(
            "warning",
            Icon::new(svg::Handle::from_memory(TRIANGLE_ALERT)).color(ink.warning),
        ),
        sample(
            "danger",
            Icon::new(svg::Handle::from_memory(CIRCLE_X)).color(ink.danger),
        ),
    ]
    .spacing(12);

    // section: Icon in Button
    let in_button = row![
        Button::new("Favourite")
            .icon(heart())
            .on_press(Message::Pressed("Favourite")),
        Button::new("Reload")
            .variant(Variant::Ghost)
            .icon(
                Icon::new(svg::Handle::from_memory(REFRESH))
                    .color(ink.muted_foreground)
                    .spinning(app.spinning),
            )
            .on_press(Message::Pressed("Reload")),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    // section: end
    let controls = row![
        iced::widget::button(if app.spinning {
            "Stop the turn"
        } else {
            "Start the turn"
        })
        .on_press(Message::ToggleSpin),
        iced::widget::button(if app.dark {
            "Light theme"
        } else {
            "Dark theme"
        })
        .on_press(Message::ToggleTheme),
    ]
    .spacing(8);

    let status_line = text(match app.pressed {
        Some(label) => format!("Last pressed: {label}"),
        None => if app.spinning {
            "The loading icons turn on their own clock — no subscription."
        } else {
            "The turn is stopped; the icons hold the angle they were at."
        }
        .to_string(),
    })
    .size(12.0)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let body = column![
        section("Basic Icon", basic),
        section("Icon with Custom Size", sizes),
        section("Icon with Custom Color", colors),
        section("Animated Loading Icon", loading),
        section("Status Icons", status),
        section("Icon in Button", in_button),
        controls,
        status_line,
    ]
    .spacing(20)
    .padding(16);

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(Background::Color(tokens.background)),
                text_color: Some(tokens.foreground),
                border: Border::default(),
                ..container::Style::default()
            }
        })
        .into()
}
