//! `date-picker` — a date field with a modal, popover or inline calendar,
//! plus the ReUI filter and range compositions.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import {
  Calendar,
  type CalendarEvent,
  type CalendarPreset,
  type CalendarRange,
} from "../components/calendar";
import { DatePicker } from "../components/date_picker";
import { Dialog } from "../components/dialog";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row, Status } from "./ui";

const presets: CalendarPreset[] = [
  { label: "Today", start: "2026-10-19", end: "2026-10-19" },
  { label: "Yesterday", start: "2026-10-18", end: "2026-10-18" },
  { label: "Last 7 days", start: "2026-10-13", end: "2026-10-19" },
  { label: "This month", start: "2026-10-01", end: "2026-10-31" },
  { label: "This quarter", start: "2026-10-01", end: "2026-12-31" },
];

const events: CalendarEvent[] = [
  { date: "2026-10-19", title: "Product Launch", time: "10:00 – 11:30", tone: "success" },
  { date: "2026-10-19", title: "Weekly Standup", time: "13:00 – 13:30", tone: "warning" },
  { date: "2026-10-21", title: "Design review", time: "15:00 – 16:00", tone: "info" },
];

function RangeField(props: {
  range: () => CalendarRange;
  onChange: (next: CalendarRange) => void;
  placeholder?: string;
  width?: number;
}) {
  const open = signal(false);
  const label = () => {
    const range = props.range();
    return range.start ? `${range.start}${range.end ? ` → ${range.end}` : ""}` : props.placeholder ?? "Select a range";
  };
  return (
    <View style={{ position: "relative", width: props.width ?? 340 }}>
      <View
        accessibilityRole="button"
        accessibilityLabel={label()}
        accessibilityState={{ expanded: open() }}
        onPress={() => open.set(!open())}
        activeOpacity={0.9}
        style={{
          height: 32,
          paddingHorizontal: 10,
          flexDirection: "row",
          alignItems: "center",
          gap: 8,
          borderRadius: 8,
          borderWidth: 1,
          borderColor: open() ? tokens.ring : tokens.input,
          backgroundColor: tokens.background,
        }}
      >
        <Icon name="calendar" size={14} color={tokens.mutedForeground} />
        <Text style={{ flex: 1, fontSize: 12, color: props.range().start ? tokens.foreground : tokens.mutedForeground }}>
          {label()}
        </Text>
        <Icon name="chevron-down" size={13} color={tokens.mutedForeground} />
      </View>
      {open() ? (
        <View
          style={{
            position: "absolute",
            top: 38,
            left: 0,
            zIndex: 30,
            padding: 12,
            borderRadius: 12,
            borderWidth: 1,
            borderColor: tokens.border,
            backgroundColor: tokens.popover,
          }}
        >
          <Calendar
            mode="range"
            range={props.range()}
            onRangeChange={(next) => {
              props.onChange(next);
              if (next.end) open.set(false);
            }}
            showOutsideDays
          />
        </View>
      ) : null}
    </View>
  );
}

