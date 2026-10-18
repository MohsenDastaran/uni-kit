//! `image` — a framed picture with fit, radius and a placeholder.
import { signal, View } from "nsh";
import cover from "../../examples/assets/cover.png";
import { ImageView } from "../components/image";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const selected = signal(0);
  // section: Start with a working image
  const working = <ImageView src={cover} fit="cover" style={{ width: 320, height: 180 }} />;
  // section: Common patterns
  const patterns = (
    <Row gap={12} style={{ flexWrap: "wrap" }}>
      <ImageView src={cover} fit="cover" style={{ width: 160, height: 100 }} />
      <ImageView src={cover} fit="contain" style={{ width: 160, height: 100 }} />
      <ImageView style={{ width: 160, height: 100 }} />
    </Row>
  );
  // section: Gallery with selection
  const gallery = (
    <Row gap={10}>
      {[0, 1, 2].map((index) => (
        <View
          onPress={() => selected.set(index)}
          style={{ borderWidth: selected() === index ? 2 : 1, borderColor: selected() === index ? "#54aeff" : "#e5e5e5", borderRadius: 8, padding: 2 }}
        >
          <ImageView src={cover} style={{ width: 110, height: 70 }} radius={6} />
        </View>
      ))}
    </Row>
  );
  // section: Accessibility
  const accessible = <ImageView src={cover} alt="A mountain lake at sunrise" style={{ width: 280, height: 160 }} />;
  // section: end
  return (
    <Page>
      <Caption>ImageView keeps the frame, fit and placeholder; the image itself uses NSH's native Image element.</Caption>
      <ExampleSection index={0} title="Start with a working image">{working}</ExampleSection>
      <ExampleSection index={1} title="Common patterns">{patterns}</ExampleSection>
      <ExampleSection index={2} title="Gallery with selection">{gallery}</ExampleSection>
      <ExampleSection index={3} title="Accessibility">{accessible}</ExampleSection>
    </Page>
  );
}
