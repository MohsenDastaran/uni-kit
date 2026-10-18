// A navigation sidebar with groups, an active row and a footer.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { tokens } from './theme';

export interface SidebarItem {
  id: string;
  label: string;
  icon?: IconName;
  badge?: string;
}

export interface SidebarProps {
  items: SidebarItem[];
  active?: string;
  onSelect?: (id: string) => void;
  header?: JSX.Element;
  footer?: JSX.Element;
  width?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Sidebar(props: SidebarProps) {
  return (
    <View
      testID={props.testID}
      style={[
        { width: props.width ?? 220, backgroundColor: tokens.sidebar, borderRightWidth: 1, borderRightColor: tokens.border },
        props.style,
      ]}
    >
      {props.header}
      <View style={{ flex: 1, gap: 2, padding: 8 }}>
        {props.items.map((item) => {
          const selected = props.active === item.id;
          return (
            <View
              accessibilityRole="button"
              accessibilityLabel={item.label}
              accessibilityState={{ selected }}
              onPress={() => props.onSelect?.(item.id)}
              activeOpacity={0.8}
              style={{
                flexDirection: 'row',
                alignItems: 'center',
                gap: 8,
                paddingHorizontal: 8,
                paddingVertical: 7,
                borderRadius: 6,
                backgroundColor: selected ? tokens.accent : 'transparent',
              }}
            >
              {item.icon ? <Icon name={item.icon} size={15} color={selected ? tokens.foreground : tokens.mutedForeground} /> : null}
              <Text style={{ flex: 1, fontSize: 13, color: selected ? tokens.foreground : tokens.mutedForeground }}>{item.label}</Text>
              {item.badge ? <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{item.badge}</Text> : null}
            </View>
          );
        })}
      </View>
      {props.footer}
    </View>
  );
}
