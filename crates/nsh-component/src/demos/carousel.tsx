//! `carousel` — a snapping horizontal viewport.
import { Text, View } from "nsh";
import { Carousel } from "../components/carousel";
import { inkOn, tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const card = (label: string, color: string) => (
    <View style={{ flex: 1, alignItems: "center", justifyContent: "center", backgroundColor: color }}>
      <Text style={{ fontSize: 20, fontWeight: "600", color: inkOn(color) }}>{label}</Text>
    </View>
  );
  // section: Basic
  const basic = <Carousel items={[card("One", tokens.info), card("Two", tokens.success), card("Three", tokens.warning)]} />;
  // section: Loop
  const loop = <Carousel loop items={[card("A", tokens.primary), card("B", tokens.secondary), card("C", tokens.muted)]} />;
  // section: end
  return (
    <Page>
      <Caption>The carousel is a paging Scroll with the buttons and dots around it.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Loop">{loop}</ExampleSection>
    </Page>
  );
}
