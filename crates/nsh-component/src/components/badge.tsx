// A compact pill with an optional leading slot, status dot, count and dismiss
// button — or, as before, a count, dot or status icon anchored to the corner of
// its child.
//
// Mirrors GPUI Component's `Badge` and the Slint version beside it: when it has
// `children` it is an overlay anchored past the child's top-right corner; when
// it has standalone content (`label`, a leading slot, a variant or a press
// handler) it is a ReUI-style badge/pill. Both share the same count, colour,
// size and dot props, so the anchored examples keep working unchanged.
//
// The standalone pill uses only the shared tokens: a soft tint and border for
// the default appearance, the tone itself for `solid`, and `inkOn` for the
// label on a filled tone. Hover and press are 150 ms colour/opacity transitions
// on the interactive badge, its dismiss button and the link variant.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  alpha,
  type ControlSize,
  hovered,
  inkOn,
  isDark,
  mix,
  tokens,
} from './theme';

export type BadgeVariant =
  | 'default'
  | 'secondary'
  | 'outline'
  | 'info'
  | 'success'
  | 'warning'
  | 'danger';

/** `soft` tints the surface; `solid` fills it with the tone. */
export type BadgeAppearance = 'soft' | 'solid';

export interface BadgeProps {
  // ---------------------------------------------------------------- overlay
  count?: number;
  max?: number;
  dot?: boolean;
  /** A Lucide glyph: the marker glyph over `children`, or the leading glyph of a pill. */
  icon?: IconName;
  /** A token colour that replaces the variant's tone. */
  color?: string;
  size?: ControlSize;
  /** The element the badge is anchored to. Without it, the badge is standalone. */
  children?: JSX.Element;

