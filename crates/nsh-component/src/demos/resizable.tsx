//! `resizable` — two panes with a draggable divider.
import { Text, View } from "nsh";
import { Resizable } from "../components/resizable";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const pane = (label: string, color: string) => (
    <View style={{ flex: 1, alignItems: "center", justifyContent: "center", backgroundColor: color }}>
      <Text style={{ fontSize: 14, color: tokens.foreground }}>{label}</Text>
    </View>
  );
  // section: Basic
  const basic = <Resizable left={pane("Left", tokens.muted)} right={pane("Right", tokens.background)} style={{ height: 220 }} />;
  // section: Vertical
  const vertical = <Resizable vertical left={pane("Top", tokens.muted)} right={pane("Bottom", tokens.background)} style={{ height: 260 }} />;
  // section: end
  return (
    <Page>
      <Caption>Drag the divider; the panes keep the application's content.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Vertical">{vertical}</ExampleSection>
    </Page>
  );
}
