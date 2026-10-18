//! `scrollable` — a scroll container with the catalog's page surface.
import { Text } from "nsh";
import { Scrollable } from "../components/scrollable";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  // section: Basic
  const basic = (
    <Scrollable style={{ height: 240, borderWidth: 1, borderColor: tokens.border, borderRadius: 8 }}>
      {Array.from({ length: 30 }, (_, index) => (
        <Text style={{ padding: 10, fontSize: 13, color: tokens.foreground }}>Line {index + 1}</Text>
      ))}
    </Scrollable>
  );
  // section: Horizontal
  const horizontal = (
    <Scrollable horizontal style={{ height: 100, borderWidth: 1, borderColor: tokens.border, borderRadius: 8 }}>
      {Array.from({ length: 12 }, (_, index) => (
        <Text style={{ width: 120, padding: 10, fontSize: 13, color: tokens.foreground }}>Card {index + 1}</Text>
      ))}
    </Scrollable>
  );
  // section: end
  return (
    <Page>
      <Caption>Scrollable is the native Scroll with the catalog's page background; the scroll itself is native.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Horizontal">{horizontal}</ExampleSection>
    </Page>
  );
}
