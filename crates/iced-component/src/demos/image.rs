//! `image` — the framed picture the Slint example shows: a source, the three
//! fits, a corner radius and a placeholder, then a thumbnail gallery that
//! selects.
//!
//! Run it with `cargo run --example image`, or in the browser as `?story=image`
//! in the gallery the website embeds.
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
//! The four titles are the page's headings, which are also the Slint example's
//! GroupBox titles. The pictures are real: a PNG the repository already tracks,
//! embedded in the module and decoded once, and a photograph fetched from a URL
//! at startup. The online frame is the loading and failure path an application
//! has to design for — while the fetch is in flight, and when it fails because
//! the reader is offline, the frame keeps its size and centres the same
//! placeholder a frame with no source draws. The placeholder icon is an inline
//! SVG, and its colour comes from the style closure, because an SVG keeps its
//! own black otherwise and disappears in the dark theme.

use iced::widget::{column, container, image, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::image::{ImageFit, ImageView};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — image")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// Which thumbnail the gallery has selected. The selection is the frame's
    /// job, not the image's: the ring is on the cell around it.
    selected: usize,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered_section: Option<&'static str>,
    pressed_source: Option<&'static str>,
    /// The picture the crate ships, decoded once at startup: a real PNG, not
    /// pixels generated per frame.
    local: image::Handle,
    /// The photograph this demo fetches, and what became of it.
    online: Online,
}

/// The online sample's three states. `Loading` and `Failed` both paint the
/// placeholder, so a slow or failed fetch looks like a frame with no source
/// rather than a blank box.
#[derive(Debug)]
enum Online {
    Loading,
    Loaded(image::Handle),
    Failed(String),
}

impl Online {
    /// The source to paint, or `None` while the frame should centre its
    /// placeholder.
    fn source(&self) -> Option<image::Handle> {
        match self {
            Self::Loaded(handle) => Some(handle.clone()),
            Self::Loading | Self::Failed(_) => None,
        }
    }

