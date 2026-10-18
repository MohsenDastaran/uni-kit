//! A numeric field with steppers. Mirrors GPUI Component's `NumberInput` and the
//! Slint version beside it: a value, a minimum and a maximum, a step, the prefix
//! and suffix slots, the invalid and disabled states, the four sizes and the
//! stepper shape, with the same fills and edges.
//!
//! Slint's `NumberInput` composes `Input` and two ghost extra-small `Button`s, and
//! so does this one: the ordinary control is [`Input`] with the two steppers in its
//! suffix slot, so the frame, the padding, the size ladder and the danger edge are
//! the field's own and there is no second copy of them. `centered` is the one shape
//! the field cannot draw itself — the value is centred and a stepper moves to
//! either end — so the port builds that frame with [`input::Style`], the same
//! style the field resolves, around a borderless [`Input`].
//!
//! **iced owns no numeric value, so the caller owns two.** `value` is the number
//! the steppers step from; `text` is the caller's editing buffer and what the
//! field shows. A control with `text` renders it verbatim, even empty, because a
//! partly typed number like `1.` is a state the field has to keep; a control
//! without it renders `value` the way Slint's `shown` does
//! (`round(value * 100) / 100`), which is [`shown`]. A step parses `text` when it
//! is a number and falls back to `value`, clamps into the range, and reports the
//! result through `on_change`, so a caller that writes the message back keeps the
//! invariant Slint holds internally.
//!
//! **Four Slint behaviours have no iced 0.14 counterpart, and none is faked:**
//!
//! - **The frame shows no focus ring and no hover edge**, for the reason `input`
//!   gives: a container resolves its style from the theme alone, and only the
//!   `text_input` inside knows its own status.
//! - **The keyboard is the application's.** Slint's field forwards the keys the text
//!   field does not handle — `↑`, `↓`, `Escape` — through `key-pressed`, and
//!   `clean-on-escape` clears on Escape. iced's field reports text and submit only,
//!   so the arrows and Escape are an application key subscription; the gallery shows
//!   one and says so. iced reports no focus either, so that subscription cannot know
//!   which field is focused.
//! - **`value` cannot be in-out.** Slint writes the clamped value back into the
//!   property and rewrites the text; iced's caller writes the message back instead,
//!   which is why [`shown`] and [`step_value`] are public — they are the two halves
//!   the component uses, and the two halves a programmatic step needs.
//! - **Slint's one `changed` is two callbacks here.** `changed` fires while typing
//!   and after a step; the caller has to tell those apart, because a step rewrites
//!   the text and typing must not. `on_input` carries the text, `on_change` the
//!   value a step produced.
//!
//! The height ladder is the field's (`input-height`: 20 / 24 / 32 / 44). Slint's
//! `NumberInput` declares `control-height` (40 at large) and puts a 44-pixel field
//! inside it, so at large the port follows the field it composes and the GPUI
//! original rather than the Slint frame.
//!
//! Two glyphs are drawn here rather than taken from the caller: a number input's
//! steppers are the control's own affordance, not the application's decoration, and
//! both Slint and GPUI draw them from their own icon sets. [`MINUS`] and [`PLUS`]
//! are the Lucide marks those sets resolve to.
//!
//! One consequence of composing rather than copying: this file reads `input` and
//! `button` beside `theme`, where the catalog's other components read `theme`
//! alone. Slint's `NumberInput` composes the same two, so a registry entry for it
//! has to carry `input.rs` and `button.rs` as well; the manifest builder reads a
//! component's crates but not its siblings yet.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{Id, container, row, svg};
use iced::{Alignment, Element, Length, Theme};

use super::button::{self, Button, Variant};
use super::input::{self, Input};
use super::theme::{self, ControlSize, Tokens};

/// The Lucide `minus` glyph Slint's `Button { icon: "minus" }` resolves to, drawn
/// at the extra-small icon box.
pub const MINUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/></svg>"#;

/// The Lucide `plus` glyph Slint's `Button { icon: "plus" }` resolves to.
pub const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

/// Slint's default `minimum: -1000000000`.
const DEFAULT_MINIMUM: f64 = -1_000_000_000.0;
/// Slint's default `maximum: 1000000000`.
const DEFAULT_MAXIMUM: f64 = 1_000_000_000.0;

