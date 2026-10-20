// A command bar: left, centre and right slots, grouped controls and separators.
import type { JSX, StyleProp, TextStyle, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { type ControlSize, controlHeight, tokens } from './theme';

export type ToolbarVariant = 'plain' | 'card' | 'muted' | 'accent';

export interface ToolbarProps {
  left?: JSX.Element;
  center?: JSX.Element;
  right?: JSX.Element;
  children?: JSX.Element;
  size?: ControlSize;
  variant?: ToolbarVariant;
  borderless?: boolean;
  /** Spread the direct children instead of packing them at the start. */
  justify?: 'start' | 'between';
  padding?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const surface = (variant: ToolbarVariant, borderless: boolean) => {
  switch (variant) {
    case 'card':
      return { backgroundColor: tokens.popover, borderWidth: 1, borderColor: tokens.border, borderRadius: 9 };
    case 'muted':
      return { backgroundColor: tokens.muted, borderRadius: 8 };
    case 'accent':
      return { backgroundColor: tokens.accent, borderWidth: 1, borderColor: tokens.border, borderRadius: 8 };
    default:
      return {
        backgroundColor: tokens.background,
        borderBottomWidth: borderless ? 0 : 1,
        borderBottomColor: tokens.border,
      };
  }
};

export function Toolbar(props: ToolbarProps) {
  const size = () => props.size ?? 'medium';
  const padded = () => props.padding ?? 8;
  const hasSlots = Boolean(props.left || props.center || props.right);
  return (
    <View
      testID={props.testID}
      accessibilityRole="toolbar"
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 6,
          minHeight: controlHeight(size()) + 8,
          paddingHorizontal: padded(),
          paddingVertical: 5,
        },
        surface(props.variant ?? 'plain', Boolean(props.borderless)),
        props.style,
      ]}
    >
      {hasSlots ? (
        <>
          <View style={{ flex: 1, minWidth: 0, flexDirection: 'row', alignItems: 'center', gap: 6 }}>{props.left}</View>
          {props.center ? <View style={{ flexShrink: 0, flexDirection: 'row', alignItems: 'center', gap: 6 }}>{props.center}</View> : null}
          <View style={{ flex: 1, minWidth: 0, flexDirection: 'row', alignItems: 'center', justifyContent: 'flex-end', gap: 6 }}>{props.right}</View>
        </>
      ) : (
        <View
          style={{
            flex: 1,
            minWidth: 0,
            flexDirection: 'row',
            alignItems: 'center',
            gap: 6,
            ...(props.justify === 'between' ? { justifyContent: 'space-between' } : {}),
          }}
        >
          {props.children}
        </View>
      )}
    </View>
  );
}

export function ToolbarGroup(props: {
  children?: JSX.Element;
  /** A quiet inset group, a bordered group, or no surface at all. */
  variant?: 'muted' | 'outline' | 'flush';
  size?: ControlSize;
  style?: StyleProp<ViewStyle>;
}) {
  const variant = () => props.variant ?? 'muted';
  return (
    <View
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 2,
          padding: variant() === 'muted' ? 2 : 0,
          borderRadius: 6,
          backgroundColor: variant() === 'muted' ? tokens.muted : 'transparent',
          borderWidth: variant() === 'outline' ? 1 : 0,
          borderColor: tokens.border,
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function ToolbarSeparator(props: { size?: ControlSize }) {
  return <View style={{ width: 1, height: props.size === 'large' ? 22 : 18, marginHorizontal: 3, backgroundColor: tokens.border }} />;
}

export function ToolbarSpacer() {
  return <View style={{ flex: 1, minWidth: 8 }} />;
}

export function ToolbarLabel(props: { children?: JSX.Element; size?: ControlSize; style?: StyleProp<TextStyle> }) {
  return (
    <Text
      numberOfLines={1}
      style={[
        { fontSize: props.size === 'small' ? 10 : props.size === 'large' ? 13 : 11, color: tokens.mutedForeground },
        props.style,
      ]}
    >
      {props.children}
    </Text>
  );
}