export function Demo() {
  const date = signal("2026-10-18");
  const initial = signal("2026-10-19");
  const range = signal<CalendarRange>({ start: "2026-10-13", end: "2026-10-19" });
  const presetRange = signal<CalendarRange>({ start: "2026-10-13", end: "2026-10-19" });
  const activePreset = signal("Last 7 days");
  const rangeOpen = signal(false);
  const operator = signal("is");
  const granularity = signal("Day");
  const filterRange = signal<CalendarRange>({ start: "2026-10-19", end: "2026-10-24" });
  const time = signal("13:45");
  const chosenEvent = signal("2026-10-19");
  const dialogOpen = signal(false);

  // section: Usage
  const usage = (
    <DatePicker
      value={date()}
      onChange={(next) => date.set(next)}
      placeholder="Select date…"
      width={320}
    />
  );

  // section: Date
  const dateExample = (
    <DatePicker
      value={date()}
      onChange={(next) => date.set(next)}
      placeholder="Select date…"
      width={320}
    />
  );

  // section: With Initial Date
  const initialDate = (
    <DatePicker
      value={initial()}
      onChange={(next) => initial.set(next)}
      presentation="inline"
      defaultOpen
      width={320}
    />
  );

  // section: Date Range Picker
  const dateRange = (
    <Column gap={8} style={{ width: 340 }}>
      <RangeField range={() => range()} onChange={(next) => range.set(next)} />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
        {range().start} → {range().end ?? "pick an end date"}
      </Text>
    </Column>
  );

  // section: Date and Time
  const dateTime = (
    <Column gap={8} style={{ width: 340 }}>
      <DatePicker
        value={date()}
        onChange={(next) => date.set(next)}
        presentation="popover"
        width={320}
      />
      <Row gap={4}>
        {["13:00", "13:45", "14:30", "15:15"].map((slot) => (
          <Button
            label={slot}
            size="xsmall"
            variant={time() === slot ? "primary" : "secondary"}
            onPress={() => time.set(slot)}
          />
        ))}
      </Row>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
        {date()} at {time()}
      </Text>
    </Column>
  );

  // section: With Placeholder
  const placeholder = (
    <DatePicker placeholder="Set a due date…" presentation="popover" width={320} />
  );

  // section: Different Sizes
  const sizes = (
    <Row gap={10} style={{ alignItems: "center" }}>
      <DatePicker value={date()} onChange={(next) => date.set(next)} size="small" width={220} />
      <DatePicker value={date()} onChange={(next) => date.set(next)} size="large" width={300} />
    </Row>
  );

  // section: Disabled State
  const disabled = (
    <DatePicker value="2026-10-19" disabled width={320} />
  );

  // section: Event Date Picker
  const eventPicker = (
    <Column gap={8} style={{ width: 340 }}>
      <DatePicker
        value={chosenEvent()}
        onChange={(next) => chosenEvent.set(next)}
        presentation="inline"
        defaultOpen
        events={events}
        width={330}
      />
    </Column>
  );

  // section: Booking System Date Range
  const bookingRange = signal<CalendarRange>({ start: "2026-10-19", end: "2026-10-21" });
  const booking = (
    <View style={{ width: 420, maxWidth: "100%", padding: 12, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover, gap: 10 }}>
      <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Book your stay</Text>
      <Calendar
        mode="range"
        range={bookingRange()}
        onRangeChange={(next) => bookingRange.set(next)}
        showOutsideDays
      />
      <Row gap={8} style={{ justifyContent: "space-between" }}>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
          {bookingRange().start} → {bookingRange().end ?? "Check-out"}
        </Text>
        <Button label="Confirm" size="small" onPress={() => {}} />
      </Row>
    </View>
  );

  // section: Financial Period Selector
  const financial = (
    <DatePicker
      value="2026-10-01"
      onChange={() => {}}
      presentation="popover"
      presets={presets}
      activePreset="This quarter"
      onPresetSelect={(preset) => {
        activePreset.set(preset.label);
        date.set(preset.start);
      }}
      width={430}
    />
  );

  // section: Date Restrictions
  const restrictions = (
    <DatePicker
      min="2026-10-10"
      max="2026-10-25"
      disabledWeekends
      disabledDates={["2026-10-14", "2026-10-21"]}
      presentation="popover"
      width={320}
      placeholder="This month only"
    />
  );

  // section: Custom Year Range
  const customYear = (
    <DatePicker
      presentation="popover"
      months={2}
      width={590}
      placeholder="Two months"
    />
  );

  // section: Preset Ranges
  const presetRanges = (
    <DatePicker
      presentation="inline"
      defaultOpen
      presets={presets}
      activePreset={activePreset()}
      onPresetSelect={(preset) => {
        activePreset.set(preset.label);
        date.set(preset.start);
      }}
      width={430}
    />
  );

  // section: Multiple Months Display
  const multiple = (
    <DatePicker presentation="popover" months={3} width={850} placeholder="Three months" />
  );

  // section: Popover
  const popover = (
    <DatePicker
      value={date()}
      onChange={(next) => date.set(next)}
      presentation="popover"
      width={320}
    />
  );

  // section: In a Dialog
  const inDialog = (
    <>
      <Button label="Open date dialog" variant="primary" onPress={() => dialogOpen.set(true)} />
      <Dialog
        open={dialogOpen()}
        onClose={() => dialogOpen.set(false)}
        title="Choose a date"
        description="The calendar is presented inside the dialog; the choice commits when you pick a day."
        size="small"
      >
        <Calendar
          value={date()}
          onSelect={(next) => {
            date.set(next);
            dialogOpen.set(false);
          }}
        />
      </Dialog>
    </>
  );

  // section: Filter builder
  const operators = ["is", "before", "after", "between"];
  const periods = ["Day", "Month", "Quarter", "Half-year", "Year"];
  const filterBuilder = (
    <View style={{ width: 640, maxWidth: "100%", padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover, gap: 10 }}>
      <Row gap={6} style={{ alignItems: "center" }}>
        <Text style={{ fontSize: 11, fontWeight: "600", color: tokens.foreground }}>Due date</Text>
        <View style={{ flexDirection: "row", gap: 2, padding: 2, borderRadius: 6, backgroundColor: tokens.muted }}>
          {operators.map((entry) => (
            <View
              accessibilityRole="button"
              accessibilityLabel={`Due date ${entry}`}
              onPress={() => operator.set(entry)}
              style={{ paddingHorizontal: 6, paddingVertical: 3, borderRadius: 4, backgroundColor: operator() === entry ? tokens.popover : "transparent" }}
            >
              <Text style={{ fontSize: 10, color: operator() === entry ? tokens.foreground : tokens.mutedForeground }}>{entry}</Text>
            </View>
          ))}
        </View>
      </Row>
      <View style={{ height: 30, paddingHorizontal: 9, flexDirection: "row", alignItems: "center", gap: 6, borderWidth: 1, borderColor: tokens.input, borderRadius: 7 }}>
        <Icon name="calendar" size={13} color={tokens.mutedForeground} />
        <Text style={{ flex: 1, fontSize: 11, color: filterRange().start ? tokens.foreground : tokens.mutedForeground }}>
          {filterRange().start ? `${filterRange().start}${filterRange().end ? ` → ${filterRange().end}` : ""}` : "Select date…"}
        </Text>
      </View>
      <Row gap={4} style={{ alignItems: "center" }}>
        <View style={{ flexDirection: "row", gap: 2, padding: 2, borderRadius: 6, borderWidth: 1, borderColor: tokens.border }}>
          {periods.map((entry) => (
            <View
              accessibilityRole="button"
              accessibilityLabel={entry}
              onPress={() => granularity.set(entry)}
              style={{ paddingHorizontal: 6, paddingVertical: 3, borderRadius: 4, backgroundColor: granularity() === entry ? tokens.muted : "transparent" }}
            >
              <Text style={{ fontSize: 10, color: granularity() === entry ? tokens.foreground : tokens.mutedForeground }}>{entry}</Text>
            </View>
          ))}
        </View>
        <View style={{ flex: 1 }} />
        <Button icon="chevron-left" size="xsmall" variant="ghost" accessibilityLabel="Previous" onPress={() => {}} />
        <Button icon="chevron-right" size="xsmall" variant="ghost" accessibilityLabel="Next" onPress={() => {}} />
      </Row>
      <Calendar
        mode="range"
        months={2}
        range={filterRange()}
        onRangeChange={(next) => filterRange.set(next)}
        showOutsideDays
      />
      <View style={{ padding: 8, borderRadius: 6, backgroundColor: tokens.muted }}>
        <Text style={{ fontSize: 10, fontFamily: "monospace", color: tokens.mutedForeground }}>
          {`{ "period": "${granularity().toLowerCase()}", "operator": "${operator()}", "start": "${filterRange().start ?? ""}", "end": "${filterRange().end ?? ""}" }`}
        </Text>
      </View>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>
        The date field wraps the Calendar. Use a modal for a committed choice, a popover for
        quick edits, or an inline panel for filters and forms.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Date">{dateExample}</ExampleSection>
      <ExampleSection index={2} title="With Initial Date">{initialDate}</ExampleSection>
      <ExampleSection index={3} title="Date Range Picker">{dateRange}</ExampleSection>
      <ExampleSection index={4} title="Date and Time">{dateTime}</ExampleSection>
      <ExampleSection index={5} title="With Placeholder">{placeholder}</ExampleSection>
      <ExampleSection index={6} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={7} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={8} title="Event Date Picker">{eventPicker}</ExampleSection>
      <ExampleSection index={9} title="Booking System Date Range">{booking}</ExampleSection>
      <ExampleSection index={10} title="Financial Period Selector">{financial}</ExampleSection>
      <ExampleSection index={11} title="Date Restrictions">{restrictions}</ExampleSection>
      <ExampleSection index={12} title="Custom Year Range">{customYear}</ExampleSection>
      <ExampleSection index={13} title="Preset Ranges">{presetRanges}</ExampleSection>
      <ExampleSection index={14} title="Multiple Months Display">{multiple}</ExampleSection>
      <ExampleSection index={15} title="Popover">{popover}</ExampleSection>
      <ExampleSection index={16} title="In a Dialog">{inDialog}</ExampleSection>
      <ExampleSection index={17} title="Filter builder">{filterBuilder}</ExampleSection>
      <Status>Selected: {date()} · preset {activePreset()}</Status>
    </Page>
  );
}
