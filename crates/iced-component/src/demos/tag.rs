//! `tag` — a chip in four variants, with a leading icon and a close control.
//!
//! Run it with `cargo run --example tag`, or in the browser as
//! `?story=tag` in the gallery the website embeds.
//!
//! The Slint example's four outlined boxes are the four here, in its order,
//! each under the heading `website/component/tag.md` documents that sample at:
//! `Tags`, `Tag Variants`, `Tag Sizes` and `Outline Tags`. The page documents
//! more headings than the Slint example draws — colours, custom HSLA, rounding,
//! collections and the category dashboards. The first three are GPUI's `color`,
//! `custom` and `rounded` builders, which this component does not carry; the
//! others compose or re-label the same chips. They get no iced panel rather than
//! a panel showing something else, and every heading the Slint example does draw
//! has one on the page, so none is invented here.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.

use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::tag::{Tag, Variant};
use crate::components::theme::{self, ControlSize, Status, Tokens};
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 4] = ["Tags", "Tag Variants", "Tag Sizes", "Outline Tags"];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — tag")
        .theme(theme)
        .run()
}

/// Which section heading the pointer is over, and which was pressed, so the code
/// icon can draw the two states the Slint heading animates to, and how many close
/// controls have been pressed.
#[derive(Debug, Clone, Default)]
struct App {
    removed: u32,
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A tag's close control was pressed.
    Close,
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
        Message::Close => app.removed += 1,
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

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn panel<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A titled section: the heading row, then the sample it names.
fn section<'a>(
    app: &App,
    index: usize,
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![heading(app, index, title), body.into()]
        .spacing(8)
        .into()
}

/// The leading element the Slint example's first tag draws with `icon: "code"`.
/// The icon set belongs to the application, so the leading element is the
/// caller's; the box is the tag's own icon size, two pixels under the theme's.
fn code_glyph<'a>() -> Element<'a, Message> {
    Icon::<Message>::new(svg::Handle::from_memory(CODE))
        .size(14.0)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Tags
    // The Slint box: a tag led by the `code` glyph, an outlined one with a close
    // control, and one reporting the success status.
    let tags = panel(
        row![
            Tag::new("Rust").icon(code_glyph()),
            Tag::new("Draft")
                .variant(Variant::Outline)
                .on_close(Message::Close),
            Tag::new("Live").status(Status::Success),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    );

    // section: Tag Variants
    // One row of the four ways a chip is filled: default, secondary, outline and
    // a status tone, which is iced's `Variant` plus `Status`.
    let variants = panel(
        row![
            Tag::new("Default"),
            Tag::new("Secondary").variant(Variant::Secondary),
            Tag::new("Outline").variant(Variant::Outline),
            Tag::new("Status").status(Status::Info),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    );

    // section: Tag Sizes
    // The three steps the component carries: 16, 24 and 32 pixels tall, which is
    // the control height less the eight a tag keeps.
    let sizes = panel(
        column![
            Tag::new("Small").size(ControlSize::Small),
            Tag::new("Medium"),
            Tag::new("Large").size(ControlSize::Large),
        ]
        .spacing(16.0)
        .align_x(Alignment::Start),
    );

    // section: Outline Tags
    // An outlined, closable tag is the usual shape for a filter the person can
    // remove; the close affordance is the same at every variant.
    let outline = panel(
        row![
            Tag::new("language:rust")
                .variant(Variant::Outline)
                .on_close(Message::Close),
            Tag::new("label:good first issue")
                .variant(Variant::Outline)
                .on_close(Message::Close),
            Tag::new("assignee:ada").variant(Variant::Outline),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    );

    // section: end
    // The one live control the demo still owns: every close control above posts
    // the same message, and this line records how many times it arrived.
    let status_line = text(format!("closed {} times", app.removed))
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        section(app, 0, SECTIONS[0], tags),
        section(app, 1, SECTIONS[1], variants),
        section(app, 2, SECTIONS[2], sizes),
        section(app, 3, SECTIONS[3], outline),
        status_line,
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

// The Lucide `</>` mark the code icon uses, from the icon set this repository
// already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three
// galleries draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

/// The Lucide `code` glyph the first tag leads with, from
/// `crates/slint-component/ui/icons/code.svg` (Lucide, ISC).
const CODE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m16 18 6-6-6-6"/><path d="m8 6-6 6 6 6"/></svg>"#;
