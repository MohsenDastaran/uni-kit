//! `textarea` — a multi-line field with auto-grow and a counter.
import { signal, Text } from "nsh";
import { Textarea, TextareaCounter } from "../components/textarea";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const value = signal("A message that can grow.");
  // section: Basic usage
  const basic = <Textarea value={value()} onChangeText={(next) => value.set(next)} placeholder="Write a message…" />;
  // section: Auto-grow
  const growing = <Textarea defaultValue={"Line one\nLine two\nLine three"} autoGrow rows={4} />;
  // section: Value and events
  const events = <Textarea value={value()} onChangeText={(next) => value.set(next)} />;
  // section: Sizes
  const sizes = <Textarea defaultValue="Small" size="small" rows={2} />;
  // section: Appearance
  const appearance = <Textarea defaultValue="Without a frame" appearance={false} />;
  // section: Atomic inline tokens
  const counter = (
    <Page>
      <Textarea value={value()} onChangeText={(next) => value.set(next)} maxLength={120} />
      <TextareaCounter value={value()} max={120} />
    </Page>
  );
  // section: end
  return (
    <Page>
      <Caption>A textarea is a multi-line input; keep the label and validation in the Form around it.</Caption>
      <ExampleSection index={0} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Auto-grow">{growing}</ExampleSection>
      <ExampleSection index={2} title="Value and events">{events}</ExampleSection>
      <ExampleSection index={3} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Appearance">{appearance}</ExampleSection>
      <ExampleSection index={5} title="Atomic inline tokens">{counter}</ExampleSection>
    </Page>
  );
}
