// A header that expands and collapses its content.
//
// The ReUI shape: a quiet bordered card, a plain chevron, an optional muted
// icon tile, a status pill and a second line. The header is the only press
// target; the content is revealed immediately, with no height animation. The
// chevron turns and the hover colour eases, and that is the whole motion.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { reduceMotion } from 'nsh/accessibility';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  RADIUS_LG,
  SPACING_SM,
  alpha,
  type ControlSize,
  iconSize,
  isDark,
  tokens,
} from './theme';

export type CollapsibleTone = 'neutral' | 'info' | 'success' | 'warning' | 'danger';

export interface CollapsibleProps {
  title?: string;
  /** A caller-drawn header; the chevron stays unless `chevron` is false. */
  trigger?: JSX.Element;
  open?: boolean;
  defaultOpen?: boolean;
  disabled?: boolean;
  /** A leading icon, drawn in a quiet square. */
  icon?: IconName;
  /** A caller-drawn leading element, such as an avatar. Wins over `icon`. */
  leading?: JSX.Element;
  /** A muted second line under the title. */
  description?: string;
  /** A short status pill after the title. */
  badge?: string;
  badgeTone?: CollapsibleTone;
  /** Draw the header and its body on one bordered card. */
  bordered?: boolean;
  /** Draw the chevron. Off when the trigger carries a mark of its own. */
  chevron?: boolean;
  size?: ControlSize;
  onOpenChange?: (open: boolean) => void;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const PADDING_X: Record<ControlSize, number> = { xsmall: 10, small: 12, medium: 14, large: 18 };
const PADDING_Y: Record<ControlSize, number> = { xsmall: 8, small: 10, medium: 13, large: 16 };
const TITLE_FONT: Record<ControlSize, number> = { xsmall: 13, small: 13, medium: 14, large: 16 };
const LEADING: Record<ControlSize, number> = { xsmall: 26, small: 28, medium: 32, large: 36 };

const toneOf = (tone: CollapsibleTone | undefined) => {
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
};

function Pill(props: { label: string; tone?: CollapsibleTone }) {
  const color = () => toneOf(props.tone);
  return (
    <View
      style={{
        paddingHorizontal: 6,
        paddingVertical: 2,
        borderRadius: 5,
        backgroundColor: alpha(color(), 0.14),
      }}
    >
      <Text style={{ fontSize: 10, lineHeight: 13, fontWeight: '600', color: color() }}>
        {props.label}
      </Text>
    </View>
  );
}

export function Collapsible(props: CollapsibleProps) {
  const own = signal(Boolean(props.defaultOpen));
  const open = () => props.open ?? own();
  const size = () => props.size ?? 'medium';
  const bordered = () => Boolean(props.bordered);
  const surface = () => (isDark() ? alpha(tokens.foreground, 0.035) : tokens.background);
  const toggle = () => {
    if (props.disabled) return;
    const next = !open();
    if (props.open === undefined) own.set(next);
    props.onOpenChange?.(next);
  };
  const chevronTransition = (): ViewStyle =>
    reduceMotion()
      ? {}
      : {
          transitionProperty: 'transform',
          transitionDuration: 150,
          transitionTimingFunction: 'ease-out',
        };
  const hasLeading = Boolean(props.leading || props.icon);
  const leadingSize = LEADING[size()];
  const indent =
    PADDING_X[size()] +
    (props.trigger ? 0 : hasLeading ? leadingSize + SPACING_SM : 0);

  return (
    <View
      testID={props.testID}
      style={[
        bordered()
          ? {
              borderWidth: 1,
              borderColor: tokens.border,
              borderRadius: RADIUS_LG,
              backgroundColor: surface(),
              overflow: 'hidden',
            }
          : {},
        props.style,
      ]}
    >
      <View
        accessibilityRole="button"
        accessibilityLabel={props.title}
        accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
        disabled={Boolean(props.disabled)}
        onPress={toggle}
        activeOpacity={props.disabled ? 1 : 0.9}
        class={
          props.disabled
            ? undefined
            : 'transition-colors duration-150 hover:bg-black/[0.035] dark:hover:bg-white/[0.045]'
        }
        style={{
          flexDirection: 'row',
          alignItems: 'center',
          gap: SPACING_SM,
          paddingHorizontal: PADDING_X[size()],
          paddingVertical: PADDING_Y[size()],
          opacity: props.disabled ? 0.45 : 1,
        }}
      >
        {props.leading ? (
          <View style={{ width: leadingSize, height: leadingSize, alignItems: 'center', justifyContent: 'center' }}>
            {props.leading}
          </View>
        ) : props.icon ? (
          <View
            style={{
              width: leadingSize,
              height: leadingSize,
              borderRadius: 8,
              alignItems: 'center',
              justifyContent: 'center',
              backgroundColor: tokens.muted,
            }}
          >
            <Icon name={props.icon} size={iconSize(size())} color={tokens.mutedForeground} />
          </View>
        ) : null}
        <View style={{ flex: 1, gap: 2 }}>
          {props.trigger ?? (
            <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6 }}>
              <Text
                numberOfLines={1}
                style={{
                  flexShrink: 1,
                  fontSize: TITLE_FONT[size()],
                  fontWeight: '600',
                  color: tokens.foreground,
                }}
              >
                {props.title ?? ''}
              </Text>
              {props.badge ? <Pill label={props.badge} tone={props.badgeTone} /> : null}
            </View>
          )}
          {props.description ? (
            <Text
              numberOfLines={1}
              style={{ fontSize: 12, lineHeight: 16, color: tokens.mutedForeground }}
            >
              {props.description}
            </Text>
          ) : null}
        </View>
        {props.trigger && props.badge ? <Pill label={props.badge} tone={props.badgeTone} /> : null}
        {props.chevron === false ? null : (
          <Icon
            name="chevron-down"
            size={16}
            color={tokens.mutedForeground}
            style={{
              transform: [{ rotate: open() ? '180deg' : '0deg' }],
              ...chevronTransition(),
            }}
          />
        )}
      </View>
      {open() ? (
        <View
          style={{
            paddingStart: indent,
            paddingEnd: PADDING_X[size()],
            paddingTop: 0,
            paddingBottom: PADDING_Y[size()] + 2,
            gap: SPACING_SM,
          }}
        >
          {props.children}
        </View>
      ) : null}
    </View>
  );
}
