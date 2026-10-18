// A dialog over a dimmed backdrop, presented by NSH's native Modal.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Icon } from './icon';
import { RADIUS_LG, SPACING_LG, SPACING_SM, tokens } from './theme';

export interface DialogProps {
  open?: boolean;
  title?: string;
  description?: string;
  children?: JSX.Element;
  footer?: JSX.Element;
  size?: 'small' | 'medium' | 'large';
  overlayClosable?: boolean;
  onClose?: () => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Dialog(props: DialogProps) {
  const width = () => (props.size === 'small' ? 320 : props.size === 'large' ? 560 : 440);
  return (
    <Modal
      visible={Boolean(props.open)}
      transparent
      animationType="fade"
      dismissible={props.overlayClosable}
      onRequestClose={props.onClose}
    >
      <View
        style={{
          flex: 1,
          backgroundColor: tokens.overlay,
          alignItems: 'center',
          justifyContent: 'center',
          padding: 24,
        }}
      >
        <View
          testID={props.testID}
          class="shadow-2xl"
          style={[
            {
              width: width(),
              maxWidth: '100%',
              backgroundColor: tokens.popover,
              borderRadius: RADIUS_LG,
              borderWidth: 1,
              borderColor: tokens.border,
              padding: SPACING_LG,
              gap: SPACING_SM,
            },
            props.style,
          ]}
        >
          <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
            <View style={{ flex: 1, gap: 2 }}>
              {props.title ? <Text style={{ fontSize: 17, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text> : null}
              {props.description ? <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{props.description}</Text> : null}
            </View>
            <View accessibilityRole="button" accessibilityLabel="Close" onPress={props.onClose} activeOpacity={0.7} class="bg-transparent transition-all duration-150 hover:bg-black/5 active:scale-90 active:bg-black/10 dark:hover:bg-white/10 dark:active:bg-white/15" style={{ width: 24, height: 24, borderRadius: 6, alignItems: 'center', justifyContent: 'center' }}>
              <Icon name="x" size={14} color={tokens.mutedForeground} />
            </View>
          </View>
          {props.children}
          {props.footer ? <View style={{ flexDirection: 'row', justifyContent: 'flex-end', gap: 8, marginTop: 8 }}>{props.footer}</View> : null}
        </View>
      </View>
    </Modal>
  );
}

export function DialogHeader(props: { children?: JSX.Element }) {
  return <View style={{ gap: 2 }}>{props.children}</View>;
}

export function DialogTitle(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 17, fontWeight: '600', color: tokens.foreground }}>{props.children}</Text>;
}

export function DialogDescription(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{props.children}</Text>;
}

export function DialogFooter(props: { children?: JSX.Element }) {
  return <View style={{ flexDirection: 'row', justifyContent: 'flex-end', gap: 8, marginTop: 8 }}>{props.children}</View>;
}
