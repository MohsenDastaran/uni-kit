//! `icon` — the Canvas/Path2D icon set at the catalog's sizes and colours.
import { View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { alpha, RADIUS_LG, tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  // section: Usage
  const usage = (
    <Row gap={12} style={{ alignItems: "center" }}>
      <Icon name="search" size={20} />
      <Icon name="settings" size={20} />
      <Icon name="bell" size={20} />
    </Row>
  );

  // section: Basic Icon
  const basic = <Icon name="search" size={20} color={tokens.foreground} />;
  // section: Icon with Custom Size
  const sizes = (
    <Row gap={16} style={{ alignItems: "center" }}>
      <Icon name="star" size={12} color={tokens.foreground} />
      <Icon name="star" size={16} color={tokens.foreground} />
      <Icon name="star" size={24} color={tokens.foreground} />
      <Icon name="star" size={32} color={tokens.foreground} />
    </Row>
  );
  // section: Icon with Custom Color
  const colors = (
    <Row gap={16}>
      <Icon name="heart" size={24} color={tokens.danger} />
      <Icon name="check" size={24} color={tokens.success} />
      <Icon name="circle-alert" size={24} color={tokens.warning} />
      <Icon name="info" size={24} color={tokens.info} />
    </Row>
  );
  // section: Animated Loading Icon
  const loading = <Icon name="loader-circle" spinning size={24} color={tokens.primary} />;
  // section: Status Icons
  const status = (
    <Row gap={16}>
      <Icon name="circle-check" size={22} color={tokens.success} />
      <Icon name="triangle-alert" size={22} color={tokens.warning} />
      <Icon name="circle-x" size={22} color={tokens.danger} />
      <Icon name="info" size={22} color={tokens.info} />
    </Row>
  );
  // section: Icon in Button
  const inButton = (
    <Row gap={8}>
      <Button label="Search" icon="search" onPress={() => {}} />
      <Button label="Next" suffixIcon="chevron-right" variant="secondary" onPress={() => {}} />
      <Button icon="settings" accessibilityLabel="Settings" onPress={() => {}} />
    </Row>
  );
  // section: Navigation Icons
  const navigation = (
    <View
      style={{
        flexDirection: "row",
        alignItems: "center",
        gap: 6,
        padding: 6,
        alignSelf: "flex-start",
        borderRadius: RADIUS_LG,
        borderWidth: 1,
        borderColor: tokens.border,
        backgroundColor: alpha(tokens.muted, 0.35),
      }}
    >
      <Button icon="panel-left" variant="ghost" accessibilityLabel="Toggle sidebar" onPress={() => {}} />
      <Button icon="search" variant="ghost" accessibilityLabel="Search" onPress={() => {}} />
      <Button icon="folder" variant="ghost" accessibilityLabel="Files" onPress={() => {}} />
      <Button icon="bell" variant="ghost" accessibilityLabel="Notifications" onPress={() => {}} />
      <View style={{ width: 1, height: 20, backgroundColor: tokens.border }} />
      <Button icon="settings" variant="ghost" accessibilityLabel="Settings" onPress={() => {}} />
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>
        The catalog's Lucide icons are drawn with Canvas and Path2D, so they tint and scale
        without an SVG asset.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Icon">{basic}</ExampleSection>
      <ExampleSection index={2} title="Icon with Custom Size">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Icon with Custom Color">{colors}</ExampleSection>
      <ExampleSection index={4} title="Animated Loading Icon">{loading}</ExampleSection>
      <ExampleSection index={5} title="Status Icons">{status}</ExampleSection>
      <ExampleSection index={6} title="Icon in Button">{inButton}</ExampleSection>
      <ExampleSection index={7} title="Navigation Icons">{navigation}</ExampleSection>
    </Page>
  );
}
