//! `message` — a whole chat turn.
import { Text } from "nsh";
import { Message } from "../components/message";
import { Button } from "../components/button";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

// section: Anatomy and basic usage
const basic = <Message role="assistant" name="Assistant" time="09:30" content="How can I help today?" />;
// section: Alignment
const alignment = <><Message role="assistant" content="From the assistant" /><Message role="user" content="From the user" /></>;
// section: Avatar
const avatar = <Message role="user" name="Ada Lovelace" content="A message with a name." />;
// section: Header
const header = <Message role="assistant" name="Assistant" time="09:31" content="A message with a header." />;
// section: Content
const content = <Message role="assistant" content="A message with a longer body that wraps in the bubble." />;
// section: Footer
const footer = <Message role="assistant" content="A message with actions." actions={<><Button label="Copy" size="small" variant="ghost" onPress={() => {}} /><Button label="Retry" size="small" variant="ghost" onPress={() => {}} /></>} />;
export function Demo() {
  // section: Rich content and actions
  const rich = <Message role="assistant" content="Rich content stays inside the bubble." reactions={["👍 1"]}><Text style={{ fontSize: 12, color: tokens.mutedForeground }}>A child element.</Text></Message>;
  // section: end
  return (
    <Page>
      <Caption>Message composes MessageRole and Bubble; the conversation list is MessageScroller.</Caption>
      <ExampleSection index={0} title="Anatomy and basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={2} title="Avatar">{avatar}</ExampleSection>
      <ExampleSection index={3} title="Header">{header}</ExampleSection>
      <ExampleSection index={4} title="Content">{content}</ExampleSection>
      <ExampleSection index={5} title="Footer">{footer}</ExampleSection>
      <ExampleSection index={6} title="Rich content and actions">{rich}</ExampleSection>
    </Page>
  );
}