  // -------------------------------------------------------------- standalone
  /** The pill's text. With `count` and no label, the formatted count is shown. */
  label?: string;
  variant?: BadgeVariant;
  appearance?: BadgeAppearance;
  /** A numeric corner radius, or `'full'` for a pill. Defaults to full. */
  radius?: number | 'full';
  /** A leading avatar, image, icon or flag drawn before the label. */
  leading?: JSX.Element;
  /** Draws a dismiss button after the label; pair it with `onRemove`. */
  closable?: boolean;
  onRemove?: () => void;
  /** Makes the standalone badge an interactive link. */
  onPress?: () => void;

  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

/** The badge's tone, read from the shared palette. */
function toneOf(variant: BadgeVariant): string {
  switch (variant) {
    case 'secondary':
      return tokens.secondary;
    case 'info':
      return tokens.info;
    case 'success':
      return tokens.success;
    case 'warning':
      return tokens.warning;
    case 'danger':
      return tokens.danger;
    default:
      return tokens.foreground;
  }
}

export function Badge(props: BadgeProps) {
  const hovering = signal(false);
  const dismissHovering = signal(false);
  const size = () => props.size ?? 'medium';
  const variant = () => props.variant ?? 'default';
  const appearance = () => props.appearance ?? 'soft';
  // A marker takes `color`; a bare variant also names its tone, so an anchored
  // count can say `variant="success"` without repeating the palette value.
  const color = () =>
    props.color ?? (props.variant !== undefined ? toneOf(variant()) : tokens.danger);
  const countLabel = () => {
    const count = props.count ?? 0;
    const max = props.max ?? 99;
    return count > max ? `${max}+` : String(count);
  };
  const hasNumber = () => !props.dot && !props.icon && (props.count ?? 0) > 0;
  const hasIcon = () => !props.dot && Boolean(props.icon);
  const hasDot = () => Boolean(props.dot) && ((props.count ?? 0) > 0 || !props.icon);

  // ---------------------------------------------------------------- markers
  const chip = () => (size() === 'large' ? 24 : size() === 'medium' ? 16 : 10);
  const numberHeight = () => (size() === 'small' || size() === 'xsmall' ? 14 : 16);

  // -------------------------------------------------------------- standalone
  const labelText = () => {
    if (props.label !== undefined) return props.label;
    return (props.count ?? 0) > 0 ? countLabel() : '';
  };
  const pill = () =>
    Boolean(
      props.label !== undefined ||
        props.leading ||
        props.closable ||
        props.onPress ||
        props.variant !== undefined ||
        props.appearance !== undefined ||
        props.radius !== undefined,
    );
  const interactive = () => Boolean(props.onPress);
  const pillHeight = () => (size() === 'small' ? 20 : size() === 'large' ? 28 : 24);
  const pillFont = () => (size() === 'small' ? 11 : size() === 'large' ? 13 : 12);
  const pillPadding = () => (size() === 'small' ? 7 : size() === 'large' ? 11 : 9);
  const pillGap = () => (size() === 'small' ? 4 : size() === 'large' ? 6 : 5);
  const leadingSize = () => (size() === 'small' ? 12 : size() === 'large' ? 16 : 14);
  const dismissSize = () => (size() === 'small' ? 14 : size() === 'large' ? 18 : 16);
  const pillRadius = () => {
    if (props.radius === 'full') return pillHeight() / 2;
    if (typeof props.radius === 'number') return props.radius;
    return pillHeight() / 2;
  };
  const colored = () => props.color !== undefined;
  const pillTone = () => (colored() ? color() : toneOf(variant()));

  const softFill = () => {
    if (colored()) return alpha(pillTone(), isDark() ? 0.18 : 0.12);
    if (variant() === 'outline') return 'transparent';
    if (variant() === 'secondary') return tokens.secondary;
    if (variant() === 'default') return tokens.muted;
    return alpha(pillTone(), isDark() ? 0.18 : 0.12);
  };
  const softBorder = () => {
    if (colored()) return alpha(pillTone(), 0.3);
    if (variant() === 'outline') return alpha(tokens.foreground, 0.28);
    if (variant() === 'default' || variant() === 'secondary') return tokens.border;
    return alpha(pillTone(), 0.3);
  };
  const softInk = () => {
    if (colored()) return isDark() ? pillTone() : mix(pillTone(), tokens.foreground, 0.55);
    if (variant() === 'secondary') return tokens.secondaryForeground;
    if (variant() === 'default' || variant() === 'outline') return tokens.foreground;
    return isDark() ? pillTone() : mix(pillTone(), tokens.foreground, 0.55);
  };
  const solidFill = () => {
    if (colored()) return pillTone();
    if (variant() === 'default') return tokens.foreground;
    if (variant() === 'secondary') return tokens.secondary;
    return pillTone();
  };
  const solidInk = () => {
    if (colored()) return inkOn(pillTone());
    if (variant() === 'default') return tokens.background;
    if (variant() === 'secondary') return tokens.secondaryForeground;
    return inkOn(pillTone());
  };
  const pillFill = () => (appearance() === 'solid' && variant() !== 'outline' ? solidFill() : softFill());
  const pillBorder = () => (appearance() === 'solid' && variant() !== 'outline' ? solidFill() : softBorder());
  const pillInk = () => (appearance() === 'solid' && variant() !== 'outline' ? solidInk() : softInk());
  const hoverFill = () => {
    const fill = pillFill();
    if (fill === 'transparent') return alpha(pillTone(), isDark() ? 0.14 : 0.08);
    return hovered(fill, isDark());
  };
  const dismissHoverFill = () => alpha(pillInk(), isDark() ? 0.16 : 0.1);

  const leadingGlyph = () => {
    if (props.leading) return props.leading;
    if (!props.icon) return null;
    return <Icon name={props.icon} size={leadingSize()} color={pillInk()} />;
  };
  const hasLeading = () => Boolean(props.leading || props.icon);
  const showPillDot = () => Boolean(props.dot) && (hasLeading() || Boolean(labelText()));
  const showPillText = () => labelText().length > 0;

  const chipView = () => (
    <>
      {hasDot() ? (
        <View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: color() }} />
      ) : null}
      {hasIcon() ? (
        <View
          style={{
            width: chip(),
            height: chip(),
            borderRadius: chip() / 2,
            borderWidth: 1,
            borderColor: tokens.background,
            backgroundColor: color(),
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Icon name={props.icon ?? 'circle'} size={chip() * 0.62} color={inkOn(color())} />
        </View>
      ) : null}
      {hasNumber() ? (
        <View
          style={{
            height: numberHeight(),
            minWidth: numberHeight(),
            paddingHorizontal: 4,
            borderRadius: numberHeight() / 2,
            backgroundColor: color(),
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Text style={{ fontSize: 10, lineHeight: 12, color: inkOn(color()), textAlign: 'center' }}>
            {countLabel()}
          </Text>
        </View>
      ) : null}
    </>
  );

  const markerView = () => (
    <>
      {hasDot() ? (
        <View
          style={{
            position: 'absolute',
            top: 0,
            right: 0,
            width: 6,
            height: 6,
            borderRadius: 3,
            backgroundColor: color(),
          }}
        />
      ) : null}
      {hasIcon() ? (
        <View
          style={{
            position: 'absolute',
            right: -chip() + 2,
            bottom: -chip() + 2,
            width: chip(),
            height: chip(),
            borderRadius: chip() / 2,
            borderWidth: 1,
            borderColor: tokens.background,
            backgroundColor: color(),
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Icon name={props.icon ?? 'circle'} size={chip() * 0.62} color={inkOn(color())} />
        </View>
      ) : null}
      {hasNumber() ? (
        <View
          style={{
            position: 'absolute',
            top: -4,
            right: -6,
            height: numberHeight(),
            minWidth: numberHeight(),
            paddingHorizontal: 4,
            borderRadius: numberHeight() / 2,
            backgroundColor: color(),
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Text style={{ fontSize: 10, lineHeight: 12, color: inkOn(color()), textAlign: 'center' }}>
            {countLabel()}
          </Text>
        </View>
      ) : null}
    </>
  );

  const pillView = () => (
    <View
      testID={props.testID}
      accessibilityRole={interactive() ? 'link' : undefined}
      accessibilityLabel={
        props.accessibilityLabel ??
        (interactive() ? labelText() || undefined : undefined)
      }
      accessible={interactive() || Boolean(props.accessibilityLabel)}
      onPress={interactive() ? props.onPress : undefined}
      onHoverIn={() => hovering.set(true)}
      onHoverOut={() => hovering.set(false)}
      activeOpacity={interactive() ? 0.9 : 1}
      class={
        interactive()
          ? 'transition-all duration-150 ease-out hover:opacity-90 hover:underline focus:underline active:opacity-75'
          : undefined
      }
      style={[
        {
          height: pillHeight(),
          paddingHorizontal: pillPadding(),
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'center',
          alignSelf: 'flex-start',
          flexShrink: 0,
          gap: pillGap(),
          borderWidth: 1,
          borderColor: pillBorder(),
          borderRadius: pillRadius(),
          backgroundColor: interactive() && hovering() ? hoverFill() : pillFill(),
        },
        props.style,
      ]}
    >
      {hasLeading() ? (
        <View style={{ alignItems: 'center', justifyContent: 'center', flexShrink: 0 }}>
          {leadingGlyph()}
        </View>
      ) : null}
      {showPillDot() ? (
        <View
          style={{
            width: size() === 'small' ? 5 : 6,
            height: size() === 'small' ? 5 : 6,
            borderRadius: size() === 'small' ? 2.5 : 3,
            backgroundColor: pillInk(),
            flexShrink: 0,
          }}
        />
      ) : null}
      {showPillText() ? (
        <Text
          numberOfLines={1}
          style={{
            fontSize: pillFont(),
            lineHeight: pillFont() + 4,
            fontWeight: '500',
            color: pillInk(),
            textDecorationLine: interactive() && hovering() ? 'underline' : 'none',
          }}
        >
          {labelText()}
        </Text>
      ) : null}
      {props.closable ? (
        <View
          accessibilityRole="button"
          accessibilityLabel={
            props.accessibilityLabel
              ? `Remove ${props.accessibilityLabel}`
              : props.label
                ? `Remove ${props.label}`
                : 'Remove badge'
          }
          onPress={props.onRemove}
          onHoverIn={() => dismissHovering.set(true)}
          onHoverOut={() => dismissHovering.set(false)}
          activeOpacity={0.6}
          class="items-center justify-center rounded-full transition-colors duration-150 ease-out hover:bg-black/10 active:bg-black/20 dark:hover:bg-white/15 dark:active:bg-white/25"
          style={{
            width: dismissSize(),
            height: dismissSize(),
            borderRadius: dismissSize() / 2,
            alignItems: 'center',
            justifyContent: 'center',
            flexShrink: 0,
            marginEnd: -2,
            // The hover classes paint the web state; the signal paints the
            // native one. Leaving the resting fill unset lets the class show.
            ...(dismissHovering() ? { backgroundColor: dismissHoverFill() } : {}),
          }}
        >
          <Icon name="x" size={dismissSize() - 5} color={pillInk()} />
        </View>
      ) : null}
    </View>
  );

  const content = props.children;

  if (!content && pill()) return pillView();
  if (!content) {
    // The standalone marker keeps the frame it always had: a row so the chip
    // sits on the baseline, and the caller's testID/label/style on the wrapper.
    return (
      <View
        testID={props.testID}
        accessibilityLabel={props.accessibilityLabel}
        style={[
          {
            position: 'relative',
            alignSelf: 'flex-start',
            flexDirection: 'row',
            alignItems: 'center',
          },
          props.style,
        ]}
      >
        {chipView()}
      </View>
    );
  }

  return (
    <View
      testID={props.testID}
      accessibilityLabel={props.accessibilityLabel}
      style={[
        { position: 'relative', alignSelf: 'flex-start' },
        props.style,
      ]}
    >
      {content}
      {markerView()}
    </View>
  );
}
