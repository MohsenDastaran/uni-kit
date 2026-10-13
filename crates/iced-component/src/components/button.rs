//! A push button. Mirrors GPUI Component's `Button` and the Slint version beside
//! it: variants, outline, sizes, a leading icon, a trailing icon, a loading state
//! and a selected state, with the same fills for rest, hover, press and disabled.
//!
//! An iced button carries its own message rather than a callback, so the message
//! is a value the caller passes in — `button("Save").on_press(Message::Save)`.
//! The icon is the caller's element, because the icon set belongs to the
//! application; Slint ships its own because its build script resolves a folder.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, container, row, text};
use iced::{Background, Border, Color, Element, Length, Shadow, Theme};

use super::theme::{self, ControlSize, Tokens};


/// How the button is filled. The names and meanings are the Slint theme's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Secondary,
    Danger,
    Warning,
    Success,
    Info,
    Ghost,
    Link,
    Text,
}

impl Variant {
    /// A status variant tints its fill from the status colour rather than from
    /// the surface, at 20% in the resting state.
    fn status(self) -> Option<theme::Status> {
        match self {
            Self::Danger => Some(theme::Status::Danger),
            Self::Warning => Some(theme::Status::Warning),
            Self::Success => Some(theme::Status::Success),
            Self::Info => Some(theme::Status::Info),
            _ => None,
        }
    }

    /// Ghost, link and text draw no fill of their own.
    fn transparent_fill(self) -> bool {
        matches!(self, Self::Ghost | Self::Link | Self::Text)
    }
}

/// A button, built with the methods below.
pub struct Button<'a, Message> {
    label: String,
    variant: Variant,
    size: ControlSize,
    outline: bool,
    loading: bool,
    selected: bool,
    icon: Option<Element<'a, Message>>,
    suffix_icon: Option<Element<'a, Message>>,
    on_press: Option<Message>,
    width: Length,
}

