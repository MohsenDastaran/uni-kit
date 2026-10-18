// A short label for a category, status or filter. Variants, sizes, an outline,
// an icon and an optional close button.
import type { StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, type ControlSize, controlFontSize, RADIUS_SM, SPACING_XS, tokens } from './theme';

export type TagVariant =
  | 'default'
  | 'secondary'
  | 'outline'
  | 'danger'
  | 'warning'
  | 'success'
  | 'info';

export interface TagProps {
  label?: string;
  variant?: TagVariant;
  size?: ControlSize;
  rounded?: boolean;
  icon?: IconName;
  color?: string;
  closable?: boolean;
  onClose?: () => void;
  onPress?: () => void;
  disabled?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Tag(props: TagProps) {
  const variant = () => props.variant ?? 'default';
  const tone = () => {
    switch (variant()) {
      case 'danger':
        return tokens.danger;
      case 'warning':
        return tokens.warning;
      case 'success':
        return tokens.success;
      case 'info':
        return tokens.info;
      case 'secondary':
        return tokens.mutedForeground;
      default:
        return tokens.foreground;
    }
  };
  const status = () => ['danger', 'warning', 'success', 'info'].includes(variant());
  const background = () => {
    if (props.color) return props.color;
    if (variant() === 'outline') return 'transparent';
    if (status()) return alpha(tone(), 0.15);
    if (variant() === 'secondary') return tokens.muted;
    return tokens.accent;
  };
  const border = () => {
    if (props.color) return props.color;
    if (variant() === 'outline') return status() ? tone() : tokens.border;
    return status() ? alpha(tone(), 0.35) : tokens.border;
  };
  const textColor = () => {
    if (props.color) return tokens.background;
    if (variant() === 'outline') return status() ? tone() : tokens.foreground;
    if (status()) return tone();
    return tokens.foreground;
  };
  const size = () => props.size ?? 'small';
  return (
    <View
      testID={props.testID}
      accessibilityRole={props.onPress ? 'button' : undefined}
      accessibilityLabel={props.label}
      onPress={props.disabled ? undefined : props.onPress}
      activeOpacity={0.8}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: SPACING_XS,
          alignSelf: 'flex-start',
          paddingHorizontal: size() === 'xsmall' ? 4 : size() === 'large' ? 10 : 7,
          paddingVertical: size() === 'xsmall' ? 1 : size() === 'large' ? 4 : 2,
          borderRadius: props.rounded ? 999 : RADIUS_SM,
          borderWidth: 1,
          borderColor: border(),
          backgroundColor: background(),
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      {props.icon ? <Icon name={props.icon} size={controlFontSize(size()) - 1} color={textColor()} /> : null}
      <Text
        style={{
          fontSize: controlFontSize(size()) - 1,
          lineHeight: controlFontSize(size()) + 2,
          color: textColor(),
          fontWeight: '500',
        }}
      >
        {props.label ?? ''}
      </Text>
      {props.closable ? (
        <View
          accessibilityRole="button"
          accessibilityLabel="Remove"
          onPress={props.onClose}
          activeOpacity={0.7}
          style={{ width: 14, height: 14, alignItems: 'center', justifyContent: 'center' }}
        >
          <Icon name="x" size={10} color={textColor()} />
        </View>
      ) : null}
    </View>
  );
}
