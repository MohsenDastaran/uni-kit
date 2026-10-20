// A push button. Mirrors GPUI Component's `Button` and the Slint version beside
// it: variants, outline, sizes, a leading icon, a trailing icon, a loading
// state, a selected state and grouped corners, with the same rest, hover, press
// and disabled fills.
//
// `nsh/ui/mobile` has a Button, but it exposes four variants and a label only:
// no sizes, icon slots, loading mark or grouped corners. This file therefore
// draws the catalog's contract from Views and Text. A plain `primary`,
// `secondary`, `danger` or `plain` button in an app can still use the built-in
// mobile Button directly.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  alpha,
  type ControlSize,
  controlFontSize,
  controlHeight,
  hovered,
  iconSize,
  isDark,
  mix,
  pressed,
  RADIUS_MD,
  tokens,
} from './theme';

export type ButtonVariant =
  | 'default'
  | 'primary'
  | 'secondary'
  | 'danger'
  | 'warning'
  | 'success'
  | 'info'
  | 'ghost'
  | 'link'
  | 'text';

export interface ButtonProps {
  label?: string;
  variant?: ButtonVariant;
  size?: ControlSize;
  outline?: boolean;
  disabled?: boolean;
  loading?: boolean;
  selected?: boolean;
  icon?: IconName;
  suffixIcon?: IconName;
  /** Square the leading corners so adjacent buttons share one edge. */
  groupStart?: boolean;
  /** Square the trailing corners. */
  groupEnd?: boolean;
  /** Square the left and right sides instead of start/end, for a vertical stack. */
  groupVertical?: boolean;
  /** A corner radius that replaces the size default. Half the height draws a pill. */
  rounded?: number;
  width?: number | 'full';
  onPress?: () => void;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

export function Button(props: ButtonProps) {
  const pressing = signal(false);
  const hovering = signal(false);
  const variant = () => props.variant ?? 'default';
  const size = () => props.size ?? 'medium';
  const label = () => props.label ?? '';
  const iconOnly = () => !label() && !props.children && Boolean(props.icon);
  const interactive = () => !props.disabled;
  const pressable = () => interactive() && !props.loading;
  const statusVariant = () => ['danger', 'warning', 'success', 'info'].includes(variant());
  const transparentFill = () => ['ghost', 'link', 'text'].includes(variant());
  const tone = () => {
    switch (variant()) {
      case 'danger':
        return tokens.danger;
      case 'warning':
        return tokens.warning;
      case 'success':
        return tokens.success;
      case 'info':
        return tokens.info;
      case 'primary':
        return tokens.primary;
      case 'secondary':
        return tokens.secondary;
      default:
        return tokens.foreground;
    }
  };
  const ink = () => {
    if (!interactive()) return tokens.mutedForeground;
    switch (variant()) {
      case 'link':
        return tokens.primary;
      case 'text':
        return alpha(tokens.foreground, 0.9);
      case 'ghost':
        return tokens.secondaryForeground;
      case 'default':
        return tokens.foreground;
      case 'primary':
        // A selected primary is drawn as a tonal surface, so its label is the
        // tone rather than the primary's on-fill ink.
        return props.selected || props.outline ? tone() : tokens.primaryForeground;
      case 'secondary':
        return tokens.secondaryForeground;
      default:
        return statusVariant() ? tone() : tokens.foreground;
    }
  };
  const restFill = () => {
    if (transparentFill() || props.outline) return 'transparent';
    if (statusVariant()) return alpha(tone(), 0.2);
    if (variant() === 'primary') return tokens.primary;
    if (variant() === 'secondary') return tokens.secondary;
    return isDark() ? alpha(tokens.input, 0.7) : tokens.background;
  };
  const disabledFill = () => {
    if (transparentFill() || props.outline) return 'transparent';
    if (variant() === 'primary') return alpha(tokens.primary, 0.15);
    if (statusVariant()) return alpha(tone(), 0.15);
    if (variant() === 'secondary') return tokens.secondary;
    return mix(tokens.input, tokens.background, 0.5);
  };
  const hoverFill = () => {
    if (variant() === 'ghost') return isDark() ? alpha(tokens.accent, 0.5) : tokens.accent;
    if (transparentFill()) return 'transparent';
    if (props.outline) {
      return statusVariant() || variant() === 'primary' ? alpha(tone(), 0.1) : tokens.accent;
    }
    if (statusVariant()) return alpha(tone(), 0.3);
    if (variant() === 'primary') return mix(tokens.primary, tokens.background, 0.9);
    if (variant() === 'secondary') return mix(tokens.secondary, tokens.background, 0.1);
    return mix(tokens.input, tokens.background, 0.5);
  };
  const selectedFill = () => {
    if (variant() === 'primary') return alpha(tokens.primary, 0.1);
    if (statusVariant()) return alpha(tone(), 0.12);
    if (props.outline) {
      return variant() === 'secondary' ? tokens.accent : alpha(tokens.primary, 0.1);
    }
    if (transparentFill()) return alpha(tokens.primary, 0.1);
    return alpha(tokens.primary, isDark() ? 0.28 : 0.12);
  };
  const pressFill = () => {
    if (variant() === 'ghost') return mix(tokens.input, tokens.background, 0.3);
    if (transparentFill()) return 'transparent';
    if (props.outline) {
      return statusVariant() || variant() === 'primary'
        ? alpha(tone(), 0.2)
        : hovered(tokens.accent, isDark());
    }
    if (statusVariant()) return alpha(tone(), 0.4);
    if (variant() === 'primary') return pressed(tokens.primary, isDark());
    if (variant() === 'secondary') return pressed(tokens.secondary, isDark());
    return mix(tokens.input, tokens.background, 0.3);
  };
  const borderTone = () => {
    if (transparentFill()) return 'transparent';
    if (variant() === 'default') return tokens.input;
    if (variant() === 'secondary') return tokens.border;
    if (statusVariant()) return alpha(tone(), props.outline ? 0.45 : 0.2);
    if (variant() === 'primary') return tokens.primary;
    return tokens.border;
  };
  const padding = () =>
    iconOnly() || variant() === 'link' || variant() === 'text'
      ? 0
      : size() === 'xsmall'
        ? 4
        : size() === 'small'
          ? 8
          : size() === 'large'
            ? 16
            : 10;
  const gap = () => (size() === 'xsmall' || size() === 'small' ? 4 : 8);
  const motion = () => {
    if (!interactive()) return undefined;
    // Hover changes the fill; it must not lift, scale or shadow the control.
    // Keeping the baseline fixed also stops a button from visibly moving under
    // the pointer while its inline background is changing.
    const base = 'transition-colors duration-150 ease-out';
    if (grouped()) return base;
    if (variant() === 'link') return `${base} hover:underline`;
    if (transparentFill()) return `${base} hover:opacity-75`;
    return base;
  };
  const radius = () => props.rounded ?? RADIUS_MD;
  const grouped = () => Boolean(props.groupStart || props.groupEnd || props.groupVertical);
  // A group's first button has only `groupEnd` (its trailing edge meets the
  // next one); every later button sets `groupStart`. The button before the
  // first is the only one that must not eat its neighbour's border.
  const firstInGroup = () => Boolean(props.groupEnd && !props.groupStart);
  const groupOverlap = () => (grouped() && !firstInGroup() ? -1 : 0);
  const corner = (side: 'start' | 'end') =>
    side === 'start' ? (props.groupStart ? 0 : radius()) : props.groupEnd ? 0 : radius();

  return (
    <View
      testID={props.testID}
      accessibilityRole="button"
      accessibilityLabel={props.accessibilityLabel ?? label()}
      accessibilityState={{
        disabled: !interactive(),
        busy: Boolean(props.loading),
        selected: Boolean(props.selected),
      }}
      disabled={!interactive()}
      onPress={pressable() ? props.onPress : undefined}
      onPressIn={pressable() ? () => pressing.set(true) : undefined}
      onPressOut={pressable() ? () => pressing.set(false) : undefined}
      onHoverIn={() => hovering.set(true)}
      onHoverOut={() => hovering.set(false)}
      class={pressable() ? motion() : undefined}
      activeOpacity={pressable() ? 0.92 : 1}
      style={[
        {
          height: controlHeight(size()),
          minWidth: iconOnly() ? controlHeight(size()) : 0,
          paddingHorizontal: padding(),
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'center',
          gap: gap(),
          alignSelf: props.width === 'full' ? 'stretch' : 'flex-start',
          ...(props.groupVertical ? { marginTop: groupOverlap() } : { marginStart: groupOverlap() }),
          zIndex: pressing() || hovering() || props.selected ? 1 : 0,
          ...(props.width === 'full' ? { width: '100%' as const } : typeof props.width === 'number' ? { width: props.width } : {}),
          borderTopLeftRadius: corner('start'),
          borderTopRightRadius: corner('end'),
          borderBottomRightRadius: corner('end'),
          borderBottomLeftRadius: corner('start'),
          borderWidth: !transparentFill() && (variant() === 'default' || props.outline) ? 1 : 0,
          borderColor: interactive() ? borderTone() : alpha(borderTone(), 0.5),
          backgroundColor: !interactive()
            ? disabledFill()
            : pressing()
              ? pressFill()
              : props.selected
                ? selectedFill()
                : hovering()
                  ? hoverFill()
                  : restFill(),
        },
        props.style,
      ]}
    >
      {props.loading || props.icon ? (
        <Icon
          name={props.loading ? 'loader-circle' : props.icon ?? 'circle'}
          spinning={Boolean(props.loading)}
          size={iconSize(size())}
          color={ink()}
        />
      ) : null}
      {iconOnly() ? null : props.children ? (
        props.children
      ) : (
        <Text
          numberOfLines={1}
          style={{
            fontSize: controlFontSize(size()),
            fontWeight: '500',
            color: ink(),
            textAlign: 'center',
          }}
        >
          {label()}
        </Text>
      )}
      {props.suffixIcon ? (
        <Icon name={props.suffixIcon} size={iconSize(size())} color={ink()} />
      ) : null}
    </View>
  );
}
