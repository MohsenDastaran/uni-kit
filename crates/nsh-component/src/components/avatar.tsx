// A user avatar with an image, initials or a fallback icon, optional status
// badges, a plain or gradient ring and a loading placeholder. A group overlaps
// several avatars with an overflow count and an optional trailing slot.
//
// Everything is built from Views, Text, Image and the shared tokens, so the
// component adds no palette of its own: badges take a token tone, the ring takes
// the ring token, and the gradient ring mixes primary with the status tones.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Image } from 'nsh/image';
import { Canvas } from 'nsh/canvas';
import { reduceMotion } from 'nsh/accessibility';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, inkOn, RADIUS_MD, tokens } from './theme';

export type AvatarSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl';
export type AvatarShape = 'circle' | 'rounded' | 'square';
export type AvatarTone = 'default' | 'info' | 'success' | 'warning' | 'danger';
export type AvatarRing = 'none' | 'plain' | 'gradient';
export type AvatarBadgePlacement = 'top-start' | 'top-end' | 'bottom-start' | 'bottom-end';

/** One status chip at an avatar corner: a dot, or a glyph on a filled circle. */
export interface AvatarBadge {
  /** A Lucide glyph; without one the badge is a small status dot. */
  icon?: IconName;
  /** A token tone; `color` overrides it. */
  tone?: AvatarTone;
  /** A token colour that replaces the tone, for a brand or presence colour. */
  color?: string;
  /** Which corner it hangs on. Several badges cycle through the four corners. */
  placement?: AvatarBadgePlacement;
  accessibilityLabel?: string;
}

export interface AvatarProps {
  src?: string | { uri: string };
  name?: string;
  fallback?: string;
  size?: AvatarSize;
  shape?: AvatarShape;
  color?: string;
  icon?: IconName;
  /** Status dots or chips at the corners; they never change the avatar's box. */
  badges?: AvatarBadge[];
  /** A hairline ring, or an animated gradient ring, around the avatar. */
  ring?: AvatarRing;
  /** Ring thickness in points. The default is 2. */
  ringWidth?: number;
  /** Shows a pulsing placeholder in the same box instead of the image or initials. */
  loading?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

const SIZE: Record<AvatarSize, number> = { xs: 20, sm: 24, md: 32, lg: 40, xl: 48 };
const FONT: Record<AvatarSize, number> = { xs: 9, sm: 10, md: 12, lg: 14, xl: 18 };
const BADGE: Record<AvatarSize, number> = { xs: 10, sm: 12, md: 14, lg: 16, xl: 18 };
const DOT: Record<AvatarSize, number> = { xs: 5, sm: 6, md: 7, lg: 8, xl: 9 };
const PLACEMENT: AvatarBadgePlacement[] = ['top-end', 'bottom-end', 'top-start', 'bottom-start'];

export function initialsOf(name?: string): string {
  if (!name) return '';
  return name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? '')
    .join('');
}

