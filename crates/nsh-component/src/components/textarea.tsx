// A multi-line text field with the catalog's sizes, appearance, label,
// description/error, counter and the existing auto-grow behavior.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Input } from 'nsh';
import { alpha, type ControlSize, inputFontSize, RADIUS_MD, tokens } from './theme';

export interface TextareaProps {
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  size?: ControlSize;
  rows?: number;
  /** Grow with the content. Defaults to true, as before. */
  autoGrow?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  invalid?: boolean;
  appearance?: boolean;
  maxLength?: number;
  /** Field label drawn above the control. */
  label?: string;
  required?: boolean;
  optional?: boolean;
  badge?: string;
  description?: string;
  /** Replaces the description when the field is invalid. */
  error?: string;
  /** Show the character count in the label row. */
  showCount?: boolean;
  labelAction?: JSX.Element;
  onChangeText?: (text: string) => void;
  onChange?: (text: string) => void;
  onFocus?: () => void;
  onBlur?: () => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Textarea(props: TextareaProps) {
  const own = signal(props.defaultValue ?? '');
  const height = signal((props.rows ?? 3) * 22 + 16);
  const focused = signal(false);
  const value = () => props.value ?? own();
  const hasHeader = Boolean(props.label || props.required || props.optional || props.badge || props.labelAction || props.showCount || props.maxLength);
  const edge = () => (props.invalid ? tokens.danger : focused() ? tokens.ring : tokens.input);
  const ring = () =>
    focused() && props.appearance !== false && !props.disabled
      ? `0 0 0 3px ${alpha(props.invalid ? tokens.danger : tokens.ring, 0.18)}`
      : undefined;
  const feedback = props.error ? (
    <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.danger }}>{props.error}</Text>
  ) : props.description ? (
    <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>{props.description}</Text>
  ) : null;
  const counter = props.maxLength || props.showCount ? (
    <Text style={{ fontSize: 9, color: props.maxLength && value().length >= props.maxLength ? tokens.danger : tokens.mutedForeground }}>
      {value().length}{props.maxLength ? ` / ${props.maxLength}` : ''}
    </Text>
  ) : null;
  const field = (
    <View
      style={[
        {
          minHeight: height(),
          borderWidth: props.appearance === false ? 0 : 1,
          borderColor: edge(),
          borderRadius: RADIUS_MD,
          padding: 8,
          backgroundColor: props.appearance === false ? 'transparent' : props.disabled ? tokens.muted : tokens.background,
          opacity: props.disabled ? 0.5 : 1,
          boxShadow: ring(),
          transitionProperty: 'border-color, box-shadow, background-color, opacity',
          transitionDuration: 160,
          transitionTimingFunction: 'ease-out',
        },
        props.appearance === false || hasHeader || feedback ? undefined : props.style,
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
        onFocus={() => {
          focused.set(true);
          props.onFocus?.();
        }}
        onBlur={() => {
          focused.set(false);
          props.onBlur?.();
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
  if (!hasHeader && !feedback) return field;
  return (
    <View style={[{ width: '100%', gap: 6 }, props.style]}>
      {hasHeader ? (
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5 }}>
          {props.label ? (
            <Text style={{ fontSize: 11, fontWeight: '500', color: tokens.foreground }}>
              {props.label}
              {props.required ? <Text style={{ color: tokens.danger }}> *</Text> : null}
            </Text>
          ) : null}
          {props.badge ? (
            <View style={{ paddingHorizontal: 5, paddingVertical: 1, borderRadius: 4, backgroundColor: alpha(tokens.success, 0.14), borderWidth: 1, borderColor: alpha(tokens.success, 0.3) }}>
              <Text style={{ fontSize: 8, lineHeight: 12, fontWeight: '600', color: tokens.success }}>{props.badge}</Text>
            </View>
          ) : null}
          {props.optional ? <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Optional</Text> : null}
          <View style={{ flex: 1 }} />
          {props.labelAction}
          {counter}
        </View>
      ) : null}
      <View style={{ width: '100%' }}>{field}</View>
      {feedback}
    </View>
  );
}

export function TextareaCounter(props: { value: string; max?: number }) {
  return (
    <Text style={{ fontSize: 9, color: props.max && props.value.length >= props.max ? tokens.danger : tokens.mutedForeground }}>
      {props.value.length}{props.max ? ` / ${props.max}` : ''}
    </Text>
  );
}
