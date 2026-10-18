//! `number-input` — a numeric field with step buttons and bounds.
import { signal } from "nsh";
import { NumberInput } from "../components/number_input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const value = signal(3);
  // section: Basic Number Input
  const basic = <NumberInput value={value()} min={0} max={10} onChange={(next) => value.set(next)} />;
  // section: Usage
  const usage = <NumberInput defaultValue={5} min={1} max={99} />;
  // section: Keyboard Navigation
  const keyboard = <NumberInput defaultValue={42} step={5} />;
  // section: Best Practices
  const best = <Row gap={16}><NumberInput defaultValue={1} min={1} max={8} /><NumberInput defaultValue={0} min={0} max={5} /></Row>;
  // section: Programmatic Control
  const programmatic = <Status>{String(value())}</Status>;
  // section: end
  return (
    <Page>
      <Caption>Use NumberInput when the value is a number; use Input for free text.</Caption>
      <ExampleSection index={0} title="Basic Number Input">{basic}</ExampleSection>
      <ExampleSection index={1} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={2} title="Keyboard Navigation">{keyboard}</ExampleSection>
      <ExampleSection index={3} title="Best Practices">{best}</ExampleSection>
      <ExampleSection index={4} title="Programmatic Control">{programmatic}</ExampleSection>
    </Page>
  );
}
