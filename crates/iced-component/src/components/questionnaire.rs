//! A stepped questionnaire: a sequence of questions, one at a time, with a
//! progress bar, a counter, single-choice answers, an optional write-your-own
//! field and an error line. Mirrors the Slint `Questionnaire`, which is the
//! authority here; GPUI Component's `Questionnaire` and its compound parts are
//! the reference behind it.
//!
//! This file adds no control of its own. A question is a [`Field`] whose label
//! is the prompt, whose description is the help line and whose message line is
//! the error; its choices are [`Radio`]s, its write-your-own row is an [`Input`],
//! the position is a [`Progress`], and Back, Next and Submit are [`Button`]s.
//! The Slint control draws its own radio disc only so a choice can carry a
//! description and a keyboard badge, which is exactly what the composition here
//! gets from [`Radio::description`] beside a small badge box.
//!
//! **The caller owns the flow.** The step, the chosen index, the freeform draft
//! and the error are values the caller passes in, and every interaction is a
//! message it takes back — the same division [`Pagination`](super::pagination)
//! and [`Stepper`](super::stepper) use, rather than the in-out properties the
//! Slint control mutates. That is what makes Back, Next and Submit the caller's
//! own buttons: the component reports which was pressed, and the caller writes
//! the step it means to move to.
//!
//! **What iced 0.14 cannot draw, and what is not faked:**
//!
//! - **No keyboard shortcuts.** Slint only draws the badges here and leaves the
//!   keys to the example; GPUI's `QuestionnaireState` binds letters and numbers
//!   to the active item. iced reports focus to the element rather than to the
//!   application, so a bare control has no key event to answer, and the badges
//!   are labels: the caller binds the keys, as the demo says it does not.
//! - **The padding around a choice is not part of its press target.** The row's
//!   surface is a `container` and the only pressable thing in it is the
//!   [`Radio`]'s handle, which spans the disc and its text. Slint makes the whole
//!   row one `TouchArea`; iced has no way to widen a built control's hit box
//!   without nesting a second button, which would report the same press twice.
//! - **The help line's place and its pairing with the error are the [`Field`]'s.**
//!   A field stacks its label, its control and one message line, so the help sits
//!   under the choices rather than under the prompt, and a set error takes its
//!   place — where Slint draws the help under the prompt and its error line
//!   beside it.
//! - **A single-selection set only.** GPUI's `with_multiple` keeps every chosen
//!   answer with checkbox semantics; the Slint authority is single-choice, so
//!   that is what this port is, and a multiple item is not faked with a radio.
//! - **No validation and no branching.** The error is a string the caller sets,
//!   as in Slint; the schema is fixed at construction and the caller disables a
//!   conditional item by not answering it.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Border, Element, Length, Theme};

use super::button::{Button, Variant};
use super::form::Field;
use super::input::Input;
use super::progress::Progress;
use super::radio::Radio;
use super::theme::{self, ControlSize, Tokens};

/// One answer a question offers: a line, and a sentence that says what it means.
pub struct Choice {
    title: String,
    detail: String,
}

impl Choice {
    /// A choice with only a title. Slint's `QuestionChoice.detail` is optional
    /// and an empty one draws nothing.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            detail: String::new(),
        }
    }

    /// The supporting sentence under the title, in the muted colour.
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }
}

/// One step: what it asks, how it helps, what it offers, and the placeholder of
/// its write-your-own row when it has one. Slint's `Question`.
pub struct Question {
    prompt: String,
    help: String,
    choices: Vec<Choice>,
    free_text: String,
}

impl Question {
    /// A question with no choices and no write-your-own row.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            help: String::new(),
            choices: Vec::new(),
            free_text: String::new(),
        }
    }

    /// The supporting line under the prompt, Slint's `help`.
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = help.into();
        self
    }

    /// One choice, in reading order.
    pub fn choice(mut self, choice: Choice) -> Self {
        self.choices.push(choice);
        self
    }

    /// The choices, in reading order.
    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.choices.extend(choices);
        self
    }

    /// The placeholder of the write-your-own row. An empty one hides the row,
    /// which is Slint's `free-text`.
    pub fn free_text(mut self, free_text: impl Into<String>) -> Self {
        self.free_text = free_text.into();
        self
    }
}

/// A stepped questionnaire, built with the methods below.
///
/// The caller owns every value the control shows — the active step, the chosen
/// index, the freeform draft and the error — and the builders below report each
/// interaction as a message. [`on_choose`](Self::on_choose) is what a choice
/// reports; [`on_free_answer`](Self::on_free_answer) is an edit of the
/// write-your-own field; and [`on_previous`](Self::on_previous),
/// [`on_next`](Self::on_next) and [`on_finish`](Self::on_finish) are the three
/// actions, with the component choosing between the last two by whether the
/// current step is the last. A control built without a message is inert, which
/// is how iced spells disabled, so a questionnaire with no actions draws its
/// buttons unpressed.
pub struct Questionnaire<'a, Message> {
    questions: Vec<Question>,
    step: usize,
    answer: Option<usize>,
    free_answer: String,
    error: String,
    badges: Vec<String>,
    show_badges: bool,
    show_progress: bool,
    show_counter: bool,
    size: ControlSize,
    id: String,
    on_choose: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_free_answer: Option<Rc<dyn Fn(String) -> Message + 'a>>,
    on_previous: Option<Message>,
    on_next: Option<Message>,
    on_finish: Option<Message>,
}

