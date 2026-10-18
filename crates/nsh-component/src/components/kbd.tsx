// A keyboard shortcut chip. `keymap.label` spells the keys the way the current
// platform writes them (⌘K on Apple, Ctrl+K elsewhere).
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { keymap } from 'nsh/keymap';
import { type ControlSize, controlFontSize, RADIUS_SM, SPACING_XS, tokens } from './theme';

export interface KbdProps {
  /** Keystrokes in `nsh/keymap` spelling, e.g. `["mod", "k"]` or `["mod+k"]`. */
  keys?: string[];
  label?: string;
  size?: ControlSize;
  styled?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function labelOf(keystroke: string): string {
  try {
    return keymap.label(keystroke);
  } catch {
    return keystroke;
  }
}

export function Kbd(props: KbdProps) {
  const size = () => props.size ?? 'small';
  const keys = () => props.keys ?? [];
  const text = () => {
    if (props.label) return props.label;
    if (keys().length === 0) return '';
    // A single `mod+k` spelling is already one shortcut; a list of modifiers
    // is joined into one.
    if (keys().length === 1) return labelOf(keys()[0]);
    return keys().map(labelOf).join('+');
  };
  const styled = () => props.styled !== false;
  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 2,
          ...(styled()
            ? {
                paddingHorizontal: SPACING_XS + 2,
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
            fontSize: controlFontSize(size()) - 1,
            lineHeight: controlFontSize(size()) + 2,
            color: styled() ? tokens.mutedForeground : tokens.foreground,
            fontFamily: 'monospace',
          }}
        >
          {text()}
        </Text>
      )}
    </View>
  );
}

export function KbdGroup(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={[{ flexDirection: 'row', alignItems: 'center', gap: SPACING_XS }, props.style]}>{props.children}</View>;
}
