// A dialog over a dimmed backdrop, presented by NSH's native Modal.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, RADIUS_LG, SPACING_MD, SPACING_SM, tokens } from './theme';

/** The tone a dialog's icon carries. */
export type DialogTone = 'default' | 'danger' | 'warning' | 'success' | 'info';

export interface DialogProps {
  open?: boolean;
  title?: string;
  description?: string;
  icon?: IconName;
  tone?: DialogTone;
  children?: JSX.Element;
  footer?: JSX.Element;
  size?: 'small' | 'medium' | 'large';
  overlayClosable?: boolean;
  /** The header's dismiss control. `true` when the prop is unset. */
  showClose?: boolean;
  onClose?: () => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Dialog(props: DialogProps) {
  const width = () => (props.size === 'small' ? 320 : props.size === 'large' ? 560 : 440);
  const tone = () => props.tone ?? 'default';
  const toneColor = () => {
    switch (tone()) {
      case 'danger':
        return tokens.danger;
      case 'warning':
        return tokens.warning;
      case 'success':
        return tokens.success;
      case 'info':
        return tokens.info;
      default:
        return tokens.primary;
    }
  };
  const showClose = () => props.showClose ?? true;
  const hasHeader = () =>
    Boolean(props.title || props.description || props.icon || showClose());
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
          class="shadow-lg"
          style={[
            {
              width: width(),
              maxWidth: '100%',
              backgroundColor: tokens.popover,
              borderRadius: RADIUS_LG,
              borderWidth: 1,
              borderColor: tokens.border,
              padding: 20,
              gap: SPACING_MD,
            },
            props.style,
          ]}
        >
          {hasHeader() ? (
            <View style={{ flexDirection: 'row', alignItems: 'flex-start', gap: SPACING_SM }}>
              {props.icon ? (
                <View
                  style={{
                    width: 36,
                    height: 36,
                    borderRadius: 10,
                    backgroundColor: alpha(toneColor(), 0.12),
                    borderWidth: 1,
                    borderColor: alpha(toneColor(), 0.24),
                    alignItems: 'center',
                    justifyContent: 'center',
                  }}
                >
                  <Icon name={props.icon} size={18} color={toneColor()} />
                </View>
              ) : null}
              <View
                style={{
                  flex: 1,
                  gap: 2,
                  paddingTop: props.icon ? (props.description ? 0 : 8) : 0,
                }}
              >
                {props.title ? <Text style={{ fontSize: 17, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text> : null}
                {props.description ? <Text style={{ fontSize: 13, lineHeight: 18, color: tokens.mutedForeground }}>{props.description}</Text> : null}
              </View>
              {showClose() ? (
                <View
                  accessibilityRole="button"
                  accessibilityLabel="Close"
                  onPress={props.onClose}
                  activeOpacity={0.7}
                  class="bg-transparent transition-all duration-150 ease-out hover:bg-black/5 active:scale-90 active:bg-black/10 dark:hover:bg-white/10 dark:active:bg-white/15"
                  style={{ width: 24, height: 24, borderRadius: 6, alignItems: 'center', justifyContent: 'center' }}
                >
                  <Icon name="x" size={14} color={tokens.mutedForeground} />
                </View>
              ) : null}
            </View>
          ) : null}
          {props.children}
          {props.footer ? (
            <View style={{ flexDirection: 'row', justifyContent: 'flex-end', alignItems: 'center', gap: SPACING_SM, marginTop: 4, width: '100%' }}>
              {props.footer}
            </View>
          ) : null}
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
