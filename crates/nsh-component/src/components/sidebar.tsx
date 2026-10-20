// Navigation sidebar: grouped sections, collapsible rows, count badges, an
// optional narrow rail, and pinned header/footer slots.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Scroll, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { tokens } from './theme';

export interface SidebarItem {
  id: string;
  label: string;
  icon?: IconName;
  badge?: string;
  /** A trailing icon, e.g. a disclosure or external-link mark. */
  suffixIcon?: IconName;
  /** A caller-drawn trailing control, such as a row action menu. */
  trailing?: JSX.Element;
  /** Nested rows, indented under their owner. */
  children?: SidebarItem[];
  /** Start a row expanded even when another row is active. */
  defaultExpanded?: boolean;
  disabled?: boolean;
}

export interface SidebarSection {
  id: string;
  title?: string;
  items: SidebarItem[];
  /** A small trailing action beside the section title, e.g. add. */
  action?: JSX.Element;
}

export interface SidebarProps {
  /** Grouped navigation. Takes precedence over `items`. */
  sections?: SidebarSection[];
  /** A flat list, kept for callers that do not need groups. */
  items?: SidebarItem[];
  active?: string;
  onSelect?: (id: string) => void;
  header?: JSX.Element;
  footer?: JSX.Element;
  width?: number;
  /** Icon-only rail. */
  collapsed?: boolean;
  railWidth?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const row = (selected: boolean) => ({
  flexDirection: 'row' as const,
  alignItems: 'center' as const,
  gap: 8,
  paddingHorizontal: 8,
  paddingVertical: 7,
  borderRadius: 6,
  backgroundColor: selected ? tokens.accent : 'transparent',
  position: 'relative' as const,
});

export function Sidebar(props: SidebarProps) {
  const expanded = signal<Record<string, boolean>>({});
  const sections = () => props.sections ?? [{ id: 'main', items: props.items ?? [] }];
  const hasActive = (item: SidebarItem): boolean =>
    item.id === props.active || Boolean(item.children?.some((child) => hasActive(child)));
  const isExpanded = (item: SidebarItem) =>
    expanded()[item.id] ?? item.defaultExpanded ?? hasActive(item);
  const toggle = (item: SidebarItem) => {
    if (!item.children?.length) {
      if (!item.disabled) props.onSelect?.(item.id);
      return;
    }
    expanded.update((current) => ({ ...current, [item.id]: !isExpanded(item) }));
  };
  const collapsed = () => Boolean(props.collapsed);

  const renderItem = (item: SidebarItem, depth = 0) => {
    const selected = props.active === item.id;
    const hasChildren = Boolean(item.children?.length);
    const open = isExpanded(item);
    return (
      <View style={{ gap: 1 }}>
        <View
          accessibilityRole="button"
          accessibilityLabel={item.label}
          accessibilityState={{ selected, expanded: hasChildren ? open : undefined, disabled: Boolean(item.disabled) }}
          onPress={() => toggle(item)}
          activeOpacity={0.85}
          class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.045]"
          style={{
            ...row(selected),
            paddingStart: 8 + depth * 12,
            opacity: item.disabled ? 0.5 : 1,
            justifyContent: collapsed() ? 'center' : 'flex-start',
          }}
        >
          {selected && !collapsed() ? (
            <View style={{ position: 'absolute', left: 0, top: 6, bottom: 6, width: 2, borderRadius: 1, backgroundColor: tokens.primary }} />
          ) : null}
          {item.icon ? <Icon name={item.icon} size={14} color={selected ? tokens.foreground : tokens.mutedForeground} /> : null}
          {!collapsed() ? (
            <Text style={{ flex: 1, fontSize: 12, fontWeight: selected ? '500' : '400', color: selected ? tokens.foreground : tokens.mutedForeground }}>
              {item.label}
            </Text>
          ) : null}
          {!collapsed() && item.badge ? (
            <View style={{ minWidth: 17, height: 16, paddingHorizontal: 4, borderRadius: 8, alignItems: 'center', justifyContent: 'center', backgroundColor: selected ? 'transparent' : tokens.muted, borderWidth: selected ? 1 : 0, borderColor: tokens.border }}>
              <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{item.badge}</Text>
            </View>
          ) : null}
          {!collapsed() && item.suffixIcon ? <Icon name={item.suffixIcon} size={12} color={tokens.mutedForeground} /> : null}
          {!collapsed() ? item.trailing : null}
          {!collapsed() && hasChildren ? <Icon name={open ? 'chevron-down' : 'chevron-right'} size={12} color={tokens.mutedForeground} /> : null}
        </View>
        {hasChildren && open && !collapsed() ? (
          <View style={{ gap: 1 }}>
            {item.children?.map((child) => renderItem(child, depth + 1))}
          </View>
        ) : null}
      </View>
    );
  };

  return (
    <View
      testID={props.testID}
      style={[
        {
          width: collapsed() ? props.railWidth ?? 58 : props.width ?? 224,
          height: '100%',
          flexDirection: 'column',
          backgroundColor: tokens.sidebar,
          borderRightWidth: 1,
          borderRightColor: tokens.border,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      {props.header}
      <Scroll style={{ flex: 1, minHeight: 0 }}>
        <View style={{ padding: collapsed() ? 6 : 8, gap: collapsed() ? 8 : 14 }}>
          {sections().map((section) => (
            <View style={{ gap: 2 }}>
              {section.title && !collapsed() ? (
                <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6, paddingHorizontal: 8, paddingTop: 4, paddingBottom: 3 }}>
                  <Text style={{ flex: 1, fontSize: 9, fontWeight: '600', letterSpacing: 0.6, color: tokens.mutedForeground }}>{section.title.toUpperCase()}</Text>
                  {section.action}
                </View>
              ) : null}
              {section.items.map((item) => renderItem(item))}
            </View>
          ))}
        </View>
      </Scroll>
      {props.footer}
    </View>
  );
}
