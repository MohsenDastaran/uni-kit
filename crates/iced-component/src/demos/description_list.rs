//! `description-list` (module `description_list`) — term and detail pairs, in
//! columns, with borders and separators.
//!
//! Run it with `cargo run --example description_list`, or in the browser as
//! `?story=description_list` in the gallery the website embeds.
//!
//! The sections are the Slint example's `GroupBox`es, in its order, each titled
//! with the `###` heading of `website/component/description-list.md` that
//! documents it. The page has two more `###` headings than the Slint example has
//! boxes — *Using DescriptionItem Builder* and *Rich Content with Custom
//! Elements* — and both are GPUI-only: one documents the builder form of the
//! same item list this port writes as `Item::new`, and the other passes a
//! markdown element into a detail, which this component cannot express because
//! its detail is a `String`. The Slint example has no `GroupBox` for either, so
//! this port has no section for them; under iced the page hides those two
//! sections rather than showing an empty panel.
//!
//! Each section carries the heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one — without it the lines that assemble the
//! gallery would be shown as part of the last section.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::description_list::{DescriptionList, Item, Layout};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = theme::SPACING_XL;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 12] = [
    "Basic Description List",
    "Different Layouts",
    "Multiple Columns with Spans",
    "With Separators",
    "Different Sizes",
    "Without Borders",
    "Custom Label Width (Horizontal Layout)",
    "Complex Example with Mixed Content",
    "User Profile Information",
    "System Information",
    "Product Specifications",
    "Configuration Settings",
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — description-list")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A muted label over a sample, the `Text` the Slint example puts above its
/// horizontal and vertical lists.
fn caption<'a>(value: &'static str) -> Element<'a, Message> {
    text(value)
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
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
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Description List
    // One bordered list at the medium insets: the Slint example's first
    // `GroupBox`.
    let basic = body_box(DescriptionList::new([
        Item::new("Name", "GPUI Kit"),
        Item::new("Version", "0.1.0"),
        Item::new("License", "Apache-2.0"),
    ]));

    // section: Different Layouts
    // Horizontal (the default) beside the stacked form, each under the caption
    // the Slint example writes above it.
    let layouts = body_box(
        column![
            caption("Horizontal"),
            DescriptionList::new([
                Item::new("Platform", "macOS, Windows, Linux"),
                Item::new("Repository", "https://github.com/longbridge/gpui-kit"),
            ]),
            caption("Vertical"),
            DescriptionList::new([
                Item::new("Name", "GPUI Kit"),
                Item::new("Description", "A comprehensive Rust desktop framework"),
            ])
            .layout(Layout::Vertical),
        ]
        .spacing(theme::SPACING_LG),
    );

    // section: Multiple Columns with Spans
    // Three columns; the description spans all three and the repository two.
    let spans = body_box(
        DescriptionList::new([
            Item::new("Name", "GPUI Kit"),
            Item::new("Version", "0.1.0"),
            Item::new("License", "Apache-2.0"),
            Item::new(
                "Description",
                "Full-featured UI components for desktop applications",
            )
            .span(3),
            Item::new("Repository", "https://github.com/longbridge/gpui-kit").span(2),
            Item::new("Author", "Longbridge"),
        ])
        .columns(3),
    );

    // section: With Separators
    // The separator band between the version and the author.
    let separators = body_box(DescriptionList::new([
        Item::new("Name", "GPUI Kit"),
        Item::new("Version", "0.1.0"),
        Item::separator(),
        Item::new("Author", "Longbridge"),
        Item::new("License", "Apache-2.0"),
    ]));

    // section: Different Sizes
    // The three named steps: large, medium and small.
    let sizes = body_box(
        column![
            DescriptionList::new([Item::new("Title", "Large Description List")])
                .size(ControlSize::Large),
            DescriptionList::new([Item::new("Title", "Medium Description List")]),
            DescriptionList::new([Item::new("Title", "Small Description List")])
                .size(ControlSize::Small),
        ]
        .spacing(theme::SPACING_LG),
    );

    // section: Without Borders
    // `bordered(false)` drops the surface edge, the label fill and the row rules.
    let borderless = body_box(
        DescriptionList::new([
            Item::new("Name", "GPUI Kit"),
            Item::new("Type", "UI Library"),
        ])
        .bordered(false),
    );

    // section: Custom Label Width (Horizontal Layout)
    // A 200px label column, so the long term keeps one line and the short one
    // gives its space to the detail.
    let label_width = body_box(
        DescriptionList::new([
            Item::new("Very Long Label Name", "Short Value"),
            Item::new("Short", "Very long value that needs more space"),
        ])
        .label_width(200.0),
    );

    // section: Complex Example with Mixed Content
    // Three columns at a 150px label width, with a full-width rule and the
    // spanned fields in the same arrangement as the Slint example.
    let complex = body_box(
        DescriptionList::new([
            Item::new("Project Name", "GPUI Kit"),
            Item::new("Version", "0.1.0"),
            Item::new("Status", "Active"),
            Item::separator(),
            Item::new(
                "Description",
                "A comprehensive Rust desktop framework built on GPUI",
            )
            .span(3),
            Item::new("Repository", "https://github.com/longbridge/gpui-kit").span(2),
            Item::new("License", "Apache-2.0"),
            Item::new("Platforms", "macOS, Windows, Linux").span(2),
            Item::new("Language", "Rust"),
        ])
        .columns(3)
        .label_width(150.0),
    );

    // section: User Profile Information
    // Two columns, with the bio spanning both under a rule.
    let profile = body_box(
        DescriptionList::new([
            Item::new("Full Name", "Ada Lovelace"),
            Item::new("Email", "ada@uni.dev"),
            Item::new("Phone", "+1 (555) 123-4567"),
            Item::new("Department", "Engineering"),
            Item::separator(),
            Item::new(
                "Bio",
                "Senior software engineer with 10+ years of experience in Rust and system programming.",
            )
            .span(2),
        ])
        .columns(2),
    );

    // section: System Information
    // The stacked form at the small size, without borders.
    let system = body_box(
        DescriptionList::new([
            Item::new("Operating System", "macOS 14.0"),
            Item::new("Architecture", "Apple Silicon (M2)"),
            Item::new("Memory", "16 GB"),
            Item::new("Storage", "512 GB SSD"),
            Item::new("GPU", "Apple M2 10-core GPU"),
        ])
        .layout(Layout::Vertical)
        .size(ControlSize::Small)
        .bordered(false),
    );

    // section: Product Specifications
    // Three columns at the large size, with a rule and the key features
    // spanning the row.
    let specifications = body_box(
        DescriptionList::new([
            Item::new("Model", "MacBook Pro"),
            Item::new("Year", "2023"),
            Item::new("Screen Size", "14-inch"),
            Item::new("Processor", "Apple M2 Pro").span(2),
            Item::new("Base Price", "$1,999"),
            Item::separator(),
            Item::new(
                "Key Features",
                "Liquid Retina XDR display, ProMotion technology, P3 wide color gamut",
            )
            .span(3),
        ])
        .columns(3)
        .size(ControlSize::Large),
    );

    // section: Configuration Settings
    // Horizontal at a 180px label width, without borders.
    let settings = body_box(
        DescriptionList::new([
            Item::new("Theme", "Dark Mode"),
            Item::new("Font Size", "14px"),
            Item::new("Auto Save", "Enabled"),
            Item::new("Backup Frequency", "Every 30 minutes"),
            Item::new("Language", "English (US)"),
        ])
        .label_width(180.0)
        .bordered(false),
    );

    // section: end
    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        basic,
        heading(app, 1, SECTIONS[1]),
        layouts,
        heading(app, 2, SECTIONS[2]),
        spans,
        heading(app, 3, SECTIONS[3]),
        separators,
        heading(app, 4, SECTIONS[4]),
        sizes,
        heading(app, 5, SECTIONS[5]),
        borderless,
        heading(app, 6, SECTIONS[6]),
        label_width,
        heading(app, 7, SECTIONS[7]),
        complex,
        heading(app, 8, SECTIONS[8]),
        profile,
        heading(app, 9, SECTIONS[9]),
        system,
        heading(app, 10, SECTIONS[10]),
        specifications,
        heading(app, 11, SECTIONS[11]),
        settings,
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
