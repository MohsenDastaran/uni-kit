// A labelled cluster of controls. The mobile `Group` already draws the rounded
// box, the row padding and the dividers, so this wraps it with the catalog's
// title/footer/variant surface.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text } from 'nsh';
import { Group } from 'nsh/ui/mobile';
import { FONT_SIZE_SM, tokens } from './theme';

export type GroupBoxVariant = 'default' | 'outline' | 'filled';

export interface GroupBoxProps {
  title?: string;
  footer?: string;
  variant?: GroupBoxVariant;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function GroupBox(props: GroupBoxProps) {
  const variant = () => props.variant ?? 'default';
  return (
    <Group
      testID={props.testID}
      header={
        props.title ? (
          <Text style={{ fontSize: FONT_SIZE_SM, fontWeight: '600', color: tokens.mutedForeground }}>
            {props.title}
          </Text>
        ) : undefined
      }
      footer={
        props.footer ? (
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.footer}</Text>
        ) : undefined
      }
      style={[
        {
          ...(variant() === 'outline'
            ? { borderWidth: 1, borderColor: tokens.border, borderRadius: 8 }
            : {}),
          ...(variant() === 'filled' ? { backgroundColor: tokens.muted } : {}),
        },
        props.style,
      ]}
    >
      {props.children}
    </Group>
  );
}
