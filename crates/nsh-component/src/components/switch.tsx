// A binary switch: a track and a knob, controlled or uncontrolled.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { type ControlSize, tokens } from './theme';

export interface SwitchProps {
  checked?: boolean;
  defaultChecked?: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  size?: ControlSize;
  color?: string;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

export function Switch(props: SwitchProps) {
  const own = signal(Boolean(props.defaultChecked));
  const checked = () => props.checked ?? own();
  const size = () => (props.size === 'small' ? 18 : props.size === 'large' ? 26 : 22);
  const trackWidth = () => size() * 1.8;
  const set = () => {
    if (props.disabled) return;
    const next = !checked();
    if (props.checked === undefined) own.set(next);
    props.onChange?.(next);
  };
  return (
    <View
      testID={props.testID}
      accessibilityRole="switch"
      accessibilityLabel={props.accessibilityLabel}
      accessibilityState={{ checked: checked(), disabled: Boolean(props.disabled) }}
      onPress={set}
      disabled={props.disabled}
      activeOpacity={0.9}
      style={[
        {
          width: trackWidth(),
          height: size(),
          borderRadius: size() / 2,
          padding: 2,
          justifyContent: 'center',
          backgroundColor: checked() ? props.color ?? tokens.primary : tokens.input,
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      <View
        style={{
          width: size() - 4,
          height: size() - 4,
          borderRadius: (size() - 4) / 2,
          backgroundColor: tokens.background,
          transform: [{ translateX: checked() ? trackWidth() - size() : 0 }],
          transitionProperty: 'transform',
          transitionDuration: 160,
          transitionTimingFunction: 'ease-out',
        }}
      />
    </View>
  );
}
