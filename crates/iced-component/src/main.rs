//! The gallery, natively.
//!
//! One iced application that shows the component named on the command line, the
//! same shape as the GPUI gallery: one module answering for every component
//! rather than a build per page. The browser build in `src/wasm.rs` calls the
//! same dispatcher, so what is on screen here is what a component page shows.
//!
//!   cargo run --bin iced-gallery -- message-role
//!   cargo run --bin iced-gallery -- button --dark
//!
//! With no name, or a name that matches nothing, it lists the components that
//! have a demo.

use iced_component::demos;

fn main() -> iced::Result {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    // The name is the first thing that is not a flag, so the flags can go
    // wherever they read best.
    let story = arguments
        .iter()
        .find(|argument| !argument.starts_with("--"))
        .cloned()
        .unwrap_or_default();
    let dark = arguments.iter().any(|argument| argument == "--dark");

    let theme = if dark { iced::Theme::Dark } else { iced::Theme::Light };
    demos::run(&story, theme)
}
