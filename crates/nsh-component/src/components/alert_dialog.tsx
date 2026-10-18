// A dialog that interrupts the reader with one decision.
import type { JSX } from 'nsh';
import { Dialog } from './dialog';
import { Button } from './button';

export interface AlertDialogProps {
  open?: boolean;
  title?: string;
  description?: string;
  children?: JSX.Element;
  confirmLabel?: string;
  cancelLabel?: string;
  destructive?: boolean;
  loading?: boolean;
  onConfirm?: () => void;
  onCancel?: () => void;
  onClose?: () => void;
}

export function AlertDialog(props: AlertDialogProps) {
  return (
    <Dialog
      open={props.open}
      title={props.title}
      description={props.description}
      size="small"
      overlayClosable={false}
      onClose={props.onClose ?? props.onCancel}
      footer={
        <>
          <Button label={props.cancelLabel ?? 'Cancel'} variant="ghost" onPress={props.onCancel} />
          <Button
            label={props.confirmLabel ?? 'Continue'}
            variant={props.destructive ? 'danger' : 'primary'}
            loading={props.loading}
            onPress={props.onConfirm}
          />
        </>
      }
    >
      {props.children}
    </Dialog>
  );
}
