//! `group-box` — labelled clusters of controls, wrapping nsh/ui/mobile Group.
import { Text, View } from "nsh";
import { GroupBox } from "../components/group_box";
import { Button } from "../components/button";
import { Switch } from "../components/switch";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  // section: Account Settings
  const account = (
    <GroupBox title="Account">
      <View style={{ flexDirection: "row", justifyContent: "space-between" }}><Text style={{ color: tokens.foreground }}>Name</Text><Text style={{ color: tokens.mutedForeground }}>Ada Lovelace</Text></View>
      <View style={{ flexDirection: "row", justifyContent: "space-between" }}><Text style={{ color: tokens.foreground }}>Email</Text><Text style={{ color: tokens.mutedForeground }}>ada@example.com</Text></View>
    </GroupBox>
  );
  // section: Preferences
  const preferences = (
    <GroupBox title="Preferences" footer="Sounds play for new messages only.">
      <View style={{ flexDirection: "row", justifyContent: "space-between" }}><Text style={{ color: tokens.foreground }}>Sounds</Text><Switch checked onChange={() => {}} /></View>
    </GroupBox>
  );
  // section: GroupBox Variants
  const variants = (
    <View style={{ gap: 12 }}>
      <GroupBox title="Default"><Text style={{ color: tokens.foreground }}>Default row</Text></GroupBox>
      <GroupBox title="Outline" variant="outline"><Text style={{ color: tokens.foreground }}>Outline row</Text></GroupBox>
      <GroupBox title="Filled" variant="filled"><Text style={{ color: tokens.foreground }}>Filled row</Text></GroupBox>
    </View>
  );
  // section: With Title
  const title = <GroupBox title="Notifications"><Text style={{ color: tokens.foreground }}>Show previews</Text></GroupBox>;
  // section: Footer outside the surface
  const footer = <GroupBox title="Danger zone" footer="These actions cannot be undone."><Button label="Delete project" variant="danger" onPress={() => {}} /></GroupBox>;
  // section: Without Title
  const untitled = <GroupBox><Button label="Action" onPress={() => {}} /></GroupBox>;
  // section: end
  return (
    <Page>
      <Caption>GroupBox is for a labelled cluster of settings; use Card for a content surface.</Caption>
      <ExampleSection index={0} title="Account Settings">{account}</ExampleSection>
      <ExampleSection index={1} title="Preferences">{preferences}</ExampleSection>
      <ExampleSection index={2} title="GroupBox Variants">{variants}</ExampleSection>
      <ExampleSection index={3} title="With Title">{title}</ExampleSection>
      <ExampleSection index={4} title="Footer outside the surface">{footer}</ExampleSection>
      <ExampleSection index={5} title="Without Title">{untitled}</ExampleSection>
    </Page>
  );
}
