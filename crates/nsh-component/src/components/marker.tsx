// A compact row for conversation status, notifications, loading and separators.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { SPACING_SM, tokens } from './theme';

export interface MarkerProps {
  icon?: IconName;
  loading?: boolean;
  separator?: boolean;
  border?: boolean;
  align?: 'start' | 'center' | 'end';
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Marker(props: MarkerProps) {
  if (props.separator) {
    return (
      <View
        testID={props.testID}
        style={[{ flexDirection: 'row', alignItems: 'center', gap: SPACING_SM }, props.style]}
      >
        <View style={{ flex: 1, height: 1, backgroundColor: tokens.border }} />
        {props.children}
        <View style={{ flex: 1, height: 1, backgroundColor: tokens.border }} />
      </View>
    );
  }
  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: SPACING_SM,
          justifyContent:
            props.align === 'end' ? 'flex-end' : props.align === 'center' ? 'center' : 'flex-start',
          paddingVertical: 4,
          paddingHorizontal: props.border ? 8 : 0,
          borderWidth: props.border ? 1 : 0,
          borderColor: tokens.border,
          borderRadius: props.border ? 6 : 0,
        },
        props.style,
      ]}
    >
      {props.loading ? <Icon name="loader-circle" spinning size={14} color={tokens.mutedForeground} /> : null}
      {props.icon ? <Icon name={props.icon} size={14} color={tokens.mutedForeground} /> : null}
      {props.children}
    </View>
  );
}

export function MarkerIcon(props: { name: IconName; color?: string; size?: number }) {
  return <Icon name={props.name} size={props.size ?? 14} color={props.color ?? tokens.mutedForeground} />;
}

export function MarkerContent(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={props.style}>{props.children}</View>;
}

export function MarkerLabel(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: 13, color: tokens.mutedForeground }, props.style]}>{props.children}</Text>
  );
}
