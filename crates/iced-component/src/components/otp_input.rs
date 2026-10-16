//! A one-time-password field: a row of single-character cells over iced's own
//! text field. Mirrors the Slint `OtpInput` beside it — the cell size from the
//! control ladder, the ring on the cell that holds the caret, the numeric value,
//! the auto-advance, the backspace that steps back and the masked cells — with
//! the caller owning the code, the way [`Input`](super::input::Input) does.
//!
//! Slint stores one string per cell because a Slint string cannot be sliced; an
//! iced value is one `String`, so this port takes the code as the caller's value
//! — `OtpInput::new("code").value(&code).length(6)` — and reads each cell out of
//! it by index. The component owns the cell edit: it keeps the digits an edit
//! carries, spreads a paste over the cells after this one, clears and steps back
//! on Backspace, and stops at the code's length. It hands the caller an [`Edit`]
//! with the code that resulted, the cell that was edited and the cell the caret
//! belongs on, so moving focus is `otp_input::focus` returned from `update`.
//!
//! A cell is iced's field, not the [`input`](super::input) component's frame
//! around it. A `container` resolves its style from the theme alone and cannot
//! see whether the field inside has focus — the gap `input` names — and the ring
//! on the focused cell is the one thing an OTP cell is asked to show. So the cell
//! *is* the field, its border is the field's own border, and `Status::Focused` is
//! what colours it. Fill, edge and text are still `input`'s rules: they come from
//! [`input::Style::resolve`], so the two controls cannot drift apart.
//!
//! Three things the Slint cell does cannot mean the same here, and none is
//! faked:
//!
//! - **A cell reports no focus and no blur**, so `InputEvent::Focus`/`Blur` and
//!   the "clear on focus" pattern are the caller's. Programmatic focus — what
//!   [`focus`] gives — is the port's focus movement.
//! - **A key is not an event iced hands out.** Backspace is still observable,
//!   because iced's field publishes the text it holds after one, empty included,
//!   which is enough for the step back. An arrow key is not, so the page's
//!   arrow-key navigation across cells has no iced expression and is absent
//!   rather than guessed at; Tab still walks the cells, because iced moves focus
//!   between focusable fields itself.
//! - **Slint keeps a hole where a middle cell was cleared**, because its `digits`
//!   array is positional. An iced value is one code, so clearing a cell in the
//!   middle closes the gap and the cells to its right move left. The code reads
//!   the same either way; only the cells move.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::font::Weight;
use iced::widget::operation;
use iced::widget::{Id, row, text_input};
use iced::{
    Alignment, Background, Border, Color, Element, Font, Length, Padding, Pixels, Task, Theme,
};

use super::input;
use super::theme::{self, ControlSize, Tokens};

/// A one-time code, built with the methods below.
///
/// The `id` names the field and every cell's `Id` is built from it — `"code"`
/// gives `code-0` … — so it takes a `String` rather than an `iced::widget::Id`,
/// which does not give its name back. The caller owns the code: it sets
/// [`value`](Self::value) from its own state and takes every edit back through
/// [`on_edit`](Self::on_edit).
pub struct OtpInput<'a, Message> {
    id: String,
    value: String,
    length: usize,
    size: ControlSize,
    masked: bool,
    disabled: bool,
    on_edit: Option<Rc<dyn Fn(Edit) -> Message + 'a>>,
}

impl<'a, Message> OtpInput<'a, Message> {
    /// A six-cell code, iced keeps each cell's editing state under — the cells
    /// are `<id>-0` … `<id>-5`, and [`length`](Self::length) changes how many
    /// are drawn.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            // Slint's `digits` starts with six entries; so does a code.
            length: 6,
            size: ControlSize::default(),
            masked: false,
            disabled: false,
            on_edit: None,
        }
    }

    /// The code the cells show, one character per cell. The caller owns it: it
    /// sets this from its own state and takes the edits back through
    /// [`on_edit`](Self::on_edit).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// How many cells the code has: Slint's `digits.length`, six by default.
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Hides every cell's character, the way Slint's `masked` does.
    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    /// The size ladder the cells take their side and their type from: 20 / 24 /
    /// 32 / 40, which is Slint's `control-height`, not the field's
    /// `input-height`.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Refuses input and draws muted, the way Slint's `enabled: false` does. A
    /// field with no [`on_edit`](Self::on_edit) is disabled without this.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Every cell edit, as an [`Edit`]: which cell reported, the code it left
    /// behind, and where the caret belongs. Slint's `edited(index, text)` is the
    /// same report with the code still in pieces.
    pub fn on_edit(mut self, on_edit: impl Fn(Edit) -> Message + 'a) -> Self {
        self.on_edit = Some(Rc::new(on_edit));
        self
    }
}

