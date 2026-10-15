//! A split button: an action half that runs its own message, and a caret half
//! that opens a [`Menu`]. Mirrors GPUI Component's `DropdownButton` and the
//! Slint `dropdown_button.slint` beside it.
//!
//! # One control, two halves
//!
//! The halves share one outline: the leading half squares the corners that meet
//! the trailing one and the trailing half squares its leading corners, which is
//! the Slint control's `group-start`/`group-end` pair. The variant decides
//! whether the join shows — a `Primary` split carries no border, so its two
//! fills read as the one button the reference draws, while a bordered variant
//! draws the 1px divider the reference's muted split has. iced has no negative
//! spacing and each half paints its own background, so the primary join keeps a
//! one-pixel seam where the two antialiased edges meet; Slint's `-1px` overlap
//! has no iced spelling.
//!
//! The trailing half's mark is built in. Slint writes `icon: "chevron-down"` on
//! the trigger and GPUI's `Button::dropdown_caret(true)` draws a `Caret`, so
//! neither asks the caller for one — and a control whose disclosure mark is
//! optional is a control that ships as an empty box. [`DropdownButton::caret`]
//! replaces the mark for a caller with its own icon set; it is no longer the
//! only way to get one.
//!
//! The variant and the size are the whole control's, which is Slint's
//! `variant`/`size` and GPUI's `effective_variant`/`effective_size`: one value
//! styles both halves. The label, the leading icon, the loading state, the
//! selected state and the main action belong to the leading half, exactly as
//! `text`, `icon`, `loading`, `selected` and the `main` callback do in Slint.
//! The [`Popover`] wraps the caret half alone, so a press on the label runs the
//! main action and never opens the menu — Slint's `main.clicked` and
//! `caret.clicked => menu.show()` are two halves because they are two actions.
//!
//! # The open state is the caller's, as a popover's is
//!
//! [`DropdownButton::open`] is what the menu is drawn from and
//! [`DropdownButton::on_open_change`] is the only way the control reports a
//! request to change it — a caret press, an outside click, or Escape. The
//! control has no state of its own, exactly like the [`Popover`] underneath it,
//! so the flag the caller holds is the flag the menu follows.
//!
//! # Where the menu goes
//!
//! The Slint control's `PopupWindow` hangs the menu from the button's trailing
//! edge (`x: parent.width - self.width`, `y: root.height + 4px`). iced's
//! [`Popover`] centres a panel on its trigger rather than aligning it to an
//! edge, and it is the positioning layer every overlay in this crate shares, so
//! this port takes [`Placement::Bottom`] at [`Popover::offset`] 4 and leaves the
//! exact corner to the popover. The menu's width is the caller's,
//! [`DropdownButton::menu_width`]; Slint's `menu-width: 0px` means "the menu's
//! own width", which in iced is the popover's fixed panel width, so the default
//! is [`DEFAULT_MENU_WIDTH`].
//!
//! # The report
//!
//! [`DropdownAction`] is the Slint `DropdownButtonAction` enum, and
//! [`DropdownButton::on_chosen`] is its `chosen(index, event)`: the leading half
//! reports [`DropdownAction::Main`] with index 0, and a menu row reports
//! [`DropdownAction::Menu`] with its own index. The menu's
//! [`Menu::on_highlight`] is not re-exported here; a caller that wants it
//! composes `Popover` and `Menu` itself, which is what the `menu` demo does.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{row, svg};
use iced::{Alignment, Element, Theme};

use super::button::{self, Button, Variant};
use super::menu::{self, Item, Menu};
use super::popover::{Placement, Popover};
use super::theme::{ControlSize, Tokens};

/// The menu panel's width when the caller names none. Slint's `menu-width: 0px`
/// means "the menu's own width", and in iced the menu's own width is
/// [`menu::PANEL_WIDTH`] — the fixed panel a standalone [`Menu`] is given, and
/// the width [`select`](super::select) and [`combobox`](super::combobox) already
/// pass their popovers. The dropdown draws the same `Menu` body, so it takes the
/// same width rather than a second number that would drift from it. The Slint
/// authority authors 280px and the iced menu grew to 320 so its labels stop
/// wrapping, which is the same reason this control's menu needs the wider panel.
pub const DEFAULT_MENU_WIDTH: f32 = menu::PANEL_WIDTH;

/// Which half of a split button reported.
///
/// The names are the Slint `DropdownButtonAction` enum's: a menu row reports
/// itself by index, and the leading half reports `Main` with no index —
/// `chosen(-1, …)` in the Slint file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownAction {
    /// The leading half was pressed.
    Main,
    /// A menu row was chosen.
    Menu,
}

