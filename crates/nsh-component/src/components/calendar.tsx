// A month calendar with single, range and multiple selection, presets,
// disabled days, custom day content and an optional event list.
//
// The visual language is the ReUI calendar: a quiet card, a compact caption,
// 34pt day cells with an 8pt selected radius, a soft range wash and a muted
// weekday row. Selection is controlled when a value/range/values prop is given;
// otherwise the calendar keeps its own state.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { Button } from './button';
import {
  addDays,
  addMonths,
  inRange,
  isWeekend,
  isoDate,
  monthCells,
  monthLabel,
  parseDate,
  sameDay,
  startOfMonth,
  WEEKDAYS,
} from './date';
import {
  alpha,
  isDark,
  type ControlSize,
  RADIUS_LG,
  RADIUS_MD,
  SPACING_SM,
  tokens,
} from './theme';

export type CalendarMode = 'single' | 'range' | 'multiple';
export type CalendarEventTone = 'info' | 'success' | 'warning' | 'danger' | 'primary';

export interface CalendarRange {
  start?: string;
  end?: string;
}

export interface CalendarPreset {
  label: string;
  /** The first day, or the single selected day in `single` mode. */
  start: string;
  /** The end of the range; omitted for a single day preset. */
  end?: string;
}

export interface CalendarEvent {
  date: string;
  title: string;
  time?: string;
  tone?: CalendarEventTone;
}

export interface CalendarDayState {
  date: string;
  selected: boolean;
  rangeStart: boolean;
  rangeEnd: boolean;
  inRange: boolean;
  disabled: boolean;
  outside: boolean;
  today: boolean;
}