impl<'a, Message> Button<'a, Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::default(),
            size: ControlSize::default(),
            outline: false,
            loading: false,
            selected: false,
            icon: None,
            suffix_icon: None,
            on_press: None,
            width: Length::Shrink,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Draws the border and keeps the fill, the shape GPUI calls outlined.
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// A control that is working: visibly busy, and refusing input.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Stays visibly on rather than looking pressed.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// A leading element, drawn before the label at the icon box's size.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// A trailing element, drawn after the label. The Slint `Button`'s
    /// `suffix-icon`, which is what makes `pagination`'s next control read
    /// "Next ›" rather than "› Next".
    pub fn suffix_icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.suffix_icon = Some(icon.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// The height GPUI's sizes use: 20 / 24 / 32 / 40.
    fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 20.0,
            ControlSize::Small => 24.0,
            ControlSize::Large => 40.0,
            ControlSize::Medium => 32.0,
        }
    }

    /// Padding inside the control. A link, a text button and an icon-only button
    /// carry none, because the label is the whole control.
    fn padding(&self) -> f32 {
        if self.icon_only() || matches!(self.variant, Variant::Link | Variant::Text) {
            0.0
        } else {
            match self.size {
                ControlSize::XSmall => 4.0,
                ControlSize::Small => 8.0,
                ControlSize::Large => 16.0,
                ControlSize::Medium => 10.0,
            }
        }
    }

    fn icon_only(&self) -> bool {
        self.label.is_empty() && self.icon.is_some()
    }

    fn label_size(&self) -> f32 {
        match self.size {
            ControlSize::Large => theme::FONT_SIZE_LG,
            other => other.control_font_size(),
        }
    }

    /// The content row: the leading icon, the label, the trailing icon, and the
    /// gaps between them.
    fn content(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let gap = match self.size {
            ControlSize::XSmall | ControlSize::Small => 4.0,
            _ => 8.0,
        };
        let label_size = self.label_size();

        let variant = self.variant;
        let outline = self.outline;
        let label = self.label.clone();

        let mut contents = row![].spacing(gap).align_y(iced::Alignment::Center);
        if let Some(icon) = self.icon {
            contents = contents.push(icon);
        }
        if !self.label.is_empty() {
            contents = contents.push(
                text(label)
                    .size(label_size)
                    .font(iced::Font {
                        weight: iced::font::Weight::Medium,
                        ..iced::Font::DEFAULT
                    })
                    .style(move |theme: &Theme| text::Style {
                        color: Some(ink(Tokens::of(theme), variant, outline)),
                    }),
            );
        }
        if let Some(suffix) = self.suffix_icon {
            contents = contents.push(suffix);
        }
        contents.into()
    }

    fn status_variant(&self) -> bool {
        self.variant.status().is_some()
    }

    /// The colour a status variant is built from, or the foreground otherwise.
    fn tone(&self, tokens: Tokens) -> Color {
        match self.variant.status() {
            Some(status) => tokens.status_color(status),
            None => match self.variant {
                Variant::Primary => tokens.primary,
                Variant::Secondary => tokens.secondary,
                _ => tokens.foreground,
            },
        }
    }

    /// The fill, given the tokens, the interaction status and the selection.
    fn fill(&self, tokens: Tokens, status: button::Status) -> Color {
        let transparent = self.variant.transparent_fill();
        let outline = self.outline;
        let tone = self.tone(tokens);
        let status_variant = self.status_variant();

        if status == button::Status::Disabled {
            return if transparent || outline {
                Color::TRANSPARENT
            } else if self.variant == Variant::Primary {
                theme::alpha(tokens.primary, 0.15)
            } else if status_variant {
                theme::alpha(tone, 0.15)
            } else if self.variant == Variant::Secondary {
                tokens.secondary
            } else {
                theme::mix(tokens.input, tokens.background, 0.5)
            };
        }

        if self.selected {
            return if self.variant == Variant::Primary {
                theme::alpha(tokens.primary, 0.10)
            } else if status_variant {
                theme::alpha(tone, 0.12)
            } else if outline {
                if self.variant == Variant::Secondary {
                    tokens.accent
                } else {
                    theme::alpha(tokens.primary, 0.10)
                }
            } else if transparent {
                theme::alpha(tokens.primary, 0.10)
            } else {
                tokens.secondary
            };
        }

        match status {
            button::Status::Hovered => {
                if self.variant == Variant::Ghost {
                    // shadcn's ghost hover is the accent surface, whole, in both
                    // appearances; halving it in the dark theme left the one
                    // control with no border and no fill with no hover either.
                    tokens.accent
                } else if transparent {
                    Color::TRANSPARENT
                } else if outline {
                    if status_variant || self.variant == Variant::Primary {
                        theme::alpha(tone, 0.1)
                    } else {
                        tokens.accent
                    }
                } else if status_variant {
                    theme::alpha(tone, 0.3)
                } else if self.variant == Variant::Primary {
                    // 90% of the fill over the page, which is shadcn's
                    // `bg-primary/90`: lighter than the fill in the dark theme,
                    // very slightly lighter in the light one. The weight here was
                    // inverted, so the fill went *to* the page and the button
                    // disappeared under the pointer.
                    theme::mix(tokens.primary, tokens.background, 0.1)
                } else if self.variant == Variant::Secondary {
                    theme::mix(tokens.secondary, tokens.background, 0.1)
                } else {
                    theme::mix(tokens.input, tokens.background, 0.5)
                }
            }
            button::Status::Pressed => {
                if self.variant == Variant::Ghost {
                    theme::mix(tokens.input, tokens.background, 0.3)
                } else if transparent {
                    Color::TRANSPARENT
                } else if outline {
                    if status_variant || self.variant == Variant::Primary {
                        theme::alpha(tone, 0.2)
                    } else {
                        theme::pressed(&tokens, tokens.accent)
                    }
                } else if status_variant {
                    theme::alpha(tone, 0.4)
                } else if self.variant == Variant::Primary {
                    theme::darken(tokens.primary, 0.1)
                } else if self.variant == Variant::Secondary {
                    theme::darken(tokens.secondary, 0.1)
                } else {
                    theme::mix(tokens.input, tokens.background, 0.3)
                }
            }
            // Rest, and the disabled case already answered above.
            _ => {
                if transparent || outline {
                    Color::TRANSPARENT
                } else if status_variant {
                    theme::alpha(tone, 0.2)
                } else if self.variant == Variant::Primary {
                    tokens.primary
                } else if self.variant == Variant::Secondary {
                    tokens.secondary
                } else if tokens.dark {
                    theme::alpha(tokens.input, 0.7)
                } else {
                    tokens.background
                }
            }
        }
    }

    /// The border colour, which only the default, the outlined and the status
    /// variants draw.
    fn border_color(&self, tokens: Tokens) -> Color {
        if self.variant.transparent_fill() {
            return Color::TRANSPARENT;
        }
        let tone = self.tone(tokens);
        // An edge has to be visible in either theme. The fixed dark greys a solid
        // control fills with disappear on a near-black page, which is what made an
        // outlined Primary and Secondary read as plain text in the dark theme; a
        // proportion of the *foreground* is a light edge there and a dark one in
        // the light theme, and an outlined primary keeps its own colour.
        match self.variant {
            Variant::Primary if self.outline => tokens.primary,
            Variant::Default | Variant::Secondary => theme::alpha(tokens.foreground, 0.25),
            _ if self.status_variant() => {
                theme::alpha(tone, if self.outline { 0.45 } else { 0.2 })
            }
            _ => Color::TRANSPARENT,
        }
    }

    fn border_width(&self) -> f32 {
        if !self.variant.transparent_fill()
            && (self.variant == Variant::Default || self.outline)
        {
            1.0
        } else {
            0.0
        }
    }
}

