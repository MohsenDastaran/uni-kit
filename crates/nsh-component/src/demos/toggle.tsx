//! `toggle` — a button that stays on, and toggle groups.
import { signal, Text, View } from "nsh";
import { Toggle, ToggleGroup } from "../components/toggle";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const pressed = signal(true);
  const group = signal("center");
  // section: Usage
  const usage = <Toggle label="Bold" defaultPressed />;
  // section: Toggle
  const basic = <Toggle label="Bold" pressed={pressed()} onChange={(next) => pressed.set(next)} />;
  // section: Group
  const groupView = <ToggleGroup items={[{ value: "left", label: "Left" }, { value: "center", label: "Center" }, { value: "right", label: "Right" }]} value={group()} onChange={(next) => group.set(next)} />;
  // section: Controlled Toggle
  const controlled = <Row gap={10} style={{ alignItems: "center" }}><Toggle label="Muted" pressed={pressed()} onChange={(next) => pressed.set(next)} /><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{pressed() ? "on" : "off"}</Text></Row>;
  // section: Toggle Variants
  const variants = <Row gap={8}><Toggle label="Default" /><Toggle label="Primary" variant="primary" defaultPressed /><Toggle label="Secondary" variant="secondary" /><Toggle label="Ghost" variant="ghost" /></Row>;
  // section: Different Sizes
  const sizes = <Row gap={8}><Toggle label="Small" size="small" /><Toggle label="Medium" /><Toggle label="Large" size="large" /></Row>;
  // section: Disabled State
  const disabled = <Row gap={8}><Toggle label="Disabled" disabled /><Toggle label="Disabled, on" disabled defaultPressed /></Row>;
  // section: Toolbar with Toggle Buttons
  const toolbar = <Row gap={8}><Toggle label="Bold" /><Toggle label="Italic" /><Toggle label="Underline" /></Row>;
  // section: Icon Only
  const iconOnly = (
    <View style={{ flexDirection: "row", alignItems: "center", gap: 2, padding: 4, borderWidth: 1, borderColor: tokens.border, borderRadius: 8, backgroundColor: tokens.popover, alignSelf: "flex-start" }}>
      <Toggle icon="bold" accessibilityLabel="Bold" defaultPressed />
      <Toggle icon="italic" accessibilityLabel="Italic" />
      <Toggle icon="underline" accessibilityLabel="Underline" />
      <Toggle icon="bookmark" accessibilityLabel="Strikethrough" />
      <View style={{ width: 1, height: 18, backgroundColor: tokens.border, marginHorizontal: 3 }} />
      <Toggle icon="list" accessibilityLabel="Bulleted list" />
      <Toggle icon="code" accessibilityLabel="Code" />
      <Toggle icon="external-link" accessibilityLabel="Link" />
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>Use a toggle for a mode that stays on; use a button for a one-shot action.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Toggle">{basic}</ExampleSection>
      <ExampleSection index={2} title="Icon Only">{iconOnly}</ExampleSection>
      <ExampleSection index={3} title="Group">{groupView}</ExampleSection>
      <ExampleSection index={4} title="Controlled Toggle">{controlled}</ExampleSection>
      <ExampleSection index={5} title="Toggle Variants">{variants}</ExampleSection>
      <ExampleSection index={6} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={7} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={8} title="Toolbar with Toggle Buttons">{toolbar}</ExampleSection>
    </Page>
  );
}