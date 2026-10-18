//! Telling the page which sample was asked for.
//!
//! The gallery runs in a frame on a component page. The code icon beside a
//! section, and the section's own title, ask that page to scroll to the sample —
//! so the click has to cross the frame boundary. This is the same bridge the
//! Slint and GPUI galleries use, with the same payload shape:
//!
//! ```json
//! {"source": "gpui-kit", "index": 0, "title": "Variants"}
//! ```
//!
//! The page checks that the message came from one of its own example frames
//! before doing anything with it, which is what makes the payload safe to accept
//! from a string.
//!
//! Natively there is no page to tell: the call does nothing, so a demo can be run
//! and clicked through without a browser.

/// Ask the embedding page to show the sample at `index`, named `title`.
#[cfg(target_arch = "wasm32")]
pub fn show(index: usize, title: &str) {
    use wasm_bindgen::JsValue;

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(parent) = window.parent().ok().flatten() else {
        return;
    };
    // Not embedded: the gallery is in a tab of its own, and there is nothing to
    // scroll. The windows are compared as JavaScript values — annotated, because
    // a browser window can be viewed as several types — which is exactly how the
    // Slint and GPUI bridges test the same thing.
    let parent_value: &JsValue = parent.as_ref();
    let window_value: &JsValue = window.as_ref();
    if parent_value == window_value {
        return;
    }
    let Ok(origin) = window.location().origin() else {
        return;
    };
    // The same tag the Slint and GPUI galleries post: the page's rule is "a
    // message from one of my example frames", not "from this framework", and it
    // verifies the frame itself before believing anything in here.
    let payload = format!(
        r#"{{"source":"gpui-kit","index":{index},"title":{}}}"#,
        json_string(title)
    );
    let _ = parent.post_message(&JsValue::from_str(&payload), &origin);
}

/// A JSON string literal, because the payload is assembled by hand.
#[cfg(target_arch = "wasm32")]
fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

#[cfg(not(target_arch = "wasm32"))]
pub fn show(_index: usize, _title: &str) {}
