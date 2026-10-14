//! A tab bar over the active tab's content. Mirrors GPUI Component's `TabBar`
//! and the Slint version beside it: the underline, pill and outline variants,
//! sizes, an optional leading icon per tab, a disabled tab, and the selected
//! tab's label in the heavier weight.
//!
//! The registry key is `tabs`; the file and the module share the name.
//!
//! **iced has no tabs widget.** A tab bar is a `row` of controls with a
//! selected state, and the body is the caller's content for the active tab —
//! which is exactly the shape the Slint component has under its own markup. So
//! this component renders the bar, the underline rule below it and the body
//! slot; the caller owns `selected` and passes the body for that index.
//!
//! The metrics are the Slint ones. The underline variant has no gap and no
//! outer padding, the other two have 4px of each. Each tab is padded
//! 6px vertically and by the size's control padding horizontally — 8px at
//! small, 12px at medium by default, 16px at large — with a 6px gap between an
//! icon and its label. A selected pill is filled with `primary`, so its label
//! takes `primary-foreground`; the other variants take the foreground. An
//! unselected label is the muted foreground, and a disabled one is the muted
//! foreground at half strength.
//!
//! The bar flicks sideways in Slint when the labels do not fit. iced scrolls
//! only through `scrollable`, so the caller wraps the bar in one if it wants
//! that; the bar itself is a plain row and lets its labels elide.
//!
//! An icon is the caller's element, because the icon set belongs to the
//! application; the built-in tab shows only its label.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, column, container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// How a tab is drawn. The names and meanings are the Slint enum's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// A rule under the selected tab, with the bar's own 1px rule below it.
    #[default]
    Underline,
    /// A capsule filled with `primary` when selected.
    Pill,
    /// A small radius filled with `muted` when selected.
    Outline,
}

/// One tab: its label, an optional leading element and whether it can be
/// chosen.
pub struct Tab<'a, Message> {
    label: String,
    icon: Option<Element<'a, Message>>,
    disabled: bool,
    on_press: Option<Message>,
}

impl<'a, Message> Tab<'a, Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
            on_press: None,
        }
    }

    /// A leading element, drawn before the label with a 6px gap.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// A tab that cannot be chosen. An iced control is also disabled by the
    /// absence of a message, so a tab with no `on_press` is disabled whatever
    /// this says — the flag is what marks a tab disabled that does have one.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The message the tab reports when it is chosen.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

/// A tab bar, built with the methods below.
pub struct Tabs<'a, Message> {
    tabs: Vec<Tab<'a, Message>>,
    selected: usize,
    variant: Variant,
    size: ControlSize,
    body: Option<Element<'a, Message>>,
}

impl<'a, Message> Tabs<'a, Message> {
    pub fn new(tabs: impl IntoIterator<Item = Tab<'a, Message>>) -> Self {
        Self {
            tabs: tabs.into_iter().collect(),
            selected: 0,
            variant: Variant::default(),
            size: ControlSize::default(),
            body: None,
        }
    }

    /// Which tab is drawn as selected. The caller's state decides.
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The active tab's content, drawn below the bar with a 16px inset above
    /// it, which is what the Slint component does with its `@children`.
    pub fn body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.body = Some(body.into());
        self
    }
}

/// A tab's label colour for its state, which is the whole rule the Slint
/// `tab-fg` private property encodes.
pub fn ink(tokens: Tokens, variant: Variant, selected: bool, disabled: bool) -> Color {
    if disabled {
        return theme::alpha(tokens.muted_foreground, 0.5);
    }
    if selected {
        return match variant {
            Variant::Pill => tokens.primary_foreground,
            _ => tokens.foreground,
        };
    }
    tokens.muted_foreground
}

/// A tab's fill for its state and the pointer over it.
pub fn fill(tokens: Tokens, variant: Variant, selected: bool, hovered: bool) -> Color {
    if selected {
        return match variant {
            Variant::Pill => tokens.primary,
            Variant::Underline => Color::TRANSPARENT,
            Variant::Outline => tokens.muted,
        };
    }
    if hovered {
        tokens.secondary
    } else {
        Color::TRANSPARENT
    }
}

