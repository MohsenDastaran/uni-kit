//! The browser entry point.
//!
//! The gallery page reads `?story=` and calls `run` with it. It is the same
//! dispatcher the native binary uses, so a component page in the browser and
//! `cargo run --bin iced-gallery -- <story>` show the same demo rather than two
//! implementations of it.

use wasm_bindgen::prelude::*;

/// Run the demo `story` names, wearing the theme the page is wearing.
#[wasm_bindgen]
pub fn run(story: String, dark: bool) -> Result<(), JsValue> {
    let theme = if dark { iced::Theme::Dark } else { iced::Theme::Light };
    crate::demos::run(&story, theme).map_err(|error| JsValue::from_str(&error.to_string()))
}
