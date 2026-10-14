//! `alert` — the callouts the Slint example shows: the default one, the four
//! status variants, the banner shape, a custom icon, the four sizes, the
//! dismissible one, the two formatted bodies, the conditional one and the
//! interactive one that reports its own dismissal.
//!
//! Run it with `cargo run --example alert`, or in the browser as
//! `?story=alert` in the gallery the website embeds.
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
//! **Two things Slint's alert draws that this one leaves to the caller.** Slint
//! picks its own status glyph, and an iced alert takes the leading element from
//! the caller, so the demo passes the same four glyphs by hand. Slint also gives
//! the alert a `@children` slot and its example fills it with a formatted
//! `TextView`; an iced alert carries a plain message string, so the two
//! formatted sections show the same words as line-broken text in the status ink
//! and name the formatting as the caller's rather than pretending a `TextView`
//! sits inside the alert.
//!
//! The page's other headings — `Alert with Title`, `Form Validation Errors`,
//! `Success Notification`, `System Status Banner` — are GPUI samples the Slint
//! example has no `GroupBox` for, so they keep the GPUI panel and get no iced
//! one, the way `checkbox`'s `Custom Tab Order` does.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Color, Element, Font, Length, Task, Theme, font};

use crate::components::alert::{Alert, Status};
use crate::components::group_box::{self, GroupBox};
use crate::components::switch::Switch;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's headings — have them. The index is what the page is asked to
/// scroll to.
const SECTIONS: [&str; 10] = [
    "Default",
    "Variants",
    "Banner",
    "Custom icon",
    "Alert Sizes",
    "Closable Alerts",
    "With Markdown Content",
    "Conditional Visibility",
    "Interactive Alert with Custom Action",
    "Multi-line Content with Formatting",
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — alert")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The banner section's dismissible bar. An iced alert owns no `open` flag,
    /// so the caller keeps the flag and stops constructing the alert.
    banner_open: bool,
    /// The variants section's info alert, dismissible for the same reason.
    variant_notice_open: bool,
    /// The closable section's alert.
    dismissible_open: bool,
    /// The conditional section's flag, which its switch writes.
    should_show_alert: bool,
    /// Whether the interactive alert has reported its dismissal.
    update_dismissed: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            banner_open: true,
            variant_notice_open: true,
            dismissible_open: true,
            should_show_alert: true,
            update_dismissed: false,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DismissBanner,
    DismissVariantNotice,
    DismissClosable,
    ShowAlert(bool),
    DismissUpdate,
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
        Message::DismissBanner => app.banner_open = false,
        Message::DismissVariantNotice => app.variant_notice_open = false,
        Message::DismissClosable => app.dismissible_open = false,
        Message::ShowAlert(shown) => app.should_show_alert = shown,
        Message::DismissUpdate => app.update_dismissed = true,
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

/// Muted text under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: impl Into<String>) -> Element<'a, Message> {
    text(line.into())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
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

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(Font {
            weight: font::Weight::Medium,
            ..Font::DEFAULT
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

/// The ink the status paints its title, message and glyph with. The alert
/// resolves the same pair inside its own style closure, so the glyph and the
/// words move together when the palette does.
fn ink(status: Status, tokens: Tokens) -> Color {
    match status {
        Status::Default => tokens.foreground,
        Status::Info => tokens.status_color(theme::Status::Info),
        Status::Success => tokens.status_color(theme::Status::Success),
        Status::Warning => tokens.status_color(theme::Status::Warning),
        Status::Danger => tokens.status_color(theme::Status::Danger),
    }
}

/// A leading glyph in the alert's own status ink: a `&[u8]` SVG is the artwork
/// iced's alert takes from its caller.
fn glyph<'a>(xml: &'static [u8], status: Status) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(xml))
        .width(16.0)
        .height(16.0)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink(status, Tokens::of(theme))),
        })
        .into()
}

/// The glyph the status draws, the four the Slint alert picks between.
fn status_glyph(status: Status) -> &'static [u8] {
    match status {
        Status::Default | Status::Info => INFO_XML,
        Status::Success => CIRCLE_CHECK_XML,
        Status::Warning => TRIANGLE_ALERT_XML,
        Status::Danger => CIRCLE_X_XML,
    }
}

