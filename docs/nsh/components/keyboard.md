---
url: https://nsh.dev/components/keyboard
description: "Dismiss the software keyboard and read its height and animation duration reactively."
---

# Keyboard

`nsh/ui/mobile` has two exports for the software keyboard: the `Keyboard` module, which hides the keyboard, and the
`keyboard()` accessor, which tells you how much of the screen the keyboard covers.

```tsx
import { Keyboard, keyboard } from "nsh/ui/mobile";
```

## Keyboard.dismiss

```ts
Keyboard.dismiss(): void
```

Unfocuses the focused input, which hides the keyboard. It does nothing if no input is focused.

```tsx
import { mount, signal, View } from "nsh";
import { Button, Keyboard, TextField } from "nsh/ui/mobile";

function App() {
  const name = signal("");
  return (
    <View style={{ flex: 1, padding: 16, paddingTop: 48, gap: 12 }}>
      <TextField label="Name" value={name()} onChange={(v) => name.set(v)} />
      <Button title="Done" onPress={() => Keyboard.dismiss()} />
    </View>
  );
}

mount(App);
```

> [!TIP]
> To hide the keyboard when the user drags a scroll view, set `keyboardDismissMode="on-drag"` or `"interactive"` on
> the [scroll view](https://nsh.dev/elements/scroll.md) or [List](https://nsh.dev/elements/list.md) instead of calling `Keyboard.dismiss()`.

## keyboard()

```ts
keyboard: Accessor<{ height: number; duration: number }>
```

`keyboard()` returns the height of the software keyboard over the NSH surface, in points (0 while it is hidden), and
the duration of the keyboard's show or hide animation, in seconds. It is reactive: an effect or JSX expression that
reads it runs again when the keyboard shows, hides or changes height.

```tsx
import { Input, View } from "nsh";
import { keyboard, useSafeAreaInsets } from "nsh/ui/mobile";

function MessageBar() {
  const insets = useSafeAreaInsets();
  return (
    <View
      style={{
        flexDirection: "row",
        padding: 8,
        // The home indicator's inset only matters while the keyboard is hidden.
        paddingBottom: 8 + (keyboard().height > 0 ? 0 : insets().bottom),
      }}
    >
      <Input placeholder="Message" style={{ flex: 1 }} />
    </View>
  );
}
```

The height is the part of the keyboard that overlaps the NSH surface. When the window itself shrinks for the keyboard
(an Android activity that resizes), or when NSH is embedded in a view that ends above the keyboard, the height is 0.

> [!NOTE]
> To keep content above the keyboard, [KeyboardAvoidingView](https://nsh.dev/components/keyboard-avoiding-view.md) is usually simpler than
> reading `keyboard()` yourself: it animates with the keyboard natively.

## Related

- [KeyboardAvoidingView](https://nsh.dev/components/keyboard-avoiding-view.md).
- [TextField](https://nsh.dev/components/text-field.md) and [Input](https://nsh.dev/elements/input.md).
- [Forms and keyboard](https://nsh.dev/docs/forms-and-keyboard.md).