impl<'a, Message> Questionnaire<'a, Message> {
    /// A questionnaire with no questions, at the first step, with the counter,
    /// the progress bar and the badges showing — the Slint defaults.
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
            step: 0,
            answer: None,
            free_answer: String::new(),
            error: String::new(),
            badges: ('A'..='H').map(String::from).collect(),
            show_badges: true,
            show_progress: true,
            show_counter: true,
            size: ControlSize::default(),
            id: String::from("questionnaire"),
            on_choose: None,
            on_free_answer: None,
            on_previous: None,
            on_next: None,
            on_finish: None,
        }
    }

    /// One question, in reading order.
    pub fn question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }

    /// The questions, in reading order.
    pub fn questions(mut self, questions: impl IntoIterator<Item = Question>) -> Self {
        self.questions.extend(questions);
        self
    }

    /// The zero-based step on screen. A step past the last question draws the
    /// last one, so a caller that lags a navigation cannot index nothing.
    pub fn step(mut self, step: usize) -> Self {
        self.step = step;
        self
    }

    /// The chosen choice, or nothing when the question is still unanswered.
    pub fn answer(mut self, answer: Option<usize>) -> Self {
        self.answer = answer;
        self
    }

    /// What the write-your-own row holds. The caller reads it alongside
    /// `answer`, which is Slint's `free-answer`.
    pub fn free_answer(mut self, free_answer: impl Into<String>) -> Self {
        self.free_answer = free_answer.into();
        self
    }

    /// A validation message for the current step. An empty one draws nothing;
    /// the value is the caller's, as it is in Slint.
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = error.into();
        self
    }

    /// The keyboard badges, one per choice. Replace them for numbered
    /// shortcuts; a choice past the end of the list draws no badge.
    pub fn badges(mut self, badges: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.badges = badges.into_iter().map(Into::into).collect();
        self
    }

    /// Whether the badge box is drawn at all, `true` by default.
    pub fn show_badges(mut self, show_badges: bool) -> Self {
        self.show_badges = show_badges;
        self
    }

    /// Whether the progress bar is drawn, `true` by default.
    pub fn show_progress(mut self, show_progress: bool) -> Self {
        self.show_progress = show_progress;
        self
    }

    /// Whether the "Question 2 of 4" counter is drawn, `true` by default.
    pub fn show_counter(mut self, show_counter: bool) -> Self {
        self.show_counter = show_counter;
        self
    }

    /// The size step the whole questionnaire takes — the prompt, the rows, the
    /// disc, the field and the actions together.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The base name of the write-your-own field, `"questionnaire"` by default.
    /// Two questionnaires on one page need different names, because iced keeps a
    /// text field's caret and focus under its id and would otherwise share them.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// The choice that was picked, as its index within the current question. The
    /// step is not reported because the caller already holds it.
    pub fn on_choose(mut self, on_choose: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_choose = Some(Rc::new(on_choose));
        self
    }

    /// The write-your-own field was edited.
    pub fn on_free_answer(mut self, on_free_answer: impl Fn(String) -> Message + 'a) -> Self {
        self.on_free_answer = Some(Rc::new(on_free_answer));
        self
    }

    /// Back was pressed. Without this the control is disabled at every step.
    pub fn on_previous(mut self, on_previous: Message) -> Self {
        self.on_previous = Some(on_previous);
        self
    }

    /// Next was pressed on a step that is not the last.
    pub fn on_next(mut self, on_next: Message) -> Self {
        self.on_next = Some(on_next);
        self
    }

    /// The primary action was pressed on the last step. Slint's `finished`.
    pub fn on_finish(mut self, on_finish: Message) -> Self {
        self.on_finish = Some(on_finish);
        self
    }
}

impl<'a, Message> Default for Questionnaire<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The row padding for a size step: 8 / 11 / 14, Slint's `row-pad`.
fn row_pad(size: ControlSize) -> f32 {
    match size {
        ControlSize::Small => theme::SPACING_SM,
        ControlSize::Large => 14.0,
        _ => 11.0,
    }
}

/// A choice row's surface: the page as the fill, and the primary colour in the
/// edge and a six-percent tint once the row is chosen — the `Questionnaire` row
/// Slint draws. Every token is read from the theme inside the closure, so the
/// dark appearance is not light tokens resolved at build time.
fn row_surface(tokens: Tokens, selected: bool) -> container::Style {
    container::Style {
        background: Some(
            if selected {
                theme::alpha(tokens.primary, 0.06)
            } else {
                tokens.background
            }
            .into(),
        ),
        border: Border {
            color: if selected {
                tokens.primary
            } else {
                tokens.border
            },
            width: 1.0,
            radius: theme::RADIUS_MD.into(),
        },
        ..container::Style::default()
    }
}

