//! `marker` — conversation status rows, separators and loading markers.
import { Text } from "nsh";
import { Marker } from "../components/marker";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  // section: Plain
  const plain = <Marker><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Today</Text></Marker>;
  // section: Separator
  const separator = <Marker separator><Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Yesterday</Text></Marker>;
  // section: Alignment
  const alignment = <Marker align="center"><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Centered marker</Text></Marker>;
  // section: Loading styles
  const loading = <Marker loading><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Loading messages…</Text></Marker>;
  // section: Icons, content, and interactive children
  const icons = <Marker icon="info"><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>A system note with an icon</Text></Marker>;
  // section: end
  return (
    <Page>
      <Caption>Markers sit between messages or list sections; they are layout, not controls.</Caption>
      <ExampleSection index={0} title="Plain">{plain}</ExampleSection>
      <ExampleSection index={1} title="Separator">{separator}</ExampleSection>
      <ExampleSection index={2} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={3} title="Loading styles">{loading}</ExampleSection>
      <ExampleSection index={4} title="Icons, content, and interactive children">{icons}</ExampleSection>
    </Page>
  );
}
