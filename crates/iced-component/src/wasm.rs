//! The browser entry point.
//!
//! The gallery page reads `?story=` and calls `run` with it. It is the same
//! dispatcher the native binary uses, so a component page in the browser and
//! `cargo run --bin iced-gallery -- <story>` show the same demo rather than two
//! implementations of it.

use iced::Color;
use wasm_bindgen::prelude::*;

use crate::components::theme::{self, Tokens};

/// Run the demo `story` names, wearing the theme the page is wearing.
///
/// Standalone, `dark` is the gallery's own light-or-dark choice. Embedded, the
/// frame wears the palette of the page around it — read from that page, the
/// same route the Slint gallery takes — and falls back to `dark` only when the
/// page publishes no palette at all.
#[wasm_bindgen]
pub fn run(story: String, dark: bool) -> Result<(), JsValue> {
    let palette = web::host_palette();
    let theme = match &palette {
        Some((name, tokens)) => {
            theme::set_site_palette(Some(*tokens));
            theme::theme_for(name, tokens)
        }
        None => theme::iced_theme(dark),
    };
    crate::demos::run(&story, theme).map_err(|error| JsValue::from_str(&error.to_string()))
}

/// The documentation page this gallery is embedded in.
mod web {
    use wasm_bindgen::JsValue;

    use super::{Color, Tokens};

    /// The palette the page around the frame is drawn in, with the name that
    /// page gives it.
    ///
    /// A component page publishes its theme as the CSS custom properties its
    /// navbar applies to `<html>`, so the frame reads them from the parent
    /// document instead of being handed them: a frame that is in the page is
    /// drawn in the page, whatever the page happens to be. `None` standalone,
    /// and also when the page publishes neither a background nor a foreground,
    /// which is the signal to keep the built-in light and dark tables.
    pub(super) fn host_palette() -> Option<(String, Tokens)> {
        let window = web_sys::window()?;
        let parent = window.parent().ok().flatten()?;
        // Not embedded: the gallery is in a tab of its own, and there is no
        // page to read. The windows are compared as JavaScript values, the way
        // the Slint and GPUI galleries compare them.
        let window_value: &JsValue = window.as_ref();
        let parent_value: &JsValue = parent.as_ref();
        if window_value == parent_value {
            return None;
        }
        let root = parent.document()?.document_element()?;
        let style = parent.get_computed_style(&root).ok().flatten()?;
        let color = |name: &str| -> Option<Color> {
            let value = style.get_property_value(name).ok()?;
            parse_css_color(&value)
        };
        let tokens = Tokens::from_site(root.class_list().contains("dark"), color)?;
        let name = root
            .get_attribute("data-theme-name")
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "uni-kit page".to_string());
        Some((name, tokens))
    }

    /// A colour from one of the forms the site's theme tokens take: `#rgb`,
    /// `#rgba`, `#rrggbb`, `#rrggbbaa`, or `rgb()`/`rgba()` with comma- or
    /// space-separated channels. A specified custom property computes to its
    /// token, so the hex forms are what the pages in this repository publish;
    /// the function form covers a page that states one directly. The Slint
    /// gallery parses the same two families from the same computed values.
    fn parse_css_color(value: &str) -> Option<Color> {
        let value = value.trim();
        if let Some(hex) = value.strip_prefix('#') {
            return parse_hex(hex);
        }
        let inner = value
            .strip_prefix("rgba(")
            .or_else(|| value.strip_prefix("rgb("))?
            .trim_end_matches(')')
            .trim();
        let parts: Vec<&str> = inner
            .split(|character: char| character == ',' || character.is_whitespace())
            .filter(|part| !part.is_empty())
            .collect();
        let channel = |text: &str| text.parse::<f32>().ok().map(|value| value / 255.0);
        // The fourth channel is already an opacity: a fraction, or a percentage.
        let opacity = |text: &str| match text.strip_suffix('%') {
            Some(percent) => percent.parse::<f32>().ok().map(|value| value / 100.0),
            None => text.parse::<f32>().ok(),
        };
        Some(Color {
            r: channel(parts.first()?)?,
            g: channel(parts.get(1)?)?,
            b: channel(parts.get(2)?)?,
            a: parts.get(3).copied().and_then(opacity).unwrap_or(1.0),
        })
    }

    /// `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa`, one nibble or two per channel.
    fn parse_hex(hex: &str) -> Option<Color> {
        let byte = |index: usize| {
            u8::from_str_radix(hex.get(index..index + 2)?, 16)
                .ok()
                .map(|value| f32::from(value) / 255.0)
        };
        let nibble = |index: usize| {
            u8::from_str_radix(hex.get(index..index + 1)?, 16)
                .ok()
                .map(|value| f32::from(value * 17) / 255.0)
        };
        match hex.len() {
            3 | 4 => Some(Color {
                r: nibble(0)?,
                g: nibble(1)?,
                b: nibble(2)?,
                a: if hex.len() == 4 { nibble(3)? } else { 1.0 },
            }),
            6 | 8 => Some(Color {
                r: byte(0)?,
                g: byte(2)?,
                b: byte(4)?,
                a: if hex.len() == 8 { byte(6)? } else { 1.0 },
            }),
            _ => None,
        }
    }
}
