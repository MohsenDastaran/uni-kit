//! Opens one of the blocks in a window, so it can be looked at without writing
//! an application around it first.
//!
//! ```text
//! cargo run -p blocks -- sidebar
//! cargo run -p blocks -- dock
//! cargo run -p blocks -- settings
//! ```
//!
//! The viewer is a convenience for this repository. The registry never copies
//! it: `uni-kit add gpui <name>` installs the block file alone.

use blocks::{dock::DockBlock, settings::SettingsBlock, sidebar::SidebarBlock};
use gpui_kit::assets::Assets;
use gpui_kit::*;

const USAGE: &str = "\
usage: cargo run -p blocks -- <block>

blocks:
  sidebar   a navigation rail beside a data table
  dock      a toolbar over a dockable workspace, with a status bar
  settings  a preferences screen";

fn main() {
    let name = std::env::args().nth(1).unwrap_or_default();
    let app = gpui_kit::application().with_assets(Assets);

    match name.as_str() {
        "sidebar" => app.run(|cx| open(cx, SidebarBlock::view)),
        "dock" => app.run(|cx| open(cx, DockBlock::view)),
        "settings" => app.run(|cx| open(cx, SettingsBlock::view)),
        _ => {
            eprintln!("{USAGE}");
            std::process::exit(2);
        }
    }
}

/// Opens one size of window for every block: wide enough for the dock and the
/// rail, short enough to sit on a laptop screen beside an editor.
fn open<T: Render>(cx: &mut App, view: fn(&mut Window, &mut App) -> Entity<T>) {
    gpui_kit::init(cx);

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::centered(size(px(1100.), px(720.)), cx)),
        ..Default::default()
    };

    gpui_kit::open_window(options, cx, |window, cx| view(window, cx))
        .expect("Failed to open window");
}
