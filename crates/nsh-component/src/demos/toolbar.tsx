//! `toolbar` — grouped commands, slot-based layouts, mixed controls and
//! compact editor/application toolbars.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { MenuButton } from "../components/menu";
import { Select } from "../components/select";
import { Toolbar, ToolbarGroup, ToolbarLabel, ToolbarSeparator, ToolbarSpacer } from "../components/toolbar";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const font = signal("inter");
  const market = signal("all");
  const order = signal("all");
  const active = signal("bold");
  const stage = (children: any, width = 780) => <View style={{ width, maxWidth: "100%" }}>{children}</View>;
  const iconButton = (icon: string, label: string, variant: any = "ghost") => <Button icon={icon as any} accessibilityLabel={label} variant={variant} size="small" onPress={() => {}} />;
  const formatting = () => (
    <ToolbarGroup>
      <Button icon="bold" accessibilityLabel="Bold" variant={active() === "bold" ? "secondary" : "ghost"} size="small" onPress={() => active.set("bold")} />
      <Button icon="italic" accessibilityLabel="Italic" variant={active() === "italic" ? "secondary" : "ghost"} size="small" onPress={() => active.set("italic")} />
      <Button icon="underline" accessibilityLabel="Underline" variant={active() === "underline" ? "secondary" : "ghost"} size="small" onPress={() => active.set("underline")} />
    </ToolbarGroup>
  );

  // section: Usage
  const usage = stage(
    <Toolbar variant="card">
      <Button label="New" icon="plus" variant="secondary" size="small" onPress={() => {}} />
      <Button label="Save" icon="file-text" variant="ghost" size="small" onPress={() => {}} />
      <ToolbarSeparator />
      <ToolbarGroup variant="flush">
        <Button icon="arrow-left" accessibilityLabel="Undo" variant="ghost" size="small" onPress={() => {}} />
        <Button icon="arrow-right" accessibilityLabel="Redo" variant="ghost" size="small" onPress={() => {}} />
      </ToolbarGroup>
      {formatting()}
      <ToolbarSeparator />
      <Select options={[{ value: "inter", label: "Inter" }, { value: "geist", label: "Geist" }, { value: "mono", label: "Mono" }]} value={font()} onChange={(next) => font.set(next)} style={{ width: 140 }} />
    </Toolbar>,
  );
  // section: Commands
  const commands = stage(
    <Toolbar variant="card">
      {iconButton("chevron-left", "Back")}
      {iconButton("chevron-right", "Forward")}
      <ToolbarSeparator />
      {iconButton("refresh-cw", "Refresh")}
      <ToolbarSpacer />
      <ToolbarLabel>Ready</ToolbarLabel>
    </Toolbar>,
  );
  // section: Sizes
  const sizes = (
    <View style={{ width: 640, maxWidth: "100%", gap: 8 }}>
      <Toolbar size="small" variant="card"><Button label="Small" size="small" onPress={() => {}} /><ToolbarLabel size="small">Small</ToolbarLabel></Toolbar>
      <Toolbar size="medium" variant="card"><Button label="Medium" onPress={() => {}} /><ToolbarLabel>Medium</ToolbarLabel></Toolbar>
      <Toolbar size="large" variant="card"><Button label="Large" size="large" onPress={() => {}} /><ToolbarLabel size="large">Large</ToolbarLabel></Toolbar>
    </View>
  );
  // section: Labels and custom elements
  const labels = stage(
    <Toolbar variant="card" left={<ToolbarLabel>3 files changed</ToolbarLabel>} center={<Input placeholder="Filter files…" prefixIcon="search" size="small" onChangeText={() => {}} style={{ width: 240 }} />} right={<Button label="Review" size="small" variant="primary" onPress={() => {}} />} />,
  );
  // section: Custom styling
  const styling = stage(
    <Toolbar variant="accent" left={<ToolbarLabel style={{ color: tokens.foreground }}>Editor</ToolbarLabel>} right={<Row gap={2}>{iconButton("panel-left", "Toggle sidebar")}{iconButton("panel-right", "Toggle inspector")}</Row>} />,
  );
  // section: Groups
  const groups = stage(
    <Toolbar variant="card">
      {iconButton("arrow-left", "Back")}
      {iconButton("arrow-right", "Forward")}
      <ToolbarSeparator />
      {formatting()}
      <ToolbarSpacer />
      <Button label="Share" size="small" variant="secondary" onPress={() => {}} />
      <MenuButton label="More actions" items={[{ label: "Duplicate", icon: "copy" }, { label: "Export", icon: "download" }, { label: "Archive", icon: "trash" }]} trigger={<View style={{ width: 24, height: 24, alignItems: "center", justifyContent: "center" }}><Icon name="ellipsis" size={14} color={tokens.mutedForeground} /></View>} align="end" />
    </Toolbar>,
  );
  // section: Editor toolbar
  const editor = stage(
    <Toolbar variant="card">
      <ToolbarGroup variant="flush">
        <Button label="New" icon="plus" variant="ghost" size="small" onPress={() => {}} />
        <Button label="Save" icon="file-text" variant="secondary" size="small" onPress={() => {}} />
      </ToolbarGroup>
      <ToolbarSeparator />
      {formatting()}
      <ToolbarSeparator />
      <Select options={[{ value: "inter", label: "Inter" }, { value: "geist", label: "Geist" }, { value: "mono", label: "Mono" }]} value={font()} onChange={(next) => font.set(next)} style={{ width: 150 }} />
      <ToolbarSpacer />
      <Button label="Preview" size="small" variant="primary" onPress={() => {}} />
    </Toolbar>,
  );
  // section: Mixed controls
  const mixed = stage(
    <Toolbar variant="card">
      <Input placeholder="Search" prefixIcon="search" size="small" onChangeText={() => {}} style={{ width: 220 }} />
      <Select options={[{ value: "all", label: "All markets" }, { value: "us", label: "United States" }, { value: "eu", label: "Europe" }]} value={market()} onChange={(next) => market.set(next)} style={{ width: 150 }} />
      <Select options={[{ value: "all", label: "Order status" }, { value: "open", label: "Open" }, { value: "paid", label: "Paid" }]} value={order()} onChange={(next) => order.set(next)} style={{ width: 150 }} />
      <ToolbarSpacer />
      {iconButton("refresh-cw", "Refresh")}
      <Button icon="share-2" accessibilityLabel="Share" size="small" variant="secondary" onPress={() => {}} />
    </Toolbar>,
  );
  // section: Justified toolbar
  const justified = stage(
    <Toolbar variant="card" justify="between">
      <Row gap={8} style={{ alignItems: "center" }}><Icon name="circle-check" size={13} color={tokens.success} /><ToolbarLabel>All changes saved</ToolbarLabel></Row>
      <Row gap={6} style={{ alignItems: "center" }}><Button label="Export" size="small" variant="ghost" onPress={() => {}} /><Button label="Publish" size="small" variant="primary" onPress={() => {}} /></Row>
    </Toolbar>,
  );
  // section: Compact toolbar
  const compact = stage(
    <Toolbar variant="muted" borderless>
      <ToolbarGroup variant="flush">{iconButton("panel-left", "Sidebar")}{iconButton("search", "Search")}</ToolbarGroup>
      <ToolbarSpacer />
      <ToolbarLabel size="small">2 selected</ToolbarLabel>
      <Button label="Delete" size="xsmall" variant="danger" onPress={() => {}} />
    </Toolbar>,
  );
  // section: end
  return (
    <Page>
      <Caption>A toolbar is layout and grouping; each command is a Button, Select, Input or menu trigger.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Commands">{commands}</ExampleSection>
      <ExampleSection index={2} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Labels and custom elements">{labels}</ExampleSection>
      <ExampleSection index={4} title="Custom styling">{styling}</ExampleSection>
      <ExampleSection index={5} title="Groups">{groups}</ExampleSection>
      <ExampleSection index={6} title="Editor toolbar">{editor}</ExampleSection>
      <ExampleSection index={7} title="Mixed controls">{mixed}</ExampleSection>
      <ExampleSection index={8} title="Justified toolbar">{justified}</ExampleSection>
      <ExampleSection index={9} title="Compact toolbar">{compact}</ExampleSection>
    </Page>
  );
}
