// A command bar: a row of controls, optional labels and groups.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { type ControlSize, controlHeight, SPACING_SM, tokens } from './theme';

export interface ToolbarProps {
  size?: ControlSize;
  borderless?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Toolbar(props: ToolbarProps) {
  return (
    <View
      testID={props.testID}
      accessibilityRole="toolbar"
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: SPACING_SM,
          minHeight: controlHeight(props.size ?? 'medium') + 8,
          paddingHorizontal: 8,
          paddingVertical: 4,
          borderBottomWidth: props.borderless ? 0 : 1,
          borderBottomColor: tokens.border,
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function ToolbarGroup(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 2,
          padding: 2,
          borderRadius: 6,
          backgroundColor: tokens.muted,
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function ToolbarLabel(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: 12, color: tokens.mutedForeground }, props.style]}>{props.children}</Text>
  );
}
