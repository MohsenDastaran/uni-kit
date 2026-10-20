---
url: https://nsh.dev/modules
description: "The modules an NSH app imports, what each one exports, and how they are organized."
---

# Modules

An NSH app imports everything it uses from a small set of modules. They are all native: the reactive graph, the renderer and every module are implemented in Rust inside the runtime, and the bundler keeps `import ... from "nsh/..."` as imports instead of copying code into your bundle. Your bundle only holds your own code.

```tsx
import { mount, signal } from "nsh";
import { createNativeValue } from "nsh/animated";
import { windowSize } from "nsh/device";
import { Button, SafeAreaView } from "nsh/ui/mobile";
```

## Two layers

- **`nsh` and `nsh/<module>`** are the primitives and the environment: reactivity, lists, canvas, animation, gestures, device information. Every UI set builds on them.
- **`nsh/ui/<set>`** is a whole UI for one form factor. Today that is `nsh/ui/mobile`: navigation, modals, the status bar, system dialogs and components such as `Button` and `TextField` (see [Components](https://nsh.dev/components.md)).

There is no catch-all module: each API lives in exactly one place. Platform-only APIs live in `nsh/ios`, `nsh/android` and `nsh/desktop` (macOS, Windows, Linux); all can be imported on every platform, so one bundle runs everywhere (their calls do nothing elsewhere, and `isAvailable` tells you which platform you are on).

Types are generated from the runtime: `nsh init`, `nsh build`, `nsh dev` and `nsh trace` write `.nsh/types` next to your app, so the TypeScript declarations always match the runtime you build with (see [Project structure](https://nsh.dev/docs/project-structure.md)).

## All modules

| Module | Exports | Page |
|---|---|---|
| `nsh` | `signal`, `computed`, `effect`, `batch`, `untracked`, `cleanup`, `context`, `split`, `mount`; `View`, `Text`, `Scroll`, `Input`; the types of elements, styles, events and JSX (`YNode`, `ViewStyle`, ...) | [nsh](https://nsh.dev/modules/nsh.md) |
| `nsh/animated` | `createNativeValue`, `NativeValue`, `NativeNode`, `Animated`, `Easing`, `LayoutAnimation` | [nsh/animated](https://nsh.dev/modules/nsh-animated.md) |
| `nsh/list` | `List` (virtualized, recycling list) | [nsh/list](https://nsh.dev/modules/nsh-list.md) |
| `nsh/canvas` | `Canvas`, `Path2D`, `CanvasRenderingContext2D`, `CanvasGradient` | [nsh/canvas](https://nsh.dev/modules/nsh-canvas.md) |
| `nsh/device` | `windowSize`, `colorScheme`, `fontScale`, `Platform`, `PixelRatio`, `I18nManager` | [nsh/device](https://nsh.dev/modules/nsh-device.md) |
| `nsh/gesture` | `PanResponder` | [nsh/gesture](https://nsh.dev/modules/nsh-gesture.md) |
| `nsh/image` | `Image` (the element), `Image.getSize`, `Image.prefetch` | [nsh/image](https://nsh.dev/modules/nsh-image.md) |
| `nsh/clipboard` | `Clipboard.setString`, `Clipboard.getString`, `Clipboard.hasString` | [nsh/clipboard](https://nsh.dev/modules/nsh-clipboard.md) |
| `nsh/keymap` | `keymap.bind` (keyboard shortcuts: `mod+k`, `ctrl+shift+p`, ...) | [nsh/keymap](https://nsh.dev/modules/nsh-keymap.md) |
| `nsh/native` | `nativeComponent` (components the embedding app registers) | [nsh/native](https://nsh.dev/modules/nsh-native.md) |
| `nsh/accessibility` | `AccessibilityInfo`, `screenReaderEnabled`, `reduceMotion` | [nsh/accessibility](https://nsh.dev/modules/nsh-accessibility.md) |
| `nsh/ios` | `isAvailable`, `InputAccessoryView` | [nsh/ios](https://nsh.dev/modules/nsh-ios.md) |
| `nsh/android` | `isAvailable`, `Toast`, `BackHandler`, `setStatusBarBackgroundColor` | [nsh/android](https://nsh.dev/modules/nsh-android.md) |
| `nsh/desktop` | `isAvailable`, `Window` (title, title bar, maximize, full screen), `titleBar`, `Menu` (application menu, context menus) | [nsh/desktop](https://nsh.dev/modules/nsh-desktop.md) |
| `nsh/webview` | `WebView` | [nsh/webview](https://nsh.dev/modules/nsh-webview.md) |
| `nsh/video` | `Video` | [nsh/video](https://nsh.dev/modules/nsh-video.md) |
| `nsh/ui/mobile` | Navigation (`createNavigator`, `Stack`, `Screen`, `Modal`), `SafeAreaView`, `StatusBar`, `Alert`, `ActionSheet`, `Keyboard`, `Button`, `TextField`, ... | [Components](https://nsh.dev/components.md) |
| Globals | `fetch`, `URL`, `TextEncoder`, `Blob`, `FormData`, `AbortController`, `Intl`, timers, `console`, `requestAnimationFrame`, ... (no import) | [Global APIs](https://nsh.dev/modules/globals.md) |

Reference pages that are not modules:

| Page | Covers |
|---|---|
| [Style reference](https://nsh.dev/modules/style-reference.md) | Every style key, its type and meaning |
| [Tailwind reference](https://nsh.dev/modules/tailwind-reference.md) | The Tailwind utilities and variants `class` accepts |

## Host elements

The host elements are components imported like everything else: `View`, `Text`, `Scroll` and `Input` from `nsh`, `Image` from `nsh/image`, `List` from `nsh/list`, `Canvas`, `WebView`, `Video` from their modules, `Modal` from `nsh/ui/mobile`, `InputAccessoryView` from `nsh/ios`. Only `If` and `Each` are globals. The compiler makes the elements in place, so the imports cost nothing at run time. Their props are documented under [Elements](https://nsh.dev/elements.md).
