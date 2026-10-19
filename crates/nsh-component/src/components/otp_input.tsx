// A one-time-password field: one box per character, focus following the code,
// optional group separators and numeric or alphanumeric input.
import type { StyleProp, ViewStyle, YNode } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Input } from './input';
import { type ControlSize, controlFontSize, inputHeight, tokens } from './theme';

export type OtpInputType = 'numeric' | 'alphanumeric';
export type OtpInputStatus = 'default' | 'error' | 'success';

export interface OtpInputProps {
  length?: number;
  value?: string;
  defaultValue?: string;
  size?: ControlSize;
  disabled?: boolean;
  /** Show dots instead of the entered characters. */
  masked?: boolean;
  type?: OtpInputType;
  status?: OtpInputStatus;
  /** Cell group sizes, e.g. `[3, 3]`; a separator is drawn between groups. */
  groups?: number[];
  autoFocus?: boolean;
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
  const clean = (text: string) =>
    props.type === 'alphanumeric'
      ? text.replace(/[^A-Za-z0-9]/g, '').toUpperCase()
      : text.replace(/\D/g, '');

  const commit = (next: string) => {
    const joined = next.slice(0, length());
    if (props.value === undefined) own.set(joined);
    props.onChange?.(joined);
    if (joined.length === length() && joined.split('').every(Boolean)) props.onComplete?.(joined);
  };

  const setAt = (index: number, text: string) => {
    const chars = clean(text).split('');
    if (chars.length === 0) {
      const next = value().split('');
      next[index] = '';
      commit(next.join(''));
      if (index > 0) refs[index - 1]?.focus();
      return;
    }
    const next = value().split('');
    let cursor = index;
    for (const char of chars) {
      if (cursor >= length()) break;
      next[cursor] = char;
      cursor += 1;
    }
    commit(next.join(''));
    if (cursor < length()) refs[cursor]?.focus();
  };

  const groups = () => {
    const list = props.groups && props.groups.length > 0 ? props.groups : [length()];
    const cells: number[][] = [];
    let cursor = 0;
    for (const group of list) {
      const next: number[] = [];
      for (let i = 0; i < group && cursor < length(); i += 1, cursor += 1) next.push(cursor);
      if (next.length) cells.push(next);
    }
    if (cursor < length()) {
      const rest: number[] = [];
      while (cursor < length()) {
        rest.push(cursor);
        cursor += 1;
      }
      cells.push(rest);
    }
    return cells;
  };

  const edge = () =>
    props.status === 'error'
      ? tokens.danger
      : props.status === 'success'
        ? tokens.success
        : tokens.input;

  return (
    <View
      testID={props.testID}
      style={[{ flexDirection: 'row', alignItems: 'center', gap: 6, flexWrap: 'wrap' }, props.style]}
    >
      {groups().map((group, groupIndex) => (
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6 }}>
          {groupIndex > 0 ? (
            <Text style={{ fontSize: 13, color: tokens.mutedForeground, marginHorizontal: 2 }}>–</Text>
          ) : null}
          {group.map((index) => (
            <Input
              ref={(node) => (refs[index] = node)}
              value={props.masked && value()[index] ? '•' : value()[index] ?? ''}
              disabled={props.disabled}
              keyboardType={props.type === 'alphanumeric' ? 'default' : 'number-pad'}
              maxLength={1}
              autoFocus={props.autoFocus && index === 0}
              accessibilityLabel={`Character ${index + 1}`}
              horizontalAlignment="center"
              onChangeText={(text) => setAt(index, text)}
              onKeyPress={(event) => {
                if (event.key === 'Backspace' && !value()[index] && index > 0) {
                  const next = value().split('');
                  next[index - 1] = '';
                  commit(next.join(''));
                  refs[index - 1]?.focus();
                }
              }}
              style={{
                width: inputHeight(props.size ?? 'medium') + 12,
                height: inputHeight(props.size ?? 'medium'),
                borderColor: edge(),
              }}
            />
          ))}
        </View>
      ))}
    </View>
  );
}
