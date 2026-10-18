// A segmented time-of-day field, on a 24-hour or 12-hour clock.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button } from './button';
import { Input } from './input';
import { type ControlSize, inputHeight, RADIUS_MD, tokens } from './theme';
import { pad2 } from './date';

export interface TimeFieldProps {
  value?: string;
  defaultValue?: string;
  seconds?: boolean;
  hour12?: boolean;
  size?: ControlSize;
  disabled?: boolean;
  invalid?: boolean;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function partsOf(value: string): number[] {
  const parts = value.split(':').map((part) => Number(part));
  return [Number.isFinite(parts[0]) ? parts[0] : 0, Number.isFinite(parts[1]) ? parts[1] : 0, Number.isFinite(parts[2]) ? parts[2] : 0];
}

export function TimeField(props: TimeFieldProps) {
  const own = signal(props.defaultValue ?? '00:00');
  const value = () => props.value ?? own();
  const parts = () => partsOf(value());
  const set = (next: number[]) => {
    const formatted = `${pad2(next[0] % 24)}:${pad2(next[1] % 60)}${props.seconds ? `:${pad2(next[2] % 60)}` : ''}`;
    if (props.value === undefined) own.set(formatted);
    props.onChange?.(formatted);
  };
  const segment = (index: number, max: number) => (
    <View style={{ alignItems: 'center', gap: 2 }}>
      <Button icon="chevron-up" variant="ghost" size="small" accessibilityLabel="Increase" onPress={() => { const next = parts(); next[index] = (next[index] + 1) % (max + 1); set(next); }} />
      <Input
        value={pad2(parts()[index] ?? 0)}
        size={props.size}
        disabled={props.disabled}
        keyboardType="number-pad"
        horizontalAlignment="center"
        maxLength={2}
        onChangeText={(text) => { const next = parts(); const parsed = Number(text); if (Number.isFinite(parsed)) { next[index] = Math.min(max, Math.max(0, parsed)); set(next); } }}
        style={{ width: 44 }}
      />
      <Button icon="chevron-down" variant="ghost" size="small" accessibilityLabel="Decrease" onPress={() => { const next = parts(); next[index] = (next[index] - 1 + max + 1) % (max + 1); set(next); }} />
    </View>
  );
  return (
    <View testID={props.testID} style={[{ flexDirection: 'row', alignItems: 'flex-start', gap: 6 }, props.style]}>
      {segment(0, 23)}
      <Text style={{ fontSize: 20, lineHeight: inputHeight() , color: tokens.mutedForeground }}>:</Text>
      {segment(1, 59)}
      {props.seconds ? (
        <>
          <Text style={{ fontSize: 20, lineHeight: inputHeight(), color: tokens.mutedForeground }}>:</Text>
          {segment(2, 59)}
        </>
      ) : null}
      {props.hour12 ? (
        <Button label={parts()[0] < 12 ? 'AM' : 'PM'} size="small" variant="secondary" onPress={() => set([(parts()[0] + 12) % 24, parts()[1], parts()[2]])} />
      ) : null}
    </View>
  );
}
