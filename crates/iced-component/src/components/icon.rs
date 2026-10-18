//! An icon. Mirrors GPUI Component's `Icon` and the Slint version beside it: an
//! SVG tinted in a theme colour, drawn on a square box at 12, 14, 16 or 24
//! pixels, optionally rotating.
//!
//! **The artwork is the caller's.** Slint ships an `icons/` folder, a `name`
//! property and a chain of `@image-url` arms to pick out of it; GPUI names its
//! glyphs with an `IconName` enum. An iced application embeds its own SVGs, so
//! [`Icon::new`] takes the [`svg::Handle`] it built — a memory handle from
//! `include_bytes!`, or a path when the application ships its assets loose. The
//! set belongs to the application and this component never picks from it.
//!
//! **The rotation is the icon's own clock.** Slint spins the image from
//! `animation-tick()`; [`Icon::spinning`] is that flag, and the turn is sampled
//! from the frame's own instant. [`Icon::spin`] stays for the determinate case:
//! an explicit turn in `0.0..=1.0` the caller sets, the same value
//! [`Spinner`](super::spinner)'s `progress` is. [`Icon::turn_to`] is the middle
//! case, and the one a disclosure's mark wants: the caller names only where the
//! mark should end up, and the icon owns the leg — [`TURN_DURATION`], the
//! collapsible reveal's duration — sampling the frame's own instant and
//! retargeting from the angle on screen if the caller changes its mind part way.
//! The icon is drawn at `turn * 2π` radians about its centre, and the rotation
//! does not move the box, so a spinning icon never shifts its neighbours.
//!
//! iced has no interval subscription under this crate's runtime, so the motion
//! is kept in the widget tree instead: while `spinning` is set, every frame asks
//! the shell for the next one and samples the elapsed time from the instant that
//! frame carries. No timer and no subscription — the mechanism
//! [`collapsible`](super::collapsible)'s reveal uses. A rotation does not change
//! layout, so nothing here invalidates it.
//!
//! One Slint property has no iced equivalent: `name`, because the handle
//! already is the artwork.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::svg::Renderer as _;
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{renderer, Clipboard, Shell};
use iced::time::{Duration, Instant};
use iced::widget::svg;
use iced::{
    mouse, Color, ContentFit, Element, Event, Length, Point, Rectangle, Radians, Size as BoxSize,
    Theme, Vector,
};

use super::theme::{ControlSize, Tokens};

/// How long one full turn takes, Slint's `800ms`: the component writes
/// `mod(animation-tick(), 800ms) / 800ms * 360deg`.
pub const SPIN_DURATION: Duration = Duration::from_millis(800);

/// How long [`Icon::turn_to`] takes to reach a new target, Slint's `150ms`:
/// the duration the collapsible gives its reveal, so a mark that turns to say
/// "expanded" moves with the panel it announces rather than beside it.
pub const TURN_DURATION: Duration = Duration::from_millis(150);

/// How big the icon's square box is.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Size {
    /// One of the four steps the catalog shares: 12, 14, 16 or 24 pixels. The
    /// Slint version defaults to 16, which is [`ControlSize::Medium`].
    #[default]
    Medium,
    XSmall,
    Small,
    Large,
    /// An exact edge length in pixels, for the `size` property Slint lets a
    /// caller set to any length.
    Custom(f32),
}

impl From<ControlSize> for Size {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::XSmall => Self::XSmall,
            ControlSize::Small => Self::Small,
            ControlSize::Medium => Self::Medium,
            ControlSize::Large => Self::Large,
        }
    }
}

impl From<f32> for Size {
    fn from(pixels: f32) -> Self {
        Self::Custom(pixels)
    }
}

impl Size {
    /// The edge length in pixels. Slint's icon box is the glyph's own size, so
    /// the four steps are the same numbers GPUI's `size_3` through `size_6`
    /// give an icon.
    pub fn pixels(self) -> f32 {
        match self {
            Self::XSmall => 12.0,
            Self::Small => 14.0,
            Self::Medium => 16.0,
            Self::Large => 24.0,
            Self::Custom(pixels) => pixels.max(0.0),
        }
    }
}