    /// The caption under the frame, so the state is legible from the page and
    /// from a screenshot rather than inferred from the picture.
    fn caption(&self) -> String {
        match self {
            Self::Loading => "From a URL — loading…".to_owned(),
            Self::Loaded(_) => "From a URL — a photograph, fetched over the network".to_owned(),
            Self::Failed(reason) => {
                format!("From a URL — unavailable ({reason}); the placeholder holds the frame")
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A gallery cell was pressed.
    Select(usize),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
    /// The online fetch finished, with the picture or the reason it did not.
    Online(Result<image::Handle, String>),
}

fn boot() -> (App, Task<Message>) {
    let app = App {
        selected: 0,
        hovered_section: None,
        pressed_source: None,
        local: local_image(),
        online: Online::Loading,
    };
    // The fetch is the one piece of this demo that cannot be ready at boot, so
    // the frame starts on its placeholder and is swapped when the bytes arrive.
    (app, Task::perform(load_online(), Message::Online))
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Select(index) => app.selected = index,
        Message::HoverSource(title) => app.hovered_section = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
        Message::Online(result) => {
            app.online = match result {
                Ok(handle) => Online::Loaded(handle),
                Err(reason) => Online::Failed(reason),
            };
        }
    }
    Task::none()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered_section == Some(title);
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

/// A frame with its caption beneath it, the way the Slint example labels each
/// variant.
fn labelled<'a>(caption: &str, picture: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![
        picture.into(),
        text(caption.to_owned())
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(theme::SPACING_XS)
    .width(Length::Fill)
    .into()
}

/// The placeholder the frame centres with no source: the Lucide `image` glyph,
/// tinted from the style closure. An SVG left to itself is black in both themes,
/// which is invisible on the dark frame.
fn placeholder_icon() -> Element<'static, Message> {
    svg(svg::Handle::from_memory(IMAGE_XML))
        .width(28.0)
        .height(28.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Start with a working image
    // A source paints the frame; no source keeps the frame and centres the
    // placeholder, so a missing asset never leaves an empty hole in a layout.
    // The third frame is fetched from a URL at startup: it shows the same
    // placeholder while the fetch is in flight and after one fails, which is
    // the state a reader offline sees.
    let working = row![
        labelled(
            "With a source",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Contain)
                .height(Length::Fixed(140.0)),
        ),
        labelled(
            "Without one",
            ImageView::new()
                .height(Length::Fixed(140.0))
                .placeholder(placeholder_icon()),
        ),
        labelled(
            &app.online.caption(),
            match app.online.source() {
                Some(source) => ImageView::new()
                    .source(source)
                    .fit(ImageFit::Contain)
                    .height(Length::Fixed(140.0)),
                None => ImageView::new()
                    .fit(ImageFit::Contain)
                    .height(Length::Fixed(140.0))
                    .placeholder(placeholder_icon()),
            },
        ),
    ]
    .spacing(theme::SPACING_LG);

    // section: Common patterns
    // `fit` decides how a source fills its frame: cover crops the overflow,
    // contain letterboxes it over the frame's fill, fill stretches it. The
    // source is a real square picture, so a wide frame makes the three
    // visibly different — a circle stays a circle, gains bars, or becomes an
    // ellipse.
    let fits = row![
        labelled(
            "cover — crops the overflow",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Cover)
                .height(Length::Fixed(120.0)),
        ),
        labelled(
            "contain — letterboxes",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Contain)
                .height(Length::Fixed(120.0)),
        ),
        labelled(
            "fill — stretches",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Fill)
                .height(Length::Fixed(120.0)),
        ),
    ]
    .spacing(theme::SPACING_MD);

    // `radius` shapes the frame, and the source is clipped to the same corner.
    // A rounded square and a circle come from the same component.
    let radii = row![
        labelled(
            "radius 3",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Cover)
                .radius(theme::RADIUS_SM)
                .width(Length::Fixed(72.0))
                .height(Length::Fixed(72.0)),
        ),
        labelled(
            "radius 8",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Cover)
                .radius(theme::RADIUS_LG)
                .width(Length::Fixed(72.0))
                .height(Length::Fixed(72.0)),
        ),
        labelled(
            "a full circle",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Cover)
                .radius(36.0)
                .width(Length::Fixed(72.0))
                .height(Length::Fixed(72.0)),
        ),
        labelled(
            "the default",
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Cover)
                .width(Length::Fixed(72.0))
                .height(Length::Fixed(72.0)),
        ),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(Alignment::Start);

    let patterns = column![fits, radii].spacing(theme::SPACING_LG);

    // section: Gallery with selection
    // The selected cell is the frame's job, not the image's: a ring around the
    // `ImageView` reads as selection without recolouring the picture. Pressing a
    // cell moves the ring, which is the one interaction this demo drives.
    let mut gallery = row![].spacing(theme::SPACING_SM);
    for index in 0..6 {
        gallery = gallery.push(thumbnail(app.local.clone(), index, app.selected == index));
    }

    // section: Accessibility
    // An image that carries meaning needs its description in the text beside it;
    // one that only decorates needs none.
    let accessibility = column![
        row![
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Contain)
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(48.0)),
            text("Meaningful: the caption carries what the picture shows.")
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        ]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Center),
        row![
            ImageView::new()
                .source(app.local.clone())
                .fit(ImageFit::Contain)
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(48.0)),
            text("Decorative: the text already says it, so the picture adds nothing.")
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Center),
    ]
    .spacing(theme::SPACING_LG);

    // section: end
    let status = text(format!(
        "Selected {} of {} — pressing a cell moves the ring, not the picture.",
        app.selected + 1,
        6
    ))
    .size(theme::FONT_SIZE_SM)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery_page = column![
        heading(app, 0, "Start with a working image"),
        body_box(working),
        heading(app, 1, "Common patterns"),
        body_box(patterns),
        heading(app, 2, "Gallery with selection"),
        body_box(gallery),
        status,
        heading(app, 3, "Accessibility"),
        body_box(accessibility),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery_page).height(Length::Fill).into()
}

