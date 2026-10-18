// A one-time-password field: one box per digit, moving focus as the user types.
import type { StyleProp, ViewStyle, YNode } from 'nsh';
import { Input, signal, View } from 'nsh';
import { type ControlSize, inputHeight, tokens } from './theme';

export interface OtpInputProps {
  length?: number;
  value?: string;
  defaultValue?: string;
  size?: ControlSize;
  disabled?: boolean;
  onChange?: (value: string) => void;
  onComplete?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function OtpInput(props: OtpInputProps) {
  const length = () => Math.max(1, props.length ?? 6);
  const own = signal(props.defaultValue ?? '');
  const value = () => props.value ?? own();
  const refs: (YNode | undefined)[] = [];
  const setAt = (index: number, digit: string) => {
    const next = value().split('');
    next[index] = digit.replace(/\D/g, '').slice(-1);
    const joined = next.join('').slice(0, length());
    if (props.value === undefined) own.set(joined);
    props.onChange?.(joined);
    if (joined.length === length()) props.onComplete?.(joined);
    if (next[index] && index + 1 < length()) refs[index + 1]?.focus();
  };
  return (
    <View
      testID={props.testID}
      style={[{ flexDirection: 'row', gap: 8 }, props.style]}
    >
      {Array.from({ length: length() }, (_, index) => (
        <Input
          ref={(node) => (refs[index] = node)}
          value={value()[index] ?? ''}
          disabled={props.disabled}
          keyboardType="number-pad"
          maxLength={1}
          onChangeText={(text) => setAt(index, text)}
          style={{ width: inputHeight(props.size ?? 'medium') + 12, height: inputHeight(props.size ?? 'medium') }}
        />
      ))}
    </View>
  );
}
