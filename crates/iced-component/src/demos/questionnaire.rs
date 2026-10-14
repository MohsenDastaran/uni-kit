//! `questionnaire` — a sequence of questions, one at a time: a progress bar, a
//! counter, prompts with a help line, described choices with keyboard badges, a
//! write-your-own row, an error line and the Back/Next pair, shown the way the
//! Slint example and the component page show it.
//!
//! Run it with `cargo run --example questionnaire`, or in the browser as
//! `?story=questionnaire` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The section titles are the component page's own `##` headings, in the order
//! the Slint example groups the same ground. The questionnaire owns no value, so
//! every section reports to this application and the next frame draws what it
//! stored: the step, the chosen index, the freeform draft and the error are all
//! fields of the [`Survey`] beside the demo. The validation section runs that
//! loop: **Next** with nothing answered writes the error line, and answering
//! clears it.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::group_box::{self, GroupBox};
use crate::components::questionnaire::{Choice, Question, Questionnaire};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — questionnaire")
        .theme(theme)
        .run()
}

/// One section's flow: the step, the chosen index, the freeform draft and the
/// step's error. The questionnaire holds none of them, so this is where every
/// section's state lives.
#[derive(Debug, Clone, Default)]
struct Survey {
    step: usize,
    answer: Option<usize>,
    free: String,
    error: String,
    finished: bool,
}

/// Which questionnaire a message belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Usage,
    Choices,
    Validation,
    Shortcuts,
    Small,
    Large,
    Composition,
    Progress,
    Navigation,
    Controlled,
    Card,
}

