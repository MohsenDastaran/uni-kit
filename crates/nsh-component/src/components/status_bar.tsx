// A horizontal status bar with left, centre and right regions. The centre
// adapts: it starts when only the centre exists, ends when only the left side
// exists, and centres when both sides are populated.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { type ControlSize, tokens } from './theme';

export type StatusBarTone = 'default' | 'success' | 'warning' | 'danger' | 'info';

export interface StatusBarProps {
  left?: JSX.Element;
  center?: JSX.Element;
  right?: JSX.Element;
  children?: JSX.Element;
  /** Bar density (default medium). */
  size?: ControlSize;
  /** The surface the bar sits on. */
  variant?: 'default' | 'inset' | 'floating';
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const heightOf = (size: ControlSize | undefined): number =>
  size === 'xsmall' ? 24 : size === 'small' ? 26 : size === 'large' ? 38 : 32;

const toneOf = (tone: StatusBarTone | undefined): string => {
  switch (tone) {
    case 'success':
      return tokens.success;
    case 'warning':
      return tokens.warning;
    case 'danger':
      return tokens.danger;
    case 'info':
      return tokens.info;
    default:
      return tokens.mutedForeground;
  }
};

export function StatusBar(props: StatusBarProps) {
  const variant = () => props.variant ?? 'default';
  const surface = () => {
    switch (variant()) {
      case 'inset':
        return {
          backgroundColor: tokens.popover,
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: 8,
        };
      case 'floating':
        return {
          backgroundColor: tokens.popover,
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: 999,
        };
      default:
        return {
          backgroundColor: tokens.muted,
          borderTopWidth: 1,
          borderTopColor: tokens.border,
        };
    }
  };
  return (
    <View
      testID={props.testID}
      accessibilityRole="toolbar"
      style={[
        {
          height: heightOf(props.size),
          flexDirection: 'row',
          alignItems: 'center',
          gap: 10,
          paddingHorizontal: 12,
        },
        surface(),
        props.style,
      ]}
    >
      {props.left ? (
        <View style={{ flex: 1, flexDirection: 'row', alignItems: 'center', gap: 10 }}>{props.left}</View>
      ) : null}
      {props.center ? (
        <View style={{ flexShrink: 0, flexDirection: 'row', alignItems: 'center', gap: 10 }}>{props.center}</View>
      ) : null}
      {props.right ? (
        <View style={{ flex: 1, flexDirection: 'row', alignItems: 'center', justifyContent: 'flex-end', gap: 10 }}>{props.right}</View>
      ) : null}
      {props.children}
    </View>
  );
}

export function StatusBarItem(props: {
  children?: JSX.Element;
  icon?: JSX.Element;
  iconName?: IconName;
  tone?: StatusBarTone;
  badge?: string;
  monospace?: boolean;
  active?: boolean;
  onPress?: () => void;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View
      accessibilityRole={props.onPress ? 'button' : undefined}
      onPress={props.onPress}
      activeOpacity={0.8}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 5,
          paddingHorizontal: props.onPress ? 5 : 0,
          paddingVertical: props.onPress ? 2 : 0,
          borderRadius: 4,
          backgroundColor: props.active ? tokens.accent : 'transparent',
        },
        props.style,
      ]}
    >
      {props.icon ?? (props.iconName ? <Icon name={props.iconName} size={12} color={toneOf(props.tone)} /> : null)}
      <Text style={{ fontSize: 11, color: toneOf(props.tone), fontFamily: props.monospace ? 'monospace' : undefined }}>
        {props.children}
      </Text>
      {props.badge ? (
        <View style={{ minWidth: 15, height: 14, paddingHorizontal: 4, borderRadius: 7, alignItems: 'center', justifyContent: 'center', backgroundColor: tokens.muted }}>
          <Text style={{ fontSize: 8, color: tokens.foreground }}>{props.badge}</Text>
        </View>
      ) : null}
    </View>
  );
}

export function StatusBarSeparator(props: { size?: ControlSize }) {
  return <View style={{ width: 1, height: props.size === 'large' ? 18 : 14, backgroundColor: tokens.border }} />;
}
