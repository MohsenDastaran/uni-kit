//! `label` — form labels, secondary text and highlighting.
import { Text, View } from "nsh";
import { Input } from "../components/input";
import { Label } from "../components/label";
import { Caption, ExampleSection, Page } from "./ui";

// section: Form labels
const form = (
  <View style={{ gap: 6 }}>
    <Label text="Email" required />
    <Input placeholder="name@example.com" onChangeText={() => {}} />
  </View>
);
// section: Basic Label
const basic = <Label text="Project name" />;
// section: Label with Secondary Text
const secondary = (
  <Label text="Workspace" secondary="Visible to your team" />
);
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>A label is typography and layout; the control beside it owns the interaction.</Caption>
      <ExampleSection index={0} title="Form labels">{form}</ExampleSection>
      <ExampleSection index={1} title="Basic Label">{basic}</ExampleSection>
      <ExampleSection index={2} title="Label with Secondary Text">{secondary}</ExampleSection>
    </Page>
  );
}