/// The `Id` of one cell of the field named `id`, so an application can address
/// it — [`focus`] is the one that moves the caret.
pub fn cell_id(id: &str, index: usize) -> Id {
    Id::from(format!("{id}-{index}"))
}

/// Puts the caret in one cell, and selects what is in it so the next character
/// replaces it instead of appending to it — which is what Slint's cell does when
/// it takes focus, and what makes a typed character land in the cell rather than
/// after the one already there.
///
/// The Task is returned from `update`; natively and in the browser it is the
/// same widget operation.
pub fn focus<Message: 'static>(id: &str, index: usize) -> Task<Message> {
    let cell = cell_id(id, index);

    Task::batch([
        operation::focus(cell.clone()),
        operation::select_all(cell),
    ])
}

/// One cell edit, as the field reports it.
///
/// The fields are private and read through the methods, so a later field can be
/// added without breaking a caller.
#[derive(Debug, Clone)]
pub struct Edit {
    index: usize,
    text: String,
    value: String,
    focus: Option<usize>,
    complete: bool,
    refused: bool,
}

impl Edit {
    /// The cell that was edited. Slint reports the cell that changed, so a
    /// Backspace in an empty cell reports the cell before it — the one it
    /// cleared.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The edited cell's content now: one character, or empty when it was
    /// cleared. Slint's `edited(index, text)` carries the same text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The whole code after the edit — what the caller stores and feeds back
    /// through [`value`](OtpInput::value).
    pub fn value(&self) -> &str {
        &self.value
    }

    /// The cell the caret belongs on, if the edit moved it. The caller returns
    /// [`focus`] with it from `update`.
    pub fn focus(&self) -> Option<usize> {
        self.focus
    }

    /// Whether every cell is filled — the page's `InputEvent::Complete`. iced
    /// has no event to emit, so completion is a read of the code the caller was
    /// handed.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Whether the input was refused because it carried no digit. The page's
    /// numeric-only rule: the code does not move, and the cell keeps the caret
    /// with its content selected.
    pub fn is_refused(&self) -> bool {
        self.refused
    }
}

/// The colours a cell draws with.
///
/// It is the field's own style — fill, edge, text — rather than the container
/// [`input`](super::input) draws around its field, because the cell is the
/// field. `pub` so an application can draw a matching cell of its own.
pub struct Style {
    pub disabled: bool,
    pub focused: bool,
}

