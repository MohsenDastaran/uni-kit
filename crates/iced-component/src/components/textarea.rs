//! A multi-line text field. Mirrors GPUI Component's `Textarea` and the Slint
//! version beside it: a placeholder, the disabled, read-only and invalid states,
//! the size ladder, a frame the caller can drop, and the rows that decide the
//! height.
//!
//! iced's ordinary field, `text_input`, holds one line and takes its value as a
//! `String` the caller owns. A textarea wraps and breaks lines, and iced's
//! multi-line widget is a different one: `text_editor`. Its state is a
//! [`Content`] the caller owns — the text, the caret and the selection all live
//! in it — and every interaction arrives as an [`Action`] through `on_action`,
//! which the caller applies with [`Content::perform`]. So this component is the
//! frame and the metrics around the caller's content, exactly as the `Input`
//! component is the frame around the caller's `String`.
//!
//! **Four things the references do that iced 0.14 cannot, and none is faked:**
//!
//! - **The focus halo is not expressible.** Slint draws the focused edge on the
//!   frame and a second, wider 2px ring around it. iced's editor draws exactly
//!   one border, from its own style, and a wrapper cannot know whether the field
//!   inside it is focused. The ring colour still reaches the one border, because
//!   the editor's status reports `Focused`, which is more than the single-line
//!   `Input` can do.
//! - **A token cannot live inside the value.** iced's `Content` is one plain
//!   string of text; a reference cannot sit in it as a unit that the caret and
//!   deletion move as one. The closest real thing is the Slint composer: chips
//!   on their own line, above a borderless field, with the text and the
//!   references reported apart. The gallery builds exactly that.
//! - **`read-only` is the widget's refusal to edit, not a mode the text model
//!   knows.** iced's editor publishes an `Edit` action for every keystroke, so
//!   this component swallows the key bindings that would edit — paste, enter,
//!   backspace, delete, cut, and an inserted character — and leaves copy, motion
//!   and selection alone. An input-method commit bypasses the key bindings and
//!   still reaches the caller, so a read-only field also asks the caller not to
//!   apply an `Edit` it is handed.
//! - **`opacity: enabled ? 1 : 0.5` has no counterpart.** A widget has no
//!   opacity of its own in iced; the disabled field takes the muted fill and
//!   muted ink, the same disabled treatment the single-line `Input` uses, rather
//!   than half of everything.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::Id;
use iced::widget::text::LineHeight;
use iced::widget::text_editor;
use iced::widget::text_editor::{Action, Binding, Content};
use iced::{Background, Border, Color, Element, Length, Pixels, Theme};

use super::theme::{self, ControlSize, Tokens};

/// A multi-line text field, built with the methods below.
///
/// The [`Content`] is the caller's: iced keeps the text, the caret and the
/// selection in it, so it has to outlive the render, and the caller applies
/// every [`Action`] it takes back through [`on_action`](Self::on_action). The
/// optional [`id`](Self::id) is the one thing `Content` does not hold — the
/// window's focus — and a name is needed only to focus the field from code.
pub struct Textarea<'a, Message> {
    content: &'a Content,
    id: Option<Id>,
    placeholder: String,
    size: ControlSize,
    disabled: bool,
    readonly: bool,
    invalid: bool,
    appearance: bool,
    height: Option<Length>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    on_action: Option<Box<dyn Fn(Action) -> Message + 'a>>,
}

impl<'a, Message> Textarea<'a, Message> {
    /// A field over the content the caller holds.
    pub fn new(content: &'a Content) -> Self {
        Self {
            content,
            id: None,
            placeholder: String::new(),
            size: ControlSize::default(),
            disabled: false,
            readonly: false,
            invalid: false,
            appearance: true,
            height: None,
            min_height: None,
            max_height: None,
            on_action: None,
        }
    }

