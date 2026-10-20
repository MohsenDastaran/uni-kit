// A compact status/separator row. Markers sit between messages or list
// sections, or form the rows of an agent run: icon, label, description, badge
// and a loading/pending mark.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, RADIUS_SM, SPACING_SM, tokens } from './theme';

export type MarkerTone = 'default' | 'muted' | 'info' | 'success' | 'warning' | 'danger';
export type MarkerStatus = 'success' | 'danger' | 'warning' | 'info' | 'pending';

export interface MarkerProps {
  icon?: IconName;
  iconTone?: MarkerTone;
  loading?: boolean;
  /** Draws the icon for a statusing row. */
  status?: MarkerStatus;
  separator?: boolean;
  border?: boolean;
  align?: 'start' | 'center' | 'end';
  /** Simple row text; `children` can replace it with rich content. */
  label?: string;
  description?: string;
  badge?: string;
  badgeTone?: MarkerTone;
  tone?: MarkerTone;
  size?: 'small' | 'medium';
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const toneOf = (tone?: MarkerTone): string => {
  switch (tone) {
    case 'info':
      return tokens.info;
    case 'success':
      return tokens.success;
    case 'warning':
      return tokens.warning;
    case 'danger':
      return tokens.danger;
    case 'muted':
      return tokens.mutedForeground;
    default:
      return tokens.foreground;
  }
};

export function Marker(props: MarkerProps) {
  const statusColor = () => {
    switch (props.status) {
      case 'success':
        return tokens.success;
      case 'danger':
        return tokens.danger;
      case 'warning':
        return tokens.warning;
      case 'info':
        return tokens.info;
      case 'pending':
        return tokens.mutedForeground;
      default:
        return tokens.mutedForeground;
    }
  };
  const statusIcon = () => {
    switch (props.status) {
      case 'success':
        return 'check' as const;
      case 'danger':
        return 'x' as const;
      case 'warning':
        return 'circle-alert' as const;
      case 'info':
        return 'info' as const;
      default:
        return 'loader-circle' as const;
    }
  };
  const mark = () => {
    if (props.status === 'pending' || props.loading) {
      return <Icon name="loader-circle" spinning size={13} color={statusColor()} />;
    }
    if (props.status) return <Icon name={statusIcon()} size={13} color={statusColor()} />;
    if (props.icon) return <Icon name={props.icon} size={13} color={toneOf(props.iconTone ?? 'muted')} />;
    return null;
  };
  if (props.separator) {
    return (
      <View
        testID={props.testID}
        style={[{ flexDirection: 'row', alignItems: 'center', gap: 8 }, props.style]}
      >
        <View style={{ flex: 1, height: 1, backgroundColor: tokens.border }} />
        <Text style={{ fontSize: 10, lineHeight: 14, color: props.tone ? toneOf(props.tone) : tokens.mutedForeground }}>
          {props.label ?? ''}{props.children}
        </Text>
        <View style={{ flex: 1, height: 1, backgroundColor: tokens.border }} />
      </View>
    );
  }
  return (
    <View
      testID={props.testID}
      style={[
        {
          width: '100%',
          flexDirection: 'row',
          alignItems: 'flex-start',
          gap: 8,
          justifyContent:
            props.align === 'end' ? 'flex-end' : props.align === 'center' ? 'center' : 'flex-start',
          paddingVertical: props.size === 'small' ? 3 : 5,
          paddingHorizontal: props.border ? 8 : 0,
          borderWidth: props.border ? 1 : 0,
          borderColor: tokens.border,
          borderRadius: props.border ? RADIUS_SM : 0,
        },
        props.style,
      ]}
    >
      {mark()}
      <View style={{ flex: props.align === 'start' || !props.align ? 1 : 0, minWidth: 0, gap: 1 }}>
        {props.label ? (
          <Text style={{ fontSize: 11, lineHeight: 16, color: toneOf(props.tone ?? 'muted') }}>{props.label}</Text>
        ) : null}
        {props.description ? (
          <Text style={{ fontSize: 9, lineHeight: 13, color: tokens.mutedForeground }}>{props.description}</Text>
        ) : null}
        {props.children}
      </View>
      {props.badge ? (
        <View style={{ paddingHorizontal: 5, paddingVertical: 1, borderRadius: 4, backgroundColor: alpha(toneOf(props.badgeTone), 0.12), borderWidth: 1, borderColor: alpha(toneOf(props.badgeTone), 0.3) }}>
          <Text style={{ fontSize: 8, lineHeight: 11, fontWeight: '600', color: toneOf(props.badgeTone) }}>{props.badge}</Text>
        </View>
      ) : null}
    </View>
  );
}

/** A compact agent-run/log surface with 8px row rhythm. */
export function MarkerGroup(props: {
  title?: string;
  description?: string;
  footer?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View
      style={[
        {
          width: '100%',
          padding: 12,
          gap: 3,
          borderRadius: 10,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.popover,
        },
        props.style,
      ]}
    >
      {props.title ? (
        <View style={{ gap: 1, paddingBottom: 6 }}>
          <Text style={{ fontSize: 12, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
          {props.description ? (
            <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
      {props.children}
      {props.footer ? <View style={{ paddingTop: 6 }}>{props.footer}</View> : null}
    </View>
  );
}

export function MarkerIcon(props: { name: IconName; color?: string; size?: number }) {
  return <Icon name={props.name} size={props.size ?? 14} color={props.color ?? tokens.mutedForeground} />;
}

export function MarkerContent(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={[{ flex: 1, minWidth: 0, gap: 2 }, props.style]}>{props.children}</View>;
}

export function MarkerLabel(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: 11, lineHeight: 16, color: tokens.mutedForeground }, props.style]}>{props.children}</Text>
  );
}
