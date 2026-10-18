// A bottom sheet over a dimmed backdrop, using the mobile Sheet.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Sheet as MobileSheet } from 'nsh/ui/mobile';
import { FONT_SIZE_SM, tokens } from './theme';

export interface SheetProps {
  open?: boolean;
  title?: string;
  description?: string;
  children?: JSX.Element;
  footer?: JSX.Element;
  onClose?: () => void;
  style?: StyleProp<ViewStyle>;
}

export function Sheet(props: SheetProps) {
  return (
    <MobileSheet open={Boolean(props.open)} onClose={props.onClose}>
      {props.title ? (
        <Text style={{ fontSize: 17, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
      ) : null}
      {props.description ? (
        <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.description}</Text>
      ) : null}
      {props.children}
      {props.footer ? <View style={{ flexDirection: 'row', justifyContent: 'flex-end', gap: 8 }}>{props.footer}</View> : null}
    </MobileSheet>
  );
}
