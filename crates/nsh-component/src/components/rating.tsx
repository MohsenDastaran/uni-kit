// An interactive star rating.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { Icon } from './icon';
import { tokens } from './theme';

export interface RatingProps {
  value?: number;
  defaultValue?: number;
  max?: number;
  size?: number;
  color?: string;
  readOnly?: boolean;
  disabled?: boolean;
  onChange?: (value: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Rating(props: RatingProps) {
  const own = signal(props.defaultValue ?? 0);
  const value = () => props.value ?? own();
  const choose = (next: number) => {
    if (props.readOnly || props.disabled) return;
    if (props.value === undefined) own.set(next);
    props.onChange?.(next);
  };
  return (
    <View
      testID={props.testID}
      accessibilityRole="adjustable"
      accessibilityLabel="Rating"
      accessibilityValue={{ min: 0, max: props.max ?? 5, now: value() }}
      style={[{ flexDirection: 'row', gap: 4, opacity: props.disabled ? 0.5 : 1 }, props.style]}
    >
      {Array.from({ length: props.max ?? 5 }, (_, index) => {
        const filled = index < value();
        return (
          <View
            accessibilityRole="button"
            accessibilityLabel={`${index + 1} stars`}
            onPress={() => choose(index + 1)}
            activeOpacity={0.7}
            style={{ width: props.size ?? 20, height: props.size ?? 20, alignItems: 'center', justifyContent: 'center' }}
          >
            <Icon name="star" size={props.size ?? 20} color={filled ? props.color ?? tokens.warning : tokens.mutedForeground} />
          </View>
        );
      })}
    </View>
  );
}