impl<'a, Message: Clone + 'a> From<Tabs<'a, Message>> for Element<'a, Message> {
    fn from(control: Tabs<'a, Message>) -> Self {
        let variant = control.variant;
        let size = control.size;
        let selected = control.selected;
        let selected_message = control.tabs.get(selected).and_then(|tab| tab.on_press.clone());

        let (spacing, padding) = match variant {
            Variant::Underline => (0.0, 0.0),
            _ => (4.0, 4.0),
        };
        let control_pad = match size {
            ControlSize::XSmall => 6.0,
            ControlSize::Small => 8.0,
            ControlSize::Large => 16.0,
            ControlSize::Medium => 12.0,
        };
        let label_size = match size {
            ControlSize::XSmall => theme::FONT_SIZE_XS,
            ControlSize::Small => theme::FONT_SIZE_SM,
            _ => theme::FONT_SIZE_MD,
        };

        let mut bar = row![].spacing(spacing).padding(padding).align_y(Alignment::Center);

        for (index, tab) in control.tabs.into_iter().enumerate() {
            let is_selected = index == selected;
            let disabled = tab.disabled || tab.on_press.is_none();

            // The underline rule sits in the tab's own bottom 2px, where Slint
            // draws it (`y: parent.height - 2px`), so the padding gives it that
            // space rather than the rule adding height to the selected tab.
            let bottom = if variant == Variant::Underline { 4.0 } else { 6.0 };
            let mut label_row = row![]
                .spacing(6.0)
                .align_y(Alignment::Center)
                .padding(iced::Padding {
                    top: 6.0,
                    right: control_pad,
                    bottom,
                    left: control_pad,
                });
            if let Some(icon) = tab.icon {
                label_row = label_row.push(icon);
            }
            label_row = label_row.push(
                text(tab.label)
                    .size(label_size)
                    .font(iced::Font {
                        weight: if is_selected {
                            iced::font::Weight::Semibold
                        } else {
                            iced::font::Weight::Medium
                        },
                        ..iced::Font::DEFAULT
                    })
                    .style(move |theme: &Theme| text::Style {
                        color: Some(ink(Tokens::of(theme), variant, is_selected, disabled)),
                    }),
            );

            // The rule is inside the control, not beside it, so the whole tab —
            // rule included — is one hit box and one hover fill. Unselected tabs
            // draw a transparent rule of the same height, which keeps every
            // label on one baseline.
            let mut content: Element<'a, Message> = label_row.into();
            if variant == Variant::Underline {
                content = column![
                    content,
                    container(iced::widget::Space::new())
                        .width(Length::Fill)
                        .height(Length::Fixed(2.0))
                        .style(move |theme: &Theme| ContainerStyle {
                            background: is_selected
                                .then(|| Tokens::of(theme).primary.into()),
                            ..ContainerStyle::default()
                        }),
                ]
                .spacing(0.0)
                .into();
            }

            let mut control = button(content)
                .padding(0.0)
                .style(move |theme: &Theme, status| {
                    let tokens = Tokens::of(theme);
                    let hovered = matches!(
                        status,
                        button::Status::Hovered | button::Status::Pressed
                    ) && !disabled;
                    let fill = fill(tokens, variant, is_selected, hovered);
                    button::Style {
                        background: if fill.a > 0.0 {
                            Some(Background::Color(fill))
                        } else {
                            None
                        },
                        text_color: ink(tokens, variant, is_selected, disabled),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: match variant {
                                Variant::Underline => 0.0.into(),
                                Variant::Outline => theme::RADIUS_SM.into(),
                                // A pill is a capsule: half its own height.
                                Variant::Pill => (theme::FONT_SIZE_MD / 2.0 + 12.0).into(),
                            },
                        },
                        ..button::Style::default()
                    }
                });

            if !disabled {
                if let Some(message) = tab.on_press {
                    control = control.on_press(message);
                }
            }

            bar = bar.push(control);
        }

        let mut sections = column![
            container(bar).width(Length::Fill),
        ]
        .width(Length::Fill);

        if variant == Variant::Underline {
            sections = sections.push(
                container(iced::widget::Space::new())
                    .width(Length::Fill)
                    .height(Length::Fixed(1.0))
                    .style(|theme: &Theme| ContainerStyle {
                        background: Some(Tokens::of(theme).border.into()),
                        ..ContainerStyle::default()
                    }),
            );
        }

        if let Some(body) = control.body {
            sections = sections.push(
                column![body]
                    .width(Length::Fill)
                    .padding(iced::Padding {
                        top: 16.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    }),
            );
        }

        let _ = selected_message;
        sections.into()
    }
}
