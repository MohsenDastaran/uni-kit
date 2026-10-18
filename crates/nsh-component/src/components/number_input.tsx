// A numeric field with − and + buttons, bounds and a step.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button } from './button';
import { Input } from './input';
import { type ControlSize, tokens } from './theme';

export interface NumberInputProps {
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  size?: ControlSize;
  disabled?: boolean;
  onChange?: (value: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function NumberInput(props: NumberInputProps) {
  const own = signal(props.defaultValue ?? 0);
  const value = () => props.value ?? own();
  const clamp = (next: number) => Math.max(props.min ?? -Infinity, Math.min(props.max ?? Infinity, next));
  const set = (next: number) => {
    const bounded = clamp(next);
    if (props.value === undefined) own.set(bounded);
    props.onChange?.(bounded);
  };
  return (
    <View
      testID={props.testID}
      style={[{ flexDirection: 'row', alignItems: 'center', gap: 4 }, props.style]}
    >
      <Button
        label="−"
        size={props.size ?? 'medium'}
        disabled={props.disabled || value() <= (props.min ?? -Infinity)}
        onPress={() => set(value() - (props.step ?? 1))}
      />
      <Input
        value={String(value())}
        size={props.size}
        disabled={props.disabled}
        keyboardType="numeric"
        horizontalAlignment="center"
        onChangeText={(text) => {
          const parsed = Number(text);
          if (Number.isFinite(parsed)) set(parsed);
        }}
        style={{ minWidth: 84 }}
      />
      <Button
        label="+"
        size={props.size ?? 'medium'}
        disabled={props.disabled || value() >= (props.max ?? Infinity)}
        onPress={() => set(value() + (props.step ?? 1))}
      />
    </View>
  );
}
