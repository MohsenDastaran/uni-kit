// A binary checkbox with a label, description and the four control sizes.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type ControlSize, controlFontSize, tokens } from './theme';

export interface CheckboxProps {
  checked?: boolean;
  defaultChecked?: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  size?: ControlSize;
  label?: string;
  description?: string;
  color?: string;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const boxSize = (size: ControlSize): number =>
  size === 'xsmall' ? 14 : size === 'small' ? 16 : size === 'large' ? 22 : 18;

export function Checkbox(props: CheckboxProps) {
  const own = signal(Boolean(props.defaultChecked));
  const checked = () => props.checked ?? own();
  const toggle = () => {
    if (props.disabled) return;
    const next = !checked();
    if (props.checked === undefined) own.set(next);
    props.onChange?.(next);
  };
  return (
    <View
      testID={props.testID}
      accessibilityRole="checkbox"
      accessibilityLabel={props.label}
      accessibilityState={{ checked: checked(), disabled: Boolean(props.disabled) }}
      onPress={toggle}
      disabled={props.disabled}
      activeOpacity={0.8}
      style={[
        { flexDirection: 'row', alignItems: 'flex-start', gap: 8, opacity: props.disabled ? 0.5 : 1 },
        props.style,
      ]}
    >
      <View
        style={{
          width: boxSize(props.size ?? 'medium'),
          height: boxSize(props.size ?? 'medium'),
          borderRadius: 4,
          borderWidth: 1,
          borderColor: checked() ? props.color ?? tokens.primary : tokens.input,
          backgroundColor: checked() ? props.color ?? tokens.primary : tokens.background,
          alignItems: 'center',
          justifyContent: 'center',
          marginTop: 1,
        }}
      >
        {checked() ? <Icon name="check" size={boxSize(props.size ?? 'medium') - 5} color={tokens.primaryForeground} /> : null}
      </View>
      {props.label || props.description ? (
        <View style={{ flex: 1, gap: 1 }}>
          <Text style={{ fontSize: controlFontSize(props.size ?? 'medium'), color: tokens.foreground }}>{props.label ?? ''}</Text>
          {props.description ? (
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
    </View>
  );
}
