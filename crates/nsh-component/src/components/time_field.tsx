// A segmented time-of-day field with an inline column picker or an outline
// trigger that opens the columns in a popover.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Scroll, Text, View } from 'nsh';
import { Icon } from './icon';
import { type ControlSize, inputHeight, RADIUS_MD, tokens } from './theme';
import { pad2 } from './date';

export type TimeFieldVariant = 'columns' | 'popover';

export interface TimeFieldProps {
  value?: string;
  defaultValue?: string;
  seconds?: boolean;
  hour12?: boolean;
  size?: ControlSize;
  disabled?: boolean;
  invalid?: boolean;
  variant?: TimeFieldVariant;
  /** Minute interval in the list; the current minute is always included. */
  minuteStep?: number;
  /** Show Now / Clear / OK under the columns. */
  showActions?: boolean;
  label?: string;
  description?: string;
  error?: string;
  placeholder?: string;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  onChange?: (value: string) => void;
  onClear?: () => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function partsOf(value: string): number[] {
  const parts = value.split(':').map((part) => Number(part));
  return [Number.isFinite(parts[0]) ? parts[0] : 0, Number.isFinite(parts[1]) ? parts[1] : 0, Number.isFinite(parts[2]) ? parts[2] : 0];
}

const format = (parts: number[], seconds: boolean) =>
  `${pad2(parts[0] % 24)}:${pad2(parts[1] % 60)}${seconds ? `:${pad2(parts[2] % 60)}` : ''}`;

const clockLabel = (parts: number[], hour12: boolean, seconds: boolean) => {
  const hour = parts[0] % 24;
  if (!hour12) return format(parts, seconds);
  const shown = hour % 12 === 0 ? 12 : hour % 12;
  return `${pad2(shown)}:${pad2(parts[1])}${seconds ? `:${pad2(parts[2])}` : ''} ${hour < 12 ? 'AM' : 'PM'}`;
};

export function TimeField(props: TimeFieldProps) {
  const own = signal(props.defaultValue && props.defaultValue !== '' ? props.defaultValue : '');
  const opened = signal(Boolean(props.open));
  const value = () => props.value ?? own();
  const isOpen = () => props.open ?? opened();
  const parts = () => partsOf(value() || '00:00');
  const size = () => props.size ?? 'medium';
  const variant = () => props.variant ?? 'columns';
  const hour12 = () => Boolean(props.hour12);
  const seconds = () => Boolean(props.seconds);
  const step = () => Math.max(1, Math.min(30, props.minuteStep ?? 1));
  const commit = (next: number[]) => {
    const formatted = format(next, seconds());
    if (props.value === undefined) own.set(formatted);
    props.onChange?.(formatted);
  };
  const setPart = (index: number, next: number) => {
    const values = parts();
    values[index] = next;
    commit(values);
  };
  const close = () => {
    if (props.open === undefined) opened.set(false);
    props.onOpenChange?.(false);
  };
  const open = () => {
    if (props.disabled) return;
    if (props.open === undefined) opened.set(true);
    props.onOpenChange?.(true);
  };
  const hourOptions = () => {
    if (!hour12()) return Array.from({ length: 24 }, (_, index) => index);
    return [12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  };
  const minuteOptions = () => {
    const selected = parts()[1];
    const values = Array.from({ length: Math.ceil(60 / step()) }, (_, index) => index * step());
    return values.includes(selected) ? values : [...values, selected].sort((a, b) => a - b);
  };
  const secondOptions = () => {
    const selected = parts()[2];
    const values = Array.from({ length: Math.ceil(60 / step()) }, (_, index) => index * step());
    return values.includes(selected) ? values : [...values, selected].sort((a, b) => a - b);
  };
  const hourLabel = (hour: number) => {
    if (!hour12()) return pad2(hour);
    const shown = hour % 12 === 0 ? 12 : hour % 12;
    return pad2(shown);
  };

  const column = (title: string, values: number[], selected: number, onSelect: (value: number) => void, label: (value: number) => string) => (
    <View style={{ flex: 1, minWidth: 58, alignItems: 'stretch', gap: 4 }}>
      <Text style={{ fontSize: 9, fontWeight: '500', color: tokens.mutedForeground, textAlign: 'center', paddingBottom: 2 }}>{title}</Text>
      <Scroll showsIndicator={false} contentOffset={{ y: Math.max(0, values.indexOf(selected) * 27 - 54) }} style={{ height: 150 }}>
        <View style={{ gap: 1, paddingHorizontal: 2 }}>
          {values.map((item) => {
            const active = item === selected;
            return (
              <View
                accessibilityRole="button"
                accessibilityLabel={`${title} ${label(item)}`}
                accessibilityState={{ selected: active }}
                onPress={() => { if (!props.disabled) onSelect(item); }}
                style={{ borderRadius: 7, paddingVertical: 6, alignItems: 'center', backgroundColor: active ? tokens.foreground : 'transparent', opacity: props.disabled ? 0.6 : 1 }}
              >
                <Text style={{ fontSize: 12, color: active ? tokens.background : tokens.foreground }}>{label(item)}</Text>
              </View>
            );
          })}
        </View>
      </Scroll>
    </View>
  );

  const columns = (
    <View style={{ gap: 0 }}>
      <View style={{ flexDirection: 'row', gap: 2, paddingHorizontal: 8, paddingTop: 8 }}>
        {column('Hour', hourOptions(), hour12() ? (parts()[0] % 12 === 0 ? 12 : parts()[0] % 12) : parts()[0], (next) => {
          const values = parts();
          const hour = hour12() ? (next === 12 ? (values[0] >= 12 ? 12 : 0) : (values[0] >= 12 ? 12 + next : next)) : next;
          setPart(0, hour);
        }, hourLabel)}
        {column('Minute', minuteOptions(), parts()[1], (next) => setPart(1, next), pad2)}
        {seconds() ? column('Second', secondOptions(), parts()[2], (next) => setPart(2, next), pad2) : null}
        {hour12() ? (
          <View style={{ width: 54, gap: 4 }}>
            <Text style={{ fontSize: 9, fontWeight: '500', color: tokens.mutedForeground, textAlign: 'center', paddingBottom: 2 }}>AM/PM</Text>
            <View style={{ gap: 3, paddingTop: 2 }}>
              {['AM', 'PM'].map((period) => {
                const active = (parts()[0] < 12 ? 'AM' : 'PM') === period;
                return (
                  <View
                    accessibilityRole="button"
                    accessibilityLabel={period}
                    accessibilityState={{ selected: active }}
                    onPress={() => {
                      if (props.disabled) return;
                      const values = parts();
                      const isPm = values[0] >= 12;
                      if (period === 'AM' && isPm) setPart(0, values[0] - 12);
                      if (period === 'PM' && !isPm) setPart(0, values[0] + 12);
                    }}
                    style={{ borderRadius: 7, paddingVertical: 6, alignItems: 'center', backgroundColor: active ? tokens.foreground : 'transparent', opacity: props.disabled ? 0.6 : 1 }}
                  >
                    <Text style={{ fontSize: 11, fontWeight: active ? '600' : '400', color: active ? tokens.background : tokens.foreground }}>{period}</Text>
                  </View>
                );
              })}
            </View>
          </View>
        ) : null}
      </View>
      {props.showActions === false ? null : (
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6, padding: 8, borderTopWidth: 1, borderTopColor: tokens.border }}>
          <View accessibilityRole="button" accessibilityLabel="Now" onPress={() => {
            if (props.disabled) return;
            const now = new Date();
            commit([now.getHours(), now.getMinutes(), now.getSeconds()]);
          }} style={{ paddingHorizontal: 6, paddingVertical: 5 }}>
            <Text style={{ fontSize: 11, fontWeight: '500', color: tokens.foreground }}>Now</Text>
          </View>
          <View accessibilityRole="button" accessibilityLabel="Clear" onPress={() => {
            if (props.disabled) return;
            if (props.value === undefined) own.set('');
            props.onClear?.();
            props.onChange?.('');
            close();
          }} style={{ paddingHorizontal: 6, paddingVertical: 5 }}>
            <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Clear</Text>
          </View>
          <View style={{ flex: 1 }} />
          <View accessibilityRole="button" accessibilityLabel="OK" onPress={close} style={{ paddingHorizontal: 8, paddingVertical: 5 }}>
            <Text style={{ fontSize: 11, fontWeight: '600', color: tokens.foreground }}>OK</Text>
          </View>
        </View>
      )}
    </View>
  );

