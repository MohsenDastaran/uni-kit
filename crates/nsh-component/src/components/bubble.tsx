// The chat surface: alignment, sender metadata, variants, folding and a
// reaction region.
//
// The visual language follows the ReUI conversation surfaces: a quiet fill, a
// compact header, a comfortable reading measure and a reaction row that sits
// just under the edge. Motion is deliberately small: interactive surfaces and
// chips ease their opacity and colour, the fold chevron turns, and nothing
// bounces or scales. `LayoutAnimation` is never started here; a folding or
// reaction demo that wants the native layout transition starts it, guarded by
// `reduceMotion()`.
//
// The seven canonical variants mirror `BubbleVariant` in GPUI and Slint:
// `filled`, `secondary`, `muted`, `tinted`, `outline`, `ghost` and
// `destructive`. `default` (the old default, now the outline surface),
// `primary` (the old filled surface) and `danger` stay as aliases so existing
// call sites keep rendering.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { reduceMotion } from 'nsh/accessibility';
import { signal, Text, View } from 'nsh';
import { Avatar } from './avatar';
import { Icon } from './icon';
import {
  alpha,
  FONT_SIZE_SM,
  RADIUS_XL,
  SPACING_SM,
  SPACING_XS,
  isDark,
  mix,
  tokens,
} from './theme';

export type BubbleAlign = 'start' | 'end';

/** The seven surfaces, plus the three names the earlier NSH port used. */
export type BubbleVariant =
  | 'filled'
  | 'secondary'
  | 'muted'
  | 'tinted'
  | 'outline'
  | 'ghost'
  | 'destructive'
  /** Backwards-compatible alias for `outline`. */
  | 'default'
  /** Backwards-compatible alias for `filled`. */
  | 'primary'
  /** Backwards-compatible alias for `destructive`. */
  | 'danger';

/** A reaction chip: an emoji, an optional count and whether the reader added it. */
export interface BubbleReactionItem {
  emoji: string;
  count?: number;
  added?: boolean;
  /** Wins over the row's `onToggle`. */
  onPress?: () => void;
  accessibilityLabel?: string;
}

/** A string is the terse form: `"👍 2"` parses to an emoji and a count. */
export type BubbleReaction = string | BubbleReactionItem;

