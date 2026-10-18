// A floating panel, presented in a transparent modal so it is never clipped by
// a scrolling page. The trigger is the application's; the open state can be
// controlled or left to the wrapper's press.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { RADIUS_LG, tokens } from './theme';

export type PopoverPlacement = 'bottom' | 'top' | 'start' | 'end';

export interface PopoverProps {
  trigger: JSX.Element;
  children?: JSX.Element;
  open?: boolean;
  defaultOpen?: boolean;
  placement?: PopoverPlacement;
  onOpenChange?: (open: boolean) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Popover(props: PopoverProps) {
  const own = signal(Boolean(props.defaultOpen));
  const open = () => props.open ?? own();
  const setOpen = (next: boolean) => {
    if (props.open === undefined) own.set(next);
    props.onOpenChange?.(next);
  };
  return (
    <View testID={props.testID} style={{ alignSelf: 'flex-start' }}>
      <View onPress={() => setOpen(!open())}>{props.trigger}</View>
      <Modal visible={open()} transparent animationType="fade" onRequestClose={() => setOpen(false)}>
        <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
          <View class="shadow-2xl" style={[{ width: 320, maxWidth: '100%', padding: 14, gap: 8, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }, props.style]}>
            {props.children}
          </View>
        </View>
      </Modal>
    </View>
  );
}
