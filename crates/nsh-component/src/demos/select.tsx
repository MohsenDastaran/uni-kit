//! `select` — a trigger and a list of options.
import { signal } from "nsh";
import { Select } from "../components/select";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const value = signal("nsh");
  const options = [
    { value: "gpui", label: "GPUI" },
    { value: "slint", label: "Slint" },
    { value: "iced", label: "iced" },
    { value: "nsh", label: "NSH" },
  ];
  // section: Framework
  const framework = <Select options={options} value={value()} onChange={(next) => value.set(next)} searchable />;
  // section: Placeholder
  const placeholder = <Select options={options} placeholder="Choose a framework" />;
  // section: Sizes
  const sizes = <Select options={options} size="large" placeholder="Large" />;
  // section: Disabled State
  const disabled = <Select options={options} disabled placeholder="Disabled" />;
  // section: Cleanable
  const cleanable = <Select options={options} value={value()} cleanable onChange={(next) => value.set(next)} />;
  // section: Empty State
  const empty = <Select options={[]} placeholder="Nothing to choose" emptyText="No options" />;
  // section: Events
  const events = <Status>Selected: {value()}</Status>;
  // section: end
  return (
    <Page>
      <Caption>Select is for a known list; Combobox is the searchable input form.</Caption>
      <ExampleSection index={0} title="Framework">{framework}</ExampleSection>
      <ExampleSection index={1} title="Placeholder">{placeholder}</ExampleSection>
      <ExampleSection index={2} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={4} title="Cleanable">{cleanable}</ExampleSection>
      <ExampleSection index={5} title="Empty State">{empty}</ExampleSection>
      <ExampleSection index={6} title="Events">{events}</ExampleSection>
    </Page>
  );
}
