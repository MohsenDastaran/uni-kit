//! Term and detail pairs. Mirrors GPUI Component's `DescriptionList` and the
//! Slint version beside it: a label column beside each detail by default or a
//! stacked form, borders, separator rows, up to three columns, and per-item
//! spans.
//!
//! The registry key is `description-list`; this file and its module are
//! `description_list`, because a hyphen is not a Rust identifier.
//!
//! The insets are the ones GPUI uses at each size — small 4×2, medium 8×4,
//! large 12×6 — and the text is text-xs, text-sm and text-md with them. A
//! bordered list keeps the label fill and the vertical divider, so the label
//! column reads as its own cell.
//!
//! Slint packs a row by hand because its layout cannot express a span. Here a
//! cell's width is its span as a fraction of the row, which is what the same
//! layout produces, and a row is the run of items between two items that open
//! one.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// One term and its detail. `span` is how many of the list's columns the pair
/// takes, and `separator` makes the item a row of its own that draws the muted
/// band between groups.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Item {
    pub term: String,
    pub detail: String,
    pub span: usize,
    pub separator: bool,
}

impl Item {
    pub fn new(term: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            detail: detail.into(),
            span: 1,
            separator: false,
        }
    }

    /// How many columns the pair takes. The list clamps it to its own count.
    pub fn span(mut self, span: usize) -> Self {
        self.span = span;
        self
    }

    /// Draws this item as a separator band instead of a pair.
    pub fn separator() -> Self {
        Self {
            term: String::new(),
            detail: String::new(),
            span: 1,
            separator: true,
        }
    }
}

/// Whether the label sits above its detail or beside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Layout {
    #[default]
    Horizontal,
    Vertical,
}

/// A description list, built with the methods below.
pub struct DescriptionList<'a, Message> {
    items: Vec<Item>,
    layout: Layout,
    bordered: bool,
    columns: usize,
    label_width: f32,
    size: ControlSize,
    _message: std::marker::PhantomData<&'a Message>,
}

impl<'a, Message> DescriptionList<'a, Message> {
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        Self {
            items: items.into_iter().collect(),
            layout: Layout::default(),
            bordered: true,
            columns: 1,
            label_width: 140.0,
            size: ControlSize::default(),
            _message: std::marker::PhantomData,
        }
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Draws the surface border, the label fill and the row separators.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// One to three columns. The list clamps the count, as the Slint one does.
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    /// The width the label column aims at. A cell that is narrower than this
    /// keeps the label to about 45% of its own width, which is what lets
    /// "Description" stay on one line.
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    fn cols(&self) -> usize {
        self.columns.clamp(1, 3)
    }

    /// The stacked form: the vertical layout, or a narrow frame.
    ///
    /// Slint stacks below a 480px viewport, which it tracks as a global. iced
    /// has no viewport token, so this follows the layout alone and the caller
    /// chooses the vertical form on a narrow frame.
    fn stacks(&self) -> bool {
        self.layout == Layout::Vertical
    }

    /// The insets and the text size GPUI uses at each step.
    fn metrics(&self) -> (f32, f32, f32) {
        match self.size {
            ControlSize::Large => (12.0, 6.0, theme::FONT_SIZE_MD),
            ControlSize::Small | ControlSize::XSmall => {
                (4.0, 2.0, theme::FONT_SIZE_XS)
            }
            ControlSize::Medium => (8.0, 4.0, theme::FONT_SIZE_SM),
        }
    }

    /// How many columns an item takes, clamped to the list's own count.
    fn take(&self, index: usize) -> usize {
        match self.items.get(index) {
            None => 0,
            Some(item) if item.separator => self.cols(),
            Some(item) => item.span.clamp(1, self.cols()),
        }
    }

    /// The columns already used in the row the item at `index` would land in.
    /// Zero means the item opens a row. A separator always opens one, because
    /// it counts as a whole row.
    fn used_before(&self, index: usize) -> usize {
        let mut used = 0;
        for i in 0..index {
            if self.items[i].separator {
                used = 0;
                continue;
            }
            used += self.take(i);
            if used >= self.cols() {
                used = 0;
            }
        }
        used
    }

    /// Whether the item at `index` opens a row. A separator always does, and so
    /// does an item that does not fit in the columns the row has left.
    fn starts(&self, index: usize) -> bool {
        if index == 0 {
            return true;
        }
        if self.items[index].separator {
            return true;
        }
        self.used_before(index) == 0
    }
}

impl<'a, Message> From<DescriptionList<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: DescriptionList<'a, Message>) -> Self {
        let (pad_x, pad_y, text_size) = control.metrics();
        let bordered = control.bordered;
        let label_width = control.label_width;
        let columns = control.cols();
        let layout = control.layout;
        let items = control.items.clone();

        // The rows: an item that opens one, followed by the items that fit
        // beside it.
        let mut bands: Vec<Vec<usize>> = Vec::new();
        let mut index = 0;
        while index < items.len() {
            if !control.starts(index) {
                index += 1;
                continue;
            }
            let mut band = vec![index];
            if layout != Layout::Vertical {
                let mut used = control.take(index);
                let mut next = index + 1;
                while next < items.len()
                    && !items[next].separator
                    && !control.starts(next)
                    && used + control.take(next) <= columns
                {
                    used += control.take(next);
                    band.push(next);
                    next += 1;
                }
            }
            bands.push(band);
            index += 1;
        }

