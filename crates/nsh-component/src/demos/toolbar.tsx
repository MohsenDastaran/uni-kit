//! `toolbar` — command rows, sizes, labels, groups and styling.
import { Text } from "nsh";
import { Button } from "../components/button";
import { Toolbar, ToolbarGroup, ToolbarLabel } from "../components/toolbar";
import { Input } from "../components/input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

// section: Commands
const commands = <Toolbar><Button label="Back" variant="ghost" icon="chevron-left" onPress={() => {}} /><Button label="Forward" variant="ghost" suffixIcon="chevron-right" onPress={() => {}} /><Button label="Refresh" icon="refresh-cw" variant="ghost" onPress={() => {}} /></Toolbar>;
// section: Sizes
const sizes = <Toolbar><Button size="small" label="Small" onPress={() => {}} /><Button size="medium" label="Medium" onPress={() => {}} /><Button size="large" label="Large" onPress={() => {}} /></Toolbar>;
// section: Labels and custom elements
const labels = <Toolbar><ToolbarLabel>Search</ToolbarLabel><Input placeholder="Find" onChangeText={() => {}} style={{ width: 220 }} /><Button label="Go" variant="primary" onPress={() => {}} /></Toolbar>;
export function Demo() {
  // section: Custom styling
  const styling = <Toolbar borderless style={{ backgroundColor: tokens.muted, borderRadius: 8 }}><Button label="Saved" icon="check" variant="ghost" onPress={() => {}} /><Text style={{ fontSize: 12, color: tokens.mutedForeground }}>All changes saved</Text></Toolbar>;
  // section: Groups
const groups = <Toolbar><ToolbarGroup><Button label="Bold" variant="ghost" onPress={() => {}} /><Button label="Italic" variant="ghost" onPress={() => {}} /><Button label="Underline" variant="ghost" onPress={() => {}} /></ToolbarGroup><ToolbarGroup><Button label="Left" variant="ghost" onPress={() => {}} /><Button label="Center" variant="ghost" onPress={() => {}} /></ToolbarGroup></Toolbar>;
  // section: end
  return (
    <Page>
      <Caption>A toolbar is layout and grouping; each command is a Button.</Caption>
      <ExampleSection index={0} title="Commands">{commands}</ExampleSection>
      <ExampleSection index={1} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={2} title="Labels and custom elements">{labels}</ExampleSection>
      <ExampleSection index={3} title="Custom styling">{styling}</ExampleSection>
      <ExampleSection index={4} title="Groups">{groups}</ExampleSection>
    </Page>
  );
}
