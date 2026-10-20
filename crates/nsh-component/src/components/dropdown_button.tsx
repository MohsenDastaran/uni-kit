// A button with an anchored menu. The panel is an absolute View, not a
// modal: it stays inside the page, so no dialog layer is involved.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { Button, type ButtonVariant } from './button';
import { Menu, type MenuItem } from './menu';
import { type ControlSize, controlHeight, RADIUS_LG, tokens } from './theme';

export interface DropdownButtonProps {
  label: string;
  items: MenuItem[];
  variant?: ButtonVariant;
  size?: ControlSize;
  disabled?: boolean;
  /** The main half's action. The menu opens only from the arrow half. */
  onPress?: () => void;
  /** Align the panel with the button's start or end edge. */
  align?: 'start' | 'end';
  /** A wider menu than the button. */
  menuWidth?: number;
  onOpenChange?: (open: boolean) => void;
  onSelect?: (index: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function DropdownButton(props: DropdownButtonProps) {
  const open = signal(false);
  return (
    <View
      testID={props.testID}
      style={[
        {
          alignSelf: 'flex-start',
          position: 'relative',
          zIndex: open() ? 1000 : 0,
        },
        props.style,
      ]}
    >
      <View style={{ flexDirection: 'row' }}>
        <Button
          label={props.label}
          variant={props.variant ?? 'secondary'}
          size={props.size}
          disabled={props.disabled}
          groupEnd
          onPress={props.onPress}
        />
        <View style={{ width: 1, height: controlHeight(props.size ?? 'medium'), marginLeft: -1, marginRight: -1, backgroundColor: tokens.border, zIndex: 2 }} />
        <Button
          icon="chevron-down"
          variant={props.variant ?? 'secondary'}
          size={props.size}
          disabled={props.disabled}
          groupStart
          accessibilityLabel="Open menu"
          onPress={() => {
            const next = !open();
            open.set(next);
            props.onOpenChange?.(next);
          }}
        />
      </View>
      {open() ? (
        <View
          class="shadow-2xl"
          style={{
            position: 'absolute',
            top: controlHeight(props.size ?? 'medium') + 4,
            ...(props.align === 'end' ? { right: 0 } : { left: 0 }),
            width: props.menuWidth ?? 240,
            maxWidth: '100%',
            zIndex: 1001,
            borderRadius: RADIUS_LG,
            borderWidth: 1,
            borderColor: tokens.border,
            backgroundColor: tokens.popover,
            overflow: 'hidden',
          }}
        >
          <Menu
            items={props.items}
            style={{ width: '100%' }}
            onSelect={(index) => {
              open.set(false);
              props.onOpenChange?.(false);
              props.onSelect?.(index);
            }}
          />
        </View>
      ) : null}
    </View>
  );
}
