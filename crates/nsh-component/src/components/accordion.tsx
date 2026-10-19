// A stack of collapsible sections, single or multiple open.
//
// The visual language follows the ReUI accordion: a quiet card or a hairline
// stack, a plain chevron, a muted icon tile and a status pill. The open row does
// not get a tinted band; the hierarchy comes from spacing, weight and the
// content reveal. Motion is deliberately small: the chevron turns and the row
// eases its hover colour. The content appears immediately.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { reduceMotion } from 'nsh/accessibility';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  RADIUS_LG,
  SPACING_MD,
  SPACING_SM,
  alpha,
  type ControlSize,
  iconSize,
  isDark,
  tokens,
} from './theme';

export type AccordionTone = 'neutral' | 'info' | 'success' | 'warning' | 'danger';

export interface AccordionItem {
  id: string;
  title: string;
  content?: JSX.Element;
  /** A leading icon, drawn in a quiet square. */
  icon?: IconName;
  /** A caller-drawn leading element, such as an avatar. Wins over `icon`. */
  leading?: JSX.Element;
  /** A muted second line under the title. */
  description?: string;
  /** A short status pill after the title. */
  badge?: string;
  badgeTone?: AccordionTone;
  disabled?: boolean;
}

export interface AccordionProps {
  items: AccordionItem[];
  /** One open section at a time, or several. */
  type?: 'single' | 'multiple';
  defaultValue?: string[];
  value?: string[];
  onChange?: (open: string[]) => void;
  /** Draw the whole stack on one bordered surface. */
  bordered?: boolean;
  /** Give every section its own card and a gap. */
  separated?: boolean;
  size?: ControlSize;
  disabled?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const PADDING_X: Record<ControlSize, number> = { xsmall: 10, small: 12, medium: 14, large: 18 };
const PADDING_Y: Record<ControlSize, number> = { xsmall: 8, small: 10, medium: 13, large: 16 };
const TITLE_FONT: Record<ControlSize, number> = { xsmall: 13, small: 13, medium: 14, large: 16 };
const LEADING: Record<ControlSize, number> = { xsmall: 26, small: 28, medium: 32, large: 36 };

const toneOf = (tone: AccordionTone | undefined) => {
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

function Pill(props: { label: string; tone?: AccordionTone }) {
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

export function Accordion(props: AccordionProps) {
  const own = signal<string[]>(props.defaultValue ?? []);
  const open = () => props.value ?? own();
  const size = () => props.size ?? 'medium';
  const type = () => props.type ?? 'single';
  const separated = () => Boolean(props.separated);
  const bordered = () => !separated() && Boolean(props.bordered);
  const surface = () => (isDark() ? alpha(tokens.foreground, 0.035) : tokens.background);
  const toggle = (id: string) => {
    const current = open();
    const next = current.includes(id)
      ? current.filter((entry) => entry !== id)
      : type() === 'single'
        ? [id]
        : [...current, id];
    if (props.value === undefined) own.set(next);
    props.onChange?.(next);
  };
  const chevronTransition = (): ViewStyle =>
    reduceMotion()
      ? {}
      : {
          transitionProperty: 'transform',
          transitionDuration: 150,
          transitionTimingFunction: 'ease-out',
        };

  return (
    <View
      testID={props.testID}
      style={[
        { gap: separated() ? 8 : 0 },
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
      {props.items.map((item, index) => {
        const expanded = open().includes(item.id);
        const disabled = Boolean(props.disabled || item.disabled);
        const last = index === props.items.length - 1;
        const hasLeading = Boolean(item.leading || item.icon);
        const leadingSize = LEADING[size()];
        const indent = PADDING_X[size()] + (hasLeading ? leadingSize + SPACING_SM : 0);
        return (
          <View
            style={[
              { flexDirection: 'column' },
              separated()
                ? {
                    borderWidth: 1,
                    borderColor: tokens.border,
                    borderRadius: RADIUS_LG,
                    backgroundColor: surface(),
                    overflow: 'hidden',
                  }
                : {},
            ]}
          >
            <View
              accessibilityRole="button"
              accessibilityLabel={item.title}
              accessibilityState={{ expanded, disabled }}
              disabled={disabled}
              onPress={disabled ? undefined : () => toggle(item.id)}
              activeOpacity={disabled ? 1 : 0.9}
              class={
                disabled
                  ? undefined
                  : 'transition-colors duration-150 hover:bg-black/[0.035] dark:hover:bg-white/[0.045]'
              }
              style={{
                flexDirection: 'row',
                alignItems: 'center',
                gap: SPACING_SM,
                paddingHorizontal: PADDING_X[size()],
                paddingVertical: PADDING_Y[size()],
                opacity: disabled ? 0.45 : 1,
              }}
            >
              {item.leading ? (
                <View style={{ width: leadingSize, height: leadingSize, alignItems: 'center', justifyContent: 'center' }}>
                  {item.leading}
                </View>
              ) : item.icon ? (
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
                  <Icon name={item.icon} size={iconSize(size())} color={tokens.mutedForeground} />
                </View>
              ) : null}
              <View style={{ flex: 1, gap: 2 }}>
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
                    {item.title}
                  </Text>
                  {item.badge ? <Pill label={item.badge} tone={item.badgeTone} /> : null}
                </View>
                {item.description ? (
                  <Text
                    numberOfLines={1}
                    style={{ fontSize: 12, lineHeight: 16, color: tokens.mutedForeground }}
                  >
                    {item.description}
                  </Text>
                ) : null}
              </View>
              <Icon
                name="chevron-down"
                size={16}
                color={tokens.mutedForeground}
                style={{
                  transform: [{ rotate: expanded ? '180deg' : '0deg' }],
                  ...chevronTransition(),
                }}
              />
            </View>
            {expanded ? (
              <View
                style={{
                  paddingStart: indent,
                  paddingEnd: PADDING_X[size()],
                  paddingTop: 0,
                  paddingBottom: PADDING_Y[size()] + 2,
                  gap: SPACING_SM,
                }}
              >
                {item.content}
              </View>
            ) : null}
            {!separated() && !last ? (
              <View style={{ height: 1, backgroundColor: tokens.border }} />
            ) : null}
          </View>
        );
      })}
    </View>
  );
}
