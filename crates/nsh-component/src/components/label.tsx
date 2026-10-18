// A text label for a form control: optional secondary text, a required mark,
// highlighting and a masked value.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { alpha, type ControlSize, controlFontSize, FONT_SIZE_SM, SPACING_XS, tokens } from './theme';

export interface LabelProps {
  text?: string;
  secondary?: string;
  size?: ControlSize;
  required?: boolean;
  disabled?: boolean;
  masked?: boolean;
  /** A substring to highlight, as a search result does. */
  highlight?: string;
  /** Where the label sits relative to its control. */
  align?: 'start' | 'center' | 'end';
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function maskedOf(value: string): string {
  if (value.length <= 2) return '••';
  return `${'•'.repeat(Math.max(2, value.length - 2))}${value.slice(-2)}`;
}

export function Label(props: LabelProps) {
  const size = () => controlFontSize(props.size ?? 'medium');
  const text = () => (props.masked ? maskedOf(props.text ?? '') : props.text ?? '');
  const parts = () => {
    const value = text();
    const needle = props.highlight ?? '';
    if (!needle) return null;
    const at = value.toLowerCase().indexOf(needle.toLowerCase());
    if (at < 0) return null;
    return { before: value.slice(0, at), match: value.slice(at, at + needle.length), after: value.slice(at + needle.length) };
  };
  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'baseline',
          justifyContent:
            props.align === 'center' ? 'center' : props.align === 'end' ? 'flex-end' : 'flex-start',
          gap: SPACING_XS,
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      {props.children ?? (
        <Text style={{ fontSize: size(), lineHeight: size() + 4, color: tokens.foreground, fontWeight: '500' }}>
          {parts() ? (
            <>
              {parts()!.before}
              <Text style={{ backgroundColor: alpha(tokens.warning, 0.3), color: tokens.foreground }}>{parts()!.match}</Text>
              {parts()!.after}
            </>
          ) : (
            text()
          )}
          {props.required ? <Text style={{ color: tokens.danger }}> *</Text> : null}
        </Text>
      )}
      {props.secondary ? (
        <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.secondary}</Text>
      ) : null}
    </View>
  );
}
