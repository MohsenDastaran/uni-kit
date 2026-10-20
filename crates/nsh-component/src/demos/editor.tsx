//! `editor` — a plain-text code editor with line numbers and events.
import { signal } from "nsh";
import { Editor } from "../components/editor";
import { Button } from "../components/button";
import { Caption, ExampleSection, Page, Status } from "./ui";

const sample = `function greet(name: string) {\n  return \`Hello, \${name}\`;\n}\n\ngreet("world");`;

export function Demo() {
  const value = signal(sample);
  // section: Basic usage
  const basic = <Editor value={value()} language="tsx" onChange={(next) => value.set(next)} />;
  // section: Editor options
  const options = <Editor defaultValue={sample} language="ts" lineNumbers={false} readOnly />;
  // section: Font
  const font = <Editor defaultValue={"const answer = 42;"} language="ts" />;
  // section: Appearance
  const appearance = <Editor defaultValue={sample} language="tsx" toolbar={<Button label="Run" size="small" variant="primary" onPress={() => {}} />} />;
  // section: Value and events
  const events = <Status>{value().split("\n").length} lines</Status>;
  // section: end
  return (
    <Page>
      <Caption>NSH's native Input cannot style spans, so this editor has no syntax colours; everything else is the catalog's editor.</Caption>
      <ExampleSection index={0} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Editor options">{options}</ExampleSection>
      <ExampleSection index={2} title="Font">{font}</ExampleSection>
      <ExampleSection index={3} title="Appearance">{appearance}</ExampleSection>
      <ExampleSection index={4} title="Value and events">{events}</ExampleSection>
    </Page>
  );
}
