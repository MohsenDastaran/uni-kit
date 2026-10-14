//! `attachment` — a file or media chip, in the lifecycle states, densities,
//! axes, slots and surfaces the Slint example shows.
//!
//! The previews are elements the caller builds: here a text glyph stands in for
//! a media glyph and a tinted panel for an image preview, because iced ships no
//! icon set and an application owns its assets. A real application passes
//! `svg(handle)` or `image(handle)` in the same place.
//!
//! Run it with `cargo run --example attachment`, or in the browser as
//! `?story=attachment` in the gallery the website embeds.
//!
//! The sections are the Slint example's `GroupBox`es, in its order, each titled
//! with the heading of `website/component/attachment.md` that documents it. The
//! page folds three pairs and one triple of those boxes under a single heading
//! each — `Anatomy and basic usage`, `Media and image previews`, `Sizes and
//! axes` — so those titles repeat, and the website tells the cards apart by the
//! section index the heading keeps, exactly as the accordion's two shared
//! samples do.
//!
//! Where iced differs from Slint the difference is stated rather than faked:
//! `shimmer` and Slint's per-child status overrides have no equivalent (iced
//! cannot paint a gradient through part of a line — see
//! `components/attachment.rs`), so the status-inheritance section shows the
//! attachment's own status deciding its title and description; `remove-variant`
//! and a coloured `icon` are the caller's element here; and a long name is
//! clipped by the card rather than elided, because iced 0.14 text has no
//! ellipsis.
//!
//! Each section carries the heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one — without it the lines that assemble the
//! gallery, the status line among them, would be shown as part of the last
//! section.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Background, Color, Element, Length, Task, Theme};

use crate::components::attachment::{Attachment, Axis, Group, Status};
use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, which the Slint example sets
/// to 24px.
const PAGE: f32 = theme::SPACING_XL;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — attachment")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// What the whole-card click last did, the notice the Slint example writes
    /// under its clickable card.
    notice: String,
    /// The counters the status line reports for the sections that act.
    removed: u32,
    opened: u32,
    retried: u32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A remove control other than the whole-card sample's was pressed.
    Remove,
    /// The whole-card sample was removed, which the Slint example reports under
    /// its own card.
    RemoveMockups,
    /// The whole card, an action or the file trigger was pressed.
    Open,
    Retry,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

impl App {
    /// The loading ring's start, advanced a tenth per activation. There is no
    /// animation driver here — the ring moves when the gallery does.
    fn phase(&self) -> f32 {
        self.opened as f32 * 0.1
    }
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Remove => app.removed += 1,
        Message::RemoveMockups => {
            app.removed += 1;
            app.notice = "Removed design-mockups.png".to_string();
        }
        Message::Open => {
            app.opened += 1;
            app.notice = "Opening design-mockups.png…".to_string();
        }
        Message::Retry => app.retried += 1,
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

/// The caller's glyph for a media slot. The media box's text colour is the
/// status's icon colour, so a failed piece's glyph turns destructive on its own.
fn glyph<'a>(label: &'static str) -> Element<'a, Message> {
    text(label).size(11.0).into()
}

/// The caller's preview. An image element goes here in an application; the
/// media box gives it a clipped square to fill.
fn preview<'a>(tint: Color, caption: &'static str) -> Element<'a, Message> {
    container(text(caption).size(10.0))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(tint)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        })
        .into()
}

