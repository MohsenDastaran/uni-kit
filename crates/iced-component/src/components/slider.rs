//! A numeric range control. Mirrors GPUI Component's `Slider` and the Slint
//! version beside it: a value inside a minimum and a maximum, a step, the four
//! sizes, a disabled state, and click-or-drag on the track.
//!
//! **The track is drawn, not composed.** The filled part of a slider is a
//! fraction of whatever width the control is given, and an iced widget does not
//! know that width until it is laid out — the same reason `progress` draws its
//! bar on a canvas. A slider also has to answer the pointer with a position that
//! maps back to a value, which a `container` cannot do. So this file implements
//! [`iced::advanced::Widget`] directly, the route `text_view` takes for the one
//! component that needs more than composition: [`layout`](Widget::layout) takes
//! the width, [`draw`](Widget::draw) paints the track, the fill and the thumb,
//! and [`update`](Widget::update) turns a press, a drag or an arrow key into the
//! value the caller is told about.
//!
//! **Slint is the authority for the drawing.** The track is 4 / 6 / 8 pixels
//! tall and the thumb is 12 / 16 / 18 across for xsmall through large; the
//! unfilled track is `secondary`, the fill and the thumb's 2-pixel edge are
//! `primary`, and the thumb's own face is the page, so it reads as a hole in the
//! track rather than a second fill of the same hue. The thumb's centre travels
//! the width minus one thumb, so the whole thumb stays on the track at either
//! end, and only the value changes are reported.
//!
//! **Every part can be given a colour, and none has to be.** GPUI's `Slider`
//! lets a caller replace the bar with `Styled::bg` and the thumb with
//! `Styled::text_color`; Slint draws the four parts from the theme and offers no
//! override at all, which is the gap the example used to state instead of
//! filling. Here [`track_color`](Slider::track_color), [`fill_color`](Slider::fill_color),
//! [`thumb_color`](Slider::thumb_color) and
//! [`thumb_border_color`](Slider::thumb_border_color) take a `fn(&Tokens) -> Color`
//! that is resolved in [`draw`](Widget::draw), so a custom slider follows a theme
//! switch and a fixed `Color` cannot freeze one appearance. Unset, each is exactly
//! the token named above, so an existing call site draws what it always drew.
//! [`reverse`](Slider::reverse) is GPUI's `reverse()` — the fill from the thumb to
//! the maximum instead of from the minimum — and [`radius`](Slider::radius) is the
//! corner `Styled::rounded` sets, pill by default.
//!
//! **A step is shared, not re-derived.** Slint's arrow handler is
//! `min(maximum, value + step)`, which is exactly what
//! [`number_input::step_value`](super::number_input::step_value) computes — the
//! arithmetic `number_input`'s own steppers use and pin down with tests. A key
//! here calls it rather than copying it, so a slider at a bound and a numeric
//! field at the same bound move the same way.
//!
//! **What iced 0.14 cannot draw, and what is not faked:**
//!
//! - **No thumb-focus ring.** Slint draws one while the slider has focus, and
//!   iced reports focus to the element rather than to the application, so there
//!   is no signal for a ring here. The arrow keys are handled when the pointer is
//!   over the control, which is the same stand-in iced's own `slider` uses; the
//!   application owns real focus and Tab order.
//! - **Disabled is fill and ink, not opacity.** Slint fades the whole control to
//!   half opacity; iced 0.14 has no subtree opacity, so the muted state is the
//!   fill and the thumb's edge at a lower weight, and the control draws no
//!   hover or press response — the same reading `checkbox` and `radio` use.
//! - **No vertical, range or logarithmic form.** Slint's slider is horizontal
//!   and single-valued and has no scale property, and so is this one. iced ships
//!   a separate `vertical_slider`; composing a range or a log scale from it is
//!   not what this component is.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::Renderer as _;
use iced::advanced::layout::{self, Limits};
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::widget::tree::{self, Tag};
use iced::advanced::{Clipboard, Layout, Shell, Widget, mouse};
use iced::{Border, Color, Element, Event, Length, Rectangle, Size, Theme};
use iced::{keyboard, touch};

use super::number_input::{self, StepAction};
use super::theme::{self, ControlSize, Tokens};