export interface BubbleProps {
  align?: BubbleAlign;
  variant?: BubbleVariant;
  content?: string;
  /** Sender name above the surface. A name without `avatar` draws initials. */
  name?: string;
  /** An image source for the sender's avatar. */
  avatar?: string | { uri: string };
  /** A caller-drawn avatar or mark; wins over `avatar` and `name`. */
  leading?: JSX.Element;
  /** Compact metadata beside the name. */
  timestamp?: string;
  /** A second piece of compact metadata, after the timestamp. */
  meta?: string;
  /** Replaces the default name/avatar/timestamp row above the surface. */
  header?: JSX.Element;
  /** A status or delivery row under the reaction and action rows. */
  footer?: JSX.Element;
  actions?: JSX.Element;
  reactions?: BubbleReaction[];
  /**
   * Fold the string `content` behind a Show more control. The first paragraph
   * is shown while folded; rich `children` are left visible.
   */
  foldable?: boolean;
  /** The folded measure. Defaults to three lines. */
  collapsedLines?: number;
  /** An alias for `collapsedLines`, matching the clamp vocabulary. */
  clamp?: number;
  /** Controlled expanded state. */
  expanded?: boolean;
  defaultExpanded?: boolean;
  onExpandedChange?: (expanded: boolean) => void;
  showMoreLabel?: string;
  showLessLabel?: string;
  /** Makes the whole surface a button with hover and press feedback. */
  onPress?: () => void;
  accessibilityLabel?: string;
  /** Application-owned surface refinements, applied after the variant. */
  surfaceStyle?: StyleProp<ViewStyle>;
  /** Attach the reaction row above or below the surface. */
  reactionSide?: 'top' | 'bottom';
  /** Cap the visible reaction chips; the rest collapse into a `+N` chip. */
  maxReactions?: number;
  onReactionToggle?: (reaction: BubbleReactionItem, index: number) => void;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export interface BubbleGroupProps {
  align?: BubbleAlign | 'stretch';
  gap?: number;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

export interface BubbleReactionsProps {
  reactions?: BubbleReaction[];
  /** Cap the visible chips; the rest become one `+N` chip. */
  max?: number;
  align?: BubbleAlign;
  /** Accepted for API parity; `Bubble` owns where the row is placed. */
  side?: 'top' | 'bottom';
  onToggle?: (reaction: BubbleReactionItem, index: number) => void;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

/** `"👍 2"` and `"🎉"` keep working; an object carries state and handlers. */
function toReaction(reaction: BubbleReaction): BubbleReactionItem {
  if (typeof reaction !== 'string') return reaction;
  const value = reaction.trim();
  const match = /^(.*?)(?:\s+(\d+))?$/.exec(value);
  const emoji = (match?.[1] ?? value).trim();
  const count = match?.[2] ? Number(match[2]) : undefined;
  return { emoji: emoji || value, count };
}

/** `filled` is the old `primary`; `outline` is the old `default`. */
function resolveVariant(variant: BubbleVariant): Exclude<BubbleVariant, 'default' | 'primary' | 'danger'> {
  switch (variant) {
    case 'default':
      return 'outline';
    case 'primary':
      return 'filled';
    case 'danger':
      return 'destructive';
    default:
      return variant;
  }
}

/** The shared 160ms colour/opacity transition, off when the reader asks for less motion. */
function surfaceMotion(): ViewStyle {
  if (reduceMotion()) return {};
  return {
    transitionProperty: 'opacity, background-color, border-color',
    transitionDuration: 160,
    transitionTimingFunction: 'ease-out',
  };
}

function chipMotion(): ViewStyle {
  if (reduceMotion()) return {};
  return {
    transitionProperty: 'background-color, border-color, opacity',
    transitionDuration: 160,
    transitionTimingFunction: 'ease-out',
  };
}

function ReactionChip(props: { reaction: BubbleReactionItem; onPress?: () => void }) {
  const added = () => Boolean(props.reaction.added);
  const count = () => props.reaction.count ?? 0;
  const read = () =>
    props.reaction.accessibilityLabel ??
    `${props.reaction.emoji}${count() > 0 ? ` ${count()}` : ''}${added() ? ', selected' : ''}`;
  return (
    <View
      accessibilityRole={props.onPress ? 'button' : undefined}
      accessibilityLabel={read()}
      accessibilityState={props.onPress ? { selected: added() } : undefined}
      onPress={props.onPress}
      activeOpacity={props.onPress ? 0.85 : 1}
      class={props.onPress ? 'transition-opacity duration-150 hover:opacity-80 active:opacity-70' : undefined}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 3,
          paddingHorizontal: 7,
          paddingVertical: 2,
          borderRadius: 999,
          borderWidth: 1,
          borderColor: added() ? alpha(tokens.primary, 0.45) : tokens.border,
          backgroundColor: added() ? alpha(tokens.primary, isDark() ? 0.18 : 0.08) : tokens.muted,
        },
        chipMotion(),
      ]}
    >
      <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.foreground }}>{props.reaction.emoji}</Text>
      {count() > 0 ? (
        <Text
          style={{
            fontSize: 11,
            lineHeight: 15,
            fontWeight: '600',
            color: added() ? tokens.primary : tokens.mutedForeground,
          }}
        >
          {count()}
        </Text>
      ) : null}
    </View>
  );
}

/**
 * A reaction row: emoji chips that are real buttons when a handler is
 * supplied, plus one `+N` chip for the reactions past `max`.
 */
export function BubbleReactions(props: BubbleReactionsProps) {
  const list = () => (props.reactions ?? []).map(toReaction);
  const limit = () => Math.max(1, props.max ?? list().length);
  const visible = () => list().map((reaction, index) => ({ reaction, index })).slice(0, limit());
  const hidden = () => list().slice(limit());
  const alignEnd = () => (props.align ?? 'start') === 'end';
  const toggle = (reaction: BubbleReactionItem, index: number) => {
    if (reaction.onPress) reaction.onPress();
    else props.onToggle?.(reaction, index);
  };

  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          flexWrap: 'wrap',
          alignItems: 'center',
          gap: 4,
          alignSelf: alignEnd() ? 'flex-end' : 'flex-start',
        },
        props.style,
      ]}
    >
      {visible().map(({ reaction, index }) => (
        <ReactionChip
          reaction={reaction}
          onPress={
            reaction.onPress || props.onToggle
              ? () => toggle(reaction, index)
              : undefined
          }
        />
      ))}
      {hidden().length > 0 ? (
        <View
          accessibilityLabel={`${hidden().length} more reactions`}
          style={{
            paddingHorizontal: 7,
            paddingVertical: 2,
            borderRadius: 999,
            borderWidth: 1,
            borderColor: tokens.border,
            backgroundColor: tokens.background,
          }}
        >
          <Text style={{ fontSize: 11, lineHeight: 15, fontWeight: '600', color: tokens.mutedForeground }}>
            +{hidden().length}
          </Text>
        </View>
      ) : null}
      {props.children}
    </View>
  );
}