/// A muted second line, the captions the Slint example writes under a section.
fn muted<'a>(value: impl Into<String>) -> Element<'a, Message> {
    text(value.into())
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is drawn in the same 28px box the button demo's heading uses. The title here
/// matches the page heading the sample is documented under, not the Slint
/// `GroupBox` title, because the iced plugin has no alias table.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        )
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Anatomy and basic usage
    // File metadata: one chip at the default density with a remove control, and
    // one small chip without — iced draws the remove control only when the card
    // carries a `Remove` message, which is how Slint's `closable: false` reads.
    let metadata = column![
        Attachment::new("quarterly-report.pdf")
            .description("PDF · 2.4 MB")
            .icon(glyph("PDF"))
            .on_remove(Message::Remove)
            .width(232.0),
        Attachment::new("research-data.csv")
            .description("CSV · 840 KB")
            .density(ControlSize::Small)
            .icon(glyph("CSV"))
            .width(200.0),
    ]
    .spacing(theme::SPACING_MD);

    // section: Whole-card click
    // The card activates as one target and the remove control rides its corner
    // as a second; the notice under it is the application's own state.
    let whole_card = column![
        Attachment::new("design-mockups.png")
            .description("PNG · 1.8 MB")
            .icon(glyph("IMG"))
            .on_press(Message::Open)
            .on_remove(Message::RemoveMockups)
            .width(272.0),
        muted(if app.notice.is_empty() {
            "Click the card to open it; the remove control is its own target.".to_string()
        } else {
            app.notice.clone()
        }),
    ]
    .spacing(theme::SPACING_SM);

    // section: Lifecycle states
    // The five states in the Slint example's order: pending, uploading with a
    // progress bar, processing with the loading ring, failed with a Retry
    // action, and complete.
    let states = column![
        Attachment::new("meeting-notes.pdf")
            .description("Ready to upload")
            .status(Status::Pending)
            .icon(glyph("PDF"))
            .width(272.0),
        Attachment::new("design-assets.zip")
            .description("Uploading · 68%")
            .status(Status::Uploading)
            .progress(68.0)
            .icon(glyph("ZIP"))
            .on_remove(Message::Remove)
            .width(272.0),
        Attachment::new("transcript.pdf")
            .description("Processing document")
            .status(Status::Processing)
            .overlay(true)
            .phase(app.phase())
            .icon(glyph("PDF"))
            .width(272.0),
        Attachment::new("archive.zip")
            .description("Upload failed")
            .status(Status::Failed)
            .icon(glyph("ZIP"))
            .action("Retry")
            .on_action(Message::Retry)
            .on_remove(Message::Remove)
            .width(272.0),
        Attachment::new("published-report.pdf")
            .description("Uploaded · 1.8 MB")
            .status(Status::Complete)
            .icon(glyph("✓"))
            .width(272.0),
    ]
    .spacing(theme::SPACING_MD);

    // section: Anatomy and basic usage
    // The optional slots: media only, metadata only, and metadata with an
    // action. A control without a message is drawn disabled, which is how the
    // action here reads until the caller supplies one.
    let optional = column![
        Attachment::new("")
            .icon(glyph("IMG"))
            .show_meta(false)
            .width(120.0),
        Attachment::new("metadata-only.txt")
            .description("Text · 1 KB")
            .show_media(false)
            .width(232.0),
        Attachment::new("ready-for-review.pdf")
            .description("Awaiting review")
            .show_media(false)
            .action("Open")
            .on_action(Message::Open)
            .width(232.0),
    ]
    .spacing(theme::SPACING_MD);

    // section: Media and image previews
    // A vertical card: the preview is the square above the metadata. An
    // application passes an image element; this demo passes a tinted panel.
    let thumbnail = Attachment::new("sdk-preview.svg")
        .description("SVG · 1280 × 720")
        .axis(Axis::Vertical)
        .preview(preview(Color::from_rgb(0.30, 0.42, 0.85), "preview"))
        .on_remove(Message::Remove);

    // section: Media and image previews
    // The loading ring over the dimmed preview while the piece is in progress.
    let overlays = Attachment::new("preview.svg")
        .description("Uploading · 72%")
        .axis(Axis::Vertical)
        .status(Status::Uploading)
        .progress(72.0)
        .preview(preview(Color::from_rgb(0.85, 0.45, 0.35), "preview"))
        .overlay(true)
        .phase(app.phase())
        .on_remove(Message::Remove);

    // section: Sizes and axes
    // The four named sizes — 28, 32, 40 and 48 pixel media boxes — and a small
    // chip whose media box is overridden to the large one.
    let sizes = column![
        Attachment::new("large.pdf")
            .description("Large · PDF · 3.1 MB")
            .density(ControlSize::Large)
            .icon(glyph("PDF"))
            .width(320.0),
        Attachment::new("medium.pdf")
            .description("Medium · PDF · 2.4 MB")
            .icon(glyph("PDF"))
            .width(272.0),
        Attachment::new("small.csv")
            .description("Small · CSV · 840 KB")
            .density(ControlSize::Small)
            .icon(glyph("CSV"))
            .width(232.0),
        Attachment::new("xsmall.txt")
            .density(ControlSize::XSmall)
            .icon(glyph("TXT"))
            .width(176.0),
        Attachment::new("custom-media.pdf")
            .description("Large media in a small attachment")
            .density(ControlSize::Small)
            .media_box(48.0)
            .icon(glyph("PDF"))
            .width(232.0),
    ]
    .spacing(theme::SPACING_MD);

    // section: Groups
    // One horizontally scrolling row, twelve pixels apart.
    let group = Group::new()
        .push(
            Attachment::new("default.pdf")
                .description("PDF · 2.4 MB")
                .icon(glyph("PDF"))
                .width(180.0),
        )
        .push(
            Attachment::new("small.csv")
                .description("CSV · 840 KB")
                .density(ControlSize::Small)
                .icon(glyph("CSV"))
                .width(160.0),
        )
        .push(
            Attachment::new("compact.txt")
                .density(ControlSize::XSmall)
                .icon(glyph("TXT"))
                .width(140.0),
        );

    // section: Sizes and axes
    // Horizontal keeps media, metadata and actions in one row; vertical puts the
    // media above the metadata. A vertical chip with no preview draws the muted
    // empty media box.
    let orientation = row![
        Attachment::new("horizontal.pdf")
            .description("Horizontal layout")
            .icon(glyph("PDF"))
            .width(232.0),
        Attachment::new("vertical.pdf")
            .description("Vertical layout")
            .axis(Axis::Vertical),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(Alignment::Start);

    // section: Status inheritance and overrides
    // The attachment's own status decides the title, the description, the border
    // and the media slot together: an `Uploading` piece draws its title in the
    // loading colour. This port has no per-child status override, which is what
    // Slint's separate title and description statuses are; the module names the
    // gap rather than faking it.
    let inheritance = column![
        Attachment::new("inherited-title.pdf")
            .description("Inherited loading appearance")
            .status(Status::Uploading)
            .icon(glyph("PDF"))
            .width(272.0),
        Attachment::new("stable-title.pdf")
            .description("An explicit status leaves the title at rest")
            .status(Status::Complete)
            .icon(glyph("PDF"))
            .width(272.0),
    ]
    .spacing(theme::SPACING_MD);

    // section: Sizes and axes
    // A 288px card holding a long name: iced 0.14 text has no ellipsis, so the
    // title is clipped by the card rather than elided.
    let long_names =
        Attachment::new("accessibility-review-and-keyboard-navigation-findings.pdf")
            .description("Final report · reviewed by the desktop experience team")
            .icon(glyph("PDF"))
            .width(288.0);

    // section: Content and actions
    // The trigger a composer puts before the chips. The icon is the caller's
    // element, so a text glyph stands in for the file mark here.
    let trigger = row![Button::new("Add files…")
        .icon(text("+").size(theme::FONT_SIZE_MD))
        .outline()
        .on_press(Message::Open)]
    .align_y(Alignment::Center);

    // section: Custom styling and theme tokens
    // Slint refines this surface from `Theme.accent` and `Theme.primary`. This
    // component takes concrete colours rather than a style closure, so the
    // override is a palette the caller owns; a chip beside it resolves its
    // colours from the active theme when the surface draws.
    let custom = Attachment::new("custom-theme.json")
        .description("JSON · 16 KB")
        .icon(glyph("JSON"))
        .fill(Color::from_rgb(0.20, 0.24, 0.62))
        .border_color(Color::from_rgb(0.33, 0.40, 0.85))
        .title_color(Color::WHITE)
        .icon_color(Color::from_rgb(0.80, 0.84, 1.0))
        .media_background(Color::from_rgb(0.30, 0.36, 0.82))
        .width(272.0);

    // section: end
    let status = muted(format!(
        "removed {} · opened {} · retried {}",
        app.removed, app.opened, app.retried
    ));

    let gallery = column![
        heading(app, 0, "Anatomy and basic usage"),
        body_box(metadata),
        heading(app, 1, "Whole-card click"),
        body_box(whole_card),
        heading(app, 2, "Lifecycle states"),
        body_box(states),
        heading(app, 3, "Anatomy and basic usage"),
        body_box(optional),
        heading(app, 4, "Media and image previews"),
        body_box(thumbnail),
        heading(app, 5, "Media and image previews"),
        body_box(overlays),
        heading(app, 6, "Sizes and axes"),
        body_box(sizes),
        heading(app, 7, "Groups"),
        body_box(group),
        heading(app, 8, "Sizes and axes"),
        body_box(orientation),
        heading(app, 9, "Status inheritance and overrides"),
        body_box(inheritance),
        heading(app, 10, "Sizes and axes"),
        body_box(long_names),
        heading(app, 11, "Content and actions"),
        body_box(trigger),
        heading(app, 12, "Custom styling and theme tokens"),
        body_box(custom),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
