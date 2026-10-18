//! `switch` — a binary track and knob.
import { signal } from "nsh";
import { Switch } from "../components/switch";
import { Text } from "nsh";
import { Label } from "../components/label";
import { Button } from "../components/button";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const checked = signal(true);
  const other = signal(false);
  // section: Basic
  const basic = <Switch checked={checked()} onChange={(next) => checked.set(next)} accessibilityLabel="Enabled" />;
  // section: Sizes
  const sizes = <Row gap={16} style={{ alignItems: "center" }}><Switch size="small" defaultChecked /><Switch defaultChecked /><Switch size="large" defaultChecked /></Row>;
  // section: Controlled Switch
  const controlled = <Row gap={12} style={{ alignItems: "center" }}><Switch checked={checked()} onChange={(next) => checked.set(next)} /><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{checked() ? "On" : "Off"}</Text></Row>;
  // section: With Label
  const withLabel = <Row gap={10} style={{ alignItems: "center" }}><Label text="Notifications" /><Switch checked={checked()} onChange={(next) => checked.set(next)} /></Row>;
  // section: Disabled State
  const disabled = <Row gap={16}><Switch disabled /><Switch disabled defaultChecked /></Row>;
  // section: Custom Color
  const color = <Row gap={16}><Switch defaultChecked color={tokens.info} /><Switch defaultChecked color={tokens.success} /></Row>;
  // section: Settings Panel
  const panel = (
    <Row gap={24}>
      <Row gap={8} style={{ alignItems: "center" }}><Label text="Sounds" /><Switch checked={checked()} onChange={(next) => checked.set(next)} /></Row>
      <Row gap={8} style={{ alignItems: "center" }}><Label text="Previews" /><Switch checked={other()} onChange={(next) => other.set(next)} /></Row>
    </Row>
  );
  // section: end
  return (
    <Page>
      <Caption>A switch commits immediately; a checkbox is for a choice that is submitted with a form.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={2} title="Controlled Switch">{controlled}</ExampleSection>
      <ExampleSection index={3} title="With Label">{withLabel}</ExampleSection>
      <ExampleSection index={4} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={5} title="Custom Color">{color}</ExampleSection>
      <ExampleSection index={6} title="Settings Panel">{panel}</ExampleSection>
      <Status><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{checked() ? "notifications on" : "notifications off"}</Text></Status>
    </Page>
  );
}