/// The track's height for a size step: 4 / 6 / 8 pixels, Slint's `track-h`.
fn track_height(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 4.0,
        ControlSize::Large => 8.0,
        _ => 6.0,
    }
}

/// The thumb's side for a size step: 12 / 16 / 18, Slint's `thumb`.
fn thumb_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 12.0,
        ControlSize::Large => 18.0,
        _ => 16.0,
    }
}

/// The control's height, `max(thumb, 24px)` the way Slint sets it — the thumb
/// has to fit, and a slider is never a hairline to hit.
fn control_height(size: ControlSize) -> f32 {
    thumb_size(size).max(24.0)
}

/// What the pointer, or an arrow key, has done to one slider. It is the widget's
/// own state: a drag outlives the frame that started it, and iced keeps the tree
/// node this is stored in across renders.
#[derive(Debug, Default)]
struct Dragging {
    dragging: bool,
}

/// A slider, built with the methods below.
///
/// The value is the caller's: the control reports what a drag or a key asks for
/// through [`on_change`](Self::on_change) and the caller writes it back, the
/// shape `number_input` uses for its steppers. A slider built without
/// `on_change` cannot be moved, which is how iced spells a disabled control;
/// [`disabled`](Self::disabled) says the same thing explicitly for a control
/// that also draws its muted state.
pub struct Slider<'a, Message> {
    value: f32,
    minimum: f32,
    maximum: f32,
    step: f32,
    size: ControlSize,
    disabled: bool,
    width: Length,
    track_color: Option<fn(&Tokens) -> Color>,
    fill_color: Option<fn(&Tokens) -> Color>,
    thumb_color: Option<fn(&Tokens) -> Color>,
    thumb_border_color: Option<fn(&Tokens) -> Color>,
    reverse: bool,
    radius: Option<f32>,
    on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

impl<'a, Message> Slider<'a, Message> {
    /// A slider at `value`. The range is 0..=100 and the step 1, Slint's
    /// defaults.
    pub fn new(value: f32) -> Self {
        Self {
            value,
            minimum: 0.0,
            maximum: 100.0,
            step: 1.0,
            size: ControlSize::default(),
            disabled: false,
            width: Length::Fill,
            track_color: None,
            fill_color: None,
            thumb_color: None,
            thumb_border_color: None,
            reverse: false,
            radius: None,
            on_change: None,
        }
    }

    /// The lower bound a drag or a key clamps to.
    pub fn minimum(mut self, minimum: f32) -> Self {
        self.minimum = minimum;
        self
    }

    /// The upper bound a drag or a key clamps to.
    pub fn maximum(mut self, maximum: f32) -> Self {
        self.maximum = maximum;
        self
    }

