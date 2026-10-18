// A key/value list: one row per item, optional borders/separators, sizes and
// multi-column spans.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import {
  type ControlSize,
  controlFontSize,
  FONT_SIZE_SM,
  RADIUS_MD,
  SPACING_MD,
  SPACING_SM,
  tokens,
} from './theme';

export interface DescriptionItem {
  label: string;
  value: string;
  span?: number;
}

export interface DescriptionListProps {
  items?: DescriptionItem[];
  columns?: number;
  bordered?: boolean;
  separators?: boolean;
  size?: ControlSize;
  labelWidth?: number;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function DescriptionList(props: DescriptionListProps) {
  const items = () => props.items ?? [];
  const columns = () => Math.max(1, props.columns ?? 1);
  const size = () => props.size ?? 'medium';
  return (
    <View
      testID={props.testID}
      style={[
        {
          borderWidth: props.bordered ? 1 : 0,
          borderColor: tokens.border,
          borderRadius: props.bordered ? RADIUS_MD : 0,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      {items().map((item, index) => (
        <View
          style={{
            flexDirection: 'row',
            alignItems: 'center',
            gap: SPACING_SM,
            paddingHorizontal: 12,
            paddingVertical: size() === 'small' ? 6 : size() === 'large' ? 12 : 9,
            borderTopWidth: props.separators !== false && index > 0 ? 1 : 0,
            borderTopColor: tokens.border,
            flex: item.span && item.span > 1 ? item.span : columns() > 1 ? 1 : undefined,
            minWidth: props.labelWidth ?? 100,
          }}
        >
          <Text
            style={{
              width: props.labelWidth ?? 110,
              fontSize: controlFontSize(size()) - 2,
              color: tokens.mutedForeground,
            }}
          >
            {item.label}
          </Text>
          <Text style={{ flex: 1, fontSize: FONT_SIZE_SM, color: tokens.foreground }}>{item.value}</Text>
        </View>
      ))}
      {props.children}
    </View>
  );
}

export function DescriptionItemView(props: {
  label: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View style={[{ gap: 4 }, props.style]}>
      <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.label}</Text>
      {props.children}
    </View>
  );
}
