// In-app toast notifications. The host is an absolute layer at the top right.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export interface NotificationItem {
  id: number;
  title: string;
  description?: string;
  variant?: 'default' | 'info' | 'success' | 'warning' | 'danger';
  icon?: IconName;
  duration?: number;
}

const items = signal<NotificationItem[]>([]);
let nextId = 1;

export function notify(item: Omit<NotificationItem, 'id'>): number {
  const id = nextId++;
  items.update((list) => [...list, { ...item, id }]);
  setTimeout(() => dismiss(id), item.duration ?? 4000);
  return id;
}

export function dismiss(id: number): void {
  items.update((list) => list.filter((entry) => entry.id !== id));
}

const toneOf = (variant: NotificationItem['variant']): string =>
  variant === 'success' ? tokens.success : variant === 'warning' ? tokens.warning : variant === 'danger' ? tokens.danger : variant === 'info' ? tokens.info : tokens.foreground;

const iconOf = (variant: NotificationItem['variant']): IconName =>
  variant === 'success' ? 'circle-check' : variant === 'warning' ? 'triangle-alert' : variant === 'danger' ? 'circle-alert' : 'info';

export function NotificationHost(props: { position?: 'top-right' | 'top-left' | 'bottom-right'; style?: StyleProp<ViewStyle> }) {
  const position = () => props.position ?? 'top-right';
  return (
    <View
      style={[
        {
          position: 'absolute',
          top: position().startsWith('top') ? 12 : undefined,
          bottom: position().startsWith('bottom') ? 12 : undefined,
          left: position().endsWith('left') ? 12 : undefined,
          right: position().endsWith('right') ? 12 : undefined,
          width: 300,
          gap: 8,
          zIndex: 40,
        },
        props.style,
      ]}
    >
      {items().map((item) => (
        <NotificationCard item={item} />
      ))}
    </View>
  );
}

export function NotificationCard(props: { item: NotificationItem; style?: StyleProp<ViewStyle> }) {
  const tone = () => toneOf(props.item.variant);
  return (
    <View
      accessibilityRole="alert"
      style={[
        {
          flexDirection: 'row',
          alignItems: 'flex-start',
          gap: 8,
          padding: 10,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: alpha(tone(), 0.35),
          backgroundColor: tokens.popover,
        },
        props.style,
      ]}
    >
      <Icon name={props.item.icon ?? iconOf(props.item.variant)} size={16} color={tone()} />
      <View style={{ flex: 1, gap: 2 }}>
        <Text style={{ fontSize: FONT_SIZE_SM, fontWeight: '600', color: tokens.foreground }}>{props.item.title}</Text>
        {props.item.description ? (
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.item.description}</Text>
        ) : null}
      </View>
      <View accessibilityRole="button" accessibilityLabel="Dismiss" onPress={() => dismiss(props.item.id)} style={{ width: 16, height: 16, alignItems: 'center', justifyContent: 'center' }}>
        <Icon name="x" size={11} color={tokens.mutedForeground} />
      </View>
    </View>
  );
}

export function Notification(props: NotificationItem & { style?: StyleProp<ViewStyle> }) {
  return <NotificationCard item={props} style={props.style} />;
}
