// A button with a menu in a transparent modal.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Button, type ButtonVariant } from './button';
import { Menu, type MenuItem } from './menu';
import { type ControlSize, RADIUS_LG, tokens } from './theme';

export interface DropdownButtonProps {
  label: string;
  items: MenuItem[];
  variant?: ButtonVariant;
  size?: ControlSize;
  disabled?: boolean;
  onSelect?: (index: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function DropdownButton(props: DropdownButtonProps) {
  const open = signal(false);
  return (
    <View testID={props.testID} style={[{ alignSelf: 'flex-start' }, props.style]}>
      <Button
        label={props.label}
        variant={props.variant ?? 'secondary'}
        size={props.size}
        disabled={props.disabled}
        suffixIcon="chevron-down"
        onPress={() => open.set(!open())}
      />
      <Modal visible={open()} transparent animationType="fade" onRequestClose={() => open.set(false)}>
        <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
          <View class="shadow-2xl" style={{ width: 280, maxWidth: '100%', borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, overflow: 'hidden' }}>
            <Menu
              items={props.items}
              style={{ width: '100%' }}
              onSelect={(index) => {
                open.set(false);
                props.onSelect?.(index);
              }}
            />
          </View>
        </View>
      </Modal>
    </View>
  );
}