/// The colour a button's label is drawn in, for the tokens in force.
pub fn ink(tokens: Tokens, variant: Variant, outline: bool) -> Color {
    let tone = match variant.status() {
        Some(status) => tokens.status_color(status),
        None => match variant {
            Variant::Primary => tokens.primary,
            Variant::Secondary => tokens.secondary,
            _ => tokens.foreground,
        },
    };
    match variant {
        Variant::Link => tokens.primary,
        Variant::Text => theme::alpha(tokens.foreground, 0.9),
        Variant::Ghost => tokens.secondary_foreground,
        Variant::Default => tokens.foreground,
        _ if variant.status().is_some() => {
            if outline {
                tone
            } else {
                // A solid status button carries its hue in the fill, so its label
                // is the ordinary foreground: near-white on the dark red, near-black
                // on the light one. Slint's `Button` falls through to `foreground`
                // for exactly these four, and the hue-on-hue it replaces is the
                // unreadable pair in the dark theme.
                tokens.foreground
            }
        }
        Variant::Primary => {
            if outline {
                tokens.primary
            } else {
                tokens.primary_foreground
            }
        }
        Variant::Secondary => tokens.secondary_foreground,
        _ => tokens.foreground,
    }
}

/// The style function iced calls for each interaction state. Components pass it
/// to `.style(..)`; it is `pub` so an application can reuse it for its own
/// buttons and get the same look.
pub struct Style {
    pub variant: Variant,
    pub outline: bool,
    pub selected: bool,
    pub loading: bool,
}

impl Style {
    pub fn new(variant: Variant) -> Self {
        Self {
            variant,
            outline: false,
            selected: false,
            loading: false,
        }
    }

    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    fn builder(&self) -> Button<'static, ()> {
        Button {
            label: String::new(),
            variant: self.variant,
            size: ControlSize::Medium,
            outline: self.outline,
            loading: self.loading,
            selected: self.selected,
            icon: None,
            suffix_icon: None,
            on_press: None,
            width: Length::Shrink,
        }
    }

    pub fn resolve(&self, tokens: Tokens, status: button::Status) -> button::Style {
        let builder = self.builder();
        let disabled = matches!(status, button::Status::Disabled) || self.loading;
        let status = if disabled {
            button::Status::Disabled
        } else {
            status
        };
        let shown_ink = if disabled {
            tokens.muted_foreground
        } else {
            ink(tokens, self.variant, self.outline)
        };
        button::Style {
            background: {
                let fill = builder.fill(tokens, status);
                if fill.a <= 0.0 {
                    None
                } else {
                    Some(Background::Color(fill))
                }
            },
            text_color: shown_ink,
            border: Border {
                color: builder.border_color(tokens),
                width: builder.border_width(),
                radius: theme::RADIUS_MD.into(),
            },
            shadow: Shadow::default(),
            // iced 0.14 snaps a button's content to whole pixels; the rest of the
            // catalog does not opt in, and a snapped label shifts by a pixel
            // against the text beside it.
            snap: false,
        }
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Button<'a, Message>) -> Self {
        let style = Style::new(control.variant)
            .outline(control.outline)
            .selected(control.selected)
            .loading(control.loading);
        let height = control.height();
        let padding = control.padding();
        let width = control.width;
        // Read before the name is shadowed by the widget below.
        let icon_only = control.icon_only();
        let disabled = control.on_press.is_none() || control.loading;
        let on_press = control.on_press.clone();
        // The content is centred down the control's height, and never across a
        // width it does not own: a `Length::Fill` here would make the button
        // measure as its padding alone, with the label spilling over whatever sits
        // beside it. An icon-only button is the exception, because its width is
        // fixed from its height rather than measured from the glyph.
        let content: Element<'a, Message> = if icon_only {
            container(control.content())
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            container(control.content()).center_y(Length::Fill).into()
        };

        let mut control = button(content)
            .height(Length::Fixed(height))
            .padding([0.0, padding])
            .style(move |theme: &Theme, status| {
                style.resolve(Tokens::of(theme), status)
            });

        if width != Length::Shrink {
            control = control.width(width);
        } else if icon_only {
            // An icon with no label is a square control, not a narrow one: the
            // width comes from the height the size already decided. Slint's
            // `Button` with only an `icon` is the same shape.
            control = control.width(Length::Fixed(height));
        }
        if !disabled {
            if let Some(message) = on_press {
                control = control.on_press(message);
            }
        }

        // `Length::Shrink`, so a button in a row keeps the height its size gives
        // it instead of being stretched to the tallest thing beside it — which is
        // what pushed every label to the top of its control.
        container(control).height(Length::Shrink).into()
    }
}

/// The pressable handle the form controls share.
///
/// `radio`, `switch` and `toggle` are one shape with three drawings. What they
/// share is how they behave: each reports the value it wants to become instead
/// of mutating its own state, and iced spells "cannot be pressed" as a control
/// with no message. What differs is the drawing and the value reported, so the
/// handle is built once, here, on iced's own [`button`]: a pressable element
/// with no padding or size of its own, whose content is the control's mark and
/// whose style is the chrome the control resolves.
///
/// The style stays a closure rather than a resolved value because iced reports
/// hover and press only to the element that owns the interaction, and this
/// element is it. iced 0.14 gives a bare button no focus ring and no Space or
/// Enter activation, so the ring and the keyboard path Slint's `FocusScope`
/// adds belong to the application.
pub fn handle<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    style: impl Fn(&Theme, button::Status) -> button::Style + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    button(content)
        .padding(0)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .on_press_maybe(on_press)
        .style(style)
        .into()
}