/// The status glyph in the status ink. Slint chooses this itself; iced takes it
/// from the caller, so every alert in the gallery passes its own.
fn status_icon<'a>(status: Status) -> Element<'a, Message> {
    glyph(status_glyph(status), status)
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Default
    // The plain callout: a title and a three-line message in the info ink over
    // the status colour's 4% tint. The leading glyph is the caller's, and the
    // Slint default is the info tone.
    let default_alert = body_box(
        Alert::new(
            "Your workspace is ready for the team.\n\
             • 12 members have access\n\
             • Billing remains with the workspace owner",
        )
        .title("Workspace settings saved")
        .icon(status_icon(Status::Info)),
    );

    // section: Variants
    // The four statuses in the Slint order. The info one is dismissible, so the
    // application owns its flag and stops constructing it once it is closed; the
    // other three take their fill, edge and ink from their own status.
    let mut variants = column![].spacing(8.0);
    if app.variant_notice_open {
        variants = variants.push(
            Alert::new("Maintenance starts Friday at 22:00 UTC.")
                .title("Scheduled maintenance")
                .status(Status::Info)
                .icon(status_icon(Status::Info))
                .on_close(Message::DismissVariantNotice),
        );
    }
    variants = variants.push(
        Alert::new("The transfer is queued and usually settles within one business day.")
            .title("Transfer submitted")
            .status(Status::Success)
            .icon(status_icon(Status::Success)),
    );
    variants = variants.push(
        Alert::new(
            "Two teammates still use recovery codes generated more than a year ago. Ask them \
             to generate a fresh set in Security settings.",
        )
        .status(Status::Warning)
        .icon(status_icon(Status::Warning)),
    );
    variants = variants.push(
        Alert::new(
            "Please verify your billing information and try again.\n\
             • Check your card details\n\
             • Ensure sufficient funds\n\
             • Verify billing address",
        )
        .title("Unable to process your payment.")
        .status(Status::Danger)
        .icon(status_icon(Status::Danger)),
    );
    let variants = body_box(variants);

    // section: Banner
    // The full-width bar: square corners, a tighter padding and no title. The
    // first is dismissible and owns its own flag; the rest are the four statuses.
    let mut banners = column![].spacing(8.0);
    if app.banner_open {
        banners = banners.push(
            Alert::new("Reporting is read-only while the nightly ledger closes.")
                .icon(status_icon(Status::Info))
                .banner(true)
                .on_close(Message::DismissBanner),
        );
    }
    banners = banners.push(
        Alert::new("A new desktop update will install after you restart.")
            .status(Status::Info)
            .icon(status_icon(Status::Info))
            .banner(true),
    );
    banners = banners.push(
        Alert::new("All 1,284 records finished importing.")
            .status(Status::Success)
            .icon(status_icon(Status::Success))
            .banner(true),
    );
    banners = banners.push(
        Alert::new("Your API key expires in 6 days. Rotate it before August 19.")
            .status(Status::Warning)
            .icon(status_icon(Status::Warning))
            .banner(true),
    );
    banners = banners.push(
        Alert::new("Live updates are disconnected. Changes may be delayed.")
            .status(Status::Danger)
            .icon(status_icon(Status::Danger))
            .banner(true),
    );
    let banners = body_box(banners);

    // section: Custom icon
    // The leading slot takes any element, so the calendar artwork replaces the
    // status glyph; the ink is still the status colour.
    let custom_icon = body_box(
        Alert::new(
            "The quarterly planning review overlaps with the APAC operations call. Move one \
             event or invite another owner before sending the agenda.",
        )
        .title("Two events overlap by 30 minutes")
        .icon(glyph(CALENDAR_XML, Status::Info)),
    );

    // section: Alert Sizes
    // The size ladder changes the padding, the gap and the corner only; the
    // message stays at text-sm at every step, as GPUI's does.
    let sizes = body_box(
        column![
            Alert::new("Message content")
                .title("XSmall Alert")
                .status(Status::Info)
                .size(ControlSize::XSmall)
                .icon(status_icon(Status::Info)),
            Alert::new("Message content")
                .title("Small Alert")
                .status(Status::Info)
                .size(ControlSize::Small)
                .icon(status_icon(Status::Info)),
            Alert::new("Message content")
                .title("Medium Alert")
                .status(Status::Info)
                .icon(status_icon(Status::Info)),
            Alert::new("Message content")
                .title("Large Alert")
                .status(Status::Info)
                .size(ControlSize::Large)
                .icon(status_icon(Status::Info)),
        ]
        .spacing(12.0),
    );

    // section: Closable Alerts
    // The dismiss control appears only when the caller passes the message it
    // sends: an iced alert owns no `open` flag, so the application closes it.
    let mut closable = column![].spacing(8.0);
    if app.dismissible_open {
        closable = closable.push(
            Alert::new("This alert can be dismissed.")
                .title("Dismissible")
                .status(Status::Info)
                .icon(status_icon(Status::Info))
                .on_close(Message::DismissClosable),
        );
    }
    closable = closable.push(muted_line(
        "Pressed, the close control sends its message and the application stops drawing the alert.",
    ));
    let closable = body_box(closable);

    // section: With Markdown Content
    // Slint fills the alert's `@children` with a formatted `TextView`; an iced
    // alert carries a plain message, so this is the same words with the list's
    // own line breaks, and the formatting is the caller's to place around it.
    let markdown = body_box(
        Alert::new(
            "Please verify your billing information and try again.\n\
             • Check your card details\n\
             • Ensure sufficient funds\n\
             • Verify billing address",
        )
        .title("Payment Failed")
        .status(Status::Danger)
        .icon(status_icon(Status::Danger)),
    );

    // section: Conditional Visibility
    // The switch and the alert share one flag: the caller owns it and simply
    // stops constructing the alert, which is how iced spells `open`.
    let mut conditional = column![
        Switch::new(app.should_show_alert)
            .label("Show alert")
            .on_toggle(Message::ShowAlert),
    ]
    .spacing(8.0);
    if app.should_show_alert {
        conditional = conditional.push(
            Alert::new("This alert may be hidden.")
                .title("Conditional")
                .status(Status::Info)
                .icon(status_icon(Status::Info)),
        );
    }
    let conditional = body_box(conditional);

    // section: Interactive Alert with Custom Action
    // The close control reports the dismissal to the application instead of
    // hiding anything itself, so the demo writes the line under it.
    let mut interactive = column![].spacing(8.0);
    if app.update_dismissed {
        interactive = interactive.push(muted_line("Update dismissed"));
    } else {
        interactive = interactive.push(
            Alert::new("A new version of the application is available.")
                .title("Update Available")
                .status(Status::Info)
                .icon(glyph(DOWNLOAD_XML, Status::Info))
                .on_close(Message::DismissUpdate),
        );
    }
    let interactive = body_box(interactive);

    // section: Multi-line Content with Formatting
    // The second formatted body: Slint's `TextView` again, and here the same
    // words as plain text in the warning ink behind the shield artwork.
    let multiline = body_box(
        Alert::new(
            "Security Notice\n\n\
             Unusual activity detected on your account.\n\n\
             Recent activity:\n\
             • Login from new device (Chrome on Windows)\n\
             • Location: San Francisco, CA\n\
             • Time: Today at 2:30 PM\n\n\
             If this wasn't you, please change your password immediately.",
        )
        .title("Security Alert")
        .status(Status::Warning)
        .icon(glyph(SHIELD_XML, Status::Warning)),
    );

    // section: end
    let status = text("Dismiss an alert or flip the conditional switch — the gallery is live.")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        default_alert,
        heading(app, 1, SECTIONS[1]),
        variants,
        heading(app, 2, SECTIONS[2]),
        banners,
        heading(app, 3, SECTIONS[3]),
        custom_icon,
        heading(app, 4, SECTIONS[4]),
        sizes,
        heading(app, 5, SECTIONS[5]),
        closable,
        heading(app, 6, SECTIONS[6]),
        markdown,
        heading(app, 7, SECTIONS[7]),
        conditional,
        heading(app, 8, SECTIONS[8]),
        interactive,
        heading(app, 9, SECTIONS[9]),
        multiline,
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    let page = container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        });

    page.into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

const INFO_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;

const CIRCLE_CHECK_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m16 9-5.5 5.5L8 12"/></svg>"#;

const TRIANGLE_ALERT_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;

const CIRCLE_X_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;

const CALENDAR_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/></svg>"#;

const DOWNLOAD_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 15V3"/><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/></svg>"#;

const SHIELD_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/></svg>"#;
