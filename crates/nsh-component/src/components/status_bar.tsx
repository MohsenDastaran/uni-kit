// A horizontal status bar with left, centre and right regions.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { tokens } from './theme';

export interface StatusBarProps {
  left?: JSX.Element;
  center?: JSX.Element;
  right?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function StatusBar(props: StatusBarProps) {
  return (
    <View
      testID={props.testID}
      accessibilityRole="toolbar"
      style={[
        {
          height: 28,
          flexDirection: 'row',
          alignItems: 'center',
          gap: 12,
          paddingHorizontal: 10,
          borderTopWidth: 1,
          borderTopColor: tokens.border,
          backgroundColor: tokens.muted,
        },
        props.style,
      ]}
    >
      <View style={{ flex: 1, flexDirection: 'row', alignItems: 'center', gap: 10 }}>{props.left}</View>
      {props.center ? <View style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>{props.center}</View> : null}
      <View style={{ flex: 1, flexDirection: 'row', alignItems: 'center', justifyContent: 'flex-end', gap: 10 }}>{props.right}</View>
      {props.children}
    </View>
  );
}

export function StatusBarItem(props: { children?: JSX.Element; icon?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'center', gap: 4 }, props.style]}>
      {props.icon}
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.children}</Text>
    </View>
  );
}

export function StatusBarSeparator() {
  return <View style={{ width: 1, height: 14, backgroundColor: tokens.border }} />;
}
