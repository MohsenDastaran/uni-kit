// Keyboard shortcut chips. Each key is its own chip; `keymap.label` spells the
// key the way the current platform writes it (⌘ on Apple, Ctrl elsewhere).
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { keymap } from 'nsh/keymap';
import { Platform } from 'nsh/device';
import { type ControlSize, controlFontSize, RADIUS_SM, SPACING_XS, tokens } from './theme';

export interface KbdProps {
  /** Keystrokes in `nsh/keymap` spelling, e.g. `["mod", "k"]` or `["mod+k"]`. */
  keys?: string[];
  label?: string;
  size?: ControlSize;
  /** `false` draws the label without the chip frame. */
  styled?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function labelOf(keystroke: string): string {
  const apple = Platform.OS === 'ios' || Platform.OS === 'macos';
  const mapped = apple
    ? { mod: '⌘', cmd: '⌘', ctrl: '⌃', alt: '⌥', shift: '⇧', escape: 'Esc', delete: 'Del', enter: 'Enter', up: '↑', down: '↓', left: '←', right: '→' }
    : { mod: 'Ctrl', cmd: 'Win', ctrl: 'Ctrl', alt: 'Alt', shift: 'Shift', escape: 'Esc', delete: 'Del', enter: 'Enter', up: '↑', down: '↓', left: '←', right: '→' };
  const key = keystroke.toLowerCase();
  if (key in mapped) return mapped[key as keyof typeof mapped];
  if (/^[a-z]$/i.test(keystroke) || /^f\d+$/i.test(keystroke)) return keystroke.toUpperCase();
  try {
    return keymap.label(keystroke);
  } catch {
    return keystroke;
  }
}

/** A single chip. Multiple keys are composed by `Kbd` below. */
function KbdChip(props: {
  text: string;
  size: ControlSize;
  styled: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}) {
  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'center',
          gap: 2,
          minWidth: props.styled ? 20 : undefined,
          ...(props.styled
            ? {
                paddingHorizontal: SPACING_XS + 3,
                paddingVertical: 2,
                borderRadius: RADIUS_SM,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.muted,
              }
            : {}),
        },
        props.style,
      ]}
    >
      {props.children ?? (
        <Text
          style={{
            fontSize: controlFontSize(props.size) - 2,
            lineHeight: controlFontSize(props.size) + 1,
            color: props.styled ? tokens.mutedForeground : tokens.foreground,
            fontFamily: 'monospace',
            textAlign: 'center',
          }}
        >
          {props.text}
        </Text>
      )}
    </View>
  );
}

export function Kbd(props: KbdProps) {
  const size = () => props.size ?? 'small';
  const styled = () => props.styled !== false;
  const parts = () => {
    if (props.label) return [props.label];
    const list = props.keys ?? [];
    return list.flatMap((entry) => entry.split('+').filter(Boolean));
  };
  if (props.children || parts().length <= 1) {
    return (
      <KbdChip
        text={props.label ?? labelOf(parts()[0] ?? '')}
        size={size()}
        styled={styled()}
        style={props.style}
        testID={props.testID}
      >
        {props.children}
      </KbdChip>
    );
  }
  return (
    <KbdGroup style={props.style} testID={props.testID}>
      {parts().map((entry) => (
        <KbdChip text={labelOf(entry)} size={size()} styled={styled()} />
      ))}
    </KbdGroup>
  );
}

export function KbdGroup(props: { children?: JSX.Element; style?: StyleProp<ViewStyle>; testID?: string }) {
  return (
    <View
      testID={props.testID}
      style={[{ flexDirection: 'row', alignItems: 'center', flexWrap: 'wrap', gap: 3 }, props.style]}
    >
      {props.children}
    </View>
  );
}

/** A small "Search …" hint: a text label followed by its key chips. */
export function KbdHint(props: { label: string; keys: string[]; size?: ControlSize; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'center', gap: 6 }, props.style]}>
      <Text style={{ fontSize: controlFontSize(props.size ?? 'small') - 2, color: tokens.mutedForeground }}>{props.label}</Text>
      <Kbd keys={props.keys} size={props.size ?? 'xsmall'} />
    </View>
  );
}
