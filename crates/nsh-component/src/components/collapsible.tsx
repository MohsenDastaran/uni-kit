// A header that expands and collapses its content.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { tokens } from './theme';

export interface CollapsibleProps {
  title?: string;
  trigger?: JSX.Element;
  open?: boolean;
  defaultOpen?: boolean;
  disabled?: boolean;
  onOpenChange?: (open: boolean) => void;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Collapsible(props: CollapsibleProps) {
  const own = signal(Boolean(props.defaultOpen));
  const open = () => props.open ?? own();
  const toggle = () => {
    if (props.disabled) return;
    const next = !open();
    if (props.open === undefined) own.set(next);
    props.onOpenChange?.(next);
  };
  return (
    <View testID={props.testID} style={[{ gap: 8 }, props.style]}>
      <View
        accessibilityRole="button"
        accessibilityLabel={props.title}
        accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
        onPress={toggle}
        disabled={props.disabled}
        activeOpacity={0.8}
        style={{ flexDirection: 'row', alignItems: 'center', gap: 8, paddingVertical: 4 }}
      >
        <View style={{ flex: 1 }}>
          {props.trigger ?? <Text style={{ fontSize: 14, fontWeight: '500', color: tokens.foreground }}>{props.title ?? ''}</Text>}
        </View>
        <Icon name={open() ? 'chevron-up' : 'chevron-down'} size={14} color={tokens.mutedForeground} />
      </View>
      {open() ? <View>{props.children}</View> : null}
    </View>
  );
}
