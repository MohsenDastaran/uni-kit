//! `icon` — an SVG the application supplies, tinted from the theme and drawn at
//! the catalog's four sizes, still or turning.
//!
//! The artwork below is the example's own: this component never carries an icon
//! set, because the set belongs to the application. A real one embeds Lucide or
//! its own brand marks the same way, through `svg::Handle::from_memory`.
//!
//! Run it with `cargo run --example icon`.

use iced::widget::{column, container, row, text};
use iced::widget::svg;
use iced::{Background, Border, Color, Element, Length, Task, Theme};

use iced_component::components::icon::Icon;
use iced_component::components::theme::{ControlSize, DARK, LIGHT, Tokens};

/// The tokens the mode on screen is using. `.theme(..)` above switches the
/// iced theme, and this is how the view names the same set for a colour the
/// caller passes in explicitly.
fn tokens(app: &App) -> Tokens {
    if app.dark { DARK } else { LIGHT }
}

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — icon")
        .theme(|app: &App| {
            Some(if app.dark {
                Theme::Dark
            } else {
                Theme::Light
            })
        })
        .run()
}

/// A heart, the Lucide `heart` glyph.
const HEART: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 0L5 15c-1.5-1.5-3-3.2-3-5.5"/></svg>"#;

/// A star, the Lucide `star` glyph.
const STAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/></svg>"#;

/// A loader ring, the Lucide `loader-circle` glyph — the one the Slint
/// component's `spinning` comment names.
const LOADER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#;

#[derive(Debug, Clone, Default)]
struct App {
    /// The turn, in `0.0..=1.0`. The Spinner example advances its value the
    /// same way; here a button stands in for a subscription.
    spin: f32,
    dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    /// A step of the animation, a tenth of a turn.
    Advance,
    ToggleTheme,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Advance => app.spin = (app.spin + 0.1).fract(),
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

    let sizes = row![
        sample("xsmall", heart().size(ControlSize::XSmall)),
        sample("small", heart().size(ControlSize::Small)),
        sample("medium", heart().size(ControlSize::Medium)),
        sample("large", heart().size(ControlSize::Large)),
        sample("32px", heart().size(32.0)),
    ]
    .spacing(12);

    // The colours: a transparent `.color(..)` follows the theme, and an
    // explicit token is the caller's own choice, which is why the theme switch
    // repaints the first and not the second.
    let ink = tokens(app);
    let colors = row![
        sample("foreground", heart()),
        sample("primary", heart().color(ink.primary)),
        sample("star", star().color(Color::from_rgb(0.95, 0.75, 0.1))),
        sample("danger", heart().color(ink.danger)),
        sample("muted", heart().color(ink.muted_foreground)),
    ]
    .spacing(12);

    let states = row![
        sample("rest", star()),
        // A tenth of the way round; press Advance to carry it further.
        sample("spinning", star().size(ControlSize::Large).spin(app.spin)),
        sample("loader", Icon::new(svg::Handle::from_memory(LOADER))
            .size(ControlSize::Large)
            .color(Color::from_rgb(0.95, 0.75, 0.1))
            .spin(app.spin)),
        // The caller's colour wins over the theme's, in either mode.
        sample("fixed", heart().color(Color::from_rgb(0.55, 0.2, 0.85))),
    ]
    .spacing(12);

    let body = column![
        section("Sizes", sizes),
        section("Colours", colors),
        section("States", states),
        row![
            iced::widget::button("Advance the turn").on_press(Message::Advance),
            iced::widget::button(if app.dark {
                "Light theme"
            } else {
                "Dark theme"
            })
            .on_press(Message::ToggleTheme),
        ]
        .spacing(8),
        text(format!("spin {:.1}", app.spin)),
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
