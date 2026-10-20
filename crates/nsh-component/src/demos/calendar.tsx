//! `calendar` — month grid, range/multiple selection, presets, events and
//! the compositions that sit around a date grid.
import { signal, Text, View } from "nsh";
import {
  Calendar,
  type CalendarEvent,
  type CalendarPreset,
  type CalendarRange,
} from "../components/calendar";
import { Button } from "../components/button";
import { DatePicker } from "../components/date_picker";
import { Icon } from "../components/icon";
import { alpha, tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row, Status } from "./ui";

const presets: CalendarPreset[] = [
  { label: "Today", start: "2026-10-19", end: "2026-10-19" },
  { label: "Yesterday", start: "2026-10-18", end: "2026-10-18" },
  { label: "Last 7 days", start: "2026-10-13", end: "2026-10-19" },
  { label: "Last week", start: "2026-10-12", end: "2026-10-18" },
  { label: "Last month", start: "2026-09-01", end: "2026-09-30" },
  { label: "Last year", start: "2025-01-01", end: "2025-12-31" },
];

const timeSlots = ["13:00", "13:15", "13:30", "13:45", "14:00", "14:15", "14:30", "14:45"];

const priceOf = (day: number) => `$${89 + ((day * 7) % 60)}`;

const calendarEvents: CalendarEvent[] = [
  { date: "2026-10-19", title: "Product Launch", time: "Oct 19, 10:00 – 11:30", tone: "success" },
  { date: "2026-10-19", title: "Weekly Standup", time: "Oct 19, 13:00 – 13:30", tone: "warning" },
  { date: "2026-10-19", title: "Code Review Session", time: "Oct 19, 15:00 – 16:00", tone: "primary" },
  { date: "2026-10-23", title: "Retro", time: "Oct 23, 16:00 – 17:00", tone: "info" },
];

