// A binary checkbox with a label, description, badge or icon, an optional
// nested list and a card surface — the ReUI checkbox shapes.
//
// One control covers the plain box, the colored and circular variants, the
// card row with icon/badge/description, and the nested group. Indeterminate is
// driven by the app (`indeterminate`), so a parent can mirror its children.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, isDark, type ControlSize, controlFontSize, RADIUS_MD, tokens } from './theme';

export type CheckboxShape = 'square' | 'circle';
export type CheckboxCardVariant = 'plain' | 'card' | 'icon';

export interface CheckboxProps {
  checked?: boolean;
  defaultChecked?: boolean;
  /** Draws a minus instead of a check; the app still owns `checked`. */
  indeterminate?: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  size?: ControlSize;
  shape?: CheckboxShape;
  label?: string;
  description?: string;
  /** A compact text badge next to the label, e.g. NEW. */
  badge?: string;
  /** A leading icon tile, used by icon cards. */
  icon?: IconName;
  /** A caller-drawn leading element (avatar, payment mark) that wins over `icon`. */
  leading?: JSX.Element;
  /** A filled box in this token colour when checked. */
  color?: string;
  /** `card` is a bordered/Bg row, `icon` is the icon-tile card. */
  variant?: CheckboxCardVariant;
  /** A wide row that fills its parent instead of hugging its label. */
  block?: boolean;
  /** Items drawn under the row, for a nested group. */
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const boxSize = (size: ControlSize): number =>
  size === 'xsmall' ? 14 : size === 'small' ? 16 : size === 'large' ? 20 : 18;

export function Checkbox(props: CheckboxProps) {
  const own = signal(Boolean(props.defaultChecked));
  const checked = () => props.checked ?? own();
  const variant = () => props.variant ?? 'plain';
  const shape = () => props.shape ?? 'square';
  const mark = () => props.color ?? tokens.primary;
  const toggle = () => {
    if (props.disabled) return;
    const next = !checked();
    if (props.checked === undefined) own.set(next);
    props.onChange?.(next);
  };
  const box = () => (
    <View
      style={{
        width: boxSize(props.size ?? 'medium'),
        height: boxSize(props.size ?? 'medium'),
        borderRadius: shape() === 'circle' ? boxSize(props.size ?? 'medium') / 2 : 4,
        borderWidth: 1,
        borderColor: checked() || props.indeterminate ? mark() : tokens.input,
        backgroundColor: checked() || props.indeterminate ? mark() : tokens.background,
        alignItems: 'center',
        justifyContent: 'center',
        marginTop: 1,
        flexShrink: 0,
        ...(props.disabled ? { opacity: 0.5 } : {}),
      }}
    >
      {props.indeterminate ? (
        <View style={{ width: boxSize(props.size ?? 'medium') * 0.5, height: 1.5, borderRadius: 1, backgroundColor: tokens.primaryForeground }} />
      ) : checked() ? (
        <Icon name="check" size={boxSize(props.size ?? 'medium') - 5} color={tokens.primaryForeground} />
      ) : null}
    </View>
  );

  const labelBlock = () => (
    <View style={{ flex: 1, gap: 1 }}>
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6 }}>
        {props.label ? (
          <Text style={{ fontSize: controlFontSize(props.size ?? 'medium'), lineHeight: 18, fontWeight: '500', color: tokens.foreground }}>
            {props.label}
          </Text>
        ) : null}
        {props.badge ? (
          <View style={{ paddingHorizontal: 5, paddingVertical: 1, borderRadius: 4, backgroundColor: tokens.muted }}>
            <Text style={{ fontSize: 9, lineHeight: 13, color: tokens.mutedForeground, letterSpacing: 0.4 }}>
              {props.badge}
            </Text>
          </View>
        ) : null}
      </View>
      {props.description ? (
        <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text>
      ) : null}
    </View>
  );

  const card = () => variant() !== 'plain';
  const row = () => {
    const flagged = card();
    const leading = () =>
      props.leading ? (
        props.leading
      ) : props.icon || variant() === 'icon' ? (
        <View
          style={{
            width: 26,
            height: 26,
            borderRadius: 7,
            backgroundColor: tokens.muted,
            alignItems: 'center',
            justifyContent: 'center',
            flexShrink: 0,
          }}
        >
          <Icon name={props.icon ?? 'circle'} size={14} color={tokens.foreground} />
        </View>
      ) : null;
    return (
      <View
        accessibilityRole="checkbox"
        accessibilityLabel={props.label}
        accessibilityState={{ checked: checked(), disabled: Boolean(props.disabled) }}
        onPress={toggle}
        disabled={props.disabled}
        activeOpacity={0.9}
        class="transition-colors duration-150 hover:bg-black/[0.03] dark:hover:bg-white/[0.04]"
        style={[
          {
            flexDirection: 'row',
            alignItems: flagged ? 'flex-start' : 'flex-start',
            gap: flagged ? 8 : 8,
            paddingHorizontal: flagged ? 10 : 0,
            paddingVertical: flagged ? 9 : 0,
            borderRadius: 10,
            borderWidth: flagged ? 1 : 0,
            borderColor: tokens.border,
            backgroundColor: flagged ? tokens.background : 'transparent',
            ...(props.block ? { alignSelf: 'stretch' } : {}),
            opacity: props.disabled ? 0.6 : 1,
          },
          props.style,
        ]}
      >
        {flagged ? leading() : leading() ?? box()}
        <View style={{ flexShrink: 1, flexGrow: 1, minWidth: 0 }}>{labelBlock()}</View>
        {flagged || props.leading || props.icon ? box() : null}
      </View>
    );
  };

  return (
    <View style={{ gap: 6 }}>
      {row()}
      {props.children ? <View style={{ marginStart: 28, gap: 6 }}>{props.children}</View> : null}
    </View>
  );
}

export interface CheckboxGroupProps {
  label?: string;
  description?: string;
  /** A frame around the items. */
  frame?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}

/** An optional title and frame around a set of independent checkboxes. */
export function CheckboxGroup(props: CheckboxGroupProps) {
  if (!props.frame && !props.label) {
    return <View style={[{ gap: 8 }, props.style]}>{props.children}</View>;
  }
  return (
    <View
      style={[
        {
          gap: 8,
          ...(props.frame
            ? {
                padding: 10,
                borderRadius: 10,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.background,
              }
            : {}),
        },
        props.style,
      ]}
    >
      {props.label ? (
        <View style={{ gap: 1 }}>
          <Text style={{ fontSize: 12, fontWeight: '600', color: tokens.foreground }}>{props.label}</Text>
          {props.description ? (
            <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
      <View style={{ gap: 4 }}>{props.children}</View>
    </View>
  );
}