/** Consecutive bubbles from one sender, or a two-sided thread. */
export function BubbleGroup(props: BubbleGroupProps) {
  const align = () => props.align ?? 'stretch';
  return (
    <View
      testID={props.testID}
      accessibilityLabel={props.accessibilityLabel}
      style={[
        {
          gap: props.gap ?? SPACING_SM,
          alignItems: align() === 'stretch' ? 'stretch' : align() === 'end' ? 'flex-end' : 'flex-start',
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function Bubble(props: BubbleProps) {
  const ownExpanded = signal(Boolean(props.defaultExpanded));
  const hovering = signal(false);
  const pressing = signal(false);

  const variant = () => resolveVariant(props.variant ?? 'default');
  const end = () => props.align === 'end';
  const foldable = () =>
    Boolean(props.foldable || props.collapsedLines !== undefined || props.clamp !== undefined) &&
    Boolean(props.content);
  const expanded = () => props.expanded ?? ownExpanded();
  const folded = () => foldable() && !expanded();
  const lines = () => props.collapsedLines ?? props.clamp ?? 3;

  /** The first paragraph while folded, so the first idea stays on screen. */
  const foldedText = () => {
    const text = props.content ?? '';
    const paragraphs = text.split(/\n\s*\n/);
    return paragraphs.length > 1 ? `${paragraphs[0].trimEnd()}…` : text;
  };

  const toggleExpanded = () => {
    const next = !expanded();
    if (props.expanded === undefined) ownExpanded.set(next);
    props.onExpandedChange?.(next);
  };

  const surfaceBackground = () => {
    switch (variant()) {
      case 'filled':
        return tokens.primary;
      case 'secondary':
        // The dark palette maps `secondary` and `muted` to one fill, so lift
        // the secondary surface a step to keep the two variants legible.
        return isDark() ? mix(tokens.secondary, tokens.foreground, 0.94) : tokens.secondary;
      case 'muted':
        return tokens.muted;
      case 'tinted':
        return mix(tokens.primary, tokens.background, isDark() ? 0.24 : 0.12);
      case 'destructive':
        return alpha(tokens.danger, isDark() ? 0.2 : 0.1);
      case 'ghost':
        return 'transparent';
      default:
        return tokens.background;
    }
  };

  const surfaceBorder = () => {
    switch (variant()) {
      case 'filled':
        return tokens.primary;
      case 'secondary':
      case 'muted':
      case 'outline':
        return tokens.border;
      case 'tinted':
        return alpha(tokens.primary, 0.24);
      case 'destructive':
        return alpha(tokens.danger, 0.35);
      case 'ghost':
        return 'transparent';
      default:
        return tokens.border;
    }
  };

  const surfaceInk = () => {
    switch (variant()) {
      case 'filled':
        return tokens.primaryForeground;
      case 'secondary':
        return tokens.secondaryForeground;
      case 'destructive':
        return tokens.danger;
      default:
        return tokens.foreground;
    }
  };

  const ghost = () => variant() === 'ghost';
  const interactive = () => Boolean(props.onPress);
  const reactionTop = () => (props.reactionSide ?? 'bottom') === 'top';
  const identity = () =>
    Boolean(props.header || props.leading || props.avatar || props.name || props.timestamp || props.meta);

  const avatarNode = () => {
    if (props.leading) return props.leading;
    if (props.avatar) return <Avatar src={props.avatar} name={props.name} size="sm" />;
    if (props.name) return <Avatar name={props.name} size="sm" />;
    return null;
  };

  const reactionsNode = () =>
    props.reactions && props.reactions.length > 0 ? (
      <BubbleReactions
        reactions={props.reactions}
        align={end() ? 'end' : 'start'}
        max={props.maxReactions}
        side={reactionTop() ? 'top' : 'bottom'}
        onToggle={props.onReactionToggle}
      />
    ) : null;

  return (
    <View
      testID={props.testID}
      style={[
        {
          alignSelf: ghost() ? 'stretch' : end() ? 'flex-end' : 'flex-start',
          maxWidth: ghost() ? '100%' : '78%',
          gap: SPACING_XS,
        },
        props.style,
      ]}
    >
      {identity() ? (
        props.header ? (
          <View style={{ alignSelf: end() ? 'flex-end' : 'flex-start', maxWidth: '100%' }}>{props.header}</View>
        ) : (
          <View
            style={{
              flexDirection: end() ? 'row-reverse' : 'row',
              alignItems: 'center',
              gap: 6,
              alignSelf: end() ? 'flex-end' : 'flex-start',
              maxWidth: '100%',
            }}
          >
            {avatarNode()}
            <View style={{ alignItems: end() ? 'flex-end' : 'flex-start', gap: 1 }}>
              {props.name ? (
                <Text style={{ fontSize: 12, lineHeight: 16, fontWeight: '600', color: tokens.foreground }}>
                  {props.name}
                </Text>
              ) : null}
              {props.timestamp || props.meta ? (
                <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>
                  {[props.timestamp, props.meta].filter(Boolean).join(' · ')}
                </Text>
              ) : null}
            </View>
          </View>
        )
      ) : null}

      {reactionTop() ? reactionsNode() : null}

      <View
        accessibilityRole={interactive() ? 'button' : undefined}
        accessibilityLabel={
          interactive() ? props.accessibilityLabel ?? props.content ?? props.name ?? 'Message' : undefined
        }
        accessibilityState={interactive() ? { disabled: false } : undefined}
        onPress={props.onPress}
        onPressIn={interactive() ? () => pressing.set(true) : undefined}
        onPressOut={interactive() ? () => pressing.set(false) : undefined}
        onHoverIn={interactive() ? () => hovering.set(true) : undefined}
        onHoverOut={interactive() ? () => hovering.set(false) : undefined}
        activeOpacity={interactive() ? 0.9 : 1}
        style={[
          {
            paddingHorizontal: ghost() ? 0 : 12,
            paddingVertical: ghost() ? 0 : 8,
            borderRadius: ghost() ? 0 : RADIUS_XL,
            borderWidth: ghost() ? 0 : 1,
            borderColor: surfaceBorder(),
            backgroundColor: surfaceBackground(),
            opacity: interactive() ? (pressing() ? 0.9 : hovering() ? 0.96 : 1) : 1,
          },
          interactive() ? surfaceMotion() : {},
          !ghost() ? { overflow: 'hidden' } : {},
          props.surfaceStyle,
        ]}
      >
        {props.content ? (
          folded() ? (
            <Text
              numberOfLines={lines()}
              ellipsizeMode="tail"
              style={{ fontSize: FONT_SIZE_SM, lineHeight: 20, color: surfaceInk() }}
            >
              {foldedText()}
            </Text>
          ) : (
            <Text style={{ fontSize: FONT_SIZE_SM, lineHeight: 20, color: surfaceInk() }}>{props.content}</Text>
          )
        ) : null}
        {props.children}
        {foldable() ? (
          <View
            accessibilityRole="button"
            accessibilityLabel={expanded() ? props.showLessLabel ?? 'Show less' : props.showMoreLabel ?? 'Show more'}
            accessibilityState={{ expanded: expanded() }}
            onPress={toggleExpanded}
            activeOpacity={0.8}
            class="transition-opacity duration-150 hover:opacity-70 active:opacity-60"
            style={{
              flexDirection: 'row',
              alignItems: 'center',
              gap: 2,
              alignSelf: end() ? 'flex-end' : 'flex-start',
              paddingTop: SPACING_XS,
            }}
          >
            <Text
              style={{
                fontSize: 12,
                lineHeight: 16,
                fontWeight: '500',
                color: variant() === 'filled' ? alpha(surfaceInk(), 0.85) : tokens.mutedForeground,
              }}
            >
              {expanded() ? props.showLessLabel ?? 'Show less' : props.showMoreLabel ?? 'Show more'}
            </Text>
            <Icon
              name="chevron-down"
              size={12}
              color={variant() === 'filled' ? alpha(surfaceInk(), 0.85) : tokens.mutedForeground}
              style={{
                transform: [{ rotate: expanded() ? '180deg' : '0deg' }],
                ...(reduceMotion()
                  ? {}
                  : {
                      transitionProperty: 'transform',
                      transitionDuration: 160,
                      transitionTimingFunction: 'ease-out',
                    }),
              }}
            />
          </View>
        ) : null}
      </View>

      {reactionTop() ? null : reactionsNode()}
      {props.actions}
      {props.footer ? (
        <View style={{ alignSelf: end() ? 'flex-end' : 'flex-start', maxWidth: '100%' }}>{props.footer}</View>
      ) : null}
    </View>
  );
}

export function BubbleContent(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={props.style}>{props.children}</View>;
}
