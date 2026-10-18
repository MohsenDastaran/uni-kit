//! A step-by-step progress indicator. Mirrors GPUI Component's `Stepper` and
//! `StepperItem` and the Slint version beside it: steps up to the selected index
//! are checked and the connectors between passed steps turn primary, in a
//! horizontal or a vertical layout, with `text_center`, the four sizes, optional
//! icons and descriptions, and per-step or whole-stepper disabling.
//!
//! **This is a progress stepper, not a numeric one.** The control reports the
//! step that was chosen and the caller owns the selection; the increment and
//! decrement live beside it, in the Previous and Next buttons a caller composes,
//! exactly as the Slint example does. That stepping is still shared rather than
//! re-derived: [`step_index`] is built on
//! [`number_input::step_value`](super::number_input::step_value), the bounded
//! step `number_input`'s own steppers perform, so a stepper at its last step and
//! a numeric field at its maximum stop in the same place.
//!
//! **The pressable shape is [`button::handle`](super::button::handle).** Slint
//! covers the whole trigger — the indicator and the label under it — with one
//! `TouchArea`, and the handle is iced's equivalent: a pressable element with no
//! padding or size of its own whose content is the step's mark, so clicking the
//! number or the label beside it selects the same step. The indicator and the
//! text block are two handles rather than one because the connector line has to
//! start at the indicator and cross the item, which a single shrinking handle
//! around both cannot lay out.
//!
//! **What iced 0.14 cannot draw, and what is not faked:**
//!
//! - **No keyboard stepping.** Slint's stepper moves with the arrow keys while
//!   its `FocusScope` holds focus, skipping a disabled step in the direction of
//!   travel. iced reports focus to the element, not to the application, so there
//!   is no signal for the control to act on. The keys are the application's — the
//!   gallery wires them, as it does `number_input`'s — and [`step_index`] is the
//!   arithmetic they should use.
//! - **No hover or press fill on the indicator.** Slint lightens a non-checked
//!   indicator's `secondary` on hover and press; iced reports those states to the
//!   handle, whose content was already built, so the indicator is drawn from its
//!   checked state alone. The label, description and indicator still take their
//!   lower-contrast values when disabled, which iced cannot do as one subtree
//!   opacity.
//! - **No animated connector.** Slint animates the connector's fill over 150 ms;
//!   iced has no transition on a laid-out child, so it snaps, the same reading
//!   `switch`'s thumb takes.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Shadow, Theme, font};

use super::button::handle;
use super::number_input::{self, StepAction};
use super::theme::{self, ControlSize, Tokens};

/// The height of a vertical connector: Slint pads a vertical step's bottom by
/// `spacing-xl` (24), and the line runs through it.
const VERTICAL_CONNECTOR: f32 = theme::SPACING_XL;

/// The indicator's side for a size step: 8 / 24 / 32, with small at 18, Slint's
/// `indicator`. At xsmall it is a bare dot, because Slint draws neither icon nor
/// number there.
fn indicator_side(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 8.0,
        ControlSize::Small => 18.0,
        ControlSize::Large => 32.0,
        ControlSize::Medium => 24.0,
    }
}

/// The connector's thickness for a size step: 1.5 / 2 / 3 pixels, Slint's
/// `line`.
fn line_width(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 1.5,
        ControlSize::Large => 3.0,
        _ => 2.0,
    }
}

/// The icon's box inside the indicator: 12 / 14 / 18, Slint's `Icon` size.
fn icon_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::Small => 12.0,
        ControlSize::Large => 18.0,
        _ => 14.0,
    }
}

/// A step's label size: text-xs on the two small steps, text-sm otherwise.
fn label_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall | ControlSize::Small => theme::FONT_SIZE_XS,
        _ => theme::FONT_SIZE_SM,
    }
}

/// One step. The label and the description are strings the stepper draws; the
/// icon is the caller's element, because iced ships no icon set — the same rule
/// [`Button`](super::button::Button) follows.
pub struct StepperItem<'a, Message> {
    label: String,
    description: String,
    icon: Option<Element<'a, Message>>,
    disabled: bool,
}