/// The keyboard badge: a 22px box in the muted surface, Slint's badge.
fn badge<'a, Message: 'a>(label: &str) -> Element<'a, Message> {
    container(
        text(label.to_string())
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(22.0)
    .height(22.0)
    .center_x(22.0)
    .center_y(22.0)
    .style(|theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(tokens.muted.into()),
            border: Border {
                color: tokens.border,
                width: 1.0,
                radius: theme::RADIUS_SM.into(),
            },
            ..container::Style::default()
        }
    })
    .into()
}

/// One choice: the radio disc and its two lines, and the badge at the far edge.
/// The disc, the label, the detail and the chosen mark are the [`Radio`]'s; this
/// only draws the row's surface and places the badge.
fn choice_row<'a, Message: Clone + 'a>(
    choice: &Choice,
    selected: bool,
    badge_label: Option<&str>,
    size: ControlSize,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let mut control = Radio::new(selected).label(choice.title.clone()).size(size);
    if !choice.detail.is_empty() {
        control = control.description(choice.detail.clone());
    }
    if let Some(message) = on_press {
        control = control.on_change(move |_| message.clone());
    }

    let mut content = row![Element::from(control)]
        .width(Length::Fill)
        .align_y(Alignment::Start);

    if let Some(badge_label) = badge_label {
        content = content.push(Space::new().width(Length::Fill));
        content = content.push(badge(badge_label));
    }

    container(content)
        .width(Length::Fill)
        .padding(row_pad(size))
        .style(move |theme: &Theme| row_surface(Tokens::of(theme), selected))
        .into()
}

impl<'a, Message> From<Questionnaire<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Questionnaire<'a, Message>) -> Self {
        let Questionnaire {
            questions,
            step,
            answer,
            free_answer,
            error,
            badges,
            show_badges,
            show_progress,
            show_counter,
            size,
            id,
            on_choose,
            on_free_answer,
            on_previous,
            on_next,
            on_finish,
        } = control;

        let total = questions.len();
        // A caller that lags a navigation cannot index nothing, which is the one
        // difference from the Slint control's own `questions[step]`.
        let step = if total == 0 { 0 } else { step.min(total - 1) };

        let fraction = if total == 0 {
            0.0
        } else {
            (step + 1) as f32 / total as f32
        };

        let mut body = column![].spacing(theme::SPACING_LG).width(Length::Fill);

        if show_progress {
            body = body.push(Progress::new(fraction));
        }

        if total > 0 {
            let question = &questions[step];

            let mut choices = column![].spacing(theme::SPACING_SM).width(Length::Fill);

            for (index, choice) in question.choices.iter().enumerate() {
                let on_press = on_choose.as_ref().map(|on_choose| on_choose(index));
                let badge_label = (show_badges)
                    .then(|| badges.get(index).map(String::as_str))
                    .flatten();
                choices = choices.push(choice_row(
                    choice,
                    answer == Some(index),
                    badge_label,
                    size,
                    on_press,
                ));
            }

            let mut field = Field::new()
                .label(question.prompt.clone())
                .description(question.help.clone())
                .error(error.clone())
                .size(size)
                .child(Element::from(choices));

            if !question.free_text.is_empty() {
                let mut input = Input::new(format!("{id}-free"))
                    .value(free_answer.clone())
                    .placeholder(question.free_text.clone())
                    .size(size)
                    .width(Length::Fill);
                if let Some(on_free_answer) = on_free_answer.as_ref() {
                    // Wrapped rather than passed through: an `Rc<dyn Fn>` is
                    // callable but does not itself implement `Fn`, which is the
                    // bound the field's builder asks for.
                    let on_free_answer = on_free_answer.clone();
                    input = input.on_input(move |value| on_free_answer(value));
                }
                field = field.child(Element::from(input));
            }

            let mut block = column![].spacing(theme::SPACING_XS).width(Length::Fill);

            if show_counter {
                block = block.push(
                    text(format!("Question {} of {total}", step + 1))
                        .size(theme::FONT_SIZE_XS)
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).muted_foreground),
                        }),
                );
            }

            block = block.push(Element::from(field));
            body = body.push(block);
        }

        let last = total > 0 && step + 1 >= total;

        let mut previous = Button::new("Back").size(size);
        if step > 0 {
            if let Some(message) = on_previous.clone() {
                previous = previous.on_press(message);
            }
        }

        let mut advance = Button::new(if last { "Submit" } else { "Next" })
            .variant(Variant::Primary)
            .size(size);
        if total > 0 {
            let message = if last {
                on_finish.clone()
            } else {
                on_next.clone()
            };
            if let Some(message) = message {
                advance = advance.on_press(message);
            }
        }

        body = body.push(
            row![
                Element::from(previous),
                Space::new().width(Length::Fill),
                Element::from(advance),
            ]
            .spacing(theme::SPACING_SM)
            .width(Length::Fill),
        );

        body.into()
    }
}
