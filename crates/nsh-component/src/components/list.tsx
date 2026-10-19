// A styled row list. The mobile ListView owns recycling, dividers and padding;
// this file owns the row language: leading avatar/icon, title, description,
// badge, trailing action and an optional chevron.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { ListView } from 'nsh/ui/mobile';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, RADIUS_SM, tokens } from './theme';

export interface ListProps<T> {
  items: T[];
  renderItem?: (item: T, index: number) => JSX.Element;
  keyExtractor?: (item: T, index: number) => string;
  variant?: 'plain' | 'inset';
  dividers?: boolean;
  header?: JSX.Element;
  footer?: JSX.Element;
  empty?: JSX.Element;
  refreshing?: boolean;
  onRefresh?: () => void;
  onEndReached?: () => void;
  onEndReachedThreshold?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function List<T>(props: ListProps<T>) {
  return (
    <ListView
      testID={props.testID}
      each={props.items}
      keyExtractor={props.keyExtractor}
      variant={props.variant}
      dividers={props.dividers}
      ListHeaderComponent={props.header}
      ListFooterComponent={props.footer}
      ListEmptyComponent={props.empty}
      refreshing={props.refreshing}
      onRefresh={props.onRefresh}
      onEndReached={props.onEndReached}
      onEndReachedThreshold={props.onEndReachedThreshold}
      style={props.style}
    >
      {(item, index) =>
        props.renderItem
          ? props.renderItem(item(), index())
          : <ListRow title={String(item())} />
      }
    </ListView>
  );
}

export function ListHeader(props: {
  title?: string;
  description?: string;
  action?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'flex-start', gap: 10, paddingBottom: 10 }, props.style]}>
      <View style={{ flex: 1, gap: 2 }}>
        {props.title ? (
          <Text style={{ fontSize: 14, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
        ) : null}
        {props.description ? (
          <Text style={{ fontSize: 10, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text>
        ) : null}
      </View>
      {props.action}
    </View>
  );
}

export function ListSectionHeader(props: {
  label: string;
  count?: number;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 6,
          paddingTop: 12,
          paddingBottom: 4,
        },
        props.style,
      ]}
    >
      <Text style={{ fontSize: 9, fontWeight: '600', letterSpacing: 0.5, color: tokens.mutedForeground }}>
        {props.label.toUpperCase()}
      </Text>
      {props.count !== undefined ? (
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{props.count}</Text>
      ) : null}
      <View style={{ flex: 1, height: 1, backgroundColor: tokens.border }} />
    </View>
  );
}

export function ListRow(props: {
  title: string;
  description?: string;
  /** A caller-drawn leading element, such as an avatar. */
  leading?: JSX.Element;
  /** An icon tile used when no leading element is given. */
  icon?: IconName;
  iconTone?: 'default' | 'info' | 'success' | 'warning' | 'danger';
  badge?: string;
  badgeTone?: 'default' | 'info' | 'success' | 'warning' | 'danger';
  trailing?: JSX.Element;
  meta?: string;
  selected?: boolean;
  disabled?: boolean;
  chevron?: boolean;
  onPress?: () => void;
  style?: StyleProp<ViewStyle>;
}) {
  const tone = () => {
    switch (props.iconTone) {
      case 'info':
        return tokens.info;
      case 'success':
        return tokens.success;
      case 'warning':
        return tokens.warning;
      case 'danger':
        return tokens.danger;
      default:
        return tokens.mutedForeground;
    }
  };
  const badgeTone = () => {
    switch (props.badgeTone) {
      case 'info':
        return tokens.info;
      case 'success':
        return tokens.success;
      case 'warning':
        return tokens.warning;
      case 'danger':
        return tokens.danger;
      default:
        return tokens.mutedForeground;
    }
  };
  return (
    <View
      accessibilityRole={props.onPress ? 'button' : undefined}
      accessibilityLabel={props.onPress ? props.title : undefined}
      onPress={props.disabled ? undefined : props.onPress}
      disabled={props.disabled}
      activeOpacity={0.9}
      class={props.onPress ? 'transition-colors duration-150 hover:bg-black/[0.03] dark:hover:bg-white/[0.04]' : undefined}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 10,
          minHeight: 22,
          marginHorizontal: -8,
          paddingHorizontal: 8,
          paddingVertical: 0,
          borderRadius: RADIUS_SM,
          backgroundColor: props.selected ? alpha(tokens.primary, 0.1) : 'transparent',
          opacity: props.disabled ? 0.45 : 1,
        },
        props.style,
      ]}
    >
      {props.leading ??
        (props.icon ? (
          <View
            style={{
              width: 30,
              height: 30,
              borderRadius: 7,
              alignItems: 'center',
              justifyContent: 'center',
              backgroundColor: alpha(tone(), 0.12),
            }}
          >
            <Icon name={props.icon} size={14} color={tone()} />
          </View>
        ) : null)}
      <View style={{ flex: 1, minWidth: 0, gap: 2 }}>
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5 }}>
          <Text style={{ fontSize: 12, lineHeight: 17, fontWeight: '500', color: tokens.foreground }} numberOfLines={1}>
            {props.title}
          </Text>
          {props.badge ? (
            <View style={{ paddingHorizontal: 4, paddingVertical: 1, borderRadius: 3, backgroundColor: alpha(badgeTone(), 0.12), borderWidth: 1, borderColor: alpha(badgeTone(), 0.28) }}>
              <Text style={{ fontSize: 8, lineHeight: 11, fontWeight: '600', color: badgeTone() }}>{props.badge}</Text>
            </View>
          ) : null}
        </View>
        {props.description ? (
          <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }} numberOfLines={1}>
            {props.description}
          </Text>
        ) : null}
      </View>
      {props.meta ? <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{props.meta}</Text> : null}
      {props.trailing}
      {props.chevron ? <Icon name="chevron-right" size={12} color={tokens.mutedForeground} /> : null}
    </View>
  );
}