/// Which way a stepper moves the value. GPUI's `StepAction`, and what
/// [`step_value`] takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAction {
    Increment,
    Decrement,
}

/// The value a stepper produces, or `None` when it cannot move the value in the
/// pressed direction.
///
/// This is the arithmetic the two buttons perform, exposed because iced has no
/// in-out value for a programmatic step to write: the application steps its own
/// number, clamping the same way. A step that would cross a bound lands on the
/// bound; a step that cannot move at all — increment at the maximum, or decrement
/// on a value already below the minimum — is `None`, which is also what disables
/// the button, so a press does nothing rather than moving the wrong way.
pub fn step_value(
    from: f64,
    action: StepAction,
    step: f64,
    minimum: f64,
    maximum: f64,
) -> Option<f64> {
    let candidate = match action {
        StepAction::Increment => from + step,
        StepAction::Decrement => from - step,
    };
    // A bound given the wrong way round is clamped rather than panicked on: the
    // Slint `clamp` would not have caught it either.
    let low = minimum.min(maximum);
    let high = maximum.max(minimum);
    let clamped = candidate.clamp(low, high);
    let moved = match action {
        StepAction::Increment => clamped > from,
        StepAction::Decrement => clamped < from,
    };

    moved.then_some(clamped)
}

/// A value as the field writes it: Slint's `shown`, `round(value * 100) / 100`.
///
/// Two fraction digits are what Slint's stepper shape keeps, and rounding is what
/// stops a step of `0.1` from printing `0.30000000000000004`. The application
/// formats its own values with it so a stepped number and a typed one read the
/// same.
pub fn shown(value: f64) -> String {
    format!("{}", (value * 100.0).round() / 100.0)
}

/// A numeric field with steppers, built with the methods below.
///
/// The `id` is what iced keeps the field's editing state under — its caret, its
/// selection and its focus — exactly as it is for [`Input`]. It has to be stable
/// across renders, so it is the field's name rather than its value.
pub struct NumberInput<'a, Message> {
    id: Id,
    text: Option<String>,
    value: f64,
    minimum: f64,
    maximum: f64,
    step: f64,
    placeholder: String,
    size: ControlSize,
    disabled: bool,
    invalid: bool,
    centered: bool,
    prefix: Option<Element<'a, Message>>,
    suffix: Option<Element<'a, Message>>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_change: Option<Box<dyn Fn(f64) -> Message + 'a>>,
    on_submit: Option<Message>,
    width: Length,
}

