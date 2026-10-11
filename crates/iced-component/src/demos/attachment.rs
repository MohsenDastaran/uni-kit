//! `attachment` — a file or media chip, in the lifecycle states, densities,
//! axes and slots the catalog shares with GPUI and Slint.
//!
//! The previews are elements the caller builds: here a text glyph stands in for
//! a media glyph and a tinted panel for an image preview, because iced ships no
//! icon set and an application owns its assets. A real application passes
//! `svg(handle)` or `image(handle)` in the same place.
//!
//! Run it with `cargo run --example attachment`, or in the browser as
//! `?story=attachment` in the gallery the website embeds.

use iced::widget::{column, container, row, scrollable, text};
use iced::{Background, Color, Element, Length, Task, Theme};

use crate::components::attachment::{Attachment, Axis, Group, Status};
use crate::components::theme::{self, ControlSize};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — attachment")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    removed: u32,
    opened: u32,
    retried: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Remove,
    Open,
    Retry,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Remove => app.removed += 1,
        Message::Open => app.opened += 1,
        Message::Retry => app.retried += 1,
    }
    Task::none()
}

/// The caller's glyph for a media slot. The media box's text colour is the
/// status's icon colour, so a failed piece's glyph turns destructive on its own.
fn glyph(label: &'static str) -> Element<'static, Message> {
    text(label).size(11.0).into()
}

/// The caller's preview. An image element goes here in an application; the
/// media box gives it a clipped square to fill.
fn preview(tint: Color, caption: &'static str) -> Element<'static, Message> {
    container(text(caption).size(10.0).align_x(iced::Center))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::Center)
        .align_y(iced::Center)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(tint)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        })
        .into()
}

