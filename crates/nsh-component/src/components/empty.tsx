// A composed empty state: media/icon, title, description and actions.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { FONT_SIZE_SM, RADIUS_LG, SPACING_MD, tokens } from './theme';

export type EmptyVariant = 'default' | 'outline' | 'background';

export interface EmptyProps {
  icon?: IconName;
  media?: JSX.Element;
  title?: string;
  description?: string;
  action?: JSX.Element;
  variant?: EmptyVariant;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Empty(props: EmptyProps) {
  const variant = () => props.variant ?? 'default';
  return (
    <View
      testID={props.testID}
      style={[
        {
          alignItems: 'center',
          justifyContent: 'center',
          gap: SPACING_MD,
          padding: 32,
          borderRadius: RADIUS_LG,
          borderWidth: variant() === 'outline' ? 1 : 0,
          borderColor: tokens.border,
          backgroundColor: variant() === 'background' ? tokens.muted : 'transparent',
        },
        props.style,
      ]}
    >
      {props.media ??
        (props.icon ? (
          <View
            style={{
              width: 56,
              height: 56,
              borderRadius: 28,
              backgroundColor: tokens.muted,
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <Icon name={props.icon} size={26} color={tokens.mutedForeground} />
          </View>
        ) : null)}
      <View style={{ alignItems: 'center', gap: 4 }}>
        <Text style={{ fontSize: 17, fontWeight: '600', color: tokens.foreground, textAlign: 'center' }}>
          {props.title ?? 'Nothing here yet'}
        </Text>
        {props.description ? (
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground, textAlign: 'center' }}>
            {props.description}
          </Text>
        ) : null}
      </View>
      {props.action}
      {props.children}
    </View>
  );
}

export function EmptyHeader(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={props.style}>{props.children}</View>;
}

export function EmptyTitle(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: 17, fontWeight: '600', color: tokens.foreground }, props.style]}>
      {props.children}
    </Text>
  );
}

export function EmptyDescription(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }, props.style]}>
      {props.children}
    </Text>
  );
}

export function EmptyContent(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={props.style}>{props.children}</View>;
}
