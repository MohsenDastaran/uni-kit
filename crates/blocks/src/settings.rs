//! A preferences screen: the heading that says what saving will do, then the
//! groups the rows belong to.
//!
//! The rows are the point of the screen, so they are [`SettingItem`]s inside
//! [`SettingGroup`]s on a [`SettingPage`], handed to [`Settings`]. The state
//! lives in this view rather than in a global, so the block drops into an
//! application as it stands: `Reset` puts every field back, `Save changes` is
//! where the application would write them.
//!
//! ```text
//! npx @dastaran/uni-kit@latest add gpui settings
//! ```
//!
//! The command writes `src/components/settings.rs` and adds `pub mod settings;`
//! to `src/components/mod.rs`. Add `mod components;` to the crate root, then:
//!
//! ```rust,ignore
//! use components::settings::SettingsBlock;
//!
//! gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
//!     SettingsBlock::view(window, cx)
//! });
//! ```

use gpui_kit::component::{
    ActiveTheme as _, Disableable as _, Icon, IconName, Sizable as _, StyledExt as _, Theme,
    ThemeMode,
    button::{Button, ButtonVariants as _},
    h_flex,
    setting::{NumberFieldOptions, SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    v_flex,
};
use gpui_kit::{
    App, AppContext as _, Axis, Context, Div, Entity, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, div,
};

/// What `Reset` puts back. The values are constants rather than a second copy
/// of the struct, so a field and its reset cannot say different things.
const DEFAULT_DENSITY: &str = "Comfortable";
const DEFAULT_FONT_SIZE: f64 = 14.0;
const DEFAULT_CLI_PATH: &str = "/usr/local/bin/uni-kit";

/// The block. Every field is read by a [`SettingField`] closure and written by
/// the matching setter, so the screen and the state cannot drift.
pub struct SettingsBlock {
    dark_mode: bool,
    /// The mode the window opened in. A reset means "what it was", which is not
    /// the same as "light" for an application that starts dark.
    initial_dark: bool,
    reduce_motion: bool,
    density: SharedString,
    font_size: f64,
    cli_path: SharedString,
    notifications: bool,
    sounds: bool,
    weekly_summary: bool,
    auto_update: bool,
    /// Whether anything has changed since the last save. The heading reads it.
    dirty: bool,
}

/// A switch or checkbox row: one accessor to read the field and one to write it.
///
/// Two clones of the entity rather than one, because the closures are `'static`
/// and each owns what it reads from.
fn switch(
    view: &Entity<SettingsBlock>,
    get: fn(&SettingsBlock) -> bool,
    set: fn(&mut SettingsBlock, bool),
) -> SettingField<bool> {
    let read = view.clone();
    let write = view.clone();

    SettingField::switch(
        move |cx: &App| get(read.read(cx)),
        move |value: bool, cx: &mut App| {
            write.update(cx, |this, cx| {
                set(this, value);
                this.dirty = true;
                cx.notify();
            });
        },
    )
}

/// A dropdown row over [`SharedString`] values.
fn dropdown(
    view: &Entity<SettingsBlock>,
    options: Vec<(SharedString, SharedString)>,
    get: fn(&SettingsBlock) -> SharedString,
    set: fn(&mut SettingsBlock, SharedString),
) -> SettingField<SharedString> {
    let read = view.clone();
    let write = view.clone();

    SettingField::dropdown(
        options,
        move |cx: &App| get(read.read(cx)),
        move |value: SharedString, cx: &mut App| {
            write.update(cx, |this, cx| {
                set(this, value);
                this.dirty = true;
                cx.notify();
            });
        },
    )
}

/// A single-line text row.
fn text(
    view: &Entity<SettingsBlock>,
    get: fn(&SettingsBlock) -> SharedString,
    set: fn(&mut SettingsBlock, SharedString),
) -> SettingField<SharedString> {
    let read = view.clone();
    let write = view.clone();

    SettingField::input(
        move |cx: &App| get(read.read(cx)),
        move |value: SharedString, cx: &mut App| {
            write.update(cx, |this, cx| {
                set(this, value);
                this.dirty = true;
                cx.notify();
            });
        },
    )
}

/// A number row.
fn number(
    view: &Entity<SettingsBlock>,
    options: NumberFieldOptions,
    get: fn(&SettingsBlock) -> f64,
    set: fn(&mut SettingsBlock, f64),
) -> SettingField<f64> {
    let read = view.clone();
    let write = view.clone();

    SettingField::number_input(
        options,
        move |cx: &App| get(read.read(cx)),
        move |value: f64, cx: &mut App| {
            write.update(cx, |this, cx| {
                set(this, value);
                this.dirty = true;
                cx.notify();
            });
        },
    )
}

impl SettingsBlock {
    /// The view to hand to a window:
    /// `cx.new(|cx| SettingsBlock::new(window, cx))`.
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let initial_dark = cx.theme().mode.is_dark();

        Self {
            dark_mode: initial_dark,
            initial_dark,
            reduce_motion: false,
            density: DEFAULT_DENSITY.into(),
            font_size: DEFAULT_FONT_SIZE,
            cli_path: DEFAULT_CLI_PATH.into(),
            notifications: true,
            sounds: false,
            weekly_summary: false,
            auto_update: true,
            dirty: false,
        }
    }

    /// The same view, already built.
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    /// Put every field back to the value it started with.
    fn reset(&mut self, cx: &mut Context<Self>) {
        self.dark_mode = self.initial_dark;
        self.reduce_motion = false;
        self.density = DEFAULT_DENSITY.into();
        self.font_size = DEFAULT_FONT_SIZE;
        self.cli_path = DEFAULT_CLI_PATH.into();
        self.notifications = true;
        self.sounds = false;
        self.weekly_summary = false;
        self.auto_update = true;
        self.dirty = false;

        // A preferences screen also shows the running application, so putting
        // the theme mode back is part of the reset.
        Theme::change(
            if self.initial_dark {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
            None,
            cx,
        );
        cx.notify();
    }

    /// Where an application would write the settings to disk.
    fn save(&mut self, cx: &mut Context<Self>) {
        self.dirty = false;
        cx.notify();
    }

    /// The pages, each group a heading with the rows under it.
    fn pages(&self, cx: &mut Context<Self>) -> Vec<SettingPage> {
        let view = cx.entity();

        // The one row whose write reaches beyond this view: choosing the dark
        // palette applies it, so the rest of the window follows the switch.
        let dark_mode = {
            let read = view.clone();
            let write = view.clone();
            SettingField::switch(
                move |cx: &App| read.read(cx).dark_mode,
                move |value: bool, cx: &mut App| {
                    write.update(cx, |this, cx| {
                        this.dark_mode = value;
                        this.dirty = true;
                        Theme::change(
                            if value {
                                ThemeMode::Dark
                            } else {
                                ThemeMode::Light
                            },
                            None,
                            cx,
                        );
                    });
                },
            )
        };

        vec![
            SettingPage::new("General")
                .icon(Icon::new(IconName::Settings2))
                .default_open(true)
                .groups(vec![
                    SettingGroup::new().title("Appearance").items(vec![
                        SettingItem::new("Dark mode", dark_mode)
                            .description("Use the dark palette for the whole application."),
                        SettingItem::new(
                            "Reduce motion",
                            switch(
                                &view,
                                |this| this.reduce_motion,
                                |this, value| this.reduce_motion = value,
                            ),
                        )
                        .description("Skip non-essential animation."),
                        SettingItem::new(
                            "Density",
                            dropdown(
                                &view,
                                vec![
                                    (DEFAULT_DENSITY.into(), DEFAULT_DENSITY.into()),
                                    ("Compact".into(), "Compact".into()),
                                ],
                                |this| this.density.clone(),
                                |this, value| this.density = value,
                            ),
                        )
                        .description("How much room each row takes."),
                    ]),
                    SettingGroup::new().title("Editor").items(vec![
                        SettingItem::new(
                            "Font size",
                            number(
                                &view,
                                NumberFieldOptions {
                                    min: 8.0,
                                    max: 72.0,
                                    ..Default::default()
                                },
                                |this| this.font_size,
                                |this, value| this.font_size = value,
                            ),
                        )
                        .description("Between 8 and 72 points."),
                        SettingItem::new(
                            "CLI path",
                            text(
                                &view,
                                |this| this.cli_path.clone(),
                                |this, value| this.cli_path = value,
                            ),
                        )
                        .layout(Axis::Vertical)
                        .description("The executable the terminal panel runs."),
                    ]),
                ]),
            SettingPage::new("Notifications")
                .icon(Icon::new(IconName::Bell))
                .groups(vec![SettingGroup::new().title("Alerts").items(vec![
                    SettingItem::new(
                        "Desktop alerts",
                        switch(
                            &view,
                            |this| this.notifications,
                            |this, value| this.notifications = value,
                        ),
                    )
                    .description("Show a banner when something happens."),
                    SettingItem::new(
                        "Sounds",
                        switch(&view, |this| this.sounds, |this, value| this.sounds = value),
                    )
                    .description("Play a sound for a new message."),
                    SettingItem::new(
                        "Weekly summary",
                        switch(
                            &view,
                            |this| this.weekly_summary,
                            |this, value| this.weekly_summary = value,
                        ),
                    )
                    .description("Email a digest every Monday."),
                ])]),
            SettingPage::new("Software update")
                .icon(Icon::new(IconName::Cpu))
                .groups(vec![
                    SettingGroup::new().title("Updates").items(vec![
                        SettingItem::new(
                            "Install updates automatically",
                            switch(
                                &view,
                                |this| this.auto_update,
                                |this, value| this.auto_update = value,
                            ),
                        )
                        .description("Download and install a release when one is published."),
                    ]),
                    SettingGroup::new().title("Version").items(vec![
                        // Some settings are an action rather than a value, and
                        // the field renderer is where one goes.
                        SettingItem::render(|options, _, _| {
                            h_flex()
                                .w_full()
                                .items_center()
                                .justify_between()
                                .gap_3()
                                .child("Uni Kit 0.7.1 is the version installed.")
                                .child(
                                    Button::new("check-updates")
                                        .icon(IconName::RefreshCw)
                                        .label("Check for updates")
                                        .outline()
                                        .with_size(options.size()),
                                )
                                .into_any_element()
                        })
                        .keywords(["version", "update"]),
                    ]),
                ]),
        ]
    }

    /// The heading. It says what the screen is for, whether anything is waiting
    /// to be written, and gives the two actions that answer it.
    fn heading(&self, cx: &mut Context<Self>) -> Div {
        let dirty = self.dirty;

        h_flex()
            .w_full()
            .items_center()
            .gap_3()
            .child(
                v_flex()
                    .min_w_0()
                    .flex_1()
                    .gap_1()
                    .child(div().text_xl().font_semibold().child("Preferences"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(if dirty {
                                "Changes apply to this device as soon as you save."
                            } else {
                                "All changes are saved."
                            }),
                    ),
            )
            .child(
                Button::new("reset-settings")
                    .icon(IconName::Undo2)
                    .label("Reset")
                    .outline()
                    .on_click(cx.listener(|this, _, _, cx| this.reset(cx))),
            )
            .child(
                Button::new("save-settings")
                    .icon(IconName::Check)
                    .label("Save changes")
                    .primary()
                    .disabled(!dirty)
                    .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
            )
    }
}

impl Render for SettingsBlock {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let heading = self.heading(cx);
        let pages = self.pages(cx);

        v_flex()
            .size_full()
            .gap_4()
            .p_4()
            .bg(cx.theme().background)
            .child(heading)
            .child(
                div()
                    .min_h_0()
                    .flex_1()
                    .child(Settings::new("settings-block").pages(pages)),
            )
    }
}