fn section<'a>(label: &'a str, content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    let content: Element<'a, Message> = content.into();
    column![text(label).size(13.0), content].spacing(8).into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The five lifecycle states. Uploading draws the metadata's bar below 100,
    // processing and uploading draw the loading ring, and a failed piece turns
    // its border, its description and a previewless media slot destructive.
    let states = column![
        row![
            Attachment::new("quarterly-report.pdf")
                .description("PDF · 2.4 MB")
                .status(Status::Complete)
                .icon(glyph("PDF"))
                .on_remove(Message::Remove)
                .width(232.0),
            Attachment::new("draft-notes.txt")
                .description("Ready to upload")
                .status(Status::Pending)
                .icon(glyph("TXT"))
                .on_remove(Message::Remove)
                .width(232.0),
        ]
        .spacing(12),
        row![
            Attachment::new("design-assets.zip")
                .description("Uploading · 68%")
                .status(Status::Uploading)
                .progress(68.0)
                .icon(glyph("ZIP"))
                .on_remove(Message::Remove)
                .width(232.0),
            Attachment::new("transcript.pdf")
                .description("Processing")
                .status(Status::Processing)
                .overlay(true)
                .phase(app.opened as f32 * 0.1)
                .icon(glyph("PDF"))
                .on_remove(Message::Remove)
                .width(232.0),
        ]
        .spacing(12),
        Attachment::new("archive.zip")
            .description("Image exceeds the 20 MB limit")
            .status(Status::Failed)
            .icon(glyph("ZIP"))
            .action("Retry")
            .on_action(Message::Retry)
            .on_remove(Message::Remove)
            .width(320.0),
    ]
    .spacing(12);

    // The four densities: 28, 32, 40 and 48 pixel media boxes.
    let densities = column![
        Attachment::new("small.png")
            .description("PNG · 128 KB")
            .density(ControlSize::XSmall)
            .icon(glyph("IMG"))
            .on_remove(Message::Remove)
            .width(200.0),
        Attachment::new("notes.txt")
            .description("TXT · 12 KB")
            .density(ControlSize::Small)
            .icon(glyph("TXT"))
            .on_remove(Message::Remove)
            .width(232.0),
        Attachment::new("report.pdf")
            .description("PDF · 2.4 MB")
            .density(ControlSize::Medium)
            .icon(glyph("PDF"))
            .on_remove(Message::Remove)
            .width(272.0),
        Attachment::new("presentation.key")
            .description("KEY · 12 MB")
            .density(ControlSize::Large)
            .icon(glyph("KEY"))
            .on_remove(Message::Remove)
            .width(320.0),
    ]
    .spacing(12);

    // The two axes. Vertical puts a square preview above the metadata and moves
    // the actions onto the preview's upper trailing corner; its media box is 120
    // pixels with metadata and 96 without.
    let axes = row![
        Attachment::new("design-mockups.png")
            .description("PNG · 1.8 MB")
            .axis(Axis::Vertical)
            .preview(preview(Color::from_rgb(0.35, 0.45, 0.85), "preview"))
            .on_remove(Message::Remove)
            .on_press(Message::Open),
        Attachment::new("sdk-preview.svg")
            .description("SVG · 1280 × 720")
            .axis(Axis::Vertical)
            .status(Status::Uploading)
            .progress(35.0)
            .preview(preview(Color::from_rgb(0.85, 0.45, 0.35), "preview"))
            .overlay(true)
            .on_remove(Message::Remove),
        Attachment::new("icon")
            .axis(Axis::Vertical)
            .show_meta(false)
            .preview(preview(Color::from_rgb(0.35, 0.65, 0.45), "image"))
            .on_remove(Message::Remove),
    ]
    .spacing(12);

    // The slots: media only, metadata only, and a whole-card click with an
    // action control. A card or action without a message is drawn disabled.
    let slots = column![
        Attachment::new("wireframe.png")
            .icon(glyph("IMG"))
            .show_meta(false)
            .on_remove(Message::Remove)
            .width(160.0),
        Attachment::new("budget.csv")
            .description("CSV · 8 KB")
            .show_media(false)
            .width(232.0),
        Attachment::new("receipt.pdf")
            .description("PDF · 96 KB")
            .icon(glyph("PDF"))
            .action("Open")
            .on_action(Message::Open)
            .on_remove(Message::Remove)
            .on_press(Message::Open)
            .width(272.0),
        Attachment::new("screenshot.png")
            .description("PNG · 340 KB")
            .icon(glyph("IMG"))
            .action("Open")
            .on_remove(Message::Remove)
            .on_press(Message::Open)
            .fill(Color::from_rgb(0.96, 0.96, 0.99))
            .width(272.0),
    ]
    .spacing(12);

    // The group: one scrolling row, twelve pixels apart.
    let group = Group::new()
        .push(
            Attachment::new("one.png")
                .description("PNG · 1 MB")
                .icon(glyph("IMG"))
                .width(180.0),
        )
        .push(
            Attachment::new("two.png")
                .description("PNG · 2 MB")
                .icon(glyph("IMG"))
                .width(180.0),
        )
        .push(
            Attachment::new("three.png")
                .description("PNG · 3 MB")
                .status(Status::Failed)
                .icon(glyph("IMG"))
                .action("Retry")
                .on_action(Message::Retry)
                .width(200.0),
        )
        .push(
            Attachment::new("four.png")
                .description("PNG · 4 MB")
                .icon(glyph("IMG"))
                .on_remove(Message::Remove)
                .width(180.0),
        );

    scrollable(
        column![
            section("States", states),
            section("Densities", densities),
            section("Axes", axes),
            section("Slots and actions", slots),
            section(
                "Group",
                container(group)
                    .width(Length::Fill)
                    .padding(4.0)
                    .style(|_: &Theme| container::Style {
                        border: iced::Border {
                            color: Color::from_rgb(0.85, 0.85, 0.85),
                            width: 1.0,
                            radius: theme::RADIUS_MD.into(),
                        },
                        ..container::Style::default()
                    }),
            ),
            text(format!(
                "removed {} · opened {} · retried {}",
                app.removed, app.opened, app.retried
            )),
        ]
        .spacing(24)
        .padding(16),
    )
    .into()
}
