---
url: https://nsh.dev/zh-CN/modules
description: "NSH 应用可以导入的模块、各模块导出的内容以及它们的组织方式。"
---

# 模块

NSH 应用用到的一切都从一小组模块导入。这些模块全部是原生的：响应式图、渲染器和每个模块都在运行时内部用 Rust 实现，打包器会把 `import ... from "nsh/..."` 原样保留为导入，而不会把代码复制进你的 bundle。你的 bundle 里只有你自己的代码。

```tsx
import { mount, signal } from "nsh";
import { createNativeValue } from "nsh/animated";
import { windowSize } from "nsh/device";
import { Button, SafeAreaView } from "nsh/ui/mobile";
```

## 两层模块

- **`nsh` 和 `nsh/<module>`** 是基础能力和运行环境：响应式、列表、画布、动画、手势、设备信息。所有 UI 套件都构建在它们之上。
- **`nsh/ui/<set>`** 是面向某一种设备形态的完整 UI。目前只有 `nsh/ui/mobile`：导航、模态框、状态栏、系统对话框，以及 `Button`、`TextField` 等组件（见[组件](https://nsh.dev/zh-CN/components.md)）。

NSH 没有“大而全”的汇总模块：每个 API 只存在于一个地方。平台专有的 API 位于 `nsh/ios`、`nsh/android` 和 `nsh/desktop`（macOS、Windows、Linux）；这些模块在所有平台上都可以导入，因此同一个 bundle 可以在所有平台运行（在其他平台上调用它们什么也不做，`isAvailable` 告诉你当前是否在该平台上）。

类型声明由运行时生成：`nsh init`、`nsh build`、`nsh dev` 和 `nsh trace` 会在应用旁边写出 `.nsh/types`，所以 TypeScript 声明始终与你所用的运行时一致（见[项目结构](https://nsh.dev/zh-CN/docs/project-structure.md)）。

## 全部模块

| 模块 | 导出 | 页面 |
|---|---|---|
| `nsh` | `signal`、`computed`、`effect`、`batch`、`untracked`、`cleanup`、`context`、`split`、`mount`；`View`、`Text`、`Scroll`、`Input`；元素、样式、事件和 JSX 的类型（`YNode`、`ViewStyle` 等） | [nsh](https://nsh.dev/zh-CN/modules/nsh.md) |
| `nsh/animated` | `createNativeValue`、`NativeValue`、`NativeNode`、`Animated`、`Easing`、`LayoutAnimation` | [nsh/animated](https://nsh.dev/zh-CN/modules/nsh-animated.md) |
| `nsh/list` | `List`（虚拟化、可复用单元格的列表） | [nsh/list](https://nsh.dev/zh-CN/modules/nsh-list.md) |
| `nsh/canvas` | `Canvas`、`Path2D`、`CanvasRenderingContext2D`、`CanvasGradient` | [nsh/canvas](https://nsh.dev/zh-CN/modules/nsh-canvas.md) |
| `nsh/device` | `windowSize`、`colorScheme`、`fontScale`、`Platform`、`PixelRatio`、`I18nManager` | [nsh/device](https://nsh.dev/zh-CN/modules/nsh-device.md) |
| `nsh/gesture` | `PanResponder` | [nsh/gesture](https://nsh.dev/zh-CN/modules/nsh-gesture.md) |
| `nsh/image` | `Image`（元素）、`Image.getSize`、`Image.prefetch` | [nsh/image](https://nsh.dev/zh-CN/modules/nsh-image.md) |
| `nsh/clipboard` | `Clipboard.setString`、`Clipboard.getString`、`Clipboard.hasString` | [nsh/clipboard](https://nsh.dev/zh-CN/modules/nsh-clipboard.md) |
| `nsh/keymap` | `keymap.bind`（键盘快捷键：`mod+k`、`ctrl+shift+p` 等） | [nsh/keymap](https://nsh.dev/zh-CN/modules/nsh-keymap.md) |
| `nsh/native` | `nativeComponent`（宿主应用注册的原生组件） | [nsh/native](https://nsh.dev/zh-CN/modules/nsh-native.md) |
| `nsh/accessibility` | `AccessibilityInfo`、`screenReaderEnabled`、`reduceMotion` | [nsh/accessibility](https://nsh.dev/zh-CN/modules/nsh-accessibility.md) |
| `nsh/ios` | `isAvailable`、`InputAccessoryView` | [nsh/ios](https://nsh.dev/zh-CN/modules/nsh-ios.md) |
| `nsh/android` | `isAvailable`、`Toast`、`BackHandler`、`setStatusBarBackgroundColor` | [nsh/android](https://nsh.dev/zh-CN/modules/nsh-android.md) |
| `nsh/desktop` | `isAvailable`、`Window`（标题、标题栏、最大化、全屏）、`titleBar`、`Menu`（应用菜单、右键菜单） | [nsh/desktop](https://nsh.dev/zh-CN/modules/nsh-desktop.md) |
| `nsh/webview` | `WebView` | [nsh/webview](https://nsh.dev/zh-CN/modules/nsh-webview.md) |
| `nsh/video` | `Video` | [nsh/video](https://nsh.dev/zh-CN/modules/nsh-video.md) |
| `nsh/ui/mobile` | 导航（`createNavigator`、`Stack`、`Screen`、`Modal`）、`SafeAreaView`、`StatusBar`、`Alert`、`ActionSheet`、`Keyboard`、`Button`、`TextField` 等 | [组件](https://nsh.dev/zh-CN/components.md) |
| 全局对象 | `fetch`、`URL`、`TextEncoder`、`Blob`、`FormData`、`AbortController`、`Intl`、定时器、`console`、`requestAnimationFrame` 等（无需导入） | [全局 API](https://nsh.dev/zh-CN/modules/globals.md) |

以下参考页面不对应模块：

| 页面 | 内容 |
|---|---|
| [样式参考](https://nsh.dev/zh-CN/modules/style-reference.md) | 每个样式键的类型和含义 |
| [Tailwind 参考](https://nsh.dev/zh-CN/modules/tailwind-reference.md) | `class` 支持的 Tailwind 工具类和变体 |

## 宿主元素

宿主元素和其他一切一样是需要导入的组件：`View`、`Text`、`Scroll`、`Input` 来自 `nsh`，`Image` 来自 `nsh/image`，`List` 来自 `nsh/list`，`Canvas`、`WebView`、`Video` 来自各自的模块，`Modal` 来自 `nsh/ui/mobile`，`InputAccessoryView` 来自 `nsh/ios`。只有 `If` 和 `Each` 是全局组件。编译器会就地创建这些元素，所以导入在运行时没有任何开销。它们的属性见[元素](https://nsh.dev/zh-CN/elements.md)。
