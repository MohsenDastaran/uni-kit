// A dock layout: a tabbed panel group, and a split between two groups.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { Resizable } from './resizable';
import { tokens } from './theme';

export interface DockPanel {
  id: string;
  label: string;
  content?: JSX.Element;
}

export interface DockProps {
  panels: DockPanel[];
  active?: string;
  onSelect?: (id: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Dock(props: DockProps) {
  const own = signal(props.active ?? props.panels[0]?.id ?? '');
  const active = () => props.active ?? own();
  const chosen = () => props.panels.find((panel) => panel.id === active());
  return (
    <View testID={props.testID} style={[{ borderWidth: 1, borderColor: tokens.border, borderRadius: 8, overflow: 'hidden', backgroundColor: tokens.background }, props.style]}>
      <View style={{ flexDirection: 'row', alignItems: 'center', backgroundColor: tokens.muted, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        {props.panels.map((panel) => (
          <View
            accessibilityRole="tab"
            accessibilityLabel={panel.label}
            accessibilityState={{ selected: panel.id === active() }}
            onPress={() => {
              if (props.active === undefined) own.set(panel.id);
              props.onSelect?.(panel.id);
            }}
            activeOpacity={0.8}
            style={{ flexDirection: 'row', alignItems: 'center', gap: 6, paddingHorizontal: 12, paddingVertical: 8, borderBottomWidth: panel.id === active() ? 2 : 0, borderBottomColor: tokens.primary }}
          >
            <Text style={{ fontSize: 12, color: panel.id === active() ? tokens.foreground : tokens.mutedForeground }}>{panel.label}</Text>
          </View>
        ))}
        <View style={{ flex: 1 }} />
        <Icon name="ellipsis" size={14} color={tokens.mutedForeground} />
      </View>
      <View style={{ flex: 1 }}>{chosen()?.content}</View>
    </View>
  );
}

export function DockSplit(props: { left: JSX.Element; right: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <Resizable left={props.left} right={props.right} initial={55} style={props.style} />;
}
