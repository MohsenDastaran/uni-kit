//! `combobox` — an input with a filtered list.
import { signal } from "nsh";
import { Combobox } from "../components/combobox";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const value = signal("button");
  const options = [
    { value: "button", label: "Button", description: "A press target" },
    { value: "input", label: "Input", description: "A text field" },
    { value: "card", label: "Card", description: "A content surface" },
    { value: "dialog", label: "Dialog", description: "A modal layer" },
  ];
  // section: Basic Single-Select
  const basic = <Combobox options={options} value={value()} onChange={(next) => value.set(next)} />;
  // section: Multi-Select
  const multi = <Combobox options={options} placeholder="Choose several (single in this port)" />;
  // section: Pre-selected Item
  const preselected = <Combobox options={options} value="input" onChange={() => {}} />;
  // section: Disabled Items
  const disabled = <Combobox options={[...options, { value: "legacy", label: "Legacy", disabled: true }]} />;
  // section: Custom Trigger
  const custom = <Combobox options={options} placeholder="Search components" />;
  // section: end
  return (
    <Page>
      <Caption>The list filters as you type; the selected label replaces the query when the list closes.</Caption>
      <ExampleSection index={0} title="Basic Single-Select">{basic}</ExampleSection>
      <ExampleSection index={1} title="Multi-Select">{multi}</ExampleSection>
      <ExampleSection index={2} title="Pre-selected Item">{preselected}</ExampleSection>
      <ExampleSection index={3} title="Disabled Items">{disabled}</ExampleSection>
      <ExampleSection index={4} title="Custom Trigger">{custom}</ExampleSection>
      <Status>Selected: {value()}</Status>
    </Page>
  );
}