/// An icon, built with the methods below. It carries no message, but it is
/// generic over one so a caller can place it in any view without annotating the
/// type.
pub struct Icon<Message> {
    handle: svg::Handle,
    size: Size,
    color: Option<Color>,
    spin: Option<f32>,
    turn: Option<f32>,
    spinning: bool,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Icon<Message> {
    /// An icon drawing `handle`, at the default 16 pixels in the foreground
    /// colour.
    pub fn new(handle: impl Into<svg::Handle>) -> Self {
        Self {
            handle: handle.into(),
            size: Size::default(),
            color: None,
            spin: None,
            turn: None,
            spinning: false,
            _message: std::marker::PhantomData,
        }
    }

    /// The edge of the icon's square box: a [`ControlSize`], a [`Size`], or a
    /// pixel count.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// The glyph's colour. The default is the theme's foreground, which is what
    /// the Slint property defaults to.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Holds the icon at the turn `progress`, in `0.0..=1.0`, about its centre:
    /// the determinate form, where the caller owns the value. `None`, the
    /// default, holds the icon still unless [`Icon::spinning`] turns it.
    pub fn spin(mut self, progress: f32) -> Self {
        self.spin = Some(progress);
        self
    }

    /// Turns the icon to `turn`, in `0.0..=1.0`, about its centre. This is the
    /// animated form of [`Icon::spin`]: the caller names only where the mark
    /// should point, and the icon owns the clock — the widget tree remembers the
    /// leg and every frame samples its own instant, the route
    /// [`Icon::spinning`] takes. The turn takes [`TURN_DURATION`], and a target
    /// that changes mid-turn is picked up from the angle on screen, so a
    /// reversal is continuous. `None`, the default, holds the icon still unless
    /// [`Icon::spinning`] turns it.
    pub fn turn_to(mut self, turn: f32) -> Self {
        self.turn = Some(turn);
        self
    }

    /// Turns the icon continuously while `spinning`, which is the Slint
    /// property of the same name. The component owns the clock — one full turn
    /// every [`SPIN_DURATION`] — and asks for a frame each frame, so the caller
    /// supplies no value. `false`, the default, holds the icon still.
    pub fn spinning(mut self, spinning: bool) -> Self {
        self.spinning = spinning;
        self
    }
}

impl<Message> Default for Icon<Message> {
    fn default() -> Self {
        Self::new(svg::Handle::from_memory(Vec::new()))
    }
}

/// The colours and geometry an icon draws with. [`Style::resolve`] fills in the
/// colour from the theme in force, so an icon built without a colour follows a
/// light or dark switch.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub size: Size,
    pub color: Option<Color>,
    /// The caller's explicit turn, `0.0..=1.0`.
    pub spin: Option<f32>,
    /// The caller's target turn for the animated leg, `0.0..=1.0`.
    pub turn: Option<f32>,
    /// Whether the icon's own clock is running.
    pub spinning: bool,
}

impl Style {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            color: None,
            spin: None,
            turn: None,
            spinning: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn spin(mut self, progress: f32) -> Self {
        self.spin = Some(progress);
        self
    }

    pub fn turn_to(mut self, turn: f32) -> Self {
        self.turn = Some(turn);
        self
    }

    pub fn spinning(mut self, spinning: bool) -> Self {
        self.spinning = spinning;
        self
    }

    /// The colour the glyph is tinted. The Slint property always has a value,
    /// so an icon without a colour of its own takes the theme's foreground in
    /// whichever mode is in force; a colour the caller named wins.
    pub fn resolve(&self, tokens: Tokens) -> svg::Style {
        svg::Style {
            color: Some(self.color.unwrap_or(tokens.foreground)),
        }
    }

    /// The edge length the icon's box takes.
    pub fn pixels(&self) -> f32 {
        self.size.pixels()
    }
}

/// The icon's own motion, held in the widget tree instead of the caller's state.
///
/// iced has no timer to subscribe to, so the turn is not stored as a tick but as
/// the instant the clock started: any frame can ask where the icon should be,
/// and the angle is read off that frame rather than advanced by a subscription.
#[derive(Debug)]
struct Spin {
    /// When the turn's clock started: the frame the icon mounted on. Slint's
    /// `animation-tick()` is absolute, so a stop and a start picks the turn up
    /// where the clock is rather than restarting it.
    started: Instant,
    /// The turn sampled for the frame being drawn, `0.0..=1.0`.
    turn: f32,
    /// The target the animated leg is heading for, and the angle it started
    /// from. A target that has not changed leaves the leg alone.
    target: f32,
    from: f32,
    /// When the animated leg started, sampled from the frame's own instant the
    /// way a continuous turn is.
    turned_at: Instant,
    /// Whether the animated leg still has distance to cover.
    moving: bool,
}

impl Spin {
    /// A turn sitting at its target: a mark that has just mounted points where
    /// the caller asked, it does not sweep there from zero.
    fn new(target: f32) -> Self {
        Self {
            started: Instant::now(),
            turn: target,
            target,
            from: target,
            turned_at: Instant::now(),
            moving: false,
        }
    }

