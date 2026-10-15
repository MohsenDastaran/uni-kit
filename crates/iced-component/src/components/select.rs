//! A closed list of options. Mirrors GPUI Component's `Select` and the Slint
//! `Select` beside it, and is the overlay family's first composition: the field's
//! frame, the popover's floating surface, and the menu's rows.
//!
//! # What it composes
//!
//! The Slint file is the authority, and it is three pieces with one line each:
//! a trigger shaped like a text field (`Theme.input-height`,
//! `Theme.input-padding-x`, `Theme.input-font-size`, `Theme.input`), a
//! `PopupWindow` whose panel is the popover fill with `radius-lg`, a one-pixel
//! border and the shadow, and a column of rows inside a 4px inset with the
//! selected row checked. None of those is built here:
//!
//! - the trigger's frame is [`input::Style`], the styled frame `input.rs` draws
//!   its own field with, resolved from the same tokens. The field's own height
//!   comes from [`ControlSize::input_height`], the table the Slint theme spells
//!   `input-height(size)`, so a select and an input beside it are the same box.
//!   Slint's select spells its own default `Theme.background`; the field's
//!   `input-background` is the token that separates a trigger from the page in
//!   both appearances, which is why the composition takes the field's frame;
//! - the floating surface is [`Popover`], asked for `filled(true)` and
//!   `padding(0.0)` so the panel's one surface is the menu body's own inset;
//! - the rows are [`Menu`], the item shape the menu and the dropdown button
//!   already share: label, icon, shortcut, disabled and the check mark, with the
//!   selected row drawn on the accent surface and its check in the trailing slot.
//!
//! # One container owns the surface
//!
//! The panel is the popover's, exactly as [`Menu`]'s own module comment requires:
//! `Menu::into_element` hands back the body — the 4px inset and the rows — and
//! draws no surface of its own, so `filled(true)` here is the only fill, edge and
//! radius the eye sees. Drawing a `MenuStyle` as well would put a second rounded
//! border a few pixels inside the first.
//!
//! # The state is the caller's
//!
//! iced 0.14 keeps no state a component can own across frames, so this port has
//! none either: the open flag, the chosen index and the highlighted index are the
//! application's, and each is both drawn from and reported back. `Select::open`
//! is what the panel is rendered from and [`Select::on_open_change`] is the only
//! way a trigger click, an outside click or Escape is reported — the popover
//! underneath supplies all three. [`Select::selected`] is the chosen row, and
//! [`Select::on_chosen`] is a row's click. The highlight, which a menu cannot keep
//! in the element tree it is rebuilt into, is [`Select::highlighted`] and
//! [`Select::on_highlight`], and both the pointer and any key the application
//! binds move it. The pointer's report is a hover and is transient, so
//! [`Select::on_leave`] is its other half; the keyboard's row is the caller's
//! own and is not cleared by it — see [`Menu::on_highlight`].
//!
//! A click on a row reports the choice and closes nothing: the caller answers with
//! its own `open` and `selected`, which is what the Slint example does when its
//! row sets `root.selected`, calls `changed` and closes the popup.
//!
//! # What the Slint component has that this does not
//!
//! - **The panel is a fixed width, not the trigger's.** Slint's `menu-width: 0`
//!   means "the trigger's own width"; [`Select::menu_width`] takes a `0` the same
//!   way, but a `Length` has no resolved width until layout and iced's
//!   [`Popover`] sizes its panel before then. A fixed trigger width is the one
//!   case where the panel matches ([`Select::width`] with `Length::Fixed`); a
//!   filling trigger falls back to the menu's preferred 280px.
//! - **The keyboard belongs to the application.** iced 0.14 has no focus trap and
//!   no focus handle an overlay can take, so `↑`/`↓`/`Home`/`End`/`Enter`/`Space`
//!   are bound by the host and move [`Select::highlighted`], exactly as they are
//!   for the menu. Escape and a click outside are the popover's for every overlay
//!   in this crate and need no binding. Slint's `Select` has no keyboard either.
//! - **No focus ring and no open ring on the trigger.** A container resolves its
//!   fill and edge from the theme alone; the frame has no focus state to read, so
//!   `input.rs`'s field and this trigger both keep their resting edge.
//! - **No elision.** iced 0.14 draws no ellipsis, so a value wider than the
//!   trigger clips at `wrapping: None` where Slint elides. The dropdown button
//!   names the same gap.
//! - **`searchable`, `cleanable`, `menu_max_h`, `accessibility_label` and the
//!   empty-state element are GPUI-only.** The Slint component the catalogue
//!   exposes has no search field, no clear control, no maximum menu height, no
//!   accessible name and no custom empty element, so none is invented here; a
//!   select with no items draws the empty panel [`Menu`] draws, which the menu's
//!   own comment already promises `select` composes.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{container, row, svg, text};
use iced::{Alignment, Color, Element, Length, Theme};

