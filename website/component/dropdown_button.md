---
title: DropdownButton
description: A DropdownButton is a combination of a button and a trigger button. It allows us to display a dropdown menu when the trigger is clicked, but the left Button can still respond to independent events.
---

# DropdownButton

A [DropdownButton] is a combination of a button and a trigger button. It allows us to display a dropdown menu when the trigger is clicked, but the left Button can still respond to independent events.

Shared variant and size can be set on the DropdownButton. Action-specific options such as its label, icon, tooltip, loading state and click handler belong to the inner [Button].

## Import

```rust
use gpui_kit::component::button::{Button, DropdownButton};
```

## Usage

```rust
use gpui_kit::Anchor;

DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
            .menu("Option 2", Box::new(MyAction))
            .separator()
            .menu("Option 3", Box::new(MyAction))
    })
```

## Basic split

The control is two buttons sharing one outline. The leading half runs its own
action; the trailing half opens the menu, so a click on the label never opens
the menu by accident.

```rust
DropdownButton::new("dropdown")
    .primary()
    .button(Button::new("btn").label("Export").on_click(|_, _, _| println!("Export")))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Export all rows (.csv)", Box::new(MyAction))
            .menu("Download report (.pdf)", Box::new(MyAction))
    })
```

### Variants

Same as [Button], DropdownButton supports different variants.

```rust
DropdownButton::new("dropdown")
    .primary()
    .button(Button::new("btn").label("Primary"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
```

Leaving the variant or size unset on the DropdownButton uses the inner button's value for both halves.

### Inner button options

```rust
DropdownButton::new("dropdown")
    .button(
        Button::new("btn")
            .label("Save")
            .compact()
            .loading(is_saving)
            .tooltip("Save the current view")
            .on_click(|_, _, _| println!("Saved")),
    )
    .dropdown_menu(|menu, _, _| {
        menu.menu("Save as…", Box::new(MyAction))
    })
```

## With custom anchor

The menu's anchor and width belong to the menu, not to the button. Set them on
the DropdownButton when the default placement does not fit.

```rust
DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu_with_anchor(Anchor::BottomRight, |menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
```

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html
[DropdownButton]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.DropdownButton.html
[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html
