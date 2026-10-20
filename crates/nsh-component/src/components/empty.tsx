// A composed empty state: media/icon, title, description and actions.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { FONT_SIZE_SM, RADIUS_LG, SPACING_MD, tokens } from './theme';

export type EmptyVariant = 'default' | 'outline' | 'background';
export type EmptySize = 'small' | 'medium' | 'large';

export interface EmptyProps {
  icon?: IconName;
  media?: JSX.Element;
  title?: string;
  description?: string;
  action?: JSX.Element;
  variant?: EmptyVariant;
  size?: EmptySize;
  /** Draw an outline variant with a dashed border. */
  dashed?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Empty(props: EmptyProps) {
  const variant = () => props.variant ?? 'default';
  const size = () => props.size ?? 'medium';
  const titleSize = () => (size() === 'small' ? 13 : size() === 'large' ? 17 : 14);
  const bodySize = () => (size() === 'small' ? 11 : size() === 'large' ? 13 : 12);
  const pad = () => (size() === 'small' ? 20 : size() === 'large' ? 36 : 28);
  return (
    <View
      testID={props.testID}
      style={[
        {
          alignItems: 'center',
          justifyContent: 'center',
          gap: size() === 'small' ? 8 : size() === 'large' ? 14 : 10,
          padding: pad(),
          borderRadius: RADIUS_LG,
          borderWidth: variant() === 'outline' ? 1 : 0,
          borderColor: tokens.border,
          borderStyle: props.dashed ? 'dashed' : 'solid',
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
        <Text style={{ fontSize: titleSize(), lineHeight: titleSize() + 4, fontWeight: '600', color: tokens.foreground, textAlign: 'center' }}>
          {props.title ?? 'Nothing here yet'}
        </Text>
        {props.description ? (
          <Text style={{ fontSize: bodySize(), lineHeight: bodySize() + 5, color: tokens.mutedForeground, textAlign: 'center' }}>
            {props.description}
          </Text>
        ) : null}
      </View>
      {props.action ? <View style={{ alignItems: 'center' }}>{props.action}</View> : null}
      {props.children ? <View style={{ alignItems: 'center' }}>{props.children}</View> : null}
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