    /// The `Id` the window keeps the field's focus under, if the application
    /// needs to focus it from code or ask whether it is focused. The text and
    /// the caret live in the [`Content`], not here.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
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

    /// Refuses input and draws muted, the way Slint's `enabled: false` does. A
    /// field with no [`on_action`](Self::on_action) is disabled without this.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Rejects edits and keeps the frame, the text, the caret and the
    /// selection: the field can still be focused, selected and copied. See the
    /// module docs for the one path — an input-method commit — the key bindings
    /// cannot swallow.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Reports the value as rejected: the edge takes the danger colour.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Draws the frame. `false` drops the fill, the edge and the radius so the
    /// field can sit inside a container that draws its own, which is what
    /// Slint's `appearance: false` is for and what the token composer uses.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// A definite height, `rows` lines of text plus the frame's padding — the
    /// `rows` the state owns in GPUI. Without it the field stands three control
    /// heights tall, which is the Slint component's own default.
    pub fn rows(mut self, rows: usize) -> Self {
        self.height = Some(Length::Fixed(self.rows_height(rows)));
        self
    }

    /// A height of the caller's choosing, when neither fixed rows nor growing
    /// says it. `Length::Fill` is the composer case: the field takes the height
    /// the frame around it gives it.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Grows with the content between `min_rows` and `max_rows` lines, the way
    /// GPUI's `auto_grow` does: shorter than the minimum it stands at the
    /// minimum, taller than the maximum it scrolls inside it. This is one place
    /// the port is ahead of the Slint component, which cannot measure itself and
    /// draws a fixed taller box instead.
    pub fn auto_grow(mut self, min_rows: usize, max_rows: usize) -> Self {
        self.height = Some(Length::Shrink);
        self.min_height = Some(self.rows_height(min_rows));
        self.max_height = Some(self.rows_height(max_rows));
        self
    }

    /// Every interaction, as the action the field performed. Apply it to the
    /// content — `content.perform(action)` — or the field will not change.
    pub fn on_action(mut self, on_action: impl Fn(Action) -> Message + 'a) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }

    /// The height Slint's `control-height(size)` gives a size, which is what its
    /// textarea multiplies by three: 20 / 24 / 32 / 40.
    fn control_height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 20.0,
            ControlSize::Small => 24.0,
            ControlSize::Large => 40.0,
            ControlSize::Medium => 32.0,
        }
    }

    /// The padding inside the frame, from the size ladder: 4 / 8 / 10 / 12.
    fn padding(&self) -> f32 {
        self.size.input_padding_x()
    }

    fn font_size(&self) -> f32 {
        self.size.input_font_size()
    }

    /// The line box, `1.45` of the font size — the multiple the Slint component
    /// uses to know how far to slide the field when the caret leaves the box.
    fn line_height(&self) -> f32 {
        self.font_size() * 1.45
    }

    /// What `rows` lines measure in the frame: the text plus the padding above
    /// and below it.
    fn rows_height(&self, rows: usize) -> f32 {
        rows as f32 * self.line_height() + 2.0 * self.padding()
    }

    /// The height the field stands at, defaulting to Slint's three control
    /// heights.
    fn resolved_height(&self) -> Length {
        self.height
            .unwrap_or(Length::Fixed(self.control_height() * 3.0))
    }

    /// A field with no way to report an action is disabled, the way a button
    /// with no message is: iced's editor reads a missing `on_action` the same
    /// way, and one it could not report would silently drop the edit.
    fn is_disabled(&self) -> bool {
        self.disabled || self.on_action.is_none()
    }

    fn style(&self) -> Style {
        Style::new()
            .invalid(self.invalid)
            .disabled(self.is_disabled())
            .appearance(self.appearance)
    }
}

/// The colours a textarea draws its frame with. Components pass it to the
/// editor's `style`; it is `pub` so an application can draw a matching field of
/// its own.
pub struct Style {
    pub invalid: bool,
    pub disabled: bool,
    pub appearance: bool,
}

