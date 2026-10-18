//! `data_table` — a typed, interactive table: sortable headers, row selection,
//! status-coloured cells, stripes and the size ladder.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example data_table`.

fn main() -> iced::Result {
    iced_component::demos::data_table::run(iced::Theme::Light)
}
