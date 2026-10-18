//! `calendar` — a month calendar with selection and restrictions.
import { signal } from "nsh";
import { Calendar } from "../components/calendar";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const date = signal("2026-10-18");
  // section: Calendar
  const basic = <Calendar value={date()} onSelect={(next) => date.set(next)} />;
  // section: Two columns
  const two = <Calendar value={date()} onSelect={(next) => date.set(next)} months={2} />;
  // section: Three columns
  const three = <Calendar value={date()} onSelect={(next) => date.set(next)} months={3} />;
  // section: Disabled Weekends
  const weekends = <Calendar value={date()} onSelect={(next) => date.set(next)} disabledWeekends />;
  // section: Disabled Date Range
  const range = <Calendar value={date()} onSelect={(next) => date.set(next)} min="2026-10-10" max="2026-10-25" />;
  // section: end
  return (
    <Page>
      <Caption>The calendar is the date grid; a date picker adds the trigger and popover.</Caption>
      <ExampleSection index={0} title="Calendar">{basic}</ExampleSection>
      <ExampleSection index={1} title="Two columns">{two}</ExampleSection>
      <ExampleSection index={2} title="Three columns">{three}</ExampleSection>
      <ExampleSection index={3} title="Disabled Weekends">{weekends}</ExampleSection>
      <ExampleSection index={4} title="Disabled Date Range">{range}</ExampleSection>
      <Status>Selected: {date()}</Status>
    </Page>
  );
}
