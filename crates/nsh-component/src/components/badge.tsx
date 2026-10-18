// A count, a dot or a status icon anchored to the corner of its child.
//
// Mirrors GPUI Component's `Badge` and the Slint version beside it: a count
// above zero, `dot`, or a small icon chip. The overlay is absolutely positioned
// over the child, so the child keeps its own layout and the badge hangs past its
// top-right corner.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { inkOn, type ControlSize, tokens } from './theme';

export interface BadgeProps {
  count?: number;
  max?: number;
  dot?: boolean;
  icon?: IconName;
  color?: string;
  size?: ControlSize;
  /** The element the badge is anchored to. Without it, the chip is drawn alone. */
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

export function Badge(props: BadgeProps) {
  const size = () => props.size ?? 'medium';
  const color = () => props.color ?? tokens.danger;
  const chip = () => (size() === 'large' ? 24 : size() === 'medium' ? 16 : 10);
  const numberHeight = () => (size() === 'small' || size() === 'xsmall' ? 14 : 16);
  const label = () => {
    const count = props.count ?? 0;
    const max = props.max ?? 99;
    return count > max ? `${max}+` : String(count);
  };
  const hasNumber = () => !props.dot && !props.icon && (props.count ?? 0) > 0;
  const hasIcon = () => !props.dot && Boolean(props.icon);
  const hasDot = () => Boolean(props.dot) && ((props.count ?? 0) > 0 || !props.icon);

  const chipView = () => (
    <>
      {hasDot() ? (
        <View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: color() }} />
      ) : null}
      {hasIcon() ? (
        <View
          style={{
            width: chip(),
            height: chip(),
            borderRadius: chip() / 2,
            borderWidth: 1,
            borderColor: tokens.background,
            backgroundColor: color(),
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Icon name={props.icon ?? 'circle'} size={chip() * 0.62} color={inkOn(color())} />
        </View>
      ) : null}
      {hasNumber() ? (
        <View
          style={{
            height: numberHeight(),
            minWidth: numberHeight(),
            paddingHorizontal: 4,
            borderRadius: numberHeight() / 2,
            backgroundColor: color(),
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Text style={{ fontSize: 10, lineHeight: 12, color: inkOn(color()), textAlign: 'center' }}>
            {label()}
          </Text>
        </View>
      ) : null}
    </>
  );

  const content = props.children;

  return (
    <View
      testID={props.testID}
      accessibilityLabel={props.accessibilityLabel}
      style={[
        {
          position: 'relative',
          alignSelf: 'flex-start',
          ...(content ? {} : { flexDirection: 'row', alignItems: 'center' }),
        },
        props.style,
      ]}
    >
      {content ?? chipView()}
      {content ? (
        <>
          {hasDot() ? (
            <View
              style={{
                position: 'absolute',
                top: 0,
                right: 0,
                width: 6,
                height: 6,
                borderRadius: 3,
                backgroundColor: color(),
              }}
            />
          ) : null}
          {hasIcon() ? (
            <View
              style={{
                position: 'absolute',
                right: -chip() + 2,
                bottom: -chip() + 2,
                width: chip(),
                height: chip(),
                borderRadius: chip() / 2,
                borderWidth: 1,
                borderColor: tokens.background,
                backgroundColor: color(),
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <Icon name={props.icon ?? 'circle'} size={chip() * 0.62} color={inkOn(color())} />
            </View>
          ) : null}
          {hasNumber() ? (
            <View
              style={{
                position: 'absolute',
                top: -4,
                right: -6,
                height: numberHeight(),
                minWidth: numberHeight(),
                paddingHorizontal: 4,
                borderRadius: numberHeight() / 2,
                backgroundColor: color(),
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <Text style={{ fontSize: 10, lineHeight: 12, color: inkOn(color()), textAlign: 'center' }}>
                {label()}
              </Text>
            </View>
          ) : null}
        </>
      ) : null}
    </View>
  );
}
