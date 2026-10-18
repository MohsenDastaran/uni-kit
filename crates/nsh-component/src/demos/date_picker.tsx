//! `date-picker` — a date field with a calendar.
import { signal } from "nsh";
import { DatePicker } from "../components/date_picker";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const date = signal("2026-10-18");
  // section: Usage
  const usage = <DatePicker value={date()} onChange={(next) => date.set(next)} />;
  // section: Date Restrictions
  const restrictions = <DatePicker min="2026-10-01" max="2026-12-31" placeholder="A date this quarter" />;
  // section: Custom Year Range
  const years = <DatePicker months={2} placeholder="Two months" />;
  // section: Preset Ranges
  const presets = <DatePicker placeholder="Pick a date" />;
  // section: Multiple Months Display
  const multiple = <DatePicker months={3} placeholder="Three months" />;
  // section: end
  return (
    <Page>
      <Caption>The trigger shows the value; the calendar is the same Calendar component.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Date Restrictions">{restrictions}</ExampleSection>
      <ExampleSection index={2} title="Custom Year Range">{years}</ExampleSection>
      <ExampleSection index={3} title="Preset Ranges">{presets}</ExampleSection>
      <ExampleSection index={4} title="Multiple Months Display">{multiple}</ExampleSection>
      <Status>Selected: {date()}</Status>
    </Page>
  );
}