impl<'a, Message> NumberInput<'a, Message> {
    /// A field iced keeps its state under, named `id` — `NumberInput::new("quantity")`.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            text: None,
            value: 0.0,
            minimum: DEFAULT_MINIMUM,
            maximum: DEFAULT_MAXIMUM,
            step: 1.0,
            placeholder: String::new(),
            size: ControlSize::default(),
            disabled: false,
            invalid: false,
            centered: false,
            prefix: None,
            suffix: None,
            on_input: None,
            on_change: None,
            on_submit: None,
            width: Length::Fill,
        }
    }

    /// The number the steppers step from, and the text the field shows when
    /// [`text`](Self::text) is not set.
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// The text the field shows — the caller's editing buffer. It takes precedence
    /// over [`value`](Self::value) so a half-typed number survives the edit, and an
    /// empty one is a legitimate state rather than a fall back to `value`.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// The lower bound a step clamps to, and where the decrement stepper disables.
    pub fn minimum(mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self
    }

    /// The upper bound a step clamps to, and where the increment stepper disables.
    pub fn maximum(mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self
    }

    /// How far one press moves the value.
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Refuses input, draws the frame muted and leaves both steppers disabled, the
    /// way Slint's `enabled: false` does. A control with no
    /// [`on_input`](Self::on_input) is disabled without this.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Reports the value as rejected: the frame's edge takes the danger colour.
    /// Slint's `NumberInput` has no such property; its field does, and the caller
    /// is the one that knows a text is not a number.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Centres the value and moves the steppers to either end, so the control reads
    /// `− 3 +` instead of `3 − +`. Slint's `centered`: the frame is then this
    /// component's own and the field inside drops its own.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// A leading element inside the frame: a currency mark, a glyph.
    pub fn prefix(mut self, prefix: impl Into<Element<'a, Message>>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// A trailing element inside the frame, before the steppers: a unit, a glyph.
    pub fn suffix(mut self, suffix: impl Into<Element<'a, Message>>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Every edit, as the text the field now holds — a partial number included.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// The value a stepper produced, already clamped to the range. Slint's
    /// `changed` fires while typing as well; iced reports a typed value as text
    /// through [`on_input`](Self::on_input), because a step rewrites the text and
    /// typing must not.
    pub fn on_change(mut self, on_change: impl Fn(f64) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// The message Enter sends. The component never sees the submitted text, so
    /// clamping a typed value on commit is the caller's, exactly as it is with
    /// [`Input`].
    pub fn on_submit(mut self, on_submit: Message) -> Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The number the steppers work from: the text when it is a number, and the
    /// `value` when it is empty, partial or not a number at all — Slint's `value`,
    /// which a call to `typed` leaves alone in exactly those cases.
    fn current(&self) -> f64 {
        match &self.text {
            Some(text) => text
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
                .unwrap_or(self.value),
            None => self.value,
        }
    }

    /// The text the field shows: the caller's buffer, or the formatted value.
    fn display(&self) -> String {
        self.text.clone().unwrap_or_else(|| shown(self.value))
    }
}

/// The button that carries a stepper glyph. A ghost extra-small control is what
/// Slint's `Button { variant: ghost, size: xsmall }` draws, and the catalogue's
/// [`Button`] is the same control, so the two steppers are those rather than a
/// second button style written here.
///
/// The glyph is muted when there is no message to send, which is where a stepper
/// at a bound ends up. The catalogue's ghost button keeps its label ink in the
/// disabled state — it only drops the fill — and a bound that looks like every
/// other value is a state the reader cannot see, which is the one thing Slint's
/// `enabled: root.value > root.minimum` exists to show.
fn stepper<'a, Message>(glyph: &'static [u8], on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let edge = ControlSize::XSmall.icon_size();
    let muted = on_press.is_none();
    let icon: Element<'a, Message> = svg(svg::Handle::from_memory(glyph))
        .width(edge)
        .height(edge)
        .style(move |theme: &Theme, _status| {
            let tokens = Tokens::of(theme);
            let color = if muted {
                tokens.muted_foreground
            } else {
                button::ink(tokens, Variant::Ghost, false)
            };
            svg::Style { color: Some(color) }
        })
        .into();

    let mut control = Button::new("")
        .variant(Variant::Ghost)
        .size(ControlSize::XSmall)
        .icon(icon);

    // A button with no message is how iced draws a disabled one, which is
    // exactly Slint's `enabled: root.value > root.minimum`.
    if let Some(message) = on_press {
        control = control.on_press(message);
    }

    control.into()
}

impl<'a, Message> From<NumberInput<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: NumberInput<'a, Message>) -> Self {
        // Read before the fields are moved out: the two questions Slint asks of
        // its own state are what a step and a render both need.
        let current = control.current();
        let display = control.display();

        let NumberInput {
            id,
            minimum,
            maximum,
            step,
            placeholder,
            size,
            disabled,
            invalid,
            centered,
            prefix,
            suffix,
            on_input,
            on_change,
            on_submit,
            width,
            ..
        } = control;

        let down = step_value(current, StepAction::Decrement, step, minimum, maximum);
        let up = step_value(current, StepAction::Increment, step, minimum, maximum);
        let decrement = if disabled {
            None
        } else {
            on_change.as_ref().and_then(|change| down.map(|value| change(value)))
        };
        let increment = if disabled {
            None
        } else {
            on_change.as_ref().and_then(|change| up.map(|value| change(value)))
        };

        let field_disabled = disabled || on_input.is_none();
        let decrement_button = stepper(MINUS, decrement);
        let increment_button = stepper(PLUS, increment);