impl<'a, Message> StepperItem<'a, Message> {
    /// An enabled step with no label. Slint's `StepperItem` defaults the same
    /// way; a label or a description is added with the builders below.
    pub fn new() -> Self {
        Self {
            label: String::new(),
            description: String::new(),
            icon: None,
            disabled: false,
        }
    }

    /// The step's name, drawn under the indicator.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// The step's supporting line, drawn under the label in the muted colour.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// The mark inside the indicator. Slint draws it only above the xsmall size;
    /// the element is the caller's, so it is also the caller that tints it.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Refuses this step alone, the way Slint's `item.disabled` does: the mark
    /// takes its muted values and a press on it selects nothing.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, Message> Default for StepperItem<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The index a step lands on, in the direction `action` names.
///
/// The clamp is [`number_input::step_value`], the bounded step `number_input`'s
/// steppers use: a step from the last index is `None`, and one that would cross
/// the end lands on it. The rule this adds is the one a stepper owns — an item in
/// the way that is disabled is passed over, still bounded, the way Slint's
/// `step-by` reads.
pub fn step_index(disabled: &[bool], selected: usize, action: StepAction) -> Option<usize> {
    let last = disabled.len().checked_sub(1)?;
    let mut from = selected as f64;
    loop {
        let next = number_input::step_value(from, action, 1.0, 0.0, last as f64)?;
        let index = next as usize;
        if !disabled.get(index).copied().unwrap_or(false) {
            return Some(index);
        }
        // The disabled step is stepped over, but the bound still stops the walk:
        // `step_value` returns `None` once it cannot move further.
        from = next;
    }
}

/// A step-by-step progress indicator, built with the methods below.
///
/// The selected index is the caller's: the stepper reports the step that was
/// pressed through [`on_change`](Self::on_change) and the caller writes it back,
/// so the previous and next actions a caller composes and the control itself
/// cannot hold two answers. A stepper built without `on_change` cannot be
/// selected, which is iced's spelling of disabled; [`disabled`](Self::disabled)
/// says the same for a control whose steps also draw their muted values.
pub struct Stepper<'a, Message> {
    items: Vec<StepperItem<'a, Message>>,
    selected: usize,
    size: ControlSize,
    disabled: bool,
    vertical: bool,
    text_center: bool,
    width: Length,
    on_change: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> Stepper<'a, Message> {
    /// A horizontal stepper with nothing selected. Slint's `selected-index`
    /// defaults to 1 and GPUI's `step` to 0; a caller names the step it means
    /// with [`selected_index`](Self::selected_index).
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
            size: ControlSize::default(),
            disabled: false,
            vertical: false,
            text_center: false,
            width: Length::Fill,
            on_change: None,
        }
    }

    /// A vertical stepper: the indicators run down the leading edge with the
    /// labels beside them.
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            ..Self::new()
        }
    }

    /// One step, in reading order.
    pub fn item(mut self, item: StepperItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// The steps, in reading order.
    pub fn items(mut self, items: impl IntoIterator<Item = StepperItem<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    /// The step that reads as current. Every step up to and including it is
    /// checked, which is Slint's `index <= selected-index`.
    pub fn selected_index(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Refuses every step, the way Slint's `enabled: false` does. The steps keep
    /// their ordinary colours; it is the whole control that is inert.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn text_center(mut self, text_center: bool) -> Self {
        self.text_center = text_center;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The step that was pressed. Without it the stepper is disabled, which is
    /// how iced spells a control that cannot be selected.
    pub fn on_change(mut self, on_change: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }
}

impl<'a, Message> Default for Stepper<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The transparent chrome the step handles wear. The drawing is the indicator
/// and the label, so the handle is a hit target and an application's extension
/// point rather than a surface of its own — the same shape `radio` uses.
fn chrome(tokens: Tokens, radius: f32) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: None,
        text_color: tokens.foreground,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: radius.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

/// A connector between two indicators: a filled rule on the horizontal axis of a
/// horizontal stepper, or down the indicator's leading edge of a vertical one.
///
/// `checked` is the one thing it reports: a connector that leads to a passed step
/// is `primary`, and one that still leads forward is `border`, the two colours
/// Slint's `index < selected-index` chooses between.
fn connector<'a, Message: 'a>(
    size: ControlSize,
    horizontal: bool,
    checked: bool,
) -> Element<'a, Message> {
    let thickness = line_width(size);
    let line = container(Space::new()).style(move |theme: &Theme| {
        let tokens = Tokens::of(theme);
        let color = if checked {
            tokens.primary
        } else {
            tokens.border
        };
        container::Style {
            background: Some(color.into()),
            border: iced::border::rounded(thickness / 2.0),
            ..container::Style::default()
        }
    });

    if horizontal {
        line.width(Length::Fill).height(thickness).into()
    } else {
        line.width(thickness).height(VERTICAL_CONNECTOR).into()
    }
}

/// The mark at a step's leading edge: the caller's icon, the step's number, or
/// — at xsmall — nothing, on a disc that is `primary` once the step is checked.
fn indicator<'a, Message: Clone + 'a>(
    number: usize,
    checked: bool,
    size: ControlSize,
    item_disabled: bool,
    icon: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let side = indicator_side(size);

    let mark: Element<'a, Message> = if size == ControlSize::XSmall {
        Space::new().into()
    } else if let Some(icon) = icon {
        icon
    } else {
        // The number's colour is resolved when the theme is known, so the mark
        // is built here and the palette read at draw time.
        text(number.to_string())
            .size(if size == ControlSize::Large {
                theme::FONT_SIZE_SM
            } else {
                theme::FONT_SIZE_XS
            })
            .font(iced::Font {
                weight: font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                let base = if checked {
                    tokens.primary_foreground
                } else {
                    tokens.secondary_foreground
                };
                text::Style {
                    color: Some(if item_disabled {
                        theme::alpha(base, 0.5)
                    } else {
                        base
                    }),
                }
            })
            .into()
    };

    container(mark)
        .width(side)
        .height(side)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            let fill = if checked {
                tokens.primary
            } else {
                tokens.secondary
            };
            container::Style {
                background: Some(
                    if item_disabled {
                        theme::alpha(fill, 0.5)
                    } else {
                        fill
                    }
                    .into(),
                ),
                border: iced::border::rounded(side / 2.0),
                ..container::Style::default()
            }
        })
        .into()
}

