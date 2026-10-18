// A user avatar with an image, initials or a fallback icon, and a group that
// overlaps several avatars with an overflow count.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Image } from 'nsh/image';
import { Icon } from './icon';
import { type IconName } from './icons';
import { FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export type AvatarSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl';
export type AvatarShape = 'circle' | 'rounded' | 'square';

export interface AvatarProps {
  src?: string | { uri: string };
  name?: string;
  fallback?: string;
  size?: AvatarSize;
  shape?: AvatarShape;
  color?: string;
  icon?: IconName;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

const SIZE: Record<AvatarSize, number> = { xs: 20, sm: 24, md: 32, lg: 40, xl: 48 };
const FONT: Record<AvatarSize, number> = { xs: 9, sm: 10, md: 12, lg: 14, xl: 18 };

export function initialsOf(name?: string): string {
  if (!name) return '';
  return name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? '')
    .join('');
}

export function Avatar(props: AvatarProps) {
  const size = () => SIZE[props.size ?? 'md'];
  const radius = () => (props.shape === 'circle' ? size() / 2 : props.shape === 'square' ? 0 : RADIUS_MD);
  const label = () => props.name ?? props.fallback ?? 'Avatar';
  return (
    <View
      testID={props.testID}
      accessibilityRole="image"
      accessibilityLabel={props.accessibilityLabel ?? label()}
      style={[
        {
          width: size(),
          height: size(),
          borderRadius: radius(),
          backgroundColor: props.color ?? tokens.muted,
          alignItems: 'center',
          justifyContent: 'center',
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      {props.src ? (
        <Image src={props.src} resizeMode="cover" style={{ width: size(), height: size(), borderRadius: radius() }} />
      ) : props.fallback || props.name ? (
        <Text style={{ fontSize: FONT[props.size ?? 'md'], fontWeight: '600', color: tokens.foreground }}>
          {props.fallback ?? initialsOf(props.name)}
        </Text>
      ) : (
        <Icon name={props.icon ?? 'circle-user'} size={size() * 0.58} color={tokens.mutedForeground} />
      )}
    </View>
  );
}

export interface AvatarGroupProps {
  items?: AvatarProps[];
  max?: number;
  size?: AvatarSize;
  shape?: AvatarShape;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function AvatarGroup(props: AvatarGroupProps) {
  const items = props.items ?? [];
  const max = props.max ?? items.length;
  const shown = items.slice(0, max);
  const overflow = items.length - shown.length;
  const size = () => SIZE[props.size ?? 'md'];
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'center' }, props.style]} testID={props.testID}>
      {shown.map((item, index) => (
        <View
          style={{
            marginStart: index === 0 ? 0 : -size() * 0.28,
            borderRadius: 999,
            borderWidth: 2,
            borderColor: tokens.background,
            backgroundColor: tokens.background,
          }}
        >
          <Avatar {...item} size={props.size} shape={props.shape} />
        </View>
      ))}
      {overflow > 0 ? (
        <View
          style={{
            marginStart: -size() * 0.28,
            width: size(),
            height: size(),
            borderRadius: props.shape === 'circle' ? size() / 2 : RADIUS_MD,
            borderWidth: 2,
            borderColor: tokens.background,
            backgroundColor: tokens.muted,
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Text style={{ fontSize: FONT_SIZE_SM, fontWeight: '600', color: tokens.mutedForeground }}>
            +{overflow}
          </Text>
        </View>
      ) : null}
      {props.children}
    </View>
  );
}
