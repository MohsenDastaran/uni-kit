// A month calendar. One or more months, a selected date and disabled days.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { Button } from './button';
import {
  addMonths,
  daysInMonth,
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
import { type ControlSize, RADIUS_MD, SPACING_SM, tokens } from './theme';

export interface CalendarProps {
  value?: string;
  defaultValue?: string;
  month?: string;
  months?: number;
  size?: ControlSize;
  min?: string;
  max?: string;
  disabledWeekends?: boolean;
  onSelect?: (date: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Calendar(props: CalendarProps) {
  const initial = parseDate(props.month ?? props.value ?? props.defaultValue) ?? new Date();
  const month = signal(startOfMonth(initial));
  const value = () => parseDate(props.value ?? props.defaultValue);
  const size = () => (props.size === 'small' ? 28 : props.size === 'large' ? 40 : 34);
  const disabled = (date: Date) =>
    (props.disabledWeekends && isWeekend(date)) || !inRange(date, props.min, props.max);
  return (
    <View testID={props.testID} style={[{ gap: SPACING_SM }, props.style]}>
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
        <Button icon="chevron-left" variant="ghost" size="small" accessibilityLabel="Previous month" onPress={() => month.set(addMonths(month(), -1))} />
        <Text style={{ flex: 1, textAlign: 'center', fontSize: 14, fontWeight: '600', color: tokens.foreground }}>
          {monthLabel(month())}
        </Text>
        <Button icon="chevron-right" variant="ghost" size="small" accessibilityLabel="Next month" onPress={() => month.set(addMonths(month(), 1))} />
      </View>
      <View style={{ flexDirection: 'row', gap: 12 }}>
        {Array.from({ length: Math.max(1, props.months ?? 1) }, (_, offset) => {
          const shown = addMonths(month(), offset);
          return (
            <View style={{ flex: 1, gap: 4 }}>
              <Text style={{ fontSize: 12, fontWeight: '500', color: tokens.mutedForeground, textAlign: 'center' }}>
                {monthLabel(shown)}
              </Text>
              <View style={{ flexDirection: 'row' }}>
                {WEEKDAYS.map((day) => (
                  <Text style={{ flex: 1, textAlign: 'center', fontSize: 11, color: tokens.mutedForeground }}>{day}</Text>
                ))}
              </View>
              {Array.from({ length: Math.ceil(monthCells(shown).length / 7) }, (_, week) => (
                <View style={{ flexDirection: 'row' }}>
                  {monthCells(shown).slice(week * 7, week * 7 + 7).map((date, index) => (
                    <View
                      accessibilityRole="button"
                      accessibilityLabel={date ? isoDate(date) : 'Empty'}
                      accessibilityState={{ selected: sameDay(date ?? undefined, value()), disabled: date ? disabled(date) : true }}
                      onPress={date && !disabled(date) ? () => props.onSelect?.(isoDate(date)) : undefined}
                      disabled={!date || disabled(date)}
                      activeOpacity={0.8}
                      style={{
                        flex: 1,
                        height: size(),
                        alignItems: 'center',
                        justifyContent: 'center',
                        borderRadius: RADIUS_MD,
                        backgroundColor: date && sameDay(date, value()) ? tokens.primary : 'transparent',
                        opacity: date && !disabled(date) ? 1 : 0.3,
                      }}
                    >
                      {date ? (
                        <Text style={{ fontSize: 13, color: date && sameDay(date, value()) ? tokens.primaryForeground : tokens.foreground }}>
                          {date.getDate()}
                        </Text>
                      ) : null}
                    </View>
                  ))}
                </View>
              ))}
            </View>
          );
        })}
      </View>
    </View>
  );
}

export { isoDate, parseDate, daysInMonth };
