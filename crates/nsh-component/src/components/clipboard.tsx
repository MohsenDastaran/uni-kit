// A copy control: puts text on the system clipboard and reports the copy.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Clipboard } from 'nsh/clipboard';
import { Icon } from './icon';
import { Button, type ButtonVariant } from './button';
import { type ControlSize, tokens } from './theme';

export interface ClipboardProps {
  text: string;
  label?: string;
  copiedLabel?: string;
  variant?: ButtonVariant;
  size?: ControlSize;
  disabled?: boolean;
  children?: JSX.Element;
  onCopy?: (text: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function ClipboardButton(props: ClipboardProps) {
  const copied = signal(false);
  const copy = () => {
    if (props.disabled) return;
    Clipboard.setString(props.text);
    copied.set(true);
    props.onCopy?.(props.text);
    setTimeout(() => copied.set(false), 1400);
  };
  return (
    <Button
      label={copied() ? props.copiedLabel ?? 'Copied' : props.label ?? 'Copy'}
      icon={copied() ? 'check' : 'copy'}
      variant={props.variant ?? 'default'}
      size={props.size}
      disabled={props.disabled}
      onPress={copy}
      style={props.style}
      testID={props.testID}
    >
      {props.children}
    </Button>
  );
}

export function ClipboardText(props: { text: string; label?: string; style?: StyleProp<ViewStyle> }) {
  return <Text style={[{ fontSize: 13, color: tokens.mutedForeground }, props.style]}>{props.text}</Text>;
}