  const trigger = (
    <View
      accessibilityRole="combobox"
      accessibilityLabel={props.label ?? props.placeholder ?? 'Select time'}
      accessibilityState={{ expanded: isOpen(), disabled: Boolean(props.disabled) }}
      onPress={() => (isOpen() ? close() : open())}
      disabled={props.disabled}
      activeOpacity={0.9}
      style={{
        height: inputHeight(size()),
        paddingHorizontal: 11,
        flexDirection: 'row',
        alignItems: 'center',
        gap: 8,
        borderRadius: RADIUS_MD,
        borderWidth: 1,
        borderColor: props.invalid ? tokens.danger : isOpen() ? tokens.ring : tokens.input,
        backgroundColor: props.disabled ? tokens.muted : tokens.background,
        opacity: props.disabled ? 0.5 : 1,
      }}
    >
      <Text style={{ flex: 1, fontSize: 12, color: value() ? tokens.foreground : tokens.mutedForeground }}>{value() ? clockLabel(parts(), hour12(), seconds()) : props.placeholder ?? 'Select time'}</Text>
      <Icon name="clock" size={14} color={tokens.mutedForeground} />
    </View>
  );

  const body = variant() === 'popover' ? trigger : null;
  const panel = (
    <View
      style={{
        borderWidth: 1,
        borderColor: props.invalid ? tokens.danger : tokens.border,
        borderRadius: 10,
        backgroundColor: tokens.popover,
        overflow: 'hidden',
        width: 300,
        opacity: props.disabled ? 0.65 : 1,
      }}
    >
      {columns}
    </View>
  );
  return (
    <View testID={props.testID} style={[{ width: variant() === 'popover' ? 300 : 300, maxWidth: '100%', gap: 6, position: 'relative' }, props.style]}>
      {props.label ? (
        <Text style={{ fontSize: 11, fontWeight: '500', color: tokens.foreground }}>
          {props.label}
        </Text>
      ) : null}
      {body}
      {variant() === 'columns' ? panel : isOpen() ? <View accessibilityRole="menu" class="absolute z-50" style={{ position: 'absolute', top: inputHeight(size()) + (props.label ? 24 : 4), left: 0, width: 300 }}>{panel}</View> : null}
      {props.error ? <Text style={{ fontSize: 10, color: tokens.danger }}>{props.error}</Text> : props.description ? <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{props.description}</Text> : null}
    </View>
  );
}

export function TimeFieldValue(props: { value: string; style?: StyleProp<ViewStyle> }) {
  return <TimeField value={props.value} style={props.style} />;
}
