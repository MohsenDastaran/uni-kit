---
title: Collapsible
description: An interactive element which expands/collapses.
---

# Collapsible

An interactive element which expands/collapses.

## Import

```rust
use gpui_kit::component::collapsible::Collapsible;
```

## Usage
### Details

```rust
Collapsible::new()
    .max_w_128()
    .gap_1()
    .open(self.open)
    .child(
        "This is a collapsible component. \
        Click the header to expand or collapse the content.",
    )
    .content(
        "This is the full content of the Collapsible component. \
        It is only visible when the component is expanded. \n\
        You can put any content you like here, including text, images, \
        or other UI elements.",
    )
    .child(
        h_flex().justify_center().child(
            Button::new("toggle1")
                .icon(IconName::ChevronDown)
                .label("Show more")
                .when(open, |this| {
                    this.icon(IconName::ChevronUp).label("Show less")
                })
                .xsmall()
                .link()
                .on_click({
                    cx.listener(move |this, _, _, cx| {
                        this.open = !this.open;
                        cx.notify();
                    })
                }),
        ),
    )
```

We can use `open` method to control the collapsed state. If false, the `content` method added child elements will be hidden.

### Animated reveal

Opt into a reversible, measured height reveal with a stable motion ID:

```rust
Collapsible::new()
    .motion_id("advanced-options")
    .open(self.open)
    .content(options)
```

The content remains mounted while closed so it can be measured and immediately reverse if toggled mid-animation. Without `motion_id`, the component keeps the immediate mount/unmount behavior. See the [GPUI Base Motion guide](../base/motion.md) for timing, reduced-motion, and performance details.

### Basic

A trigger beside the title, with a summary that stays visible.

```rust
Collapsible::new()
    .open(self.is_open("order"))
    .child(
        h_flex()
            .justify_between()
            .child("Order #4189")
            .child(
                Button::new("order")
                    .ghost()
                    .xsmall()
                    .icon(IconName::ChevronsUpDown)
                    .on_click(cx.listener(|this, _, _, cx| this.toggle("order", cx))),
            ),
    )
    .child(status_row)
    .content(order_details)
```

### Row trigger

The whole row is the trigger.

```rust
Collapsible::new()
    .open(self.is_open("faq"))
    .child(
        h_flex()
            .id("faq")
            .justify_between()
            .on_click(cx.listener(|this, _, _, cx| this.toggle("faq", cx)))
            .child("How do I reset my password?")
            .child(chevron),
    )
    .content("Click the Forgot Password link on the sign in page.")
```

### Bottom trigger

The trigger sits on the bottom edge of the card it opens.

```rust
Collapsible::new()
    .open(self.is_open("usage"))
    .child(usage_summary)
    .content(usage_breakdown)
```

### Settings

Holds optional controls, keeping the default view short.

```rust
Collapsible::new()
    .open(self.is_open("settings"))
    .child(
        Button::new("settings")
            .outline()
            .label("Notification settings")
            .on_click(cx.listener(|this, _, _, cx| this.toggle("settings", cx))),
    )
    .content(notification_checkboxes)
```

### Row actions

Actions live beside the trigger, in the header and in every row.

```rust
Collapsible::new()
    .open(self.is_open("api-keys"))
    .child(
        h_flex()
            .child(api_keys_trigger)
            .child(Button::new("add-key").ghost().xsmall().icon(IconName::Plus)),
    )
    .content(api_key_rows)
```

### Nested

Panels nest to any depth.

```rust
Collapsible::new()
    .open(self.is_open("components-dir"))
    .child(folder_row("components"))
    .content(
        Collapsible::new()
            .open(self.is_open("ui-dir"))
            .child(folder_row("ui"))
            .content(v_flex().children(["button.rs", "card.rs", "dialog.rs"])),
    )
```

### Profile

Shows who someone is, and their details only on request.

```rust
Collapsible::new()
    .open(self.is_open("profile"))
    .child(
        h_flex()
            .child(Avatar::new().name("Jason Lee").xsmall())
            .child("@huacnlee")
            .child(chevron),
    )
    .content(profile_fields)
```

[Collapsible]: https://docs.rs/gpui-component/latest/gpui_component/collapsible/struct.Collapsible.html