use super::input;
use super::menu::{self, Item, Menu};
use super::popover::{Placement, Popover};
use super::theme::{self, ControlSize, Tokens};

/// One colour of the trigger's surface, as a function of the theme.
///
/// The appearance in force is not known where the trigger is built — a container
/// resolves its style inside the frame — so a surface colour has to be a function
/// of the theme rather than a value, the same reason [`Popover::panel`] takes one.
/// A value resolved once would be the wrong colour in the other appearance.
type ColorFor<'a> = Box<dyn Fn(&Theme) -> Color + 'a>;

/// A closed list of options, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is a
/// [`Popover`] whose trigger is the field-shaped frame and whose panel is a
/// [`Menu`] — which is exactly what the Slint control is: an input-shaped row
/// with a `PopupWindow` holding the rows.
pub struct Select<'a, Message> {
    items: Vec<Item>,
    /// The chosen row, or nothing while the placeholder shows. The Slint
    /// component spells it `selected: -1`.
    selected: Option<usize>,
    placeholder: String,
    /// Written before the selected label, so the trigger can say what the value
    /// is: `"Country: Portugal"`.
    title_prefix: String,
    size: ControlSize,
    disabled: bool,
    appearance: bool,
    open: bool,
    /// The row the caller says is highlighted — the row under the pointer, or the
    /// one a key moved to. See the module comment for why it is the caller's.
    highlighted: Option<usize>,
    width: Length,
    /// The panel's width, or `0` for the trigger's own where it is known. See the
    /// module comment.
    menu_width: f32,
    radius: Option<f32>,
    /// The trigger's fill and edge, as functions of the theme, because the
    /// appearance is not known where the trigger is built — a container resolves
    /// its style inside the frame. [`input::Style`] is the default.
    background: Option<ColorFor<'a>>,
    border_color: Option<ColorFor<'a>>,
    /// The trailing glyph. The chevron is built in, because a select without a
    /// disclosure mark is not recognisable; a caller with its own icon set
    /// replaces it, as [`DropdownButton`](super::dropdown_button::DropdownButton)
    /// leaves its caret to the caller.
    caret: Option<Rc<dyn Fn() -> Element<'a, Message> + 'a>>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_chosen: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_highlight: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_leave: Option<Rc<dyn Fn(Option<usize>) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> Select<'a, Message> {
    /// A select over `items`, with nothing chosen and the panel closed.
    ///
    /// The rows are [`Item`]s, the shape the menu and the dropdown button already
    /// use, so an option can carry an icon or a shortcut. A [`Item::separator`]
    /// is a band between groups of options; it is not actionable and is never the
    /// chosen row.
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        Self {
            items: items.into_iter().collect(),
            selected: None,
            placeholder: String::from("Select…"),
            title_prefix: String::new(),
            size: ControlSize::Medium,
            disabled: false,
            appearance: true,
            open: false,
            highlighted: None,
            width: Length::Fill,
            menu_width: 0.0,
            radius: None,
            background: None,
            border_color: None,
            caret: None,
            on_open_change: None,
            on_chosen: None,
            on_highlight: None,
            on_leave: None,
        }
    }

    /// The chosen row, the Slint component's `selected`. `None` shows the
    /// placeholder, and the chosen row is drawn on the accent surface with its
    /// check mark, which is what the Slint row does when `selected == index`.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// The text the trigger shows while nothing is chosen, `"Select…"` by
    /// default, which is the Slint component's own default.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// A label for the value rather than the value alone, written before it:
    /// `title_prefix("Country: ")` shows `Country: Portugal`. The Slint
    /// `title-prefix`.
    pub fn title_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.title_prefix = prefix.into();
        self
    }

    /// The size of the trigger, [`ControlSize::Medium`] by default. It picks the
    /// height, the inset, the type and the chevron together, the way the Slint
    /// component's `size` picks `Theme.input-height`, `input-padding-x`,
    /// `input-font-size` and `icon-size` together.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// A select that answers nothing: the trigger is drawn muted and the popover
    /// is told no open state, so it cannot be opened. It is the Slint `enabled:
    /// false`, drawn the way this crate draws a disabled field rather than at
    /// half opacity, which iced 0.14 has no subtree control for.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Draw the trigger's frame. `false` drops the fill, the edge and the radius,
    /// which is the Slint `appearance` and is for a host that supplies the
    /// surface itself.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// Whether the panel is showing. The caller owns it, and
    /// [`on_open_change`](Self::on_open_change) is how the caller learns it
    /// should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The row drawn on the accent surface: the row the pointer is over, or the
    /// one a key moved to. The caller owns it; see the module comment.
    pub fn highlighted(mut self, highlighted: Option<usize>) -> Self {
        self.highlighted = highlighted;
        self
    }

    /// The trigger's width, `Length::Fill` by default so the control spans the
    /// form column it is placed in, which is the Slint `preferred-width: 100%`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The panel's width. `0`, the default, means the trigger's own width where
    /// it is known — the Slint `menu-width: 0px` — and a fixed trigger width is
    /// the only case iced can answer; a filling trigger falls back to the menu's
    /// preferred 280px. See the module comment.
    pub fn menu_width(mut self, width: f32) -> Self {
        self.menu_width = width;
        self
    }

    /// The trigger's corner radius. The default is the field's `radius-md`,
    /// which is what the Slint component's `radius` defaults to through
    /// `Theme.radius-md`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// The trigger's fill, as a function of the theme, because the appearance in
    /// force is not known where the trigger is built. The default is the input
    /// frame's fill, `input-background`.
    ///
    /// ```ignore
    /// Select::new(items).background(|theme| Tokens::of(theme).primary.scale_alpha(0.08))
    /// ```
    pub fn background(mut self, background: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.background = Some(Box::new(background));
        self
    }

    /// The trigger's edge, as a function of the theme, for the reason
    /// [`background`](Self::background) takes one. The default is the input
    /// frame's edge, the `input` token.
    pub fn border_color(mut self, border_color: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.border_color = Some(Box::new(border_color));
        self
    }

    /// The disclosure glyph at the trailing edge, replacing the built-in
    /// chevrons. The glyph belongs to the application, as every icon in this
    /// crate does except the few the control itself needs.
    pub fn caret(mut self, caret: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.caret = Some(Rc::new(caret));
        self
    }

    /// Report every request to change the open state: a trigger click wants
    /// `true`, an outside click or Escape wants `false`. A disabled select is
    /// given no callback, so its trigger cannot open anything — this crate's
    /// spelling for a control that answers nothing.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Report the row that was chosen, by index, on a click. The caller owns the
    /// items and asks itself what index 2 means; the Slint `changed(index)` and
    /// iced's own `combo_box` report the same number.
    ///
    /// The panel is not closed by a choice. The caller answers with its own
    /// `open`, exactly as the Slint example's row sets `selected`, calls
    /// `changed` and closes the popup.
    pub fn on_chosen(mut self, on_chosen: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_chosen = Some(Rc::new(on_chosen));
        self
    }

    /// Report the row the pointer entered, by index, the way
    /// [`Menu::on_highlight`] does. The caller draws it by passing the same index
    /// to [`highlighted`](Self::highlighted).
    ///
    /// It is a **hover**, as it is on the menu, so it is transient: pass the
    /// matching [`on_leave`](Self::on_leave) to clear it. A composer that also
    /// owns the row a key moved to keeps the two apart, or the pointer's leave
    /// takes the keyboard's row with it.
    pub fn on_highlight(mut self, on_highlight: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_highlight = Some(Rc::new(on_highlight));
        self
    }

    /// Report that the pointer left a row, or left the panel: `Some(index)` is
    /// the row it left and `None` is the panel as a whole, the same report
    /// [`Menu::on_leave`] makes and for the same reason. It is the half that
    /// clears the hover [`on_highlight`](Self::on_highlight) sets; clear only
    /// the row it was on, because a lower row's leave follows an upper row's
    /// entry.
    pub fn on_leave(mut self, on_leave: impl Fn(Option<usize>) -> Message + 'a) -> Self {
        self.on_leave = Some(Rc::new(on_leave));
        self
    }

    /// The whole control: the Popover that floats the Menu, with the
    /// field-shaped trigger.
    fn into_element(self) -> Element<'a, Message> {
        let Select {
            items,
            selected,
            placeholder,
            title_prefix,
            size,
            disabled,
            appearance,
            open,
            highlighted,
            width,
            menu_width,
            radius,
            background,
            border_color,
            caret,
            on_open_change,
            on_chosen,
            on_highlight,
            on_leave,
        } = self;

        // The chosen row carries the check mark, which is where the Slint row
        // puts `Icon { name: "check" }` when `selected == index`. The mark is the
        // select's own state, so a checked option keeps its check and the chosen
        // one gains it.
        let items: Vec<Item> = items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                if Some(index) == selected && !item.is_separator() {
                    item.checked(true)
                } else {
                    item
                }
            })
            .collect();

        let has_selection = selected.is_some();
        let label = match selected.and_then(|index| items.get(index)) {
            Some(item) => format!("{title_prefix}{}", item.label()),
            None => placeholder,
        };

        // Slint's `menu-width: 0` means the trigger's own width; a fixed trigger
        // width is the only case iced can answer before layout. See the module
        // comment.
        let panel_width = if menu_width > 0.0 {
            menu_width
        } else if let Length::Fixed(fixed) = width {
            fixed
        } else {
            menu::PANEL_WIDTH
        };

        let caret: Element<'a, Message> = match caret {
            Some(caret) => caret(),
            None => svg(svg::Handle::from_memory(CHEVRONS_UP_DOWN))
                .width(size.icon_size())
                .height(size.icon_size())
                .style(move |theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                })
                .into(),
        };

        let trigger = container(
            row![
                text(label)
                    .size(size.input_font_size())
                    .width(Length::Fill)
                    // iced 0.14 draws no ellipsis, so the value clips at the
                    // trigger's edge where Slint elides. See the module comment.
                    .wrapping(text::Wrapping::None),
                caret,
            ]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center)
            // The row has to fill the trigger's height: a `Shrink` row is
            // aligned to the top of the fixed-height frame, so `align_y` would
            // centre the label and the caret against the row's own top edge and
            // the row would ride high. `input.rs` fills its row for the same
            // reason.
            .height(Length::Fill)
            .width(Length::Fill),
        )
        .padding([0.0, size.input_padding_x()])
        .height(Length::Fixed(size.input_height()))
        .width(width)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            // The frame is the field's: the same fill, edge and radius `input.rs`
            // draws its own frame with, resolved from the same tokens. It is
            // `pub` for exactly this, a caller drawing a matching frame around a
            // field-shaped control.
            let mut style = input::Style::new()
                .appearance(appearance)
                .disabled(disabled)
                .resolve(tokens);
            if let Some(radius) = radius {
                style.border.radius = radius.into();
            }
            if let Some(background) = &background {
                style.background = Some(background(theme).into());
            }
            if let Some(border_color) = &border_color {
                style.border.color = border_color(theme);
            }
            // A chosen value reads as a value, a placeholder and a disabled value
            // read as muted — the Slint `color: selected >= 0 ? Theme.foreground
            // : Theme.muted-foreground`, and this crate's disabled ink.
            style.text_color = Some(if disabled {
                tokens.muted_foreground
            } else if has_selection {
                tokens.foreground
            } else {
                tokens.muted_foreground
            });
            style
        });

        // One surface, and the popover owns it. `Menu::into_element` hands back
        // the body — the 4px inset and the rows — and draws no surface of its
        // own, so `filled(true)` here is the panel the eye sees, never a menu
        // border inside a popover border. `padding(0.0)` keeps that surface on
        // the body itself: the body carries the 4px inset, so a nonzero popover
        // inset would put the two rectangles at different sizes. The gap is the
        // Slint popup's `y: root.height + 4px`.
        let mut popover = Popover::new(trigger)
            .open(open && !disabled)
            .placement(Placement::Bottom)
            .offset(4.0)
            .panel_width(panel_width)
            .filled(true)
            .padding(0.0);

        // A disabled control gets no report, so its trigger cannot open
        // anything: iced's own spelling for a control that answers nothing.
        if !disabled && let Some(on_open_change) = on_open_change {
            popover = popover.on_open_change(on_open_change);
        }

        let mut menu = Menu::new(items).selected(selected).highlighted(highlighted);
        if let Some(on_chosen) = on_chosen.clone() {
            menu = menu.on_chosen(move |index| on_chosen(index));
        }
        if let Some(on_highlight) = on_highlight {
            menu = menu.on_highlight(move |index| on_highlight(index));
        }
        if let Some(on_leave) = on_leave {
            menu = menu.on_leave(move |row| on_leave(row));
        }

        popover.content(menu.into_element()).into()
    }
}

impl<'a, Message> From<Select<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(select: Select<'a, Message>) -> Self {
        select.into_element()
    }
}

/// The disclosure mark at the trigger's trailing edge, drawn from
/// `crates/slint-component/ui/icons/chevrons-up-down.svg` (Lucide, ISC), so the
/// two galleries draw the same glyph rather than two lookalikes. A row's own
/// icons are the caller's; this one glyph is the exception a select's disclosure
/// mark needs, the same exception [`Menu`] makes for its check.
const CHEVRONS_UP_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#;
