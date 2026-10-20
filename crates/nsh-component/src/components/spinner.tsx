// An animated loading indicator. `Icon` already knows how to spin; the spinner
// adds the catalog's sizes, status colours and an optional overlay.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { type ControlSize, FONT_SIZE_SM, tokens } from './theme';

export interface SpinnerProps {
  size?: ControlSize | number;
  color?: string;
  status?: 'default' | 'info' | 'success' | 'warning' | 'danger';
  icon?: IconName;
  label?: string;
  overlay?: boolean;
  visible?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const sizeOf = (size: ControlSize | number | undefined): number => {
  if (typeof size === 'number') return size;
  switch (size) {
    case 'xsmall':
      return 14;
    case 'small':
      return 16;
    case 'large':
      return 32;
    default:
      return 22;
  }
};

export function Spinner(props: SpinnerProps) {
  if (props.visible === false) return null;
  const tone = () => {
    switch (props.status) {
      case 'info':
        return tokens.info;
      case 'success':
        return tokens.success;
      case 'warning':
        return tokens.warning;
      case 'danger':
        return tokens.danger;
      default:
        return props.color ?? tokens.primary;
    }
  };
  const spinner = () => (
    <Icon
      name={props.icon ?? 'loader-circle'}
      spinning
      size={sizeOf(props.size)}
      color={tone()}
      accessibilityLabel={props.label ?? 'Loading'}
    />
  );
  if (props.overlay) {
    return (
      <View
        style={[
          {
            ...(StyleSheetFill()),
            backgroundColor: tokens.overlay,
            alignItems: 'center',
            justifyContent: 'center',
            gap: 8,
          },
          props.style,
        ]}
      >
        {spinner()}
        {props.label ? <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.foreground }}>{props.label}</Text> : null}
      </View>
    );
  }
  return (
    <View
      testID={props.testID}
      style={[
        { flexDirection: 'row', alignItems: 'center', gap: 8 },
        props.style,
      ]}
    >
      {spinner()}
      {props.label ? <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.label}</Text> : null}
    </View>
  );
}

/** Absolute fill, kept as a helper so the overlay has no layout of its own. */
function StyleSheetFill(): ViewStyle {
  return { position: 'absolute', top: 0, right: 0, bottom: 0, left: 0 };
}

export function SpinnerOverlay(props: { visible?: boolean; label?: string; children?: JSX.Element }) {
  return (
    <View style={{ flex: 1 }}>
      {props.children}
      <Spinner overlay visible={props.visible} label={props.label} />
    </View>
  );
}