impl Style {
    pub fn new() -> Self {
        Self {
            invalid: false,
            disabled: false,
            appearance: true,
        }
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// The fill: the page in the light theme, the edge at a third of its weight
    /// over it in the dark one, where a field that filled with the page would be
    /// a hole. A disabled field takes the muted surface instead.
    fn fill(&self, tokens: Tokens) -> Color {
        if !self.appearance {
            Color::TRANSPARENT
        } else if self.disabled {
            tokens.muted
        } else {
            tokens.input_background
        }
    }

    /// The edge, the way Slint's `edge` reads: the danger colour while the value
    /// is rejected, the ring colour while the field is focused, the input token
    /// otherwise.
    fn edge(&self, tokens: Tokens, status: text_editor::Status) -> Color {
        if !self.appearance {
            Color::TRANSPARENT
        } else if self.invalid {
            tokens.danger
        } else if matches!(status, text_editor::Status::Focused { .. }) {
            tokens.ring
        } else {
            tokens.input
        }
    }

    pub fn resolve(&self, tokens: Tokens, status: text_editor::Status) -> text_editor::Style {
        text_editor::Style {
            background: Background::Color(self.fill(tokens)),
            border: Border {
                color: self.edge(tokens, status),
                width: if self.appearance { 1.0 } else { 0.0 },
                radius: if self.appearance {
                    theme::RADIUS_MD
                } else {
                    0.0
                }
                .into(),
            },
            placeholder: tokens.muted_foreground,
            value: if self.disabled {
                tokens.muted_foreground
            } else {
                tokens.foreground
            },
            selection: theme::alpha(tokens.selection, 0.35),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Whether a key binding would change the text, which is what a read-only field
/// refuses. Copy, the motions, selection and unfocus are not edits and stay.
fn edits_text<Message>(binding: &Binding<Message>) -> bool {
    matches!(
        binding,
        Binding::Paste
            | Binding::Enter
            | Binding::Backspace
            | Binding::Delete
            | Binding::Insert(_)
            | Binding::Cut
    )
}

impl<'a, Message> From<Textarea<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Textarea<'a, Message>) -> Self {
        let style = control.style();
        let disabled = control.is_disabled();
        let readonly = control.readonly;
        let placeholder = control.placeholder.clone();
        let height = control.resolved_height();
        let padding = control.padding();
        let font_size = control.font_size();
        let min_height = control.min_height;
        let max_height = control.max_height;

        let mut field = text_editor(control.content)
            // The frame owns the padding and the edge, so the text sits inside
            // the border rather than over it.
            .padding(padding)
            .size(font_size)
            // The unit matters: a bare `f32` is a *multiple* of the font size,
            // so `1.45` is the factor Slint's `line` computes.
            .line_height(LineHeight::Relative(1.45))
            .height(height)
            .style(move |theme: &Theme, status| style.resolve(Tokens::of(theme), status));

        if !placeholder.is_empty() {
            field = field.placeholder(placeholder);
        }
        if let Some(id) = control.id {
            field = field.id(id);
        }
        if let Some(min_height) = min_height {
            field = field.min_height(Pixels(min_height));
        }
        if let Some(max_height) = max_height {
            field = field.max_height(Pixels(max_height));
        }

        // A disabled field carries no message: iced reads a missing `on_action`
        // as the disabled state, and one it cannot report would be a field that
        // silently drops the edit.
        if !disabled {
            if let Some(on_action) = control.on_action {
                field = field.on_action(on_action);
            }
            if readonly {
                // Read-only, at the keyboard: the editing bindings produce no
                // action at all, so there is nothing for the caller to apply.
                field = field.key_binding(|key_press| {
                    let binding = Binding::from_key_press(key_press)?;
                    if edits_text(&binding) {
                        None
                    } else {
                        Some(binding)
                    }
                });
            }
        }

        field.into()
    }
}
