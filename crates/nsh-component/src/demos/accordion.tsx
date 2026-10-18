//! `accordion` — collapsible sections, one or several open.
import { signal, Text } from "nsh";
import { Accordion } from "../components/accordion";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const value = signal<string[]>(["one"]);
  const makeItems = () => [
    { id: "one", title: "Getting started", content: <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Install the CLI and create an app.</Text> },
    { id: "two", title: "Components", content: <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Copy the files you need.</Text> },
    { id: "three", title: "Theming", content: <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Point nsh.json at the shared theme.</Text> },
  ];
  // section: Single
  const single = <Accordion items={makeItems()} type="single" value={value()} onChange={(next) => value.set(next)} />;
  // section: Multiple
  const multiple = <Accordion items={makeItems()} type="multiple" defaultValue={["one", "two"]} />;
  // section: Controlled
  const controlled = <Accordion items={makeItems()} value={value()} onChange={(next) => value.set(next)} />;
  // section: end
  return (
    <Page>
      <Caption>Accordion is the coordination; Collapsible is the section.</Caption>
      <ExampleSection index={0} title="Single">{single}</ExampleSection>
      <ExampleSection index={1} title="Multiple">{multiple}</ExampleSection>
      <ExampleSection index={2} title="Controlled">{controlled}</ExampleSection>
      <Status>Open: {value().join(", ") || "none"}</Status>
    </Page>
  );
}