export interface CalendarProps {
  mode?: CalendarMode;
  value?: string;
  defaultValue?: string;
  range?: CalendarRange;
  defaultRange?: CalendarRange;
  onRangeChange?: (range: CalendarRange) => void;
  values?: string[];
  defaultValues?: string[];
  onValuesChange?: (values: string[]) => void;
  maxSelected?: number;
  month?: string;
  months?: number;
  size?: ControlSize;
  min?: string;
  max?: string;
  disabledWeekends?: boolean;
  disabledWeekdays?: number[];
  disabledDates?: string[];
  showOutsideDays?: boolean;
  /** A quiet bordered card around the grid. */
  card?: boolean;
  presets?: CalendarPreset[];
  activePreset?: string;
  onPresetSelect?: (preset: CalendarPreset) => void;
  events?: CalendarEvent[];
  dayStyle?: (date: string, state: CalendarDayState) => StyleProp<ViewStyle>;
  dayContent?: (date: string, state: CalendarDayState) => JSX.Element;
  footer?: JSX.Element;
  header?: JSX.Element;
  onSelect?: (date: string) => void;
  onDayPress?: (date: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const eventTone = (tone?: CalendarEventTone): string =>
  tone === 'success'
    ? tokens.success
    : tone === 'warning'
      ? tokens.warning
      : tone === 'danger'
        ? tokens.danger
        : tone === 'primary'
          ? tokens.primary
          : tokens.info;

const longDate = (iso: string): string => {
  const date = parseDate(iso);
  if (!date) return iso;
  const months = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
  return `${months[date.getMonth()]} ${date.getDate()}, ${date.getFullYear()}`;
};

export function Calendar(props: CalendarProps) {
  const mode = () => props.mode ?? 'single';
  const months = () => Math.max(1, props.months ?? 1);
  const initialValue =
    props.month ??
    props.value ??
    props.defaultValue ??
    props.range?.start ??
    props.defaultRange?.start ??
    props.values?.[0] ??
    props.defaultValues?.[0];
  const initial = parseDate(initialValue) ?? new Date();
  const month = signal(startOfMonth(initial));
  const ownValue = signal(props.defaultValue ?? '');
  const ownStart = signal(props.defaultRange?.start ?? '');
  const ownEnd = signal(props.defaultRange?.end ?? '');
  const ownValues = signal<string[]>(props.defaultValues ?? props.values ?? []);

  const single = () => props.value ?? ownValue();
  const rangeStart = () => props.range?.start ?? ownStart();
  const rangeEnd = () => props.range?.end ?? ownEnd();
  const selectedValues = () => props.values ?? ownValues();
  const size = () =>
    props.size === 'small' || props.size === 'xsmall' ? 30 : props.size === 'large' ? 38 : 34;
  const dayHeight = () => (props.dayContent ? 46 : size());

  const disabled = (date: Date) => {
    const iso = isoDate(date);
    return (
      (props.disabledWeekends && isWeekend(date)) ||
      Boolean(props.disabledWeekdays?.includes(date.getDay())) ||
      Boolean(props.disabledDates?.includes(iso)) ||
      !inRange(date, props.min, props.max)
    );
  };

  const isRangeStart = (iso: string) => mode() === 'range' && Boolean(rangeStart()) && iso === rangeStart();
  const isRangeEnd = (iso: string) => mode() === 'range' && Boolean(rangeEnd()) && iso === rangeEnd();
  const isInRange = (iso: string) =>
    mode() === 'range' && Boolean(rangeStart()) && Boolean(rangeEnd()) && iso > rangeStart() && iso < rangeEnd();
  const isSelected = (iso: string) => {
    if (mode() === 'range') return isRangeStart(iso) || isRangeEnd(iso);
    if (mode() === 'multiple') return selectedValues().includes(iso);
    return iso === single();
  };

  const setRange = (start: string, end?: string) => {
    ownStart.set(start);
    ownEnd.set(end ?? '');
    if (!props.range) {
      month.set(startOfMonth(parseDate(start) ?? month()));
    }
    props.onRangeChange?.({ start, end });
  };

  const setValues = (next: string[]) => {
    ownValues.set(next);
    props.onValuesChange?.(next);
  };

  const select = (date: Date, outside: boolean) => {
    const iso = isoDate(date);
    if (disabled(date)) return;
    props.onDayPress?.(iso);
    // Clicking a leading/trailing day pages the visible month.
    if (outside) month.set(startOfMonth(date));
    if (mode() === 'range') {
      const start = rangeStart();
      const end = rangeEnd();
      if (!start || end) {
        setRange(iso);
      } else if (iso < start) {
        setRange(iso, start);
      } else {
        setRange(start, iso);
      }
      return;
    }
    if (mode() === 'multiple') {
      const current = selectedValues();
      if (current.includes(iso)) {
        setValues(current.filter((item) => item !== iso));
        return;
      }
      if (props.maxSelected !== undefined && current.length >= props.maxSelected) return;
      setValues([...current, iso].sort());
      return;
    }
    if (props.value === undefined) ownValue.set(iso);
    if (outside) month.set(startOfMonth(date));
    props.onSelect?.(iso);
  };

  const applyPreset = (preset: CalendarPreset) => {
    props.onPresetSelect?.(preset);
    const start = parseDate(preset.start);
    if (start) month.set(startOfMonth(start));
    if (mode() === 'range') {
      setRange(preset.start, preset.end ?? preset.start);
    } else if (mode() === 'multiple') {
      setValues([preset.start]);
    } else {
      if (props.value === undefined) ownValue.set(preset.start);
      props.onSelect?.(preset.start);
    }
  };

  /** Leading/trailing days are real dates when `showOutsideDays` is on. */
  const cellsFor = (shown: Date): (Date | null)[] => {
    const base = monthCells(shown);
    if (!props.showOutsideDays) return base;
    const first = base.findIndex(Boolean);
    const last = base.length - 1 - [...base].reverse().findIndex(Boolean);
    return base.map((date, index) => {
      if (date) return date;
      if (index < first) return addDays(base[first] as Date, index - first);
      return addDays(base[last] as Date, index - last);
    });
  };

  const eventsOn = (iso: string) => (props.events ?? []).filter((event) => event.date === iso);
  const selectedEventDate = () =>
    mode() === 'range' ? rangeStart() || rangeEnd() : mode() === 'multiple' ? selectedValues()[0] ?? '' : single();
  const selectedEvents = () => eventsOn(selectedEventDate());
  const hasEvents = (iso: string) => eventsOn(iso).length > 0;

  const stateFor = (date: Date, outside: boolean): CalendarDayState => {
    const iso = isoDate(date);
    return {
      date: iso,
      selected: isSelected(iso),
      rangeStart: isRangeStart(iso),
      rangeEnd: isRangeEnd(iso),
      inRange: isInRange(iso),
      disabled: disabled(date),
      outside,
      today: sameDay(date, new Date()),
    };
  };

  const dayCell = (date: Date | null, shown: Date) => {
    if (!date) return <View style={{ flex: 1, height: dayHeight() }} />;
    const outside = date.getMonth() !== shown.getMonth() || date.getFullYear() !== shown.getFullYear();
    const iso = isoDate(date);
    const state = stateFor(date, outside);
    const selected = state.selected;
    const inRange = state.inRange;
    const start = state.rangeStart;
    const end = state.rangeEnd;
    const dim = state.disabled || outside;
    const selectedInk = tokens.primaryForeground;
    const dayBackground = selected
      ? tokens.primary
      : inRange
        ? alpha(tokens.primary, isDark() ? 0.2 : 0.1)
        : undefined;
    return (
      <View
        accessibilityRole="button"
        accessibilityLabel={longDate(iso)}
        accessibilityState={{ selected, disabled: state.disabled }}
        onPress={state.disabled ? undefined : () => select(date, outside)}
        disabled={state.disabled}
        activeOpacity={0.8}
        class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.06]"
        style={[
          {
            flex: 1,
            height: dayHeight(),
            alignItems: 'center',
            justifyContent: 'center',
            borderRadius: RADIUS_MD,
            borderTopStartRadius: start || !inRange ? RADIUS_MD : 0,
            borderBottomStartRadius: start || !inRange ? RADIUS_MD : 0,
            borderTopEndRadius: end || !inRange ? RADIUS_MD : 0,
            borderBottomEndRadius: end || !inRange ? RADIUS_MD : 0,
            backgroundColor: dayBackground,
            opacity: dim ? 0.34 : 1,
          },
          props.dayStyle?.(iso, state),
        ]}
      >
        {props.dayContent ? (
          props.dayContent(iso, state)
        ) : (
          <Text
            style={{
              fontSize: 13,
              fontWeight: selected ? '600' : '400',
              color: selected ? selectedInk : outside ? tokens.mutedForeground : tokens.foreground,
            }}
          >
            {date.getDate()}
          </Text>
        )}
        {!props.dayContent && hasEvents(iso) ? (
          <View
            style={{
              position: 'absolute',
              bottom: 3,
              width: 3,
              height: 3,
              borderRadius: 2,
              backgroundColor: selected ? selectedInk : tokens.primary,
            }}
          />
        ) : null}
      </View>
    );
  };

  const monthColumn = (shown: Date) => (
    <View style={{ width: 270, gap: 4 }}>
      {months() > 1 ? (
        <Text
          style={{
            fontSize: 12,
            fontWeight: '600',
            color: tokens.foreground,
            textAlign: 'center',
            paddingBottom: 2,
          }}
        >
          {monthLabel(shown)}
        </Text>
      ) : null}
      <View style={{ flexDirection: 'row' }}>
        {WEEKDAYS.map((day) => (
          <Text
            style={{
              flex: 1,
              textAlign: 'center',
              fontSize: 11,
              lineHeight: 24,
              color: tokens.mutedForeground,
            }}
          >
            {day}
          </Text>
        ))}
      </View>
      {Array.from({ length: Math.ceil(cellsFor(shown).length / 7) }, (_, week) => (
        <View style={{ flexDirection: 'row' }}>
          {cellsFor(shown)
            .slice(week * 7, week * 7 + 7)
            .map((date) => dayCell(date, shown))}
        </View>
      ))}
    </View>
  );

  const header = () =>
    props.header ? (
      props.header
    ) : (
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 4 }}>
        <Button
          icon="chevron-left"
          variant="ghost"
          size="small"
          accessibilityLabel="Previous month"
          onPress={() => month.set(addMonths(month(), -1))}
        />
        <Text
          style={{
            flex: 1,
            textAlign: 'center',
            fontSize: 14,
            fontWeight: '600',
            color: tokens.foreground,
          }}
        >
          {months() === 1
            ? monthLabel(month())
            : `${monthLabel(month())} – ${monthLabel(addMonths(month(), months() - 1))}`}
        </Text>
        <Button
          icon="chevron-right"
          variant="ghost"
          size="small"
          accessibilityLabel="Next month"
          onPress={() => month.set(addMonths(month(), 1))}
        />
      </View>
    );

  const presetsNode = () =>
    props.presets && props.presets.length > 0 ? (
      <View style={{ width: 116, gap: 2, paddingEnd: 8, borderRightWidth: 1, borderColor: tokens.border }}>
        {props.presets.map((preset) => {
          const active = props.activePreset === preset.label;
          return (
            <View
              accessibilityRole="button"
              accessibilityLabel={preset.label}
              accessibilityState={{ selected: active }}
              onPress={() => applyPreset(preset)}
              activeOpacity={0.8}
              class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.06]"
              style={{
                paddingHorizontal: 8,
                paddingVertical: 7,
                borderRadius: 7,
                backgroundColor: active ? tokens.muted : 'transparent',
              }}
            >
              <Text
                style={{
                  fontSize: 12,
                  lineHeight: 16,
                  fontWeight: active ? '600' : '400',
                  color: active ? tokens.foreground : tokens.mutedForeground,
                }}
              >
                {preset.label}
              </Text>
            </View>
          );
        })}
      </View>
    ) : null;

  const eventsNode = () => {
    const date = selectedEventDate();
    const events = selectedEvents();
    if (!props.events || !date || events.length === 0) return null;
    return (
      <View style={{ gap: 6, paddingTop: SPACING_SM, borderTopWidth: 1, borderColor: tokens.border }}>
        <View style={{ flexDirection: 'row', alignItems: 'center' }}>
          <Text style={{ flex: 1, fontSize: 12, fontWeight: '600', color: tokens.foreground }}>
            {longDate(date)}
          </Text>
          <Icon name="plus" size={13} color={tokens.mutedForeground} />
        </View>
        {events.map((event) => (
          <View
            style={{
              flexDirection: 'row',
              alignItems: 'stretch',
              gap: 8,
              paddingVertical: 5,
              paddingHorizontal: 6,
              borderRadius: 6,
              backgroundColor: alpha(eventTone(event.tone), 0.06),
            }}
          >
            <View style={{ width: 2.5, borderRadius: 2, backgroundColor: eventTone(event.tone) }} />
            <View style={{ flex: 1, gap: 1 }}>
              <Text style={{ fontSize: 12, lineHeight: 16, fontWeight: '500', color: tokens.foreground }}>
                {event.title}
              </Text>
              {event.time ? (
                <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>
                  {event.time}
                </Text>
              ) : null}
            </View>
          </View>
        ))}
      </View>
    );
  };

  return (
    <View
      testID={props.testID}
      style={[
        {
          alignSelf: 'flex-start',
          gap: 10,
          ...(props.card
            ? {
                padding: 12,
                borderRadius: RADIUS_LG,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.popover,
              }
            : {}),
        },
        props.style,
      ]}
    >
      {header()}
      <View style={{ flexDirection: 'row', gap: 12 }}>
        {presetsNode()}
        <View
          style={{
            flexDirection: 'row',
            gap: 12,
            borderRadius: props.showOutsideDays ? 8 : 0,
            overflow: props.showOutsideDays ? 'hidden' : 'visible',
          }}
        >
          {Array.from({ length: months() }, (_, offset) => monthColumn(addMonths(month(), offset)))}
        </View>
      </View>
      {eventsNode()}
      {props.footer ? <View>{props.footer}</View> : null}
    </View>
  );
}

export { isoDate, parseDate };
