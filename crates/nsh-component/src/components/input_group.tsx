// A frame that groups an input with leading/trailing text, icons or buttons.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Button } from './button';
import { Input } from './input';
import { type ControlSize, inputHeight, RADIUS_MD, tokens } from './theme';

export interface InputGroupProps {
  size?: ControlSize;
  disabled?: boolean;
  invalid?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function InputGroup(props: InputGroupProps) {
  return (
    <View
      testID={props.testID}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 4,
          height: inputHeight(props.size ?? 'medium') + 8,
          paddingHorizontal: 6,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: props.invalid ? tokens.danger : tokens.input,
          backgroundColor: tokens.background,
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function InputGroupText(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{props.children}</Text>;
}

export function InputGroupButton(props: { label: string; onPress: () => void; disabled?: boolean }) {
  return <Button label={props.label} size="small" variant="ghost" disabled={props.disabled} onPress={props.onPress} />;
}

export function InputGroupInput(props: {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  onChangeText?: (text: string) => void;
}) {
  return (
    <Input
      value={props.value}
      placeholder={props.placeholder}
      disabled={props.disabled}
      appearance={false}
      onChangeText={props.onChangeText}
      style={{ flex: 1 }}
    />
  );
}
