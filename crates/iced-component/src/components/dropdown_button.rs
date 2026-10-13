//! A split button: an action half that runs its own message, and a caret half
//! that opens a [`Menu`]. Mirrors GPUI Component's `DropdownButton` and the
//! Slint `dropdown_button.slint` beside it.
//!
//! # One control, two halves
//!
//! Slint joins the halves with `spacing: -1px` and lets each square the edge
//! that meets the other, so the pair reads as one outline. iced 0.14 has no
//! negative spacing and this crate's [`Button`](super::button::Button) draws its
//! own radius on all four corners, so the two halves sit flush at zero spacing
//! with the seam the two edges make between them. That is the one place this
//! port is visibly not the reference, and it is named rather than faked with a
//! painted divider.
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

use iced::widget::row;
use iced::{Alignment, Element};

use super::button::{Button, Variant};
use super::menu::{Item, Menu};
use super::popover::{Placement, Popover};
use super::theme::ControlSize;

/// The menu panel's width when the caller names none. It is a little wider than
/// the medium control, so a menu of ordinary labels does not wrap, and it is
/// closer to the Slint panel's own 280px than to the button's width, because
/// iced's popover panel is a fixed width and Slint's `menu-width: 0px` means
/// "the menu's own".
pub const DEFAULT_MENU_WIDTH: f32 = 240.0;

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
    /// The caret glyph. The bytes belong to the application, as every icon in
    /// this crate does; a caller that leaves it out gets an empty caret half.
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
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> DropdownButton<'a, Message> {
    /// A split button whose leading half reads `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            caret: None,
            variant: Variant::Primary,
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

    /// The variant both halves are drawn in. [`Variant::Primary`] is the
    /// default, which is the Slint component's.
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

    /// The caret the trailing half draws, as [`icon`](Self::icon) does. The
    /// glyph belongs to the application: the icon set is not this crate's to
    /// resolve, which is why the Slint control's `icon: "chevron-down"` has no
    /// counterpart here.
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
        self.on_open_change = Some(Box::new(on_open_change));
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
            .selected(self.selected);
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
            .selected(self.selected || self.open);
        if self.outline {
            button = button.outline();
        }
        if let Some(caret) = &self.caret {
            button = button.icon(caret());
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

        let mut popover = Popover::new(caret)
            .filled(true)
            .placement(Placement::Bottom)
            .offset(4.0)
            .panel_width(self.menu_width)
            .content(menu.into_element())
            .open(self.open && !self.disabled);

        if !self.disabled
            && let Some(on_open_change) = self.on_open_change
        {
            popover = popover.on_open_change(on_open_change);
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
