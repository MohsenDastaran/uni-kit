// A row of tab labels and the active panel.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { type ControlSize, controlFontSize, RADIUS_MD, tokens } from './theme';

export interface TabItem {
  value: string;
  label: string;
  content?: JSX.Element;
  disabled?: boolean;
}

export interface TabsProps {
  items: TabItem[];
  value?: string;
  defaultValue?: string;
  onChange?: (value: string) => void;
  size?: ControlSize;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Tabs(props: TabsProps) {
  const own = signal(props.defaultValue ?? props.items[0]?.value ?? '');
  const value = () => props.value ?? own();
  const active = () => props.items.find((item) => item.value === value());
  return (
    <View testID={props.testID} style={[{ gap: 10 }, props.style]}>
      <View
        accessibilityRole="tablist"
        style={{ flexDirection: 'row', gap: 4, borderBottomWidth: 1, borderBottomColor: tokens.border }}
      >
        {props.items.map((item) => {
          const selected = item.value === value();
          return (
            <View
              accessibilityRole="tab"
              accessibilityLabel={item.label}
              accessibilityState={{ selected, disabled: Boolean(item.disabled) }}
              onPress={() => {
                if (item.disabled) return;
                if (props.value === undefined) own.set(item.value);
                props.onChange?.(item.value);
              }}
              disabled={item.disabled}
              activeOpacity={0.8}
              style={{
                paddingHorizontal: 10,
                paddingVertical: 7,
                borderBottomWidth: selected ? 2 : 0,
                borderBottomColor: tokens.primary,
                opacity: item.disabled ? 0.5 : 1,
              }}
            >
              <Text style={{ fontSize: controlFontSize(props.size ?? 'medium') - 1, fontWeight: selected ? '600' : '400', color: selected ? tokens.foreground : tokens.mutedForeground }}>
                {item.label}
              </Text>
            </View>
          );
        })}
      </View>
      <View style={{ paddingVertical: 4 }}>{active()?.content}</View>
    </View>
  );
}
