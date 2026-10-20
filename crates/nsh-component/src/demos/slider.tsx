//! `slider` — a draggable value along a track.
import { signal, Text } from "nsh";
import { Slider } from "../components/slider";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const volume = signal(40);
  const low = signal(20);
  const high = signal(80);
  // section: Basic Slider
  const basic = <Slider value={volume()} onChange={(next) => volume.set(next)} />;
  // section: Slider with Event Handling
  const events = <Slider value={volume()} min={0} max={100} step={1} onChange={(next) => volume.set(next)} />;
  // section: Range Slider
  const range = (
    <Row gap={12}>
      <Slider value={low()} min={0} max={100} onChange={(next) => low.set(next)} style={{ flex: 1 }} />
      <Slider value={high()} min={0} max={100} onChange={(next) => high.set(next)} style={{ flex: 1 }} />
    </Row>
  );
  // section: Vertical Slider
  const vertical = <Row gap={20} style={{ alignItems: "center" }}><Slider value={volume()} vertical onChange={(next) => volume.set(next)} /><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{volume()}</Text></Row>;
  // section: Custom Step Intervals
  const step = <Slider value={volume()} min={0} max={100} step={10} onChange={(next) => volume.set(next)} />;
  // section: Min/Max Configuration
  const bounds = <Slider value={volume()} min={20} max={80} onChange={(next) => volume.set(next)} />;
  // section: Disabled State
  const disabled = <Slider value={50} disabled />;
  // section: end
  return (
    <Page>
      <Caption>Slider uses the touch position on the track; a Stepper is precise and discrete.</Caption>
      <ExampleSection index={0} title="Basic Slider">{basic}</ExampleSection>
      <ExampleSection index={1} title="Slider with Event Handling">{events}</ExampleSection>
      <ExampleSection index={2} title="Range Slider">{range}</ExampleSection>
      <ExampleSection index={3} title="Vertical Slider">{vertical}</ExampleSection>
      <ExampleSection index={4} title="Custom Step Intervals">{step}</ExampleSection>
      <ExampleSection index={5} title="Min/Max Configuration">{bounds}</ExampleSection>
      <ExampleSection index={6} title="Disabled State">{disabled}</ExampleSection>
      <Status>Volume: {volume()} · range {low()}–{high()}</Status>
    </Page>
  );
}