/// The label and description under or beside a step, in one transparent handle.
fn step_text<'a, Message: Clone + 'a>(
    label: String,
    description: String,
    size: ControlSize,
    item_disabled: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let mut block = column![].spacing(theme::SPACING_XXS);

    if !label.is_empty() {
        let size = label_size(size);
        block = block.push(
            text(label)
                .size(size)
                .font(iced::Font {
                    weight: font::Weight::Medium,
                    ..iced::Font::DEFAULT
                })
                .style(move |theme: &Theme| {
                    let tokens = Tokens::of(theme);
                    text::Style {
                        color: Some(if item_disabled {
                            tokens.muted_foreground
                        } else {
                            tokens.foreground
                        }),
                    }
                }),
        );
    }

    if !description.is_empty() {
        block = block.push(text(description).size(theme::FONT_SIZE_XS).style(
            move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                text::Style {
                    color: Some(if item_disabled {
                        theme::alpha(tokens.muted_foreground, 0.6)
                    } else {
                        tokens.muted_foreground
                    }),
                }
            },
        ));
    }

    handle(block, on_press, move |theme: &Theme, _status| {
        chrome(Tokens::of(theme), 0.0)
    })
}

/// One step of a horizontal stepper: the indicator and the connector that leads
/// to the next step on one line, the text beneath it.
#[allow(clippy::too_many_arguments)]
fn horizontal_step<'a, Message: Clone + 'a>(
    index: usize,
    count: usize,
    selected: usize,
    size: ControlSize,
    text_center: bool,
    item_disabled: bool,
    icon: Option<Element<'a, Message>>,
    label: String,
    description: String,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let checked = index <= selected;
    let side = indicator_side(size);
    let mark = handle(
        indicator(index + 1, checked, size, item_disabled, icon),
        on_press.clone(),
        move |theme: &Theme, _status| chrome(Tokens::of(theme), side / 2.0),
    );

    // The connector starts at the indicator and runs to the item's trailing
    // edge, where the next item's indicator begins. Without `text_center` the
    // indicator sits at the leading edge and only the trailing connector takes
    // the spare width; with it the indicator is centred, so the connector before
    // the first item and after the last is a blank of the same weight.
    let leading: Element<'a, Message> = if text_center && index > 0 {
        connector(size, true, index - 1 < selected)
    } else if text_center {
        Space::new().width(Length::Fill).into()
    } else {
        Space::new().into()
    };
    let trailing: Element<'a, Message> = if index + 1 < count {
        connector(size, true, index < selected)
    } else if text_center {
        Space::new().width(Length::Fill).into()
    } else {
        Space::new().into()
    };
    let top = row![leading, mark, trailing]
        .align_y(Alignment::Center)
        .width(Length::Fill);

    let body = step_text(label, description, size, item_disabled, on_press);

    column![top, body]
        .spacing(theme::SPACING_XS)
        .width(Length::Fill)
        .align_x(if text_center {
            Alignment::Center
        } else {
            Alignment::Start
        })
        .into()
}

