// A date field with a calendar in a transparent modal.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Calendar } from './calendar';
import { Icon } from './icon';
import { isoDate, parseDate } from './date';
import { inputHeight, RADIUS_LG, RADIUS_MD, tokens } from './theme';

export interface DatePickerProps {
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  min?: string;
  max?: string;
  months?: number;
  disabled?: boolean;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function DatePicker(props: DatePickerProps) {
  const open = signal(false);
  const own = signal(props.defaultValue ?? '');
  const value = () => props.value ?? own();
  return (
    <View testID={props.testID} style={[{ gap: 4 }, props.style]}>
      <View
        accessibilityRole="button"
        accessibilityLabel={value() || props.placeholder || 'Pick a date'}
        accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
        onPress={() => !props.disabled && open.set(true)}
        disabled={props.disabled}
        style={{
          height: inputHeight(),
          paddingHorizontal: 10,
          flexDirection: 'row',
          alignItems: 'center',
          gap: 8,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: open() ? tokens.ring : tokens.input,
          backgroundColor: tokens.background,
          opacity: props.disabled ? 0.5 : 1,
        }}
      >
        <Text style={{ flex: 1, fontSize: 14, color: value() ? tokens.foreground : tokens.mutedForeground }}>
          {value() || props.placeholder || 'Pick a date'}
        </Text>
        <Icon name="calendar" size={14} color={tokens.mutedForeground} />
      </View>
      <Modal visible={open()} transparent animationType="fade" onRequestClose={() => open.set(false)}>
        <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
          <View class="shadow-2xl" style={{ width: 380, maxWidth: '100%', padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: RADIUS_LG, backgroundColor: tokens.popover }}>
            <Calendar
              value={value()}
              min={props.min}
              max={props.max}
              months={props.months}
              onSelect={(date) => {
                if (props.value === undefined) own.set(date);
                props.onChange?.(date);
                open.set(false);
              }}
            />
          </View>
        </View>
      </Modal>
    </View>
  );
}

export function DateField(props: { value?: string; onChange?: (value: string) => void; label?: string }) {
  return <DatePicker value={props.value} onChange={props.onChange} placeholder={props.label ?? 'Select a date'} />;
}

export { isoDate, parseDate };
