//! `bubble` — the chat surface: alignment, variants, reactions.
import { Text } from "nsh";
import { Bubble } from "../components/bubble";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

// section: Anatomy and basic usage
const basic = <Bubble content="A short message." />;
// section: Alignment
const alignment = <Row gap={12} style={{ flexDirection: "column" }}><Bubble content="From the assistant" /><Bubble align="end" variant="primary" content="From the user" /></Row>;
// section: Variants
const variants = <Row gap={12}><Bubble content="Default" /><Bubble variant="muted" content="Muted" /><Bubble variant="primary" content="Primary" /></Row>;
// section: Rich content and long messages
const rich = <Bubble content="A longer message that wraps inside the bubble's maximum width and keeps the reading measure comfortable." />;
// section: Reactions and interactive content
const reactions = <Bubble content="A message with reactions" reactions={["👍 2", "🎉 1"]} />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>A bubble owns alignment and the surface; the message record and actions are the application's.</Caption>
      <ExampleSection index={0} title="Anatomy and basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={2} title="Variants">{variants}</ExampleSection>
      <ExampleSection index={3} title="Rich content and long messages">{rich}</ExampleSection>
      <ExampleSection index={4} title="Reactions and interactive content">{reactions}</ExampleSection>
    </Page>
  );
}