    /// How far one arrow key moves the value, and the interval a drag snaps to.
    /// Zero leaves the value unsnapped, the way the Slint original's
    /// `step > 0 ? round(raw / step) * step : raw` reads.
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Refuses the pointer and the keys and draws the muted track. A control
    /// with no [`on_change`](Self::on_change) is disabled without this.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The unfilled track's colour, default [`Tokens::secondary`]. The function
    /// receives the active tokens, so the colour is resolved when iced draws the
    /// slider and a theme switch moves it; a fixed `Color` would freeze one
    /// appearance.
    pub fn track_color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// The filled part of the track's colour, default [`Tokens::primary`].
    pub fn fill_color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// The thumb's face, default [`Tokens::background`] — the page showing
    /// through the track rather than a second fill.
    pub fn thumb_color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// The thumb's two-pixel edge, default [`Tokens::primary`], the fill's hue
    /// so the thumb stays a hole in either appearance.
    pub fn thumb_border_color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.thumb_border_color = Some(color);
        self
    }

    /// Fill from the thumb to the maximum instead of from the minimum to the
    /// thumb, GPUI's `reverse()`. Default `false`; values and interaction do not
    /// change, only the filled side.
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// The track's corner radius in pixels, default half the track's height —
    /// the pill Slint draws. GPUI's `Styled::rounded` is the same override.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// The value a drag or a key produced, already clamped to the range. Slint's
    /// `changed`, with iced's caller writing the message back rather than an
    /// in-out property.
    pub fn on_change(mut self, on_change: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Whether the control answers the pointer and the keys.
    fn is_disabled(&self) -> bool {
        self.disabled || self.on_change.is_none()
    }

    /// The span the fraction is measured over. Slint's `max(0.0001, maximum -
    /// minimum)` keeps a zero-width range from dividing by nothing.
    fn span(&self) -> f32 {
        (self.maximum - self.minimum).abs().max(0.0001)
    }

    /// The value's place on the track, `0.0..=1.0`, the way Slint computes
    /// `clamp((value - minimum) / span, 0, 1)`.
    fn fraction(&self) -> f32 {
        ((self.value - self.minimum) / self.span()).clamp(0.0, 1.0)
    }

    /// The value at a pointer position inside `bounds`, snapped to the step and
    /// clamped to the range — Slint's `apply`, run backwards.
    ///
    /// The thumb's half-width is held back at each end so the value follows the
    /// pointer at either extreme, and a width too small to travel in reports
    /// nothing rather than dividing by it.
    fn locate(&self, bounds: Rectangle, x: f32) -> Option<f32> {
        let travel = (bounds.width - thumb_size(self.size)).max(0.0);
        if travel <= 0.0 {
            return None;
        }
        let fraction = ((x - bounds.x - thumb_size(self.size) / 2.0) / travel).clamp(0.0, 1.0);
        let raw = self.minimum + fraction * self.span();
        let stepped = if self.step > 0.0 {
            (raw / self.step).round() * self.step
        } else {
            raw
        };
        // A bound given the wrong way round is clamped rather than panicked on,
        // the same reading `step_value` uses.
        let low = self.minimum.min(self.maximum);
        let high = self.maximum.max(self.minimum);
        Some(stepped.clamp(low, high))
    }

    /// Publishes `next` when it differs from the value already shown. A drag
    /// reports every pixel of travel, so an unchanged value is not a change and
    /// must not produce a message.
    fn report(&self, next: f32, shell: &mut Shell<'_, Message>) {
        if (next - self.value).abs() > f32::EPSILON
            && let Some(on_change) = &self.on_change
        {
            shell.publish(on_change(next));
        }
    }

    /// The four colours a slider draws with — the unfilled track, the filled
    /// track, the thumb's edge and the thumb's face — with any caller override
    /// resolved against the active tokens.
    ///
    /// The defaults are the tokens the Slint authority names: the unfilled track
    /// is `secondary` and the fill `primary`, so the two halves differ by
    /// lightness in both appearances rather than by hue alone. The thumb's face
    /// is the page with an edge of the fill, which is how it stays visible on the
    /// one hue it sits against.
    ///
    /// Disabled lowers the weights instead of fading a subtree iced cannot fade,
    /// and the edge keeps enough of the fill to be an edge in either theme. A
    /// caller's own colours are muted the same way, so a custom slider disabled
    /// by `on_change` being absent still reads as one family.
    fn colors(&self, tokens: Tokens, disabled: bool) -> (Color, Color, Color, Color) {
        let track = self
            .track_color
            .unwrap_or(|tokens: &Tokens| tokens.secondary)(&tokens);
        let fill = self.fill_color.unwrap_or(|tokens: &Tokens| tokens.primary)(&tokens);
        let edge = self
            .thumb_border_color
            .unwrap_or(|tokens: &Tokens| tokens.primary)(&tokens);
        let face = self
            .thumb_color
            .unwrap_or(|tokens: &Tokens| tokens.background)(&tokens);

        if disabled {
            (
                theme::alpha(track, 0.7),
                theme::alpha(fill, 0.45),
                theme::alpha(edge, 0.45),
                face,
            )
        } else {
            (track, fill, edge, face)
        }
    }
}

