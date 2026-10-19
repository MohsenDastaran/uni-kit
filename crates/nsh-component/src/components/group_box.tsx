// A labelled cluster of controls. The title sits outside the surface, the body
// gets the iced implementation's 16px child rhythm, and the footer hangs below
// as muted text. `normal` has no surface, `fill` uses the group-box colour and
// `outline` draws a one-pixel border.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { RADIUS_MD, tokens } from './theme';

/** `default` and `filled` remain aliases for the earlier NSH API. */
export type GroupBoxVariant = 'normal' | 'fill' | 'outline' | 'default' | 'filled';

export interface GroupBoxProps {
  title?: string;
  footer?: string;
  variant?: GroupBoxVariant;
  /** Surface inset. `normal` ignores it, as the iced version does. */
  contentPadding?: number;
  cornerRadius?: number;
  titleColor?: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const shape = (variant: GroupBoxVariant): 'normal' | 'fill' | 'outline' => {
  if (variant === 'fill' || variant === 'filled') return 'fill';
  if (variant === 'outline') return 'outline';
  return 'normal';
};

export function GroupBox(props: GroupBoxProps) {
  const variant = () => shape(props.variant ?? 'normal');
  const padding = () => (variant() === 'normal' ? 0 : props.contentPadding ?? 16);
  const radius = () => props.cornerRadius ?? RADIUS_MD;
  const surface = (
    <View
      style={{
        width: '100%',
        gap: 16,
        padding: padding(),
        borderRadius: radius(),
        ...(variant() === 'fill'
          ? { backgroundColor: tokens.groupBox }
          : variant() === 'outline'
            ? { borderWidth: 1, borderColor: tokens.border }
            : {}),
      }}
    >
      {props.children}
    </View>
  );
  return (
    <View
      testID={props.testID}
      style={[
        {
          width: '100%',
          gap: variant() === 'normal' ? 16 : 12,
        },
        props.style,
      ]}
    >
      {props.title ? (
        <Text style={{ fontSize: 12, lineHeight: 16, fontWeight: '500', color: props.titleColor ?? tokens.groupBoxTitleForeground }}>
          {props.title}
        </Text>
      ) : null}
      {surface}
      {props.footer ? (
        <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>{props.footer}</Text>
      ) : null}
    </View>
  );
}

/** A label on the leading edge and its control on the trailing edge. */
export function GroupBoxRow(props: {
  label: string;
  description?: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View style={[{ width: '100%', flexDirection: 'row', alignItems: 'center', gap: 12 }, props.style]}>
      <View style={{ flex: 1, minWidth: 0, gap: 2 }}>
        <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.foreground }}>{props.label}</Text>
        {props.description ? (
          <Text style={{ fontSize: 10, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text>
        ) : null}
      </View>
      {props.children}
    </View>
  );
}
