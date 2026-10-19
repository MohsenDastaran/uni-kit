// A date field with a calendar. The calendar can open in a modal (the
// default), as an inline panel under the trigger, or anchored as a popover.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Calendar, type CalendarEvent, type CalendarPreset } from './calendar';
import { Icon } from './icon';
import { isoDate, parseDate } from './date';
import { inputHeight, type ControlSize, RADIUS_LG, RADIUS_MD, tokens } from './theme';

export type DatePickerPresentation = 'modal' | 'popover' | 'inline';

export interface DatePickerProps {
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  min?: string;
  max?: string;
  months?: number;
  size?: ControlSize;
  disabled?: boolean;
  /** How the calendar is presented. Defaults to the modal. */
  presentation?: DatePickerPresentation;
  /** Start a popover or inline sample open. */
  defaultOpen?: boolean;
  /** A wider field/panel. */
  width?: number;
  /** Preset ranges or dates inside the calendar. */
  presets?: CalendarPreset[];
  activePreset?: string;
  onPresetSelect?: (preset: CalendarPreset) => void;
  /** Disabled days carried through to the Calendar. */
  disabledWeekends?: boolean;
  disabledWeekdays?: number[];
  disabledDates?: string[];
  showOutsideDays?: boolean;
  /** Event dots on the calendar. */
  events?: CalendarEvent[];
  /** A footer under the calendar. */
  footer?: JSX.Element;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
const display = (iso: string): string => {
  const date = parseDate(iso);
  if (!date) return iso;
  return `${MONTHS[date.getMonth()]} ${date.getDate()}, ${date.getFullYear()}`;
};

export function DatePicker(props: DatePickerProps) {
  const open = signal(Boolean(props.defaultOpen));
  const own = signal(props.defaultValue ?? '');
  const value = () => props.value ?? own();
  const presentation = () => props.presentation ?? 'modal';
  const width = () => props.width ?? (props.months && props.months > 1 ? 590 : props.presets ? 430 : 330);
  const select = (date: string) => {
    if (props.value === undefined) own.set(date);
    props.onChange?.(date);
    if (presentation() !== 'inline') open.set(false);
  };
  const calendar = () => (
    <Calendar
      value={value()}
      min={props.min}
      max={props.max}
      months={props.months}
      presets={props.presets}
      activePreset={props.activePreset}
      onPresetSelect={props.onPresetSelect}
      disabledWeekends={props.disabledWeekends}
      disabledWeekdays={props.disabledWeekdays}
      disabledDates={props.disabledDates}
      showOutsideDays={props.showOutsideDays}
      events={props.events}
      footer={props.footer}
      onSelect={select}
    />
  );
  const panel = (
    <View
      class="shadow-2xl"
      style={{
        width: width(),
        maxWidth: '100%',
        padding: 12,
        borderWidth: 1,
        borderColor: tokens.border,
        borderRadius: RADIUS_LG,
        backgroundColor: tokens.popover,
      }}
    >
      {calendar()}
    </View>
  );
  const trigger = (
    <View
      accessibilityRole="button"
      accessibilityLabel={value() || props.placeholder || 'Pick a date'}
      accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
      onPress={() => !props.disabled && open.set(!open())}
      disabled={props.disabled}
      activeOpacity={0.9}
      class="transition-colors duration-150 hover:bg-black/[0.02] dark:hover:bg-white/[0.04]"
      style={{
        height: inputHeight(props.size ?? 'medium'),
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
      <Text style={{ flex: 1, fontSize: 13, color: value() ? tokens.foreground : tokens.mutedForeground }}>
        {value() ? display(value()) : props.placeholder || 'Pick a date'}
      </Text>
      <Icon name="calendar" size={14} color={tokens.mutedForeground} />
    </View>
  );

  if (presentation() === 'inline') {
    return (
      <View testID={props.testID} style={[{ gap: 6, width: props.width, maxWidth: '100%' }, props.style]}>
        {trigger}
        {open() ? panel : null}
      </View>
    );
  }

  if (presentation() === 'popover') {
    return (
      <View testID={props.testID} style={[{ position: 'relative', width: width(), maxWidth: '100%' }, props.style]}>
        {trigger}
        {open() ? (
          <View style={{ position: 'absolute', top: inputHeight(props.size ?? 'medium') + 6, left: 0, zIndex: 50 }}>
            {panel}
          </View>
        ) : null}
      </View>
    );
  }

  return (
    <View testID={props.testID} style={[{ gap: 4, width: props.width, maxWidth: '100%' }, props.style]}>
      {trigger}
      <Modal visible={open()} transparent animationType="fade" onRequestClose={() => open.set(false)}>
        <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
          {panel}
        </View>
      </Modal>
    </View>
  );
}

export function DateField(props: { value?: string; onChange?: (value: string) => void; label?: string }) {
  return <DatePicker value={props.value} onChange={props.onChange} placeholder={props.label ?? 'Select a date'} />;
}

export { isoDate, parseDate };
