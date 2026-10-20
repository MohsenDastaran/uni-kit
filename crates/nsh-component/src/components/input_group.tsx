// A frame that groups an input with leading/trailing text, icons, selects or
// actions. The inner Input keeps its normal behaviour but drops its own frame,
// so the group owns one border and one focus surface.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Button, type ButtonVariant } from './button';
import { Input, type InputProps } from './input';
import { Select, type SelectProps } from './select';
import { type ControlSize, controlFontSize, inputHeight, RADIUS_MD, inputPaddingX, tokens } from './theme';

export interface InputGroupProps {
  size?: ControlSize;
  disabled?: boolean;
  invalid?: boolean;
  /** Let the content grow to multiple lines, as in a textarea composer. */
  multiline?: boolean;
  /** Use the muted surface rather than the page background. */
  filled?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function InputGroup(props: InputGroupProps) {
  const size = () => props.size ?? 'medium';
  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          alignItems: props.multiline ? 'flex-start' : 'center',
          gap: 2,
          ...(props.multiline ? { minHeight: inputHeight(size()) + 8, paddingVertical: 4 } : { height: inputHeight(size()) + 8 }),
          paddingHorizontal: 5,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: props.invalid ? tokens.danger : tokens.input,
          backgroundColor: props.filled ? tokens.muted : tokens.background,
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function InputGroupText(props: {
  children?: JSX.Element;
  tone?: 'muted' | 'default';
  mono?: boolean;
  size?: ControlSize;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <Text
      style={[
        {
          fontSize: controlFontSize(props.size ?? 'medium') - 1,
          color: props.tone === 'default' ? tokens.foreground : tokens.mutedForeground,
          ...(props.mono ? { fontFamily: 'monospace' } : {}),
        },
        props.style,
      ]}
    >
      {props.children}
    </Text>
  );
}

/** A compact addon chip: a country code, category label, keyboard hint, etc. */
export function InputGroupAddon(props: {
  children?: JSX.Element;
  onPress?: () => void;
  disabled?: boolean;
  tone?: 'muted' | 'plain';
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View
      accessibilityRole={props.onPress ? 'button' : undefined}
      onPress={props.disabled ? undefined : props.onPress}
      disabled={props.disabled}
      activeOpacity={0.85}
      class={props.onPress ? 'transition-colors duration-150 hover:bg-black/[0.05] dark:hover:bg-white/[0.08]' : undefined}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 3,
          paddingHorizontal: 7,
          paddingVertical: 4,
          borderRadius: 5,
          backgroundColor: props.tone === 'plain' ? 'transparent' : tokens.muted,
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function InputGroupButton(props: {
  label?: string;
  icon?: InputGroupButtonIcon;
  variant?: ButtonVariant;
  disabled?: boolean;
  accessibilityLabel?: string;
  onPress: () => void;
}) {
  return (
    <Button
      label={props.label}
      icon={props.icon as never}
      size="xsmall"
      variant={props.variant ?? 'ghost'}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel ?? props.label}
      onPress={props.onPress}
    />
  );
}

/** Kept local so callers do not have to reach into the icon module for a type. */
type InputGroupButtonIcon = NonNullable<Parameters<typeof Button>[0]['icon']>;

export function InputGroupInput(props: Omit<InputProps, 'appearance'>) {
  return <Input {...props} appearance={false} style={[{ flex: 1, minWidth: 0 }, props.style]} />;
}

/** A Select without its own frame, for the leading or trailing addon. */
export function InputGroupSelect(props: SelectProps) {
  return <Select {...props} appearance={false} style={[{ flexShrink: 1, minWidth: 0 }, props.style]} />;
}

/** The inset used by a full-width action row inside the group. */
export function InputGroupSpacer() {
  return <View style={{ flex: 1 }} />;
}

export function inputGroupPadding(size: ControlSize = 'medium') {
  return inputPaddingX(size);
}