impl Style {
    pub fn new() -> Self {
        Self {
            disabled: false,
            focused: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The ring on the cell that holds the caret. iced's field knows its own
    /// focus, which is why the cell can draw it where the `input` frame cannot.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// The fill and the edge: the `input` component's own rules, so a cell and a
    /// field beside it are the same surface, with the ring on the edge of the
    /// focused one.
    pub fn resolve(&self, tokens: Tokens) -> text_input::Style {
        let frame = input::Style::new().disabled(self.disabled).resolve(tokens);

        text_input::Style {
            background: frame
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border: if self.focused && !self.disabled {
                Border {
                    color: tokens.ring,
                    ..frame.border
                }
            } else {
                frame.border
            },
            icon: tokens.muted_foreground,
            placeholder: tokens.muted_foreground,
            value: frame.text_color.unwrap_or(tokens.foreground),
            selection: theme::alpha(tokens.selection, 0.35),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// The cell's side, from the Slint theme's `control-height`: 20 / 24 / 32 / 40.
/// Not `input-height` (20 / 24 / 32 / 44): the Slint `OtpCell` takes its size
/// from the control ladder, and a cell is square.
fn cell_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 20.0,
        ControlSize::Small => 24.0,
        ControlSize::Large => 40.0,
        ControlSize::Medium => 32.0,
    }
}

/// The code, the cell that was edited and the caret, after cell `index` reported
/// the text `raw` it now holds.
///
/// Slint's cell does this across six copies of itself; one code needs it once.
fn edit(value: &str, length: usize, index: usize, raw: &str) -> Edit {
    let mut cells: Vec<char> = value.chars().collect();
    let old = cells.get(index).copied();
    let last = length.saturating_sub(1);

    // The digits the field reports. A cell holds one character, and iced appends
    // to a cell that already had one; Slint selects the cell's content when it
    // takes focus so the same keystroke replaces it, and dropping the character
    // that was already there leaves the same thing behind.
    let mut digits: Vec<char> = raw.chars().filter(|character| character.is_ascii_digit()).collect();
    if digits.len() > 1 {
        if let Some(old) = old {
            if let Some(at) = digits.iter().position(|digit| *digit == old) {
                digits.remove(at);
            }
        }
    }

    let (at, text, focus, refused);
    if raw.is_empty() {
        // Backspace. Slint's cell clears itself and always steps back; a cell
        // that was already empty is what clears the cell before it, and a cell
        // past the end of the code has an empty cell before it and clears
        // nothing.
        if index < cells.len() {
            cells.remove(index);
            at = index;
        } else {
            if index == cells.len() {
                cells.pop();
            }
            at = index.saturating_sub(1);
        }
        text = String::new();
        // The first cell has nothing before it to step back to, and Slint's
        // first cell asks for no focus at all when it is the one cleared.
        focus = (index > 0).then(|| (index - 1).min(last));
        refused = false;
    } else if digits.is_empty() {
        // Not a digit: the edit is refused, the code does not move, and the
        // caret stays on the cell with its content selected, so the next
        // character replaces it as it would after Slint's cell took focus.
        at = index;
        text = old.map(String::from).unwrap_or_default();
        focus = Some(index.min(last));
        refused = true;
    } else {
        // The digit — or the run a paste carries — takes this cell and the ones
        // after it. A cell past the first empty one fills that one, so the code
        // is always the cells that are filled, in order.
        let write = index.min(cells.len());
        for (offset, digit) in digits.iter().enumerate() {
            let cell = write + offset;
            if cell >= length {
                break;
            }
            if cell < cells.len() {
                cells[cell] = *digit;
            } else {
                cells.push(*digit);
            }
        }
        at = write;
        text = cells
            .get(write)
            .map(|character| character.to_string())
            .unwrap_or_default();
        let next = (write + digits.len()).min(last);
        // The last cell has nowhere to advance to, and neither has a cell the
        // caret is already on: Slint asks for no focus then, and a cell that is
        // already focused is not selected again on the keystroke that filled it.
        focus = (next != index).then_some(next);
        refused = false;
    }

    let value: String = cells.into_iter().take(length).collect();
    Edit {
        index: at,
        text,
        complete: length > 0 && value.chars().count() == length,
        value,
        focus,
        refused,
    }
}

impl<'a, Message> From<OtpInput<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: OtpInput<'a, Message>) -> Self {
        let OtpInput {
            id,
            value,
            length,
            size,
            masked,
            disabled,
            on_edit,
        } = control;

        // A field with no way to report an edit is disabled, the way a button
        // with no message is: iced would otherwise accept a character and drop it.
        let disabled = disabled || on_edit.is_none();
        let cell = cell_size(size);
        let font_size = size.control_font_size();
        // The cell's frame is the field itself, so the inset is vertical only —
        // the horizontal room is the digit's. The vertical inset is the field's
        // own rule, `input::field_inset_y`: what the digit's line leaves of the
        // cell, floored and capped at the authored `Theme.input-padding-x(size)`.
        // Padding the cell with that value on both sides left 8px of a 24px
        // `small` cell for a 14px line, and iced clipped the digit against it.
        let cell_padding = input::field_inset_y(cell, font_size, size.input_padding_x());

        let mut cells = row![].spacing(theme::SPACING_SM).align_y(Alignment::Center);

        for index in 0..length {
            let shown = value
                .chars()
                .nth(index)
                .map(String::from)
                .unwrap_or_default();
            let edit_value = value.clone();
            let callback = on_edit.clone();

            let mut field = text_input("", &shown)
                .id(cell_id(&id, index))
                // The cell is the field's own frame, so the field draws text and
                // border on the whole cell and no horizontal padding is spent on
                // a frame that would hold it. The vertical padding insets the
                // caret from the cell's top and bottom edge, and the line box is
                // the cell less that padding, so the border stays on the edge of
                // the box and the cell beside it does not move.
                .padding(Padding {
                    top: cell_padding,
                    bottom: cell_padding,
                    ..Padding::ZERO
                })
                .width(Length::Fixed(cell))
                .size(font_size)
                .line_height(Pixels(cell - 2.0 * cell_padding))
                .align_x(Alignment::Center)
                .secure(masked)
                .font(Font {
                    weight: Weight::Semibold,
                    ..Font::DEFAULT
                })
                .style(move |theme: &Theme, status| {
                    Style::new()
                        .disabled(disabled)
                        .focused(matches!(status, text_input::Status::Focused { .. }))
                        .resolve(Tokens::of(theme))
                });

            if let Some(on_edit) = callback {
                field = field.on_input(move |raw| {
                    on_edit(edit(edit_value.as_str(), length, index, &raw))
                });
            }

            cells = cells.push(field);
        }

        cells.into()
    }
}

#[cfg(test)]
mod tests {
    use super::edit;

