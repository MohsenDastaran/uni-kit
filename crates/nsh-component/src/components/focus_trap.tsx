// Keeps keyboard focus inside a region. NSH has no tab-focus API, so the trap is
// the native modal layer: while it is active, the rest of the app is inert and
// Escape asks to close.
import type { JSX } from 'nsh';
import { View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { tokens } from './theme';

export interface FocusTrapProps {
  active?: boolean;
  children?: JSX.Element;
  onEscape?: () => void;
}

export function FocusTrap(props: FocusTrapProps) {
  if (!props.active) return <>{props.children}</>;
  return (
    <Modal visible transparent animationType="none" onRequestClose={props.onEscape}>
      <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
        {props.children}
      </View>
    </Modal>
  );
}
