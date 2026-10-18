// A number with − and + buttons. The mobile Stepper already runs the repeat
// timer natively, so this wraps it with the catalog's value/range names.
import type { StyleProp, ViewStyle } from 'nsh';
import { Stepper as MobileStepper } from 'nsh/ui/mobile';
import { tokens } from './theme';

export interface StepperProps {
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  onChange?: (value: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Stepper(props: StepperProps) {
  return (
    <MobileStepper
      value={props.value}
      defaultValue={props.defaultValue ?? 0}
      min={props.min ?? 0}
      max={props.max ?? 100}
      step={props.step ?? 1}
      onChange={props.onChange}
      style={props.style}
      testID={props.testID}
    />
  );
}

export function StepperValue(props: { value: number; style?: StyleProp<ViewStyle> }) {
  return <MobileStepper value={props.value} onChange={() => {}} style={props.style} />;
}
