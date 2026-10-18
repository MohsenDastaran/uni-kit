// A radio group: one choice among several.
import type { StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { type ControlSize, controlFontSize, tokens } from './theme';

export interface RadioOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

export interface RadioGroupProps {
  options: RadioOption[];
  value?: string;
  defaultValue?: string;
  onChange?: (value: string) => void;
  orientation?: 'horizontal' | 'vertical';
  size?: ControlSize;
  disabled?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const dot = (size: ControlSize): number =>
  size === 'xsmall' ? 14 : size === 'small' ? 16 : size === 'large' ? 22 : 18;

export function RadioGroup(props: RadioGroupProps) {
  const value = () => props.value ?? props.defaultValue ?? '';
  const choose = (next: string) => {
    if (props.disabled) return;
    props.onChange?.(next);
  };
  return (
    <View
      testID={props.testID}
      accessibilityRole="radiogroup"
      style={[
        {
          flexDirection: props.orientation === 'horizontal' ? 'row' : 'column',
          flexWrap: 'wrap',
          gap: props.orientation === 'horizontal' ? 16 : 10,
        },
        props.style,
      ]}
    >
      {props.options.map((option) => {
        const selected = value() === option.value;
        const disabled = props.disabled || option.disabled;
        return (
          <View
            accessibilityRole="radio"
            accessibilityLabel={option.label}
            accessibilityState={{ checked: selected, disabled: Boolean(disabled) }}
            onPress={() => choose(option.value)}
            disabled={disabled}
            activeOpacity={0.8}
            style={{ flexDirection: 'row', alignItems: 'flex-start', gap: 8, opacity: disabled ? 0.5 : 1 }}
          >
            <View
              style={{
                width: dot(props.size ?? 'medium'),
                height: dot(props.size ?? 'medium'),
                borderRadius: dot(props.size ?? 'medium') / 2,
                borderWidth: 1,
                borderColor: selected ? tokens.primary : tokens.input,
                alignItems: 'center',
                justifyContent: 'center',
                marginTop: 1,
              }}
            >
              {selected ? (
                <View
                  style={{
                    width: dot(props.size ?? 'medium') / 2,
                    height: dot(props.size ?? 'medium') / 2,
                    borderRadius: dot(props.size ?? 'medium') / 4,
                    backgroundColor: tokens.primary,
                  }}
                />
              ) : null}
            </View>
            <View style={{ gap: 1 }}>
              <Text style={{ fontSize: controlFontSize(props.size ?? 'medium'), color: tokens.foreground }}>{option.label}</Text>
              {option.description ? (
                <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{option.description}</Text>
              ) : null}
            </View>
          </View>
        );
      })}
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
