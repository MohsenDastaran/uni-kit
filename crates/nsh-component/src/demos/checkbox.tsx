//! `checkbox` — binary choices with labels and sizes.
import { signal, Text } from "nsh";
import { Checkbox } from "../components/checkbox";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const checked = signal(true);
  // section: Basic
  const basic = <Checkbox checked={checked()} onChange={(next) => checked.set(next)} label="Accept the terms" />;
  // section: Sizes
  const sizes = <Row gap={16}><Checkbox size="small" defaultChecked label="Small" /><Checkbox defaultChecked label="Medium" /><Checkbox size="large" defaultChecked label="Large" /></Row>;
  // section: Controlled Checkbox
  const controlled = <Checkbox checked={checked()} onChange={(next) => checked.set(next)} label={checked() ? "Checked" : "Unchecked"} />;
  // section: Disabled State
  const disabled = <Row gap={16}><Checkbox disabled label="Disabled" /><Checkbox disabled defaultChecked label="Disabled, checked" /></Row>;
  // section: Without Label
  const withoutLabel = <Checkbox defaultChecked />;
  // section: Labels
  const labels = <Checkbox defaultChecked label="Email me about updates" description="No more than once a month." />;
  // section: Checkbox List
  const list = <Row gap={12} style={{ flexDirection: "column" }}><Checkbox defaultChecked label="Design" /><Checkbox label="Engineering" /><Checkbox defaultChecked label="Docs" /></Row>;
  // section: end
  return (
    <Page>
      <Caption>Use a checkbox for an independent choice; a radio group is for one of several.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={2} title="Controlled Checkbox">{controlled}</ExampleSection>
      <ExampleSection index={3} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={4} title="Without Label">{withoutLabel}</ExampleSection>
      <ExampleSection index={5} title="Labels">{labels}</ExampleSection>
      <ExampleSection index={6} title="Checkbox List">{list}</ExampleSection>
    </Page>
  );
}
