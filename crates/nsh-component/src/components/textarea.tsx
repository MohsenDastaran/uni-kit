// A multi-line text field with the catalog's sizes, auto-grow and appearance.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Input } from 'nsh';
import { type ControlSize, inputFontSize, RADIUS_MD, tokens } from './theme';

export interface TextareaProps {
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  size?: ControlSize;
  rows?: number;
  autoGrow?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  invalid?: boolean;
  appearance?: boolean;
  maxLength?: number;
  onChangeText?: (text: string) => void;
  onChange?: (text: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Textarea(props: TextareaProps) {
  const own = signal(props.defaultValue ?? '');
  const height = signal((props.rows ?? 3) * 22 + 16);
  const value = () => props.value ?? own();
  return (
    <View
      style={[
        {
          minHeight: height(),
          borderWidth: props.appearance === false ? 0 : 1,
          borderColor: props.invalid ? tokens.danger : tokens.input,
          borderRadius: RADIUS_MD,
          padding: 8,
          backgroundColor: props.appearance === false ? 'transparent' : tokens.background,
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
      testID={props.testID}
    >
      <Input
        multiline
        value={value()}
        placeholder={props.placeholder}
        editable={!props.disabled && !props.readOnly}
        maxLength={props.maxLength}
        onChangeText={(text) => {
          if (props.value === undefined) own.set(text);
          props.onChangeText?.(text);
          props.onChange?.(text);
        }}
        onContentSizeChange={(size) => {
          if (props.autoGrow !== false) height.set(Math.max((props.rows ?? 3) * 22, size.height + 16));
        }}
        style={{
          height: height(),
          padding: 0,
          fontSize: inputFontSize(props.size ?? 'medium'),
          color: tokens.foreground,
          backgroundColor: 'transparent',
        }}
      />
    </View>
  );
}

export function TextareaCounter(props: { value: string; max?: number }) {
  return (
    <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
      {props.value.length}{props.max ? ` / ${props.max}` : ''}
    </Text>
  );
}
