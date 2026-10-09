# blocks

Composed GPUI screens that [`uni-kit`](../../registry) copies into an
application:

```bash
npx @dastaran/uni-kit@latest add gpui sidebar
bunx @dastaran/uni-kit@latest add gpui sidebar
pnpm dlx @dastaran/uni-kit@latest add gpui sidebar
```

| Block | Screen | The file it writes |
| --- | --- | --- |
| `sidebar` | a navigation rail beside a data table | `src/components/sidebar.rs` |
| `dock` | a toolbar over a dockable workspace, with a status bar | `src/components/dock.rs` |
| `settings` | a preferences screen | `src/components/settings.rs` |

Each command writes one file and adds its `pub mod` line to
`src/components/mod.rs`. Add `mod components;` to the crate root to compile
what was installed.

A GPUI application gets every control from the `gpui-kit` crate, so what is
worth copying for this framework is a screen: the controls arranged for a
purpose, with the spacing, grouping and keyboard path already decided. The
files import `gpui_kit` and `std`, use the built-in `IconName` set, and never
reach for a gallery helper or another block, so installing one of them alone is
enough.

Look at one without writing an application around it:

```bash
cargo run -p blocks -- sidebar
cargo run -p blocks -- dock
cargo run -p blocks -- settings
```

The viewer is a convenience for this repository. The registry never copies it:
an install is the block file and nothing else.
