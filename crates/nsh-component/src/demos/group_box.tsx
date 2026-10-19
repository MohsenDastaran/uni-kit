//! `group-box` — a titled cluster in the normal, filled and outlined shapes,
//! following the iced port: title outside the surface, footer below it.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Checkbox } from "../components/checkbox";
import { GroupBox, GroupBoxRow } from "../components/group_box";
import { RadioGroup } from "../components/radio";
import { Switch } from "../components/switch";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const privateProfile = signal(true);
  const contributions = signal(false);
  const allActivity = signal(true);
  const productUpdates = signal(true);
  const accountActivity = signal(false);
  const appearance = signal("System");

  // section: Usage
  const usage = (
    <GroupBox title="Notifications" variant="outline" footer="You can change this at any time." style={{ width: 460 }}>
      <GroupBoxRow label="Product updates" description="Release notes and deprecations.">
        <Switch checked={productUpdates()} onChange={(next) => productUpdates.set(next)} />
      </GroupBoxRow>
    </GroupBox>
  );

  // section: Account Settings
  const account = (
    <GroupBox title="Account Settings" variant="fill" style={{ width: 520 }}>
      <GroupBoxRow label="Make profile private and hide activity">
        <Switch checked={privateProfile()} onChange={(next) => privateProfile.set(next)} />
      </GroupBoxRow>
      <GroupBoxRow label="Include private contributions on my profile">
        <Switch checked={contributions()} onChange={(next) => contributions.set(next)} />
      </GroupBoxRow>
      <Row>
        <Button label="Save Changes" variant="primary" size="small" onPress={() => {}} />
      </Row>
    </GroupBox>
  );

  // section: Preferences
  const preferences = (
    <GroupBox variant="outline" style={{ width: 520 }}>
      <Checkbox checked={allActivity()} onChange={(next) => allActivity.set(next)} label="All activity" description="Every mention, reply, and review." />
      <Checkbox checked={productUpdates()} onChange={(next) => productUpdates.set(next)} label="Product updates" description="Release notes and deprecations." />
      <Checkbox checked={accountActivity()} onChange={(next) => accountActivity.set(next)} label="Account activity" description="Sign-ins and billing changes." />
    </GroupBox>
  );

  // section: GroupBox Variants
  const variants = (
    <Row gap={12} style={{ alignItems: "stretch" }}>
      <View style={{ flex: 1, minWidth: 170 }}>
        <GroupBox title="Default" contentPadding={14}>
          <Text style={{ fontSize: 11, lineHeight: 17, color: tokens.mutedForeground }}>No visual container. The title alone groups the content.</Text>
        </GroupBox>
      </View>
      <View style={{ flex: 1, minWidth: 170 }}>
        <GroupBox title="Fill" variant="fill" contentPadding={14}>
          <Text style={{ fontSize: 11, lineHeight: 17, color: tokens.mutedForeground }}>A muted background, for a recessed settings surface.</Text>
        </GroupBox>
      </View>
      <View style={{ flex: 1, minWidth: 170 }}>
        <GroupBox title="Outline" variant="outline" contentPadding={14}>
          <Text style={{ fontSize: 11, lineHeight: 17, color: tokens.mutedForeground }}>A one-pixel border, for a card that must read as a unit.</Text>
        </GroupBox>
      </View>
    </Row>
  );

  // section: With Title
  const title = (
    <GroupBox title="Email notifications" variant="fill" style={{ width: 520 }}>
      <Checkbox checked={productUpdates()} onChange={(next) => productUpdates.set(next)} label="Product updates" description="Release notes and deprecations." />
      <Checkbox checked={accountActivity()} onChange={(next) => accountActivity.set(next)} label="Account activity" description="Sign-ins and billing changes." />
      <Row>
        <Button label="Save preferences" variant="primary" size="small" onPress={() => {}} />
      </Row>
    </GroupBox>
  );

  // section: Footer outside the surface
  const footer = (
    <GroupBox
      title="Contributions & activity"
      variant="fill"
      footer="Private contributions never reveal repository names."
      style={{ width: 520 }}
    >
      <GroupBoxRow label="Make profile private and hide activity">
        <Switch checked={privateProfile()} onChange={(next) => privateProfile.set(next)} />
      </GroupBoxRow>
    </GroupBox>
  );

  // section: Custom Title Styling
  const titleStyling = (
    <GroupBox title="Appearance" variant="outline" titleColor={tokens.foreground} style={{ width: 520 }}>
      <RadioGroup
        value={appearance()}
        onChange={(next) => appearance.set(next)}
        options={[
          { value: "Light", label: "Light" },
          { value: "Dark", label: "Dark" },
          { value: "System", label: "System" },
        ]}
      />
    </GroupBox>
  );

  // section: Custom Content Styling
  const contentStyling = (
    <GroupBox title="Roomy surface" variant="outline" contentPadding={24} cornerRadius={12} style={{ width: 520 }}>
      <Text style={{ fontSize: 12, color: tokens.foreground }}>A 24px inset on a 12px radius.</Text>
    </GroupBox>
  );

  // section: Without Title
  const withoutTitle = (
    <GroupBox variant="outline" style={{ width: 520 }}>
      <GroupBoxRow label="Make profile private and hide activity">
        <Switch checked={privateProfile()} onChange={(next) => privateProfile.set(next)} />
      </GroupBoxRow>
    </GroupBox>
  );
  // section: end

  return (
    <Page>
      <Caption>
        GroupBox is for a labelled cluster of settings; use Card for a content
        surface. The title sits outside the surface and the footer hangs below it.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Account Settings">{account}</ExampleSection>
      <ExampleSection index={2} title="Preferences">{preferences}</ExampleSection>
      <ExampleSection index={3} title="GroupBox Variants">{variants}</ExampleSection>
      <ExampleSection index={4} title="With Title">{title}</ExampleSection>
      <ExampleSection index={5} title="Footer outside the surface">{footer}</ExampleSection>
      <ExampleSection index={6} title="Custom Title Styling">{titleStyling}</ExampleSection>
      <ExampleSection index={7} title="Custom Content Styling">{contentStyling}</ExampleSection>
      <ExampleSection index={8} title="Without Title">{withoutTitle}</ExampleSection>
    </Page>
  );
}
