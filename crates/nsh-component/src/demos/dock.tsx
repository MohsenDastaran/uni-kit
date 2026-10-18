//! `dock` — a tabbed panel group and a split.
import { Text, View } from "nsh";
import { Dock, DockSplit } from "../components/dock";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const panel = (label: string) => (
    <View style={{ flex: 1, alignItems: "center", justifyContent: "center" }}>
      <Text style={{ fontSize: 14, color: tokens.foreground }}>{label}</Text>
    </View>
  );
  const makePanels = () => [
    { id: "explorer", label: "Explorer", content: panel("Explorer") },
    { id: "search", label: "Search", content: panel("Search") },
    { id: "outline", label: "Outline", content: panel("Outline") },
  ];
  // section: Basic
  const basic = <Dock panels={makePanels()} style={{ height: 260 }} />;
  // section: Split
  const split = <DockSplit left={<Dock panels={makePanels()} style={{ height: 260 }} />} right={panel("Editor")} style={{ height: 260 }} />;
  // section: end
  return (
    <Page>
      <Caption>NSH has no dock host; this is a tabbed panel and a resizable split, with the layout owned by the app.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Split">{split}</ExampleSection>
    </Page>
  );
}