/// One gallery cell: the picture inset in a rounded box whose edge is the
/// selection. The inner frame keeps the picture's own radius, so the ring and
/// the picture have their own corners.
fn thumbnail(handle: image::Handle, index: usize, selected: bool) -> Element<'static, Message> {
    let picture = ImageView::new()
        .source(handle)
        .fit(ImageFit::Contain)
        .radius(theme::RADIUS_MD)
        .height(Length::Fill);

    mouse_area(
        container(picture)
            .width(Length::Fill)
            .height(Length::Fixed(104.0))
            .padding(8.0)
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                let (width, color) = if selected {
                    (2.0, tokens.primary)
                } else {
                    (1.0, tokens.border)
                };
                container::Style {
                    background: Some(theme::alpha(tokens.muted, 0.3).into()),
                    border: iced::border::rounded(theme::RADIUS_LG)
                        .color(color)
                        .width(width),
                    ..container::Style::default()
                }
            }),
    )
    .on_press(Message::Select(index))
    .into()
}

/// The picture the crate ships: a PNG the repository already tracks under the
/// site's `public/`, embedded in the module at compile time so the browser
/// gallery needs no asset request and the file is reachable by construction.
/// 144x144 and 7.8 KB.
const LOCAL_PNG: &[u8] = include_bytes!("../../../../website/public/bun.png");

/// The one online source: a real photograph (a cat) from Wikimedia Commons,
/// 1280x960 JPEG. Its response allows cross-origin reads, so the browser can
/// fetch it; it is also a request that can fail, which is the other half of
/// what section one shows.
const ONLINE_URL: &str =
    "https://upload.wikimedia.org/wikipedia/commons/thumb/3/3a/Cat03.jpg/1280px-Cat03.jpg";

/// The embedded PNG, decoded once. `Handle::from_rgba` is what an application
/// gets after decoding a file, so the component is handed real pixels.
fn local_image() -> image::Handle {
    decode(LOCAL_PNG).expect("the shipped PNG decodes with the crate's PNG codec")
}

/// Fetches [`ONLINE_URL`] and decodes it into the same kind of handle.
async fn load_online() -> Result<image::Handle, String> {
    let bytes = fetch(ONLINE_URL).await?;
    decode(&bytes).ok_or_else(|| "the response was not a decodable PNG or JPEG".to_owned())
}

/// Encoded bytes to a decoded handle, the conversion an application does when it
/// reads its own files. `None` when the bytes are not an image this build can
/// decode, which the frame answers with its placeholder.
fn decode(bytes: &[u8]) -> Option<image::Handle> {
    let decoded = ::image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = decoded.dimensions();
    Some(image::Handle::from_rgba(width, height, decoded.into_raw()))
}

/// The bytes at `url`. The browser fetches them; the native example has no HTTP
/// client and reports the same failure an offline browser does, because that
/// failure is part of the demo rather than an accident of the platform.
#[cfg(target_arch = "wasm32")]
async fn fetch(url: &str) -> Result<Vec<u8>, String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or_else(|| "no browser window".to_owned())?;
    let response = JsFuture::from(window.fetch_with_str(url))
        .await
        .map_err(describe)?;
    let response: web_sys::Response = response
        .dyn_into()
        .map_err(|_| "the fetch did not answer with a Response".to_owned())?;
    if !response.ok() {
        return Err(format!("the server answered HTTP {}", response.status()));
    }
    let buffer = JsFuture::from(response.array_buffer().map_err(describe)?)
        .await
        .map_err(describe)?;
    Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}

/// A failed fetch arrives as a JS value. The message on the error object is the
/// useful part — a browser's `TypeError: Failed to fetch` — and `Debug` on the
/// whole object prints a stack that reads as noise under the frame.
#[cfg(target_arch = "wasm32")]
fn describe(error: wasm_bindgen::JsValue) -> String {
    use wasm_bindgen::JsCast;

    if let Some(text) = error.as_string() {
        return text;
    }
    if let Some(error) = error.dyn_ref::<js_sys::Error>() {
        return format!("{}: {}", error.name(), error.message());
    }
    "the request failed".to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch(_url: &str) -> Result<Vec<u8>, String> {
    Err("the native example has no HTTP client".to_owned())
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

const IMAGE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>"#;
