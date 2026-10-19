// A dock layout: a tabbed panel group, and a split between two groups. The
// application owns the tree; Dock draws one group and DockSplit gives the
// editor-style workspace its draggable seams.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { Resizable } from './resizable';
import { tokens } from './theme';

export interface DockPanel {
  id: string;
  label: string;
  icon?: IconName;
  content?: JSX.Element;
  /** A small count or status badge on the tab. */
  badge?: string;
}

export interface DockProps {
  panels: DockPanel[];
  active?: string;
  onSelect?: (id: string) => void;
  /** The trailing tab-bar control; `ellipsis` by default, `false` to hide it. */
  actions?: JSX.Element | false;
  header?: JSX.Element;
  footer?: JSX.Element;
  emptyText?: string;
  style?: StyleProp<ViewStyle>;
  contentStyle?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Dock(props: DockProps) {
  const own = signal(props.active ?? props.panels[0]?.id ?? '');
  const active = () => props.active ?? own();
  const chosen = () => props.panels.find((panel) => panel.id === active());
  return (
    <View
      testID={props.testID}
      style={[
        {
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: 8,
          overflow: 'hidden',
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      <View
        style={{
          flexDirection: 'row',
          alignItems: 'center',
          backgroundColor: tokens.muted,
          borderBottomWidth: 1,
          borderBottomColor: tokens.border,
          paddingHorizontal: 3,
          minHeight: 30,
        }}
      >
        {props.panels.map((panel) => {
          const selected = panel.id === active();
          return (
            <View
              accessibilityRole="tab"
              accessibilityLabel={panel.label}
              accessibilityState={{ selected }}
              onPress={() => {
                if (props.active === undefined) own.set(panel.id);
                props.onSelect?.(panel.id);
              }}
              activeOpacity={0.85}
              class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.06]"
              style={{
                flexDirection: 'row',
                alignItems: 'center',
                gap: 5,
                paddingHorizontal: 9,
                paddingVertical: 7,
                backgroundColor: selected ? tokens.background : 'transparent',
                borderTopLeftRadius: 5,
                borderTopRightRadius: 5,
              }}
            >
              {panel.icon ? (
                <Icon name={panel.icon} size={12} color={selected ? tokens.foreground : tokens.mutedForeground} />
              ) : null}
              <Text style={{ fontSize: 11, fontWeight: selected ? '600' : '400', color: selected ? tokens.foreground : tokens.mutedForeground }}>
                {panel.label}
              </Text>
              {panel.badge ? (
                <View style={{ minWidth: 14, height: 14, paddingHorizontal: 3, borderRadius: 7, backgroundColor: tokens.accent, alignItems: 'center', justifyContent: 'center' }}>
                  <Text style={{ fontSize: 8, color: tokens.mutedForeground }}>{panel.badge}</Text>
                </View>
              ) : null}
            </View>
          );
        })}
        <View style={{ flex: 1 }} />
        {props.actions === false ? null : props.actions ?? (
          <View style={{ width: 24, height: 22, alignItems: 'center', justifyContent: 'center' }}>
            <Icon name="ellipsis" size={13} color={tokens.mutedForeground} />
          </View>
        )}
      </View>
      {props.header}
      <View style={[{ flex: 1, minHeight: 0 }, props.contentStyle]}>
        {chosen() ? (
          chosen()!.content ?? null
        ) : (
          <View style={{ flex: 1, alignItems: 'center', justifyContent: 'center', padding: 16 }}>
            <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.emptyText ?? 'No panels'}</Text>
          </View>
        )}
      </View>
      {props.footer}
    </View>
  );
}

export function DockSplit(props: {
  left: JSX.Element;
  right: JSX.Element;
  initial?: number;
  min?: number;
  max?: number;
  vertical?: boolean;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <Resizable
      left={props.left}
      right={props.right}
      initial={props.initial ?? 55}
      min={props.min}
      max={props.max}
      vertical={props.vertical}
      style={props.style}
    />
  );
}