#[derive(Default)]
struct App {
    usage: Survey,
    choices: Survey,
    validation: Survey,
    shortcuts: Survey,
    small: Survey,
    large: Survey,
    composition: Survey,
    progress: Survey,
    navigation: Survey,
    controlled: Survey,
    card: Survey,
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl App {
    fn survey(&self, section: Section) -> &Survey {
        match section {
            Section::Usage => &self.usage,
            Section::Choices => &self.choices,
            Section::Validation => &self.validation,
            Section::Shortcuts => &self.shortcuts,
            Section::Small => &self.small,
            Section::Large => &self.large,
            Section::Composition => &self.composition,
            Section::Progress => &self.progress,
            Section::Navigation => &self.navigation,
            Section::Controlled => &self.controlled,
            Section::Card => &self.card,
        }
    }

    fn survey_mut(&mut self, section: Section) -> &mut Survey {
        match section {
            Section::Usage => &mut self.usage,
            Section::Choices => &mut self.choices,
            Section::Validation => &mut self.validation,
            Section::Shortcuts => &mut self.shortcuts,
            Section::Small => &mut self.small,
            Section::Large => &mut self.large,
            Section::Composition => &mut self.composition,
            Section::Progress => &mut self.progress,
            Section::Navigation => &mut self.navigation,
            Section::Controlled => &mut self.controlled,
            Section::Card => &mut self.card,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Choose(Section, usize),
    Free(Section, String),
    Previous(Section),
    Next(Section),
    Finish(Section),
    /// The controlled section's own Previous/Next buttons.
    Offset(i32),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App {
        // The bar's own section starts on its second question, as the Slint
        // example does.
        progress: Survey {
            step: 1,
            ..Survey::default()
        },
        ..App::default()
    }
}

/// The validation section's rule: nothing chosen and nothing written.
fn unanswered_validation(app: &App) -> bool {
    app.validation.answer.is_none() && app.validation.free.trim().is_empty()
}

/// The message that clears the current step and moves to the one after it, the
/// `step += 1; answer = 0; free-answer = ""` the Slint control runs.
fn advance(survey: &mut Survey) {
    survey.step += 1;
    survey.answer = None;
    survey.free.clear();
    survey.error.clear();
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Choose(section, index) => {
            let survey = app.survey_mut(section);
            survey.answer = Some(index);
            // Choosing an answer is what the validation section was waiting for.
            survey.error.clear();
        }
        Message::Free(section, value) => app.survey_mut(section).free = value,
        Message::Previous(section) => {
            let survey = app.survey_mut(section);
            if survey.step > 0 {
                survey.step -= 1;
                survey.answer = None;
                survey.free.clear();
                survey.error.clear();
            }
        }
        Message::Next(section) => {
            // The one section whose advance validates rather than moves: the
            // error is the application's string, which is all the questionnaire
            // knows about validity.
            if section == Section::Validation && unanswered_validation(app) {
                app.validation.error =
                    String::from("Choose an answer, or write one in the field above.");
            } else {
                advance(app.survey_mut(section));
            }
        }
        Message::Finish(section) => {
            // The validation section has a single question, so its primary action
            // is Submit and the same check has to run here.
            if section == Section::Validation && unanswered_validation(app) {
                app.validation.error =
                    String::from("Choose an answer, or write one in the field above.");
            } else {
                app.survey_mut(section).finished = true;
            }
        }
        Message::Offset(delta) => {
            let step = app.controlled.step as i32 + delta;
            app.controlled.step = step.clamp(0, 1) as usize;
        }
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| container::Style {
            text_color: Some(Tokens::of(theme).muted_foreground),
            ..container::Style::default()
        }),
    )
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    let icon = mouse_area(
        container(
            svg(svg::Handle::from_memory(CODE_XML))
                .width(16.0)
                .height(16.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).primary),
                }),
        )
        .width(28.0)
        .height(28.0)
        .center_x(28.0)
        .center_y(28.0)
        .style(move |theme: &Theme| {
            let primary = Tokens::of(theme).primary;
            let (fill, line) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(line))
                    .width(1.0),
                ..container::Style::default()
            }
        }),
    )
    .on_enter(Message::HoverSource(Some(title)))
    .on_exit(Message::HoverSource(None))
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    row![label, icon]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// Wire one section's flow: its values in, its messages out. Every section reads
/// its own [`Survey`], which is what keeps them independent.
fn wired<'a>(
    app: &App,
    section: Section,
    questions: impl IntoIterator<Item = Question>,
) -> Questionnaire<'a, Message> {
    let survey = app.survey(section);
    let mut control = Questionnaire::new()
        .id(format!("questionnaire-{:?}", section).to_lowercase())
        .questions(questions)
        .step(survey.step)
        .answer(survey.answer)
        .free_answer(survey.free.clone())
        .on_choose(move |index| Message::Choose(section, index))
        .on_free_answer(move |value| Message::Free(section, value))
        .on_previous(Message::Previous(section))
        .on_next(Message::Next(section))
        .on_finish(Message::Finish(section));
    if let Section::Validation = section {
        control = control.error(survey.error.clone());
    }
    control
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The flow the whole control is built for: a counter, a prompt with a
    // sentence under it, choices that say what they mean, a write-your-own row,
    // and a keyboard badge on each choice.
    let complete = wired(
        app,
        Section::Usage,
        [
            Question::new("What should we prototype next?")
                .help("Choose one direction or write your own.")
                .choices([
                    Choice::new("Delegation").detail("Show how work moves to a specialist."),
                    Choice::new("Question prompts")
                        .detail("Show choices while the interface waits."),
                    Choice::new("Both together"),
                ])
                .free_text("Type another direction…"),
            Question::new("Which tools belong in it?")
                .help("Pick the ones the prototype has to show.")
                .choices([
                    Choice::new("Editor").detail("The text surface and its menus."),
                    Choice::new("Terminal").detail("A running command and its output."),
                    Choice::new("Browser"),
                ])
                .free_text("Add another tool…"),
            Question::new("What tone should the interface use?")
                .help("This question can be skipped.")
                .choices([
                    Choice::new("Direct").detail("Short labels, no ceremony."),
                    Choice::new("Warm").detail("Sentences, and room to breathe."),
                ]),
        ],
    );

    let usage = column![
        complete,
        muted_line(if app.usage.finished {
            "Finished the flow."
        } else {
            "Answer, then Next — the last step submits."
        }),
    ]
    .spacing(12.0);

    // section: Choices
    // The plain case: a prompt and a list, no counter, no progress, no badges.
    let choices_only = wired(
        app,
        Section::Choices,
        [Question::new("How often do you ship?").choices([
            Choice::new("Daily"),
            Choice::new("Weekly"),
            Choice::new("Monthly"),
            Choice::new("Rarely"),
        ])],
    )
    .show_counter(false)
    .show_progress(false)
    .show_badges(false);

    // section: Validation
    // A rejected step: the message sits under the choices, above the actions, and
    // it is the application's string. **Next** writes it while nothing is
    // answered; answering clears it.
    let validation = wired(
        app,
        Section::Validation,
        [Question::new("Choose a public handle")
            .help("The validator rejects short handles.")
            .choices([
                Choice::new("Use my account name").detail("Taken from your profile."),
                Choice::new("Pick a new handle").detail("At least three characters."),
            ])
            .free_text("Public handle")],
    );

    // section: Keyboard shortcuts
    // The same choices with numbered badges instead of letters. The badges are
    // labels: the keys belong to the application, which this demo does not bind.
    let shortcuts = wired(
        app,
        Section::Shortcuts,
        [Question::new("Choose an answer or type your own")
            .help("The badges name the key that picks each choice.")
            .choices([
                Choice::new("First choice").detail("Press 1 to pick this."),
                Choice::new("Second choice").detail("Press 2 to pick this."),
                Choice::new("Third choice").detail("Press 3 to pick this."),
            ])
            .free_text("Type without triggering 1, 2, or 3…")],
    )
    .badges(["1", "2", "3"]);

    // section: Sizes and theming
    // The disc, the row padding and the prompt scale together, from the theme.
    let small = wired(
        app,
        Section::Small,
        [Question::new("Small")
            .choices([Choice::new("First choice"), Choice::new("Second choice")])],
    )
    .size(theme::ControlSize::Small);

    let large = wired(
        app,
        Section::Large,
        [Question::new("Large")
            .help("The disc, the row padding, and the prompt scale together.")
            .choices([
                Choice::new("First choice").detail("With a sentence under it."),
                Choice::new("Second choice"),
            ])],
    )
    .size(theme::ControlSize::Large);

    let sizes = column![small, large].spacing(24.0);

    // section: Composition
    // Only the parts a step needs: a prompt, its choices, and the actions.
    let composition = wired(
        app,
        Section::Composition,
        [Question::new("Confirm the target directory")
            .help("The control is only the question and its actions here.")
            .choices([
                Choice::new("ui/components").detail("Where an install puts files."),
                Choice::new("src/components").detail("The crate that builds the gallery."),
            ])],
    )
    .show_counter(false)
    .show_progress(false);

    // section: Progress
    // The bar on its own, with the counter turned off, on the second question.
    let progress = wired(
        app,
        Section::Progress,
        [
            Question::new("First question").choices([Choice::new("A"), Choice::new("B")]),
            Question::new("Second question").choices([Choice::new("A"), Choice::new("B")]),
            Question::new("Third question").choices([Choice::new("A"), Choice::new("B")]),
        ],
    )
    .show_counter(false);

    // section: Navigation and submission
    // Back and Next step through the list; the last step submits.
    let navigation = wired(
        app,
        Section::Navigation,
        [
            Question::new("Confirm the release channel")
                .choices([Choice::new("Stable"), Choice::new("Nightly")]),
            Question::new("Confirm the rollout")
                .choices([Choice::new("Everyone"), Choice::new("One team first")]),
        ],
    )
    .show_counter(false)
    .show_progress(false);

    let navigation = column![
        navigation,
        muted_line(if app.navigation.finished {
            "Submitted."
        } else {
            "The primary action reads Submit on the last step."
        }),
    ]
    .spacing(12.0);

    // section: Controlling the state
    // The host owns the step, so its own controls can drive the flow.
    let mut previous = Button::new("Previous question").outline();
    if app.controlled.step > 0 {
        previous = previous.on_press(Message::Offset(-1));
    }
    let mut next = Button::new("Next question").outline();
    if app.controlled.step < 1 {
        next = next.on_press(Message::Offset(1));
    }

    let controlled = wired(
        app,
        Section::Controlled,
        [
            Question::new("First")
                .help("Driven from outside.")
                .choices([Choice::new("A"), Choice::new("B")]),
            Question::new("Second")
                .help("The control's own buttons move this too.")
                .choices([Choice::new("A"), Choice::new("B")]),
        ],
    )
    .show_progress(false);

    let controlled = column![
        row![previous, next].spacing(8.0).align_y(Alignment::Center),
        controlled,
    ]
    .spacing(12.0);

    // section: Card and Dialog composition
    // The questionnaire keeps its progress and actions; the card owns the surface
    // around them.
    let card = Card::new()
        .title("Prototype brief")
        .description("The card owns the surface; the questionnaire owns the steps.")
        .body(wired(
            app,
            Section::Card,
            [Question::new("Who is the prototype for?").choices([
                Choice::new("The design team"),
                Choice::new("A new contributor"),
            ])],
        ));

    // section: end
    let status = text(
        "Click a choice, then Next — or a heading and its code icon; the page scrolls to that sample.",
    );

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Choices"),
        body_box(choices_only),
        heading(app, 2, "Validation"),
        body_box(validation),
        heading(app, 3, "Keyboard shortcuts"),
        body_box(shortcuts),
        heading(app, 4, "Sizes and theming"),
        body_box(sizes),
        heading(app, 5, "Composition"),
        body_box(composition),
        heading(app, 6, "Progress"),
        body_box(progress),
        heading(app, 7, "Navigation and submission"),
        body_box(navigation),
        heading(app, 8, "Controlling the state"),
        body_box(controlled),
        heading(app, 9, "Card and Dialog composition"),
        body_box(card),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
