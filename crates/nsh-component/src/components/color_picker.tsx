// A colour picker: featured swatches, a hex field and an alpha slider.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Input } from './input';
import { Button } from './button';
import { type ControlSize, RADIUS_MD, tokens } from './theme';

export interface ColorPickerProps {
  value?: string;
  defaultValue?: string;
  featured?: string[];
  size?: ControlSize;
  disabled?: boolean;
  showAlpha?: boolean;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const DEFAULT_FEATURED = ['#ef4444', '#f97316', '#eab308', '#22c55e', '#06b6d4', '#3b82f6', '#8b5cf6', '#ec4899', '#171717', '#737373', '#ffffff'];

export function ColorPicker(props: ColorPickerProps) {
  const own = signal(props.defaultValue ?? '#171717');
  const value = () => props.value ?? own();
  const set = (next: string) => {
    if (props.value === undefined) own.set(next);
    props.onChange?.(next);
  };
  return (
    <View testID={props.testID} style={[{ gap: 10, opacity: props.disabled ? 0.5 : 1 }, props.style]}>
      <View style={{ flexDirection: 'row', flexWrap: 'wrap', gap: 8 }}>
        {(props.featured ?? DEFAULT_FEATURED).map((color) => (
          <View
            accessibilityRole="button"
            accessibilityLabel={color}
            accessibilityState={{ selected: value().toLowerCase() === color.toLowerCase() }}
            onPress={props.disabled ? undefined : () => set(color)}
            disabled={props.disabled}
            activeOpacity={0.8}
            style={{
              width: props.size === 'large' ? 32 : 26,
              height: props.size === 'large' ? 32 : 26,
              borderRadius: 13,
              backgroundColor: color,
              borderWidth: value().toLowerCase() === color.toLowerCase() ? 3 : 1,
              borderColor: value().toLowerCase() === color.toLowerCase() ? tokens.ring : tokens.border,
            }}
          />
        ))}
      </View>
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
        <View style={{ width: 32, height: 32, borderRadius: 6, backgroundColor: value(), borderWidth: 1, borderColor: tokens.border }} />
        <Input
          value={value()}
          disabled={props.disabled}
          placeholder="#171717"
          onChangeText={set}
          style={{ flex: 1 }}
        />
      </View>
      {props.showAlpha ? (
        <View style={{ gap: 4 }}>
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Alpha</Text>
          <View style={{ height: 10, borderRadius: RADIUS_MD, backgroundColor: tokens.muted, overflow: 'hidden' }}>
            <View style={{ width: '100%', height: '100%', backgroundImage: `linear-gradient(90deg, transparent, ${value()})` }} />
          </View>
        </View>
      ) : null}
    </View>
  );
}

export function ColorSwatch(props: { color: string; selected?: boolean; onPress?: () => void; size?: number }) {
  return (
    <View onPress={props.onPress} style={{ width: props.size ?? 24, height: props.size ?? 24, borderRadius: (props.size ?? 24) / 2, backgroundColor: props.color, borderWidth: props.selected ? 3 : 1, borderColor: props.selected ? tokens.ring : tokens.border }} />
  );
}