export function Demo() {
  const basicDate = signal("2026-10-19");
  const range = signal<CalendarRange>({ start: "2026-10-13", end: "2026-10-19" });
  const multiple = signal(["2026-10-02", "2026-10-13", "2026-10-19"]);
  const presetRange = signal<CalendarRange>({ start: "2026-10-13", end: "2026-10-19" });
  const activePreset = signal("Last 7 days");
  const appointmentDate = signal("2026-10-19");
  const appointmentTime = signal("13:45");
  const eventDate = signal("2026-10-19");
  const footerDate = signal("2026-10-19");

  // section: Usage
  const usage = (
    <Column gap={8}>
      <Calendar
        card
        value={basicDate()}
        onSelect={(next) => basicDate.set(next)}
        showOutsideDays
        style={{ maxWidth: 300 }}
      />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
        Selected: {basicDate()}
      </Text>
    </Column>
  );

  // section: Basic calendar
  const basic = (
    <Calendar card value={basicDate()} onSelect={(next) => basicDate.set(next)} showOutsideDays />
  );

  // section: Range calendar
  const rangeCalendar = (
    <Column gap={8}>
      <Calendar
        card
        mode="range"
        range={range()}
        onRangeChange={(next) => range.set(next)}
        onDayPress={() => {}}
      />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
        {range().start} → {range().end ?? "pick an end date"}
      </Text>
    </Column>
  );

  // section: Multiple day selection
  const multipleCalendar = (
    <Column gap={8}>
      <Calendar
        card
        mode="multiple"
        values={multiple()}
        onValuesChange={(next) => multiple.set(next)}
        maxSelected={4}
        showOutsideDays
      />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
        {multiple().length} dates selected
      </Text>
    </Column>
  );

  // section: Disabled dates
  const disabledCalendar = (
    <Calendar
      card
      value={basicDate()}
      onSelect={(next) => basicDate.set(next)}
      disabledWeekends
      disabledWeekdays={[3]}
      disabledDates={["2026-10-14", "2026-10-22"]}
      showOutsideDays
    />
  );

  // section: Display 2 months
  const twoMonths = (
    <Calendar
      card
      months={2}
      mode="range"
      range={range()}
      onRangeChange={(next) => range.set(next)}
      showOutsideDays
    />
  );

  // section: Calendar with presets
  const presetCalendar = (
    <Calendar
      card
      mode="range"
      range={presetRange()}
      onRangeChange={(next) => presetRange.set(next)}
      onDayPress={() => activePreset.set("")}
      presets={presets}
      activePreset={activePreset()}
      onPresetSelect={(preset) => activePreset.set(preset.label)}
      style={{ maxWidth: 430 }}
    />
  );

  // section: Range calendar with presets
  const rangePresets = (
    <Column gap={8}>
      <Calendar
        card
        mode="range"
        range={presetRange()}
        onRangeChange={(next) => presetRange.set(next)}
        onDayPress={() => activePreset.set("")}
        presets={presets}
        activePreset={activePreset()}
        onPresetSelect={(preset) => activePreset.set(preset.label)}
      />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
        Active preset: {activePreset() || "Custom range"}
      </Text>
    </Column>
  );

  // section: Appointment calendar
  const appointment = (
    <View
      style={{
        width: 430,
        maxWidth: "100%",
        gap: 10,
        padding: 12,
        borderWidth: 1,
        borderColor: tokens.border,
        borderRadius: 12,
        backgroundColor: tokens.popover,
      }}
    >
      <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>
        Book your appointment
      </Text>
      <View style={{ flexDirection: "row", gap: 12 }}>
        <Calendar
          value={appointmentDate()}
          onSelect={(next) => appointmentDate.set(next)}
          showOutsideDays
        />
        <View style={{ width: 116, gap: 4, paddingTop: 32 }}>
          {timeSlots.map((slot) => (
            <Button
              label={slot}
              size="small"
              variant={appointmentTime() === slot ? "primary" : "secondary"}
              onPress={() => appointmentTime.set(slot)}
              style={{ width: "100%" }}
            />
          ))}
        </View>
      </View>
      <View
        style={{
          flexDirection: "row",
          alignItems: "center",
          gap: 8,
          paddingTop: 8,
          borderTopWidth: 1,
          borderColor: tokens.border,
        }}
      >
        <Icon name="clock" size={14} color={tokens.mutedForeground} />
        <Text style={{ flex: 1, fontSize: 11, color: tokens.mutedForeground }}>
          Monday, October 19 at {appointmentTime()}
        </Text>
        <Button label="Confirm" size="small" onPress={() => {}} />
      </View>
    </View>
  );

  // section: Price calendar
  const priceCalendar = (
    <Calendar
      card
      value={basicDate()}
      onSelect={(next) => basicDate.set(next)}
      dayContent={(iso, state) => {
        const day = Number(iso.slice(8));
        return (
          <View style={{ alignItems: "center", gap: 0 }}>
            <Text
              style={{
                fontSize: 12,
                fontWeight: state.selected ? "600" : "400",
                color: state.selected ? tokens.primaryForeground : tokens.foreground,
              }}
            >
              {day}
            </Text>
            <Text
              style={{
                fontSize: 9,
                lineHeight: 11,
                color: state.selected ? tokens.primaryForeground : tokens.mutedForeground,
              }}
            >
              {priceOf(day)}
            </Text>
          </View>
        );
      }}
    />
  );

  // section: Calendar with event list
  const eventCalendar = (
    <Calendar
      card
      value={eventDate()}
      onSelect={(next) => eventDate.set(next)}
      events={calendarEvents}
      style={{ maxWidth: 330 }}
    />
  );

  // section: Footer presets
  const footerPresets = (
    <Calendar
      card
      value={footerDate()}
      onSelect={(next) => footerDate.set(next)}
      footer={
        <View style={{ flexDirection: "row", flexWrap: "wrap", gap: 4 }}>
          <Button label="Today" size="small" variant="ghost" onPress={() => footerDate.set("2026-10-19")} />
          <Button label="Tomorrow" size="small" variant="ghost" onPress={() => footerDate.set("2026-10-20")} />
          <Button label="3 days" size="small" variant="ghost" onPress={() => footerDate.set("2026-10-22")} />
          <Button label="Week" size="small" variant="ghost" onPress={() => footerDate.set("2026-10-25")} />
          <Button label="2 weeks" size="small" variant="ghost" onPress={() => footerDate.set("2026-11-02")} />
        </View>
      }
    />
  );

  // section: Custom day styling
  const customStyle = (
    <Calendar
      card
      value={basicDate()}
      onSelect={(next) => basicDate.set(next)}
      dayStyle={(iso, state) => {
        const weekday = new Date(`${iso}T00:00:00`).getDay();
        return !state.selected && (weekday === 0 || weekday === 6)
          ? { backgroundColor: alpha(tokens.warning, 0.1) }
          : {};
      }}
    />
  );

  // section: Date picker
  const datePicker = <DatePicker defaultValue="2026-10-19" placeholder="Pick a date" />;
  // section: end

  return (
    <Page>
      <Caption>
        The calendar is the date grid. Presets, time slots, event details and the card around it
        are compositions; the app still owns the selected dates.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic calendar">{basic}</ExampleSection>
      <ExampleSection index={2} title="Range calendar">{rangeCalendar}</ExampleSection>
      <ExampleSection index={3} title="Multiple day selection">{multipleCalendar}</ExampleSection>
      <ExampleSection index={4} title="Disabled dates">{disabledCalendar}</ExampleSection>
      <ExampleSection index={5} title="Display 2 months">{twoMonths}</ExampleSection>
      <ExampleSection index={6} title="Calendar with presets">{presetCalendar}</ExampleSection>
      <ExampleSection index={7} title="Range calendar with presets">{rangePresets}</ExampleSection>
      <ExampleSection index={8} title="Appointment calendar">{appointment}</ExampleSection>
      <ExampleSection index={9} title="Price calendar">{priceCalendar}</ExampleSection>
      <ExampleSection index={10} title="Calendar with event list">{eventCalendar}</ExampleSection>
      <ExampleSection index={11} title="Footer presets">{footerPresets}</ExampleSection>
      <ExampleSection index={12} title="Custom day styling">{customStyle}</ExampleSection>
      <ExampleSection index={13} title="Date picker">{datePicker}</ExampleSection>
      <Status>Selected: {basicDate()} · range {range().start} to {range().end ?? "open"}</Status>
    </Page>
  );
}