/// One step of a vertical stepper: the indicator with its connector running down
/// the leading edge, the text beside it.
#[allow(clippy::too_many_arguments)]
fn vertical_step<'a, Message: Clone + 'a>(
    index: usize,
    count: usize,
    selected: usize,
    size: ControlSize,
    item_disabled: bool,
    icon: Option<Element<'a, Message>>,
    label: String,
    description: String,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let checked = index <= selected;
    let side = indicator_side(size);
    let mark = handle(
        indicator(index + 1, checked, size, item_disabled, icon),
        on_press.clone(),
        move |theme: &Theme, _status| chrome(Tokens::of(theme), side / 2.0),
    );

    let mut edge = column![mark].align_x(Alignment::Center).width(side);
    if index + 1 < count {
        edge = edge.push(connector(size, false, index < selected));
    }

    let body = step_text(label, description, size, item_disabled, on_press);

    row![edge, body]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Start)
        .into()
}

impl<'a, Message> From<Stepper<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(stepper: Stepper<'a, Message>) -> Self {
        let Stepper {
            items,
            selected,
            size,
            disabled,
            vertical,
            text_center,
            width,
            on_change,
        } = stepper;
        let count = items.len();

        let steps: Vec<Element<'a, Message>> = items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let StepperItem {
                    label,
                    description,
                    icon,
                    disabled: item_disabled,
                } = item;
                // A whole-stepper or per-step disable is the absence of a
                // message, so the handle is inert rather than ignored.
                let on_press = if disabled || item_disabled {
                    None
                } else {
                    on_change.as_ref().map(|on_change| on_change(index))
                };

                if vertical {
                    vertical_step(
                        index,
                        count,
                        selected,
                        size,
                        item_disabled,
                        icon,
                        label,
                        description,
                        on_press,
                    )
                } else {
                    horizontal_step(
                        index,
                        count,
                        selected,
                        size,
                        text_center,
                        item_disabled,
                        icon,
                        label,
                        description,
                        on_press,
                    )
                }
            })
            .collect();

        if vertical {
            column(steps).spacing(theme::SPACING_XS).width(width).into()
        } else {
            row(steps).align_y(Alignment::Start).width(width).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StepAction, step_index};

    /// The bounded step a stepper takes, and the disabled step it passes over.
    #[test]
    fn test_step_index() {
        let open = [false, false, false];
        assert_eq!(step_index(&open, 0, StepAction::Increment), Some(1));
        assert_eq!(step_index(&open, 2, StepAction::Increment), None);
        assert_eq!(step_index(&open, 0, StepAction::Decrement), None);
        assert_eq!(step_index(&open, 2, StepAction::Decrement), Some(1));

        // A disabled step is stepped over, but the last bound still stops the
        // walk: from 0 with 1 disabled, the step lands on 2; from 2 it cannot.
        let gapped = [false, true, false];
        assert_eq!(step_index(&gapped, 0, StepAction::Increment), Some(2));
        assert_eq!(step_index(&gapped, 2, StepAction::Decrement), Some(0));
        assert_eq!(step_index(&gapped, 2, StepAction::Increment), None);
        assert_eq!(step_index(&gapped, 0, StepAction::Decrement), None);
        assert_eq!(step_index(&[], 0, StepAction::Increment), None);
    }
}
