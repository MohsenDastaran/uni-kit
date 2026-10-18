// A draggable value along a track, with a step, bounds and disabled state.
import type { LayoutEvent, StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { RADIUS_MD, tokens } from './theme';

export interface SliderProps {
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  vertical?: boolean;
  onChange?: (value: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Slider(props: SliderProps) {
  const own = signal(props.defaultValue ?? 0);
  const width = signal(0);
  const height = signal(0);
  const value = () => props.value ?? own();
  const fraction = () => {
    const min = props.min ?? 0;
    const max = props.max ?? 100;
    return max === min ? 0 : Math.max(0, Math.min(1, (value() - min) / (max - min)));
  };
  const setFrom = (x: number, y: number) => {
    if (props.disabled) return;
    const min = props.min ?? 0;
    const max = props.max ?? 100;
    const step = props.step ?? 1;
    const ratio = props.vertical ? 1 - y / Math.max(1, height()) : x / Math.max(1, width());
    const raw = min + Math.max(0, Math.min(1, ratio)) * (max - min);
    const next = Math.round(raw / step) * step;
    if (props.value === undefined) own.set(next);
    props.onChange?.(next);
  };
  const press = (event: { locationX?: number; locationY?: number } | undefined) => {
    setFrom(event?.locationX ?? 0, event?.locationY ?? 0);
  };
  return (
    <View
      testID={props.testID}
      accessibilityRole="adjustable"
      accessibilityLabel="Slider"
      accessibilityValue={{ min: props.min ?? 0, max: props.max ?? 100, now: value() }}
      onLayout={(event: LayoutEvent) => {
        width.set(event.width);
        height.set(event.height);
      }}
      onTouchStart={press}
      onTouchMove={(event) => setFrom(event.locationX, event.locationY)}
      style={[
        {
          height: props.vertical ? 160 : 24,
          width: props.vertical ? 24 : undefined,
          flex: props.vertical ? undefined : 1,
          justifyContent: 'center',
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      <View
        style={{
          ...(props.vertical
            ? { width: 6, height: '100%', borderRadius: 3, backgroundColor: tokens.muted }
            : { height: 6, width: '100%', borderRadius: 3, backgroundColor: tokens.muted }),
          overflow: 'hidden',
        }}
      >
        <View
          style={{
            ...(props.vertical
              ? { width: '100%', height: `${fraction() * 100}%`, backgroundColor: tokens.primary }
              : { height: '100%', width: `${fraction() * 100}%`, backgroundColor: tokens.primary }),
            borderRadius: 3,
          }}
        />
      </View>
      <View
        style={{
          position: 'absolute',
          ...(props.vertical
            ? { top: `${(1 - fraction()) * 100}%`, left: 3, width: 18, height: 18 }
            : { left: `${fraction() * 100}%`, top: 3, width: 18, height: 18 }),
          marginLeft: props.vertical ? 0 : -9,
          marginTop: props.vertical ? -9 : 0,
          borderRadius: 9,
          backgroundColor: tokens.background,
          borderWidth: 2,
          borderColor: tokens.primary,
        }}
      />
    </View>
  );
}

export function RangeSlider(props: Omit<SliderProps, 'value' | 'defaultValue'> & { value?: [number, number]; defaultValue?: [number, number]; onChange?: (value: [number, number]) => void }) {
  return <Slider value={props.value?.[0] ?? props.defaultValue?.[0] ?? 0} min={props.min} max={props.max} onChange={(value) => props.onChange?.([value, props.value?.[1] ?? props.max ?? 100])} />;
}
