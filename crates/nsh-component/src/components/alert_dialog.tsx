// A dialog that interrupts the reader with one decision.
import type { JSX } from 'nsh';
import { Dialog, type DialogTone } from './dialog';
import { Button } from './button';
import { type IconName } from './icons';

export interface AlertDialogProps {
  open?: boolean;
  title?: string;
  description?: string;
  /** An icon tile in the header, tinted by `tone`. */
  icon?: IconName;
  /** The icon tile's tone. `destructive` implies `danger` when unset. */
  tone?: DialogTone;
  children?: JSX.Element;
  /** Replace the default Cancel/Confirm footer. */
  footer?: JSX.Element;
  confirmLabel?: string;
  cancelLabel?: string;
  destructive?: boolean;
  loading?: boolean;
  /** Draw the Cancel action. `true` when the prop is unset. */
  showCancel?: boolean;
  /** Draw the header's dismiss control. `false` when the prop is unset. */
  showClose?: boolean;
  onConfirm?: () => void;
  onCancel?: () => void;
  onClose?: () => void;
}

export function AlertDialog(props: AlertDialogProps) {
  const tone = (): DialogTone => props.tone ?? (props.destructive ? 'danger' : 'default');
  // The icon carries the tone; the decision itself stays the primary action
  // unless it is destructive. A tinted info/warning confirm is too pale to read
  // as the decision on the light surface.
  const confirmVariant = () => (props.destructive || tone() === 'danger' ? 'danger' : 'primary');
  // A loading confirmation has already been accepted; a second press is not a
  // second decision. The Button marks itself busy, and the default footer guards
  // the callback as well so a fast press cannot repeat the work.
  const confirm = () => {
    if (props.loading) return;
    props.onConfirm?.();
  };
  const close = () => {
    props.onCancel?.();
  };
  const defaultFooter = () => (
    <>
      {props.showCancel ?? true ? (
        <Button label={props.cancelLabel ?? 'Cancel'} variant="ghost" onPress={close} style={{ flex: 1 }} />
      ) : null}
      <Button
        label={props.confirmLabel ?? 'Continue'}
        variant={confirmVariant()}
        loading={props.loading}
        onPress={confirm}
        style={{ flex: 1 }}
      />
    </>
  );
  return (
    <Dialog
      open={props.open}
      title={props.title}
      description={props.description}
      icon={props.icon}
      tone={tone()}
      size="small"
      overlayClosable={false}
      showClose={props.showClose ?? false}
      onClose={props.onClose ?? props.onCancel}
      footer={props.footer ?? defaultFooter()}
    >
      {props.children}
    </Dialog>
  );
}
