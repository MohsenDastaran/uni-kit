// A stack of collapsible sections, single or multiple open.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { tokens } from './theme';

export interface AccordionItem {
  id: string;
  title: string;
  content?: JSX.Element;
}

export interface AccordionProps {
  items: AccordionItem[];
  type?: 'single' | 'multiple';
  defaultValue?: string[];
  value?: string[];
  onChange?: (open: string[]) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Accordion(props: AccordionProps) {
  const own = signal<string[]>(props.defaultValue ?? []);
  const open = () => props.value ?? own();
  const toggle = (id: string) => {
    const current = open();
    const next = current.includes(id)
      ? current.filter((entry) => entry !== id)
      : props.type === 'single'
        ? [id]
        : [...current, id];
    if (props.value === undefined) own.set(next);
    props.onChange?.(next);
  };
  return (
    <View testID={props.testID} style={[{ gap: 0 }, props.style]}>
      {props.items.map((item) => {
        const expanded = open().includes(item.id);
        return (
          <View style={{ borderBottomWidth: 1, borderBottomColor: tokens.border }}>
            <View
              accessibilityRole="button"
              accessibilityLabel={item.title}
              accessibilityState={{ expanded }}
              onPress={() => toggle(item.id)}
              activeOpacity={0.8}
              style={{ flexDirection: 'row', alignItems: 'center', gap: 8, paddingVertical: 10 }}
            >
              <Text style={{ flex: 1, fontSize: 14, fontWeight: '500', color: tokens.foreground }}>{item.title}</Text>
              <Icon name={expanded ? 'chevron-up' : 'chevron-down'} size={14} color={tokens.mutedForeground} />
            </View>
            {expanded ? <View style={{ paddingBottom: 12 }}>{item.content}</View> : null}
          </View>
        );
      })}
    </View>
  );
}
