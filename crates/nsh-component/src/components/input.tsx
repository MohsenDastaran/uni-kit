// A single-line text field: placeholder, prefix/suffix slots, a clear button, a
// masked value with a reveal toggle, the invalid and disabled states, the four
// sizes and the focus ring.
//
// NSH's `<Input>` is a native text field; it has no frame, adornments or
// clear/reveal controls. This component is the frame around it, the same shape
// the iced port draws around `text_input`, so an application gets the catalog's
// control while NSH keeps the typing, caret, selection and keyboard.
import type { JSX, StyleProp, ViewStyle, YNode } from 'nsh';
import { Input as NativeInput, signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  alpha,
  type ControlSize,
  inputFontSize,
  inputHeight,
  inputPaddingX,
  iconSize,
  RADIUS_MD,
  tokens,
} from './theme';

export interface InputProps {
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  size?: ControlSize;
  disabled?: boolean;
  readOnly?: boolean;
  invalid?: boolean;
  /** A field label drawn above, or beside, the control. */
  label?: string;
  required?: boolean;
  optional?: boolean;
  /** A compact badge after the label, e.g. New. */
  badge?: string;
  /** Supporting text under the field. */
  description?: string;
  /** An error message replacing the description. */
  error?: string;
  /** A small info affordance beside the label. */
  help?: boolean;
  /** A trailing label action, such as "Forgot password?". */
  labelAction?: JSX.Element;
  /** Put the label in a fixed left column. */
  orientation?: 'vertical' | 'horizontal';
  labelWidth?: number;
  /** A soft background pulse that stops while the field is focused. */
  pulse?: boolean;
  /** Hide the value as a password. */
  masked?: boolean;
  /** Show the eye a masked field uses to reveal what is typed. */
  maskToggle?: boolean;
  /** Show the × while the field has text. */
  cleanable?: boolean;
  /** Escape clears the field, as Slint's `clean-on-escape` does. */
  cleanOnEscape?: boolean;
  /** Draw the frame. `false` lets a parent draw its own, as in InputGroup. */
  appearance?: boolean;
  prefixIcon?: IconName;
  prefixText?: string;
  suffixIcon?: IconName;
  suffixText?: string;
  horizontalAlignment?: 'left' | 'center' | 'right';
  keyboardType?: 'default' | 'number-pad' | 'decimal-pad' | 'numeric' | 'email-address' | 'phone-pad' | 'url';
  returnKeyType?: 'default' | 'done' | 'go' | 'next' | 'search' | 'send';
  autoCapitalize?: 'none' | 'sentences' | 'words' | 'characters';
  autoFocus?: boolean;
  maxLength?: number;
  /** Receives the native input node, so a parent can move focus to it. */
  ref?: (node: YNode) => void;
  onChangeText?: (text: string) => void;
  onChange?: (text: string) => void;
  onSubmit?: (text: string) => void;
  onClear?: () => void;
  onFocus?: () => void;
  onBlur?: () => void;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

export function Input(props: InputProps) {
  const own = signal(props.defaultValue ?? '');
  const focused = signal(false);
  const revealed = signal(false);
  let field: YNode | undefined;

  const value = () => props.value ?? own();
  const size = () => props.size ?? 'medium';
  const setValue = (next: string) => {
    if (props.value === undefined) own.set(next);
    props.onChangeText?.(next);
    props.onChange?.(next);
  };
  const clear = () => {
    setValue('');
    props.onClear?.();
  };
  const edge = () => {
    if (props.invalid) return tokens.danger;
    return focused() ? tokens.ring : tokens.input;
  };
  const focusRing = () =>
    focused() && props.appearance !== false && !props.disabled
      ? `0 0 0 3px ${alpha(props.invalid ? tokens.danger : tokens.ring, 0.18)}`
      : undefined;
  const hasLabel = Boolean(
    props.label || props.required || props.optional || props.badge || props.description || props.error || props.labelAction,
  );
  const feedback = props.error ? (
    <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.danger }}>{props.error}</Text>
  ) : props.description ? (
    <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>{props.description}</Text>
  ) : null;
  const labelRow = (
    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5, minWidth: 0 }}>
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
      {props.help ? <Icon name="info" size={10} color={tokens.mutedForeground} /> : null}
      <View style={{ flex: 1 }} />
      {props.labelAction}
    </View>
  );

  const inputField = (
    <View
      testID={props.testID}
      accessibilityLabel={props.accessibilityLabel ?? props.placeholder}
      onPress={() => {
        if (!props.disabled) field?.focus();
      }}
      class={
        props.appearance === false || props.disabled
          ? undefined
          : props.pulse && !focused()
            ? 'animate-pulse transition-all duration-200 ease-out hover:shadow-sm'
            : 'transition-all duration-200 ease-out hover:shadow-sm'
      }
      style={[
        {
          height: inputHeight(size()),
          flexDirection: 'row',
          alignItems: 'center',
          gap: size() === 'xsmall' || size() === 'small' ? 4 : 8,
          paddingHorizontal: props.appearance === false ? 0 : inputPaddingX(size()),
          borderRadius: RADIUS_MD,
          borderWidth: props.appearance === false ? 0 : 1,
          borderColor: edge(),
          backgroundColor:
            props.appearance === false
              ? 'transparent'
              : props.disabled
                ? tokens.muted
                : props.pulse && !focused()
                  ? tokens.muted
                  : tokens.background,
          opacity: props.disabled ? 0.5 : 1,
          boxShadow: focusRing(),
          transitionProperty: 'border-color, box-shadow, background-color, opacity',
          transitionDuration: 180,
          transitionTimingFunction: 'ease-out',
        },
        hasLabel ? { width: '100%' } : props.style,
      ]}
    >
      {props.prefixIcon ? (
        <Icon name={props.prefixIcon} size={iconSize(size())} color={tokens.mutedForeground} />
      ) : null}
      {props.prefixText ? (
        <Text style={{ fontSize: inputFontSize(size()), color: tokens.mutedForeground }}>
          {props.prefixText}
        </Text>
      ) : null}
      <NativeInput
        ref={(node) => {
          field = node;
          props.ref?.(node);
        }}
        value={value()}
        placeholder={props.placeholder}
        placeholderTextColor={tokens.mutedForeground}
        selectionColor={tokens.selection}
        editable={!props.disabled && !props.readOnly}
        secureTextEntry={Boolean(props.masked && !revealed())}
        keyboardType={props.keyboardType}
        returnKeyType={props.returnKeyType}
        autoCapitalize={props.autoCapitalize}
        autoFocus={props.autoFocus}
        maxLength={props.maxLength}
        onChangeText={setValue}
        onSubmitEditing={(event) => props.onSubmit?.(event.text)}
        onFocus={() => {
          focused.set(true);
          props.onFocus?.();
        }}
        onBlur={() => {
          focused.set(false);
          props.onBlur?.();
        }}
        onKeyPress={(event) => {
          if (props.cleanOnEscape && event.key === 'Escape' && value() !== '') clear();
        }}
        style={{
          flex: 1,
          height: '100%',
          padding: 0,
          fontSize: inputFontSize(size()),
          color: tokens.foreground,
          textAlign: props.horizontalAlignment ?? 'left',
          backgroundColor: 'transparent',
        }}
      />
      {props.suffixText ? (
        <Text style={{ fontSize: inputFontSize(size()), color: tokens.mutedForeground }}>
          {props.suffixText}
        </Text>
      ) : null}
      {props.suffixIcon ? (
        <Icon name={props.suffixIcon} size={iconSize(size())} color={tokens.mutedForeground} />
      ) : null}
      {props.masked && props.maskToggle ? (
        <View
          accessibilityRole="button"
          accessibilityLabel={revealed() ? 'Hide the value' : 'Show the value'}
          onPress={() => revealed.set(!revealed())}
          activeOpacity={0.7}
          class="transition-opacity duration-150 hover:opacity-70 active:opacity-50"
          style={{ width: 20, height: 20, alignItems: 'center', justifyContent: 'center' }}
        >
          <Icon
            name={revealed() ? 'eye-off' : 'eye'}
            size={iconSize(size())}
            color={tokens.mutedForeground}
          />
        </View>
      ) : null}
      {props.cleanable && value() !== '' ? (
        <View
          accessibilityRole="button"
          accessibilityLabel="Clear"
          onPress={clear}
          activeOpacity={0.7}
          class="transition-all duration-150 hover:opacity-70 active:scale-90 active:opacity-50"
          style={{
            width: 20,
            height: 20,
            borderRadius: 10,
            alignItems: 'center',
            justifyContent: 'center',
            backgroundColor: tokens.muted,
          }}
        >
          <Icon name="x" size={12} color={tokens.mutedForeground} />
        </View>
      ) : null}
      {props.children}
    </View>
  );
  if (!hasLabel) return inputField;
  if (props.orientation === 'horizontal') {
    return (
      <View
        testID={props.testID ? `${props.testID}-field` : undefined}
        style={[{ width: '100%', flexDirection: 'row', alignItems: 'flex-start', gap: 12 }, props.style]}
      >
        <View style={{ width: props.labelWidth ?? 130, flexShrink: 0, paddingTop: 9 }}>{labelRow}</View>
        <View style={{ flex: 1, minWidth: 0, gap: 5 }}>
          {inputField}
          {feedback}
        </View>
      </View>
    );
  }
  return (
    <View style={[{ width: '100%', gap: 6 }, props.style]}>
      {labelRow}
      {inputField}
      {feedback}
    </View>
  );
}
