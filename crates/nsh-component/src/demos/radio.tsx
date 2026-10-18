//! `radio` — one choice among several.
import { signal } from "nsh";
import { RadioGroup } from "../components/radio";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const value = signal("standard");
  const options = [
    { value: "standard", label: "Standard", description: "The usual delivery." },
    { value: "express", label: "Express", description: "Arrives tomorrow." },
    { value: "pickup", label: "Pick up", description: "Collect it yourself." },
  ];
  // section: Group
  const group = <RadioGroup options={options} value={value()} onChange={(next) => value.set(next)} />;
  // section: Standalone
  const standalone = <RadioGroup options={[{ value: "one", label: "One option" }]} value="one" />;
  // section: Different Sizes
  const sizes = <RadioGroup options={options} value={value()} onChange={(next) => value.set(next)} size="large" />;
  // section: Disabled State
  const disabled = <RadioGroup options={[{ value: "a", label: "Disabled", disabled: true }, { value: "b", label: "Also disabled", disabled: true }]} />;
  // section: Settings Panel
  const panel = <RadioGroup options={[{ value: "light", label: "Light" }, { value: "dark", label: "Dark" }, { value: "system", label: "System" }]} value={value()} onChange={(next) => value.set(next)} orientation="horizontal" />;
  // section: end
  return (
    <Page>
      <Caption>Radio buttons are for exactly one choice; a checkbox list is for independent choices.</Caption>
      <ExampleSection index={0} title="Group">{group}</ExampleSection>
      <ExampleSection index={1} title="Standalone">{standalone}</ExampleSection>
      <ExampleSection index={2} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={4} title="Settings Panel">{panel}</ExampleSection>
    </Page>
  );
}
