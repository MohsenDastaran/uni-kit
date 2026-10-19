// A radio group: one choice among several. A plain row is the default; card,
// segmented and grid layouts cover settings, pricing and icon pickers.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type ControlSize, controlFontSize, RADIUS_MD, tokens } from './theme';

export type RadioTone = 'default' | 'info' | 'success' | 'warning' | 'danger';
export type RadioVariant = 'plain' | 'card' | 'segmented' | 'grid';

export interface RadioOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
  /** Leading Lucide icon, used by card and grid layouts. */
  icon?: string;
  /** A custom leading element, such as an Avatar. */
  leading?: JSX.Element;
  /** A small status chip beside the label. */
  badge?: string;
  badgeTone?: RadioTone;
  /** Right-aligned metadata, such as a price or duration. */
  meta?: string;
  /** The selected dot's colour. */
  tone?: RadioTone;
  /** A per-option right-hand element. */
  trailing?: JSX.Element;
}

export interface RadioGroupProps {
  options: RadioOption[];
  value?: string;
  defaultValue?: string;
  onChange?: (value: string) => void;
  orientation?: 'horizontal' | 'vertical';
  size?: ControlSize;
  disabled?: boolean;
  variant?: RadioVariant;
  /** Put the radio control after the content. */
  controlPosition?: 'start' | 'end';
  /** Grid columns for the grid variant. */
  columns?: 1 | 2;
  legend?: string;
  description?: string;
  invalid?: boolean;
  error?: string;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const dot = (size: ControlSize): number =>
  size === 'xsmall' ? 14 : size === 'small' ? 16 : size === 'large' ? 22 : 18;

const toneColor = (tone: RadioTone | undefined, fallback: string): string => {
  switch (tone) {
    case 'info':
      return tokens.info;
    case 'success':
      return tokens.success;
    case 'warning':
      return tokens.warning;
    case 'danger':
      return tokens.danger;
    case 'default':
      return tokens.primary;
    default:
      return fallback;
  }
};

export function RadioGroup(props: RadioGroupProps) {
  const value = () => props.value ?? props.defaultValue ?? '';
  const choose = (next: string) => {
    if (props.disabled) return;
    props.onChange?.(next);
  };
  const variant = () => props.variant ?? 'plain';
  const end = () => props.controlPosition === 'end';

  const indicator = (option: RadioOption, selected: boolean, disabled: boolean) => {
    const accent = toneColor(option.tone, props.invalid ? tokens.danger : tokens.primary);
    return (
      <View
        style={{
          width: dot(props.size ?? 'medium'),
          height: dot(props.size ?? 'medium'),
          borderRadius: dot(props.size ?? 'medium') / 2,
          borderWidth: props.invalid ? 2 : 1,
          borderColor: selected ? accent : props.invalid ? tokens.danger : tokens.input,
          alignItems: 'center',
          justifyContent: 'center',
          flexShrink: 0,
        }}
      >
        {selected ? (
          <View
            style={{
              width: dot(props.size ?? 'medium') / 2,
              height: dot(props.size ?? 'medium') / 2,
              borderRadius: dot(props.size ?? 'medium') / 4,
              backgroundColor: accent,
            }}
          />
        ) : null}
      </View>
    );
  };

  const content = (option: RadioOption, selected: boolean) => (
    <View style={{ flex: 1, gap: 3 }}>
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6, flexWrap: 'wrap' }}>
        <Text style={{ fontSize: controlFontSize(props.size ?? 'medium'), fontWeight: '500', color: tokens.foreground }}>
          {option.label}
        </Text>
        {option.badge ? (
          <View
            style={{
              paddingHorizontal: 6,
              paddingVertical: 2,
              borderRadius: 999,
              backgroundColor: option.badgeTone === 'info' ? tokens.accent : tokens.muted,
              borderWidth: 1,
              borderColor: option.badgeTone === 'info' ? tokens.info : tokens.border,
            }}
          >
            <Text style={{ fontSize: 8, fontWeight: '600', letterSpacing: 0.5, color: option.badgeTone === 'info' ? tokens.info : tokens.mutedForeground }}>
              {option.badge}
            </Text>
          </View>
        ) : null}
      </View>
      {option.description ? (
        <Text style={{ fontSize: 11, lineHeight: 16, color: tokens.mutedForeground }}>{option.description}</Text>
      ) : null}
    </View>
  );

  const leading = (option: RadioOption) => {
    if (option.leading) return option.leading;
    if (!option.icon) return null;
    return (
      <View style={{ width: 28, height: 28, borderRadius: 7, alignItems: 'center', justifyContent: 'center', backgroundColor: tokens.muted }}>
        <Icon name={option.icon} size={14} color={tokens.mutedForeground} />
      </View>
    );
  };

  const optionRow = (option: RadioOption) => {
    const selected = value() === option.value;
    const disabled = Boolean(props.disabled || option.disabled);
    const contentNode = content(option, selected);
    const control = indicator(option, selected, disabled);
    const lead = leading(option);
    const meta = option.meta ? (
      <Text style={{ fontSize: 10, color: selected ? tokens.foreground : tokens.mutedForeground }}>{option.meta}</Text>
    ) : null;
    const base = {
      opacity: disabled ? 0.5 : 1,
      activeOpacity: 0.85,
      accessibilityRole: 'radio' as const,
      accessibilityLabel: option.label,
      accessibilityState: { checked: selected, disabled: Boolean(disabled) },
      onPress: () => choose(option.value),
      disabled,
    };

    if (variant() === 'grid') {
      return (
        <View
          {...base}
          style={{
            width: '100%',
            minWidth: 140,
            gap: 9,
            padding: 12,
            borderRadius: 10,
            borderWidth: 1,
            borderColor: selected ? tokens.primary : tokens.border,
            backgroundColor: selected ? tokens.accent : tokens.background,
          }}
        >
          <View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' }}>
            {lead ?? <View />}
            {control}
          </View>
          {contentNode}
          {meta}
        </View>
      );
    }

    if (variant() === 'card') {
      return (
        <View
          {...base}
          style={{
            width: '100%',
            flexDirection: 'row',
            alignItems: 'center',
            gap: 9,
            padding: 11,
            borderRadius: 9,
            borderWidth: 1,
            borderColor: selected ? tokens.primary : props.invalid ? tokens.danger : tokens.border,
            backgroundColor: selected ? tokens.accent : tokens.background,
          }}
        >
          {lead}
          {contentNode}
          {meta}
          {control}
          {option.trailing}
        </View>
      );
    }

    if (variant() === 'segmented') {
      return (
        <View
          {...base}
          style={{
            width: '100%',
            flexDirection: 'row',
            alignItems: 'center',
            gap: 9,
            paddingHorizontal: 12,
            paddingVertical: 10,
            backgroundColor: selected ? tokens.accent : 'transparent',
          }}
        >
          {!end() ? control : null}
          {lead}
          {contentNode}
          {meta}
          {end() ? control : null}
        </View>
      );
    }

    return (
      <View
        {...base}
        style={{
          width: props.orientation === 'horizontal' ? undefined : '100%',
          flexDirection: 'row',
          alignItems: 'flex-start',
          gap: 8,
          opacity: disabled ? 0.5 : 1,
        }}
      >
        {!end() ? control : null}
        {lead}
        {contentNode}
        {meta}
        {end() ? control : null}
      </View>
    );
  };

  return (
    <View
      testID={props.testID}
      accessibilityRole="radiogroup"
      style={[
        {
          flexDirection: 'column',
          gap: variant() === 'segmented' ? 0 : 10,
        },
        props.style,
      ]}
    >
      {props.legend ? (
        <View style={{ gap: 2 }}>
          <Text style={{ fontSize: 14, fontWeight: '600', color: tokens.foreground }}>{props.legend}</Text>
          {props.description ? <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.description}</Text> : null}
        </View>
      ) : null}
      <View
        style={
          variant() === 'segmented'
            ? { borderWidth: 1, borderColor: props.invalid ? tokens.danger : tokens.border, borderRadius: 10, overflow: 'hidden' }
            : {
              flexDirection:
                variant() === 'grid'
                  ? 'row'
                  : variant() === 'card' || variant() === 'segmented'
                    ? 'column'
                    : props.orientation === 'horizontal'
                      ? 'row'
                      : 'column',
              flexWrap: 'wrap',
              gap: 10,
            }
        }
      >
        {props.options.map((option, index) => (
          <View
            style={
              variant() === 'grid'
                ? { width: props.columns === 1 ? '100%' : '44%', minWidth: 140 }
                : variant() === 'card'
                  ? { width: '100%' }
                  : variant() === 'segmented'
                    ? {
                        width: '100%',
                        ...(index < props.options.length - 1
                          ? { borderBottomWidth: 1, borderBottomColor: tokens.border }
                          : {}),
                      }
                    : props.orientation === 'horizontal'
                      ? undefined
                      : { width: '100%' }
            }
          >
            {optionRow(option)}
          </View>
        ))}
      </View>
      {props.invalid ? (
        <Text style={{ fontSize: 10, color: tokens.danger }}>{props.error ?? 'Please choose an option.'}</Text>
      ) : null}
    </View>
  );
}

export function Radio(props: { label?: string; selected?: boolean; onPress?: () => void; disabled?: boolean }) {
  return (
    <RadioGroup
      options={[{ value: 'radio', label: props.label ?? 'Radio', disabled: props.disabled }]}
      value={props.selected ? 'radio' : ''}
      onChange={() => props.onPress?.()}
    />
  );
}