impl<'a, Message> Widget<Message, Theme, iced::Renderer> for Slider<'a, Message>
where
    Message: 'a,
{
    fn tag(&self) -> Tag {
        Tag::of::<Dragging>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Dragging::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Fixed(control_height(self.size)))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, control_height(self.size))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let dragging = tree.state.downcast_mut::<Dragging>();
        let bounds = layout.bounds();
        let disabled = self.is_disabled();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if !disabled && let Some(position) = cursor.position_over(bounds) {
                    if let Some(next) = self.locate(bounds, position.x) {
                        self.report(next, shell);
                    }
                    dragging.dragging = true;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                dragging.dragging = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if dragging.dragging && !disabled {
                    if let Some(next) = cursor
                        .land()
                        .position()
                        .and_then(|position| self.locate(bounds, position.x))
                    {
                        self.report(next, shell);
                    }
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                if !disabled && bounds.contains(*position) {
                    if let Some(next) = self.locate(bounds, position.x) {
                        self.report(next, shell);
                    }
                    dragging.dragging = true;
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                dragging.dragging = false;
            }
            Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if dragging.dragging && !disabled {
                    if let Some(next) = self.locate(bounds, position.x) {
                        self.report(next, shell);
                    }
                    shell.capture_event();
                }
            }
            // iced reports no focus to a widget, so the pointer stands in for it:
            // the keys act on the slider the cursor is over, which is the same
            // rule iced's own slider follows.
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                if !disabled && cursor.is_over(bounds) =>
            {
                let action = match key.as_ref() {
                    keyboard::Key::Named(keyboard::key::Named::ArrowRight)
                    | keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                        Some(StepAction::Increment)
                    }
                    keyboard::Key::Named(keyboard::key::Named::ArrowLeft)
                    | keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                        Some(StepAction::Decrement)
                    }
                    _ => None,
                };
                if let Some(action) = action {
                    // The bounded step `number_input`'s steppers perform, so a
                    // slider at a bound and a field at the same bound agree.
                    if let Some(next) = number_input::step_value(
                        self.value as f64,
                        action,
                        self.step as f64,
                        self.minimum as f64,
                        self.maximum as f64,
                    ) {
                        self.report(next as f32, shell);
                    }
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let tokens = Tokens::of(theme);
        let disabled = self.is_disabled();
        let track = track_height(self.size);
        let thumb = thumb_size(self.size);
        let rail_y = bounds.y + bounds.height / 2.0;
        let travel = (bounds.width - thumb).max(0.0);
        let centre = bounds.x + thumb / 2.0 + travel * self.fraction();

        let (unfilled, filled, edge, face) = self.colors(tokens, disabled);
        let radius = self.radius.unwrap_or(track / 2.0);
        // GPUI's fill spans the thumb to the end of the track when reversed;
        // otherwise it runs from the start of the track to the thumb.
        let (fill_start, fill_end) = if self.reverse {
            (centre, bounds.x + bounds.width)
        } else {
            (bounds.x, centre)
        };

        // The track, then the fill over it from the minimum end to the thumb.
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y - track / 2.0,
                    width: bounds.width,
                    height: track,
                },
                border: Border {
                    radius: radius.into(),
                    ..Border::default()
                },
                ..renderer::Quad::default()
            },
            unfilled,
        );
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: fill_start,
                    y: rail_y - track / 2.0,
                    width: (fill_end - fill_start).max(0.0),
                    height: track,
                },
                border: Border {
                    radius: radius.into(),
                    ..Border::default()
                },
                ..renderer::Quad::default()
            },
            filled,
        );

        // The thumb is the page with an edge of the fill's hue, the two-pixel
        // ring Slint draws, so it stays a hole in the track in either theme.
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: centre - thumb / 2.0,
                    y: rail_y - thumb / 2.0,
                    width: thumb,
                    height: thumb,
                },
                border: Border {
                    radius: (thumb / 2.0).into(),
                    width: 2.0,
                    color: edge,
                },
                ..renderer::Quad::default()
            },
            face,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        // A slider is grabbed, not clicked, so the cursor says so while it is
        // over or held — the one place a non-default cursor is the affordance.
        let dragging = &tree.state.downcast_ref::<Dragging>().dragging;
        if *dragging {
            if cfg!(target_os = "windows") {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grabbing
            }
        } else if cursor.is_over(layout.bounds()) {
            if cfg!(target_os = "windows") {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grab
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message> From<Slider<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Slider<'a, Message>) -> Self {
        Element::new(control)
    }
}