        if centered {
            let mut field = Input::new(id)
                .value(display)
                .placeholder(placeholder)
                .size(size)
                .invalid(invalid)
                .disabled(field_disabled)
                .appearance(false)
                .align_x(Alignment::Center)
                .width(Length::Fill);

            if let Some(prefix) = prefix {
                field = field.prefix(prefix);
            }
            if let Some(suffix) = suffix {
                field = field.suffix(suffix);
            }
            if !field_disabled {
                if let Some(on_input) = on_input {
                    field = field.on_input(move |value| on_input(value));
                }
                if let Some(on_submit) = on_submit {
                    field = field.on_submit(on_submit);
                }
            }

            // The same style the field resolves, so the frame a `centered` control
            // draws and the frame an ordinary one draws cannot drift apart.
            let style = input::Style::new().invalid(invalid).disabled(field_disabled);
            let contents = row![decrement_button, field, increment_button]
                .spacing(theme::SPACING_XXS)
                .align_y(Alignment::Center)
                .padding([0.0, theme::SPACING_XXS]);

            return container(contents)
                .width(width)
                .style(move |theme: &Theme| style.resolve(Tokens::of(theme)))
                .into();
        }

        // The steppers go in the field's suffix slot, which is where Slint's
        // `@children` land: inside the frame, after the caller's own suffix, with
        // the field's own row gap between the two.
        let gap = match size {
            ControlSize::XSmall | ControlSize::Small => theme::SPACING_XS,
            _ => theme::SPACING_SM,
        };
        let steppers = row![decrement_button, increment_button]
            .spacing(theme::SPACING_XXS)
            .align_y(Alignment::Center);
        let trailing: Element<'a, Message> = match suffix {
            Some(suffix) => row![suffix, steppers]
                .spacing(gap)
                .align_y(Alignment::Center)
                .into(),
            None => steppers.into(),
        };

        let mut field = Input::new(id)
            .value(display)
            .placeholder(placeholder)
            .size(size)
            .invalid(invalid)
            .suffix(trailing)
            .width(width);

        if let Some(prefix) = prefix {
            field = field.prefix(prefix);
        }
        if !field_disabled {
            if let Some(on_input) = on_input {
                field = field.on_input(move |value| on_input(value));
            }
            if let Some(on_submit) = on_submit {
                field = field.on_submit(on_submit);
            }
        }

        field.into()
    }
}

#[cfg(test)]
mod tests {
    use super::{StepAction, shown, step_value};

    /// The stepping rules GPUI's `test_step_value` pins down, on the numbers this
    /// control actually carries. A step that would cross a bound lands on it; a
    /// step that cannot move in the pressed direction is a no-op, so a button at
    /// the bound sends nothing instead of moving the wrong way.
    #[test]
    fn test_step_value() {
        // Plain steps in each direction.
        assert_eq!(step_value(1.0, StepAction::Increment, 1.0, -10.0, 10.0), Some(2.0));
        assert_eq!(step_value(1.0, StepAction::Decrement, 1.0, -10.0, 10.0), Some(0.0));
        assert_eq!(step_value(3.0, StepAction::Increment, 2.0, 0.0, 99.0), Some(5.0));

        // At a bound the step lands on the bound, and then cannot move at all.
        assert_eq!(step_value(99.5, StepAction::Increment, 1.0, 0.0, 100.0), Some(100.0));
        assert_eq!(step_value(100.0, StepAction::Increment, 5.0, 0.0, 100.0), None);
        assert_eq!(step_value(0.0, StepAction::Decrement, 1.0, 0.0, 100.0), None);

        // A value already outside the range moves back toward it, never further
        // out: this is the no-op the Slint and GPUI originals both describe.
        assert_eq!(step_value(1000.0, StepAction::Increment, 1.0, 0.0, 100.0), None);
        assert_eq!(step_value(1000.0, StepAction::Decrement, 1.0, 0.0, 100.0), Some(100.0));
        assert_eq!(step_value(5.0, StepAction::Decrement, 1.0, 10.0, 99.0), None);

        // A fractional step keeps its two-decimal reading, and a wrong-way-round
        // pair of bounds is clamped rather than panicked on.
        assert_eq!(shown(step_value(0.1, StepAction::Increment, 0.2, 0.0, 1.0).unwrap()), "0.3");
        assert_eq!(step_value(20.0, StepAction::Increment, 1.0, 10.0, 0.0), None);
    }

    #[test]
    fn test_shown() {
        assert_eq!(shown(3.0), "3");
        assert_eq!(shown(19.99), "19.99");
        assert_eq!(shown(1250.0), "1250");
        // 0.1 + 0.2 must not read as 0.30000000000000004.
        assert_eq!(shown(0.1 + 0.2), "0.3");
    }
}