/// A split button, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is a
/// [`Popover`] whose trigger is the two halves and whose panel is a [`Menu`] —
/// which is exactly what the Slint control is: a `Button` pair with a
/// `PopupWindow` holding a `Menu`.
pub struct DropdownButton<'a, Message> {
    label: String,
    icon: Option<Rc<dyn Fn() -> Element<'a, Message> + 'a>>,
    /// The caret glyph, or `None` for the built-in Lucide `chevron-down` the
    /// authority hardcodes on the trigger.
    caret: Option<Rc<dyn Fn() -> Element<'a, Message> + 'a>>,
    variant: Variant,
    size: ControlSize,
    outline: bool,
    loading: bool,
    selected: bool,
    disabled: bool,
    open: bool,
    /// The row the caller says is highlighted, drawn by the menu.
    highlighted: Option<usize>,
    items: Vec<Item>,
    menu_width: f32,
    on_chosen: Option<Rc<dyn Fn(usize, DropdownAction) -> Message + 'a>>,
    on_highlight: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> DropdownButton<'a, Message> {
    /// A split button whose leading half reads `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            caret: None,
            variant: Variant::default(),
            size: ControlSize::Medium,
            outline: false,
            loading: false,
            selected: false,
            disabled: false,
            open: false,
            highlighted: None,
            items: Vec::new(),
            menu_width: DEFAULT_MENU_WIDTH,
            on_chosen: None,
            on_highlight: None,
            on_open_change: None,
        }
    }

    /// The variant both halves are drawn in. [`Variant::Default`] is the
    /// default, which is the Slint control's `ButtonVariant.default` and GPUI's
    /// `ButtonVariant::default()`. A caller that wants the solid primary split
    /// asks for it, the way the Slint example's `variant: ButtonVariant.primary`
    /// does.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// The size of both halves, [`ControlSize::Medium`] by default.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Draw both halves outlined, the Slint `outline` property.
    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    /// The leading half's icon, drawn before its label. It is a function rather
    /// than a value because the trigger is rebuilt from `&self` when the runtime
    /// asks for the overlay, and an `Element` — a boxed, statically dispatched
    /// widget — cannot be cloned.
    pub fn icon(mut self, icon: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.icon = Some(Rc::new(icon));
        self
    }

    /// The caret the trailing half draws, replacing the built-in
    /// `chevron-down`. The glyph belongs to the application, as every
    /// overridable icon in this crate does; the default is the authority's own
    /// mark, so a caller that says nothing still gets a disclosure.
    pub fn caret(mut self, caret: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.caret = Some(Rc::new(caret));
        self
    }

    /// The leading half's loading state, the Slint `loading` property. A
    /// loading control does not answer a press.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Draw both halves as selected, the Slint `selected` property. The caret
    /// half is drawn selected while the menu is open as well, which is the
    /// Slint `selected: root.selected || menu.is-open`.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// A control that answers nothing: both halves are drawn disabled and no
    /// popover is attached, so the menu cannot be opened at all.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Whether the menu is showing. The caller owns this, and
    /// [`on_open_change`](Self::on_open_change) is how the caller learns it
    /// should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The row drawn on the accent surface, by index. The menu reports the row
    /// the pointer entered through [`on_highlight`](Self::on_highlight); this is
    /// what draws it, because an element cannot keep the highlight itself.
    pub fn highlighted(mut self, highlighted: Option<usize>) -> Self {
        self.highlighted = highlighted;
        self
    }

    /// Report the row the pointer entered, by index.
    pub fn on_highlight(
        mut self,
        on_highlight: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        self.on_highlight = Some(Rc::new(on_highlight));
        self
    }

    /// The menu's rows, in the Slint `items` order.
    pub fn items(mut self, items: impl IntoIterator<Item = Item>) -> Self {
        self.items = items.into_iter().collect();
        self
    }

    /// The menu panel's width, [`DEFAULT_MENU_WIDTH`] by default.
    pub fn menu_width(mut self, width: f32) -> Self {
        self.menu_width = width;
        self
    }

    /// Report the chosen row: its index with [`DropdownAction::Menu`], or
    /// [`DropdownAction::Main`] with index 0 when the leading half was pressed.
    pub fn on_chosen(
        mut self,
        on_chosen: impl Fn(usize, DropdownAction) -> Message + 'a,
    ) -> Self {
        self.on_chosen = Some(Rc::new(on_chosen));
        self
    }

    /// Report every request to change the open state — a caret press wants
    /// `true`, an outside click or Escape wants `false` — the way
    /// [`Popover::on_open_change`] does.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    /// The leading half: the label, the icon, and the main action. It is a
    /// plain button, not part of the popover's trigger, so pressing it can never
    /// open the menu — the Slint control's `main` half.
    fn main_half(&self) -> Element<'a, Message> {
        let mut button = Button::new(self.label.clone())
            .variant(self.variant)
            .size(self.size)
            .loading(self.loading)
            .selected(self.selected)
            // The leading half owns the join's edge: its trailing corners are
            // square, so the two fills meet without a notch between them.
            .group_end(true);
        if self.outline {
            button = button.outline();
        }
        if let Some(icon) = &self.icon {
            button = button.icon(icon());
        }
        match &self.on_chosen {
            // Without a message the leading half is drawn disabled, which is
            // this crate's spelling for a control that cannot be pressed.
            Some(on_chosen) if !self.disabled => {
                let on_chosen = on_chosen.clone();
                button.on_press(on_chosen(0, DropdownAction::Main))
            }
            _ => button,
        }
        .into()
    }

    /// The trailing half: the caret, and the popover's whole trigger. Only this
    /// half opens the menu, which is what makes a split button two controls and
    /// not one wide one.
    fn caret_half(&self) -> Element<'a, Message> {
        let mut button = Button::new("")
            .variant(self.variant)
            .size(self.size)
            .selected(self.selected || self.open)
            // The trailing half squares the corners that meet the leading one,
            // so the pair reads as one control.
            .group_start(true);
        if self.outline {
            button = button.outline();
        }

        // The mark. The authority hardcodes `chevron-down` on the trigger, so
        // the built-in case draws it; the ink is read inside the style closure
        // because a colour sampled while the element is built is the light
        // theme's in the dark one.
        let ink_variant = self.variant;
        let ink_outline = self.outline;
        let ink_disabled = self.disabled;
        let caret: Element<'a, Message> = match &self.caret {
            Some(caret) => caret(),
            None => svg(svg::Handle::from_memory(CHEVRON_DOWN))
                .width(self.size.icon_size())
                .height(self.size.icon_size())
                .style(move |theme: &Theme, _status| svg::Style {
                    color: Some(if ink_disabled {
                        Tokens::of(theme).muted_foreground
                    } else {
                        button::ink(Tokens::of(theme), ink_variant, ink_outline)
                    }),
                })
                .into(),
        };
        button = button.icon(caret);

        // iced draws a `button` with no message as `Status::Disabled`, and the
        // press that opens the panel belongs to the [`Popover`] around this
        // half. Carrying the same request as the half's own message keeps the
        // half in the control's enabled fill and gives it the hover and press
        // states the leading half has; the popover publishes the identical
        // request on the same press, so the state moves once.
        if !self.disabled
            && let Some(on_open_change) = &self.on_open_change
        {
            button = button.on_press(on_open_change(true));
        }
        button.into()
    }

    /// The popover the whole control is: the caret as its trigger, the menu, and
    /// the two reports, hung beside the leading half. A disabled control gets no
    /// `on_open_change`, so its caret cannot open anything — iced's own spelling
    /// for a control that answers nothing.
    fn into_element(self) -> Element<'a, Message> {
        let main = self.main_half();
        let caret = self.caret_half();
        // A menu without a callback still needs one: the report is where the
        // control's own `on_chosen` lives. A control built without a callback
        // still draws its rows and still highlights them; only the report of a
        // choice has nowhere to go, so it publishes nothing.
        let on_chosen = self.on_chosen.clone();
        let mut menu = Menu::new(self.items)
            .selected(self.highlighted)
            .highlighted(self.highlighted)
            .on_chosen_maybe(move |index| {
                on_chosen
                    .as_ref()
                    .map(|on_chosen| on_chosen(index, DropdownAction::Menu))
            });
        if let Some(on_highlight) = self.on_highlight.clone() {
            menu = menu.on_highlight(move |index| on_highlight(index));
        }

        // One surface, and the popover owns it. `Menu::into_element` hands back
        // the body — the 4px inset and the rows — and draws no surface of its
        // own, so `filled(true)` here is the panel the eye sees, never a menu
        // border inside a popover border. `padding(0.0)` keeps that surface on
        // the body itself: the menu body carries the 4px inset, so a nonzero
        // popover inset would put the two rectangles at different sizes.
        let mut popover = Popover::new(caret)
            .filled(true)
            .padding(0.0)
            .placement(Placement::Bottom)
            .offset(4.0)
            .panel_width(self.menu_width)
            .content(menu.into_element())
            .open(self.open && !self.disabled);

        if !self.disabled
            && let Some(on_open_change) = self.on_open_change.clone()
        {
            popover = popover.on_open_change(move |open| on_open_change(open));
        }

        row![main, popover]
            .spacing(0.0)
            .align_y(Alignment::Center)
            .into()
    }
}

impl<'a, Message> From<DropdownButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(button: DropdownButton<'a, Message>) -> Self {
        button.into_element()
    }
}

/// The disclosure mark the authority hardcodes on the trigger, taken from the
/// icon set this repository already ships
/// (`crates/slint-component/ui/icons/chevron-down.svg`, Lucide, ISC). The
/// application owns its assets, but a select draws its own disclosure for the
/// same reason: a control whose mark is optional ships as an empty box.
const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;
