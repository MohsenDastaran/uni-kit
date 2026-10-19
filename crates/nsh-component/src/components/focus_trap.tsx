// Keeps keyboard focus inside a region. NSH has no tab-focus API, so the trap
// uses the native modal layer and, when the app supplies a focus order, cycles
// those inputs itself with Tab / Shift+Tab. Escape closes through onEscape.
import type { JSX, YNode } from 'nsh';
import { cleanup, effect, signal, View } from 'nsh';
import { keymap } from 'nsh/keymap';
import { Modal } from 'nsh/ui/mobile';
import { tokens } from './theme';

export interface FocusTrapProps {
  active?: boolean;
  children?: JSX.Element;
  /** Inputs to cycle in order while the trap is active. */
  focusOrder?: YNode[];
  onEscape?: () => void;
}

export function FocusTrap(props: FocusTrapProps) {
  const index = signal(0);
  const order = () => props.focusOrder ?? [];
  // Cycle the supplied inputs only while the trap is active. The binding is
  // removed with the trap and when it closes.
  effect(() => {
    if (!props.active || order().length === 0) return;
    const focusAt = (next: number) => {
      const list = order();
      if (list.length === 0) return;
      const at = ((next % list.length) + list.length) % list.length;
      index.set(at);
      list[at]?.focus();
    };
    focusAt(0);
    const unbindTab = keymap.bind(
      'tab',
      () => focusAt(index() + 1),
      { when: 'always' },
    );
    const unbindBack = keymap.bind(
      'shift+tab',
      () => focusAt(index() - 1),
      { when: 'always' },
    );
    const unbindEscape = props.onEscape
      ? keymap.bind('escape', () => props.onEscape?.(), { when: 'always' })
      : undefined;
    cleanup(() => {
      unbindTab();
      unbindBack();
      unbindEscape?.();
    });
  });

  return (
    <Modal visible={Boolean(props.active)} transparent animationType="none" onRequestClose={props.onEscape}>
      <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
        {props.children}
      </View>
    </Modal>
  );
}
