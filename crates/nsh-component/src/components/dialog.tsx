// A dialog over a dimmed backdrop, presented by NSH's native Modal.
//
// The native layer only fades as a whole, which read as a pop. The panel rides
// a native value instead: it fades, rises and scales in on open, and replays
// the reverse before the layer closes.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { cleanup, effect, signal, Text, View } from 'nsh';
import { reduceMotion } from 'nsh/accessibility';
import { createNativeValue } from 'nsh/animated';
import { keymap } from 'nsh/keymap';
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

const ENTER_MS = 200;
const EXIT_MS = 150;

export function Dialog(props: DialogProps) {
  // `mounted` keeps the modal presented through the exit; `enter` is the
  // native value both the panel and the backdrop follow.
  const mounted = signal(Boolean(props.open));
  const enter = createNativeValue(props.open ? 1 : 0);
  effect(() => {
    if (props.open) {
      mounted.set(true);
      if (reduceMotion()) enter.set(1);
      else enter.timing(1, { duration: ENTER_MS, easing: 'easeOut' });
      return;
    }
    if (!mounted()) return;
    const run = reduceMotion() ? Promise.resolve(false) : enter.timing(0, { duration: EXIT_MS, easing: 'easeIn' });
    run.then(() => {
      if (!props.open) mounted.set(false);
    });
  });
  // Escape closes the dialog while it is presented; the binding is removed
  // with the component and with each close.
  effect(() => {
    if (!mounted() || !props.onClose) return;
    const unbind = keymap.bind('escape', () => props.onClose?.());
    cleanup(() => unbind());
  });
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
      visible={mounted()}
      transparent
      animationType={reduceMotion() ? 'fade' : 'none'}
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
          ...(reduceMotion() ? {} : { opacity: enter }),
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
              ...(reduceMotion()
                ? {}
                : {
                    opacity: enter,
                    transform: [
                      { scale: enter.interpolate({ inputRange: [0, 1], outputRange: [0.96, 1] }) },
                      { translateY: enter.interpolate({ inputRange: [0, 1], outputRange: [8, 0] }) },
                    ],
                  }),
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