    /// Samples the turn at `now`, the clock `animation-tick()` gives Slint.
    fn sample(&mut self, now: Instant) {
        let elapsed = now.duration_since(self.started);
        self.turn = (elapsed.as_secs_f32() / SPIN_DURATION.as_secs_f32()).fract();
    }

    /// Points the animated leg at `target`, from the angle on screen. A target
    /// that has not changed leaves a leg already in flight alone.
    fn retarget(&mut self, target: f32, now: Instant) {
        if self.target == target {
            return;
        }
        self.from = self.turn;
        self.target = target;
        self.turned_at = now;
        self.moving = true;
    }

    /// Samples the animated leg at `now`, and answers whether there is more of
    /// it to come. The curve is linear, the same straight line the collapsible
    /// reveal draws.
    fn sample_turn(&mut self, now: Instant, duration: Duration) -> bool {
        let elapsed = now.duration_since(self.turned_at);
        if duration.is_zero() || elapsed >= duration {
            self.turn = self.target;
            self.moving = false;
            return false;
        }
        let t = elapsed.as_secs_f32() / duration.as_secs_f32();
        self.turn = self.from + (self.target - self.from) * t;
        true
    }
}

/// The icon as a widget, because the rotation is a draw-time value the widget
/// tree can carry. [`Icon`] builds one; it is private because the caller gets an
/// `Element`.
struct Turning {
    handle: svg::Handle,
    style: Style,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Turning {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Spin>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Spin::new(self.style.turn.unwrap_or(0.0)))
    }

    fn size(&self) -> BoxSize<Length> {
        let pixels = self.style.pixels();
        BoxSize::new(Length::Fixed(pixels), Length::Fixed(pixels))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let pixels = self.style.pixels();
        // The wrapper the SVG widget used to sit in fixed the same square; doing
        // it here is what keeps the box steady through the turn.
        layout::Node::new(limits.resolve(
            Length::Fixed(pixels),
            Length::Fixed(pixels),
            BoxSize::new(pixels, pixels),
        ))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        // The frame carries its own instant, so the angle drawn and the angle
        // sampled are the same frame's.
        let frame = match event {
            Event::Window(iced::window::Event::RedrawRequested(at)) => Some(*at),
            _ => None,
        };

        if self.style.spinning {
            if let Some(at) = frame {
                tree.state.downcast_mut::<Spin>().sample(at);
            }
            // A spinner's motion never ends while it is active, so every frame
            // asks for the next one. This is the reveal's `request_redraw`, with
            // the compositor's next frame as the target.
            shell.request_redraw();
            return;
        }

        let Some(target) = self.style.turn else {
            return;
        };
        let now = frame.unwrap_or_else(Instant::now);
        let moving = {
            let spin = tree.state.downcast_mut::<Spin>();
            spin.retarget(target, now);
            match frame {
                Some(at) => spin.sample_turn(at, TURN_DURATION),
                // Without a frame there is nothing to sample against; the
                // request below is what turns one into a look at the leg.
                None => spin.moving,
            }
        };
        if moving {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let turn = if self.style.spinning || self.style.turn.is_some() {
            tree.state.downcast_ref::<Spin>().turn
        } else {
            self.style.spin.unwrap_or(0.0)
        };
        let bounds = layout.bounds();

        // The SVG widget's own draw, with the frame's rotation: measure the
        // glyph, fit it into the square box and centre it. Replacing that widget
        // is what lets the rotation change between frames.
        let image = renderer.measure_svg(&self.handle);
        let image_size = BoxSize::new(image.width as f32, image.height as f32);
        let fit = ContentFit::Contain.fit(image_size, bounds.size());
        let scale = Vector::new(fit.width / image_size.width, fit.height / image_size.height);
        let final_size = image_size * scale;
        let position = Point::new(
            bounds.center_x() - final_size.width / 2.0,
            bounds.center_y() - final_size.height / 2.0,
        );

        renderer.draw_svg(
            iced::advanced::svg::Svg {
                handle: self.handle.clone(),
                color: self.style.resolve(Tokens::of(theme)).color,
                rotation: Radians(turn * std::f32::consts::TAU),
                opacity: 1.0,
            },
            Rectangle::new(position, final_size),
            bounds,
        );
    }
}

impl<'a, Message> From<Icon<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Icon<Message>) -> Self {
        Element::new(Turning {
            handle: control.handle,
            style: Style {
                size: control.size,
                color: control.color,
                spin: control.spin,
                turn: control.turn,
                spinning: control.spinning,
            },
        })
    }
}