        let mut list = column![].width(Length::Fill);
        let last_band = bands.len().saturating_sub(1);

        for (band_index, band) in bands.iter().enumerate() {
            let starts_with_separator = items[band[0]].separator;
            let mut cells: Vec<Element<'a, Message>> = Vec::new();

            if starts_with_separator {
                cells.push(
                    container(
                        container(iced::widget::Space::new())
                            .height(Length::Fixed(8.0))
                            .width(Length::Fill),
                    )
                    .width(Length::Fill)
                    .style(move |theme: &Theme| ContainerStyle {
                        background: Some(if bordered {
                            Tokens::of(theme).description_label.into()
                        } else {
                            iced::Background::Color(iced::Color::TRANSPARENT)
                        }),
                        ..ContainerStyle::default()
                    })
                    .into(),
                );
            } else {
                for &i in band {
                    cells.push(pair(
                        &items[i],
                        layout == Layout::Vertical,
                        bordered,
                        label_width,
                        pad_x,
                        pad_y,
                        text_size,
                        control.take(i),
                        layout != Layout::Vertical,
                    ));
                }
            }

            let row: Element<'a, Message> = if starts_with_separator {
                cells.pop().expect("a separator band has one cell")
            } else {
                row(cells)
                    .width(Length::Fill)
                    .spacing(0.0)
                    .into()
            };

            let row: Element<'a, Message> = if bordered && !starts_with_separator
                && band_index != last_band
            {
                container(row)
                    .width(Length::Fill)
                    .style(|theme: &Theme| ContainerStyle {
                        border: Border {
                            color: Tokens::of(theme).border,
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        ..ContainerStyle::default()
                    })
                    .into()
            } else {
                row
            };

            list = list.push(row);
        }

        let surface = container(list)
            .width(Length::Fill)
            .padding(if bordered { 1.0 } else { 0.0 })
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                ContainerStyle {
                    border: Border {
                        color: tokens.border,
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: if bordered {
                            theme::RADIUS_MD.into()
                        } else {
                            0.0.into()
                        },
                    },
                    ..ContainerStyle::default()
                }
            });

        surface.into()
    }
}

/// One cell: the label and the detail, beside each other or stacked.
#[allow(clippy::too_many_arguments)]
fn pair<'a, Message: 'a>(
    item: &Item,
    stacked: bool,
    bordered: bool,
    label_width: f32,
    pad_x: f32,
    pad_y: f32,
    text_size: f32,
    span: usize,
    as_column: bool,
) -> Element<'a, Message> {
    let term = item.term.clone();
    let detail = item.detail.clone();

    let label = container(
        text(term)
            .size(text_size)
            .style(move |theme: &Theme| text::Style {
                color: Some(if bordered {
                    Tokens::of(theme).description_label_foreground
                } else {
                    Tokens::of(theme).foreground
                }),
            }),
    )
    .width(Length::Fill)
    .padding([pad_y, pad_x])
    .style(move |theme: &Theme| ContainerStyle {
        background: if bordered {
            Some(Tokens::of(theme).description_label.into())
        } else {
            None
        },
        ..ContainerStyle::default()
    });

    let value = container(
        text(detail)
            .size(text_size)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    )
    .width(Length::Fill)
    .padding([pad_y, pad_x]);

    let cell: Element<'a, Message> = if stacked {
        // The stacked form fills the row, keeps the label fill, and closes the
        // label cell with a border underneath it.
        column![
            container(label)
                .width(Length::Fill)
                .style(move |theme: &Theme| ContainerStyle {
                    border: Border {
                        color: Tokens::of(theme).border,
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: 0.0.into(),
                    },
                    ..ContainerStyle::default()
                }),
            value
        ]
        .width(Length::Fill)
        .into()
    } else {
        // The label keeps GPUI's right border inside its own width, so the
        // value cell starts at the label width.
        row![
            container(label)
                .width(Length::Fixed(label_width))
                .style(move |theme: &Theme| ContainerStyle {
                    border: Border {
                        color: if bordered {
                            Tokens::of(theme).border
                        } else {
                            iced::Color::TRANSPARENT
                        },
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: 0.0.into(),
                    },
                    ..ContainerStyle::default()
                }),
            value
        ]
        .width(Length::Fill)
        .into()
    };

    if as_column {
        container(cell)
            .width(Length::FillPortion(span as u16))
            .into()
    } else {
        container(cell).width(Length::Fill).into()
    }
}

/// A separator band, for a list a caller assembles without [`Item::separator`].
pub fn separator<'a, Message: 'a>(bordered: bool) -> Element<'a, Message> {
    container(iced::widget::Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(8.0))
        .style(move |theme: &Theme| ContainerStyle {
            background: Some(if bordered {
                Tokens::of(theme).description_label.into()
            } else {
                iced::Background::Color(iced::Color::TRANSPARENT)
            }),
            ..ContainerStyle::default()
        })
        .into()
}