    #[test]
    fn a_character_fills_the_next_cell_and_the_caret_follows_it() {
        let filled = edit("12", 6, 2, "3");
        assert_eq!(filled.value(), "123");
        assert_eq!(filled.text(), "3");
        assert_eq!(filled.focus(), Some(3));
        assert!(!filled.is_complete());
    }

    #[test]
    fn the_last_cell_has_nowhere_to_advance_to() {
        let complete = edit("12345", 6, 5, "6");
        assert_eq!(complete.value(), "123456");
        assert_eq!(complete.focus(), None);
        assert!(complete.is_complete());
    }

    #[test]
    fn a_character_replaces_one_already_in_the_cell() {
        // iced appends to the cell's own text; the character that was there is
        // dropped so the keystroke replaces it, as Slint's select-on-focus does.
        let replaced = edit("123", 6, 1, "29");
        assert_eq!(replaced.value(), "193");
        assert_eq!(replaced.text(), "9");
        assert_eq!(replaced.focus(), Some(2));
    }

    #[test]
    fn backspace_clears_the_cell_and_steps_back() {
        let cleared = edit("123", 6, 2, "");
        assert_eq!(cleared.value(), "12");
        assert_eq!(cleared.index(), 2);
        assert_eq!(cleared.text(), "");
        assert_eq!(cleared.focus(), Some(1));

        // An already empty cell is what clears the one before it.
        let stepped = edit("12", 6, 2, "");
        assert_eq!(stepped.value(), "1");
        assert_eq!(stepped.index(), 1);
        assert_eq!(stepped.text(), "");
        assert_eq!(stepped.focus(), Some(1));

        // The first cell has nothing before it, so it keeps the caret.
        let first = edit("1", 6, 0, "");
        assert_eq!(first.value(), "");
        assert_eq!(first.index(), 0);
        assert_eq!(first.focus(), None);
    }

    #[test]
    fn a_paste_spreads_over_the_cells_after_this_one() {
        let pasted = edit("", 6, 0, "1234");
        assert_eq!(pasted.value(), "1234");
        assert_eq!(pasted.focus(), Some(4));

        // Past the code's length, the rest is dropped rather than grown past the
        // cells the field draws.
        let cut = edit("12", 4, 2, "3456");
        assert_eq!(cut.value(), "1234");
        assert_eq!(cut.focus(), Some(3));
    }

    #[test]
    fn a_character_that_is_not_a_digit_is_refused() {
        let refused = edit("12", 6, 2, "x");
        assert_eq!(refused.value(), "12");
        assert_eq!(refused.text(), "");
        assert_eq!(refused.focus(), Some(2));
        assert!(refused.is_refused());
        assert!(!refused.is_complete());
    }
}
