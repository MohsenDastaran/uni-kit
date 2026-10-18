// A button that stays visibly on, and a group of them.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { Button, type ButtonVariant } from './button';
import { type ControlSize, tokens } from './theme';
import { SPACING_XS } from './theme';

export interface ToggleProps {
  pressed?: boolean;
  defaultPressed?: boolean;
  onChange?: (pressed: boolean) => void;
  variant?: ButtonVariant;
  size?: ControlSize;
  disabled?: boolean;
  children?: JSX.Element;
  label?: string;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Toggle(props: ToggleProps) {
  const own = signal(Boolean(props.defaultPressed));
  const pressed = () => props.pressed ?? own();
  return (
    <Button
      label={props.label}
      variant={props.variant ?? 'default'}
      size={props.size}
      selected={pressed()}
      disabled={props.disabled}
      onPress={() => {
        const next = !pressed();
        if (props.pressed === undefined) own.set(next);
        props.onChange?.(next);
      }}
      style={props.style}
      testID={props.testID}
    >
      {props.children}
    </Button>
  );
}

export interface ToggleGroupProps {
  items: { value: string; label: string }[];
  value?: string;
  defaultValue?: string;
  onChange?: (value: string) => void;
  size?: ControlSize;
  disabled?: boolean;
  style?: StyleProp<ViewStyle>;
}

export function ToggleGroup(props: ToggleGroupProps) {
  const own = signal(props.defaultValue ?? '');
  const value = () => props.value ?? own();
  return (
    <View style={[{ flexDirection: 'row', gap: SPACING_XS }, props.style]}>
      {props.items.map((item) => (
        <Button
          label={item.label}
          size={props.size}
          selected={value() === item.value}
          disabled={props.disabled}
          onPress={() => {
            if (props.value === undefined) own.set(item.value);
            props.onChange?.(item.value);
          }}
        />
      ))}
    </View>
  );
}
