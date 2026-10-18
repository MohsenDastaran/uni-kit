//! `stepper` — a bounded number with native repeat.
import { signal } from "nsh";
import { Stepper } from "../components/stepper";
import { Card } from "../components/card";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const value = signal(2);
  // section: Basic Stepper
  const basic = <Stepper value={value()} min={0} max={10} onChange={(next) => value.set(next)} />;
  // section: Disabled State
  const disabled = <Stepper value={4} disabled onChange={() => {}} />;
  // section: Sizes and disabled
  const sizes = <Row gap={16}><Stepper value={1} min={0} max={10} onChange={() => {}} /><Stepper value={3} min={0} max={10} step={0.5} onChange={() => {}} /></Row>;
  // section: Text center
  const centered = <Card title="Guests" description="A number within a range."><Stepper value={value()} min={1} max={12} onChange={(next) => value.set(next)} /></Card>;
  // section: Multi-step Form
  const form = <Row gap={16}><Stepper defaultValue={2} min={1} max={8} /><Stepper defaultValue={1} min={0} max={4} /></Row>;
  // section: end
  return (
    <Page>
      <Caption>Stepper is for a small number within a known range; NumberInput is the general numeric field.</Caption>
      <ExampleSection index={0} title="Basic Stepper">{basic}</ExampleSection>
      <ExampleSection index={1} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={2} title="Sizes and disabled">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Text center">{centered}</ExampleSection>
      <ExampleSection index={4} title="Multi-step Form">{form}</ExampleSection>
    </Page>
  );
}
