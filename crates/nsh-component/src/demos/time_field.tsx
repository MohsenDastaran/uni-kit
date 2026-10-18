//! `time-field` — a segmented time of day.
import { signal } from "nsh";
import { TimeField } from "../components/time_field";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const time = signal("09:30");
  // section: Basic Time Field
  const basic = <TimeField value={time()} onChange={(next) => time.set(next)} />;
  // section: With Seconds
  const seconds = <TimeField defaultValue="09:30:15" seconds />;
  // section: 12-Hour Clock
  const twelve = <TimeField defaultValue="14:30" hour12 />;
  // section: Sizes, Disabled and Invalid
  const states = <TimeField defaultValue="08:00" disabled />;
  // section: Handle Changes
  const changes = <Status>Time: {time()}</Status>;
  // section: end
  return (
    <Page>
      <Caption>TimeField uses numeric segments; a 12-hour clock adds the AM/PM control.</Caption>
      <ExampleSection index={0} title="Basic Time Field">{basic}</ExampleSection>
      <ExampleSection index={1} title="With Seconds">{seconds}</ExampleSection>
      <ExampleSection index={2} title="12-Hour Clock">{twelve}</ExampleSection>
      <ExampleSection index={3} title="Sizes, Disabled and Invalid">{states}</ExampleSection>
      <ExampleSection index={4} title="Handle Changes">{changes}</ExampleSection>
    </Page>
  );
}