/** The fill a badge tone names, read from the shared palette. */
function toneOf(tone?: AvatarTone): string {
  switch (tone) {
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
}

export function Avatar(props: AvatarProps) {
  const sizeKey = () => props.size ?? 'md';
  const size = () => SIZE[sizeKey()];
  const shape = () => props.shape ?? 'circle';
  const radius = () => (shape() === 'circle' ? size() / 2 : shape() === 'square' ? 0 : RADIUS_MD);
  const label = () => props.name ?? props.fallback ?? 'Avatar';

  // The ring sits outside the avatar box, so the avatar keeps its catalog size
  // and the wrapper grows by the ring and the breathing gap around it.
  const ring = () => props.ring ?? 'none';
  const ringWidth = () => (ring() === 'none' ? 0 : Math.max(1, props.ringWidth ?? 2));
  const ringGap = () => (ring() === 'none' ? 0 : 2);
  const outer = () => size() + (ringWidth() + ringGap()) * 2;
  const outerRadius = () =>
    shape() === 'circle' ? outer() / 2 : shape() === 'square' ? 0 : RADIUS_MD + ringWidth() + ringGap();
  const ringColors = () => [
    alpha(tokens.info, 0.9),
    alpha(tokens.primary, 0.35),
    alpha(tokens.success, 0.9),
    alpha(tokens.warning, 0.85),
    alpha(tokens.info, 0.9),
  ];
  // Reduced motion leaves the gradient in place and stops the sweep: no
  // perpetual animation, and no change in size when the preference changes.
  const ringMotion = (): ViewStyle =>
    reduceMotion()
      ? {}
      : {
          animationName: 'spin',
          animationDuration: 5000,
          animationIterationCount: 'infinite',
          animationTimingFunction: 'linear',
        };
  const loadingMotion = (): ViewStyle =>
    reduceMotion()
      ? {}
      : {
          animationName: 'pulse',
          animationDuration: 1400,
          animationIterationCount: 'infinite',
          animationTimingFunction: 'ease-in-out',
        };

  return (
    <View
      testID={props.testID}
      accessibilityRole="image"
      accessibilityLabel={props.loading ? `${label()}, loading` : props.accessibilityLabel ?? label()}
      accessibilityState={{ busy: Boolean(props.loading) }}
      style={[
        {
          width: outer(),
          height: outer(),
          position: 'relative',
          alignItems: 'center',
          justifyContent: 'center',
        },
        props.style,
      ]}
    >
      {ring() === 'plain' ? (
        <View
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            width: outer(),
            height: outer(),
            borderRadius: outerRadius(),
            backgroundColor: tokens.ring,
            zIndex: 0,
          }}
        />
      ) : ring() === 'gradient' ? (
        <Canvas
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            width: outer(),
            height: outer(),
            zIndex: 0,
            ...(shape() === 'circle' ? ringMotion() : {}),
          }}
          draw={(ctx) => {
            const line = ringWidth();
            const inset = line / 2;
            const side = outer() - line;
            const gradient = ctx.createConicGradient(0, outer() / 2, outer() / 2);
            const stops = ringColors();
            stops.forEach((color, index) => gradient.addColorStop(index / (stops.length - 1), color));
            ctx.strokeStyle = gradient;
            ctx.lineWidth = line;
            ctx.beginPath();
            if (shape() === 'circle') {
              ctx.arc(outer() / 2, outer() / 2, side / 2, 0, Math.PI * 2);
            } else if (shape() === 'square') {
              ctx.rect(inset, inset, side, side);
            } else {
              ctx.roundRect(inset, inset, side, side, outerRadius() - inset);
            }
            ctx.stroke();
          }}
        />
      ) : null}
      {ring() !== 'none' ? (
        <View
          style={{
            position: 'absolute',
            width: size() + ringGap() * 2,
            height: size() + ringGap() * 2,
            borderRadius: shape() === 'circle' ? (size() + ringGap() * 2) / 2 : shape() === 'square' ? 0 : RADIUS_MD + ringGap(),
            backgroundColor: tokens.background,
            zIndex: 1,
          }}
        />
      ) : null}
      <View
        style={{
          width: size(),
          height: size(),
          borderRadius: radius(),
          zIndex: 2,
          backgroundColor: props.loading ? tokens.muted : props.color ?? tokens.muted,
          alignItems: 'center',
          justifyContent: 'center',
          overflow: 'hidden',
          ...(props.loading ? loadingMotion() : {}),
        }}
      >
        {props.loading ? (
          <Icon
            name="loader-circle"
            spinning={!reduceMotion()}
            size={size() * 0.42}
            color={tokens.mutedForeground}
          />
        ) : props.src ? (
          <Image src={props.src} resizeMode="cover" style={{ width: size(), height: size(), borderRadius: radius() }} />
        ) : props.fallback || props.name ? (
          <Text style={{ fontSize: FONT[sizeKey()], fontWeight: '600', color: tokens.foreground }}>
            {props.fallback ?? initialsOf(props.name)}
          </Text>
        ) : (
          <Icon name={props.icon ?? 'circle-user'} size={size() * 0.58} color={tokens.mutedForeground} />
        )}
      </View>
      {(props.badges ?? []).map((badge, index) => {
        const place = badge.placement ?? PLACEMENT[index % PLACEMENT.length];
        const chip = badge.icon ? BADGE[sizeKey()] : DOT[sizeKey()];
        const offset = -Math.round(chip * 0.3);
        const fill = badge.color ?? toneOf(badge.tone);
        return (
          <View
            accessibilityLabel={badge.accessibilityLabel ?? `${label()} ${badge.tone ?? 'status'}`}
            style={{
              position: 'absolute',
              width: chip,
              height: chip,
              borderRadius: chip / 2,
              backgroundColor: fill,
              alignItems: 'center',
              justifyContent: 'center',
              borderWidth: badge.icon ? 2 : 1,
              borderColor: tokens.background,
              ...(place === 'top-end'
                ? { top: offset, end: offset }
                : place === 'top-start'
                  ? { top: offset, start: offset }
                  : place === 'bottom-end'
                    ? { bottom: offset, end: offset }
                    : { bottom: offset, start: offset }),
            }}
          >
            {badge.icon ? <Icon name={badge.icon} size={chip * 0.58} color={inkOn(fill)} /> : null}
          </View>
        );
      })}
    </View>
  );
}

export interface AvatarGroupProps {
  items?: AvatarProps[];
  max?: number;
  size?: AvatarSize;
  shape?: AvatarShape;
  /** A trailing control after the overflow chip, usually an add button. */
  addon?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function AvatarGroup(props: AvatarGroupProps) {
  const size = () => SIZE[props.size ?? 'md'];
  const overflowRadius = () => ((props.shape ?? 'circle') === 'circle' ? size() / 2 : props.shape === 'square' ? 0 : RADIUS_MD);
  const items = () => props.items ?? [];
  const limit = () => Math.min(props.max ?? items().length, items().length);
  const overflow = () => items().length - limit();
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'center' }, props.style]} testID={props.testID}>
      {items()
        .slice(0, limit())
        .map((item, index) => (
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
      {overflow() > 0 ? (
        <View
          style={{
            marginStart: -size() * 0.28,
            minWidth: size(),
            height: size(),
            paddingHorizontal: 4,
            borderRadius: overflowRadius(),
            borderWidth: 2,
            borderColor: tokens.background,
            backgroundColor: tokens.muted,
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Text
            style={{
              fontSize: FONT[props.size ?? 'md'],
              fontWeight: '600',
              color: tokens.mutedForeground,
            }}
          >
            +{overflow()}
          </Text>
        </View>
      ) : null}
      {props.addon ? (
        <View
          style={{
            marginStart: -size() * 0.28,
            borderRadius: 999,
            borderWidth: 2,
            borderColor: tokens.background,
            backgroundColor: tokens.background,
          }}
        >
          {props.addon}
        </View>
      ) : null}
      {props.children}
    </View>
  );
}
