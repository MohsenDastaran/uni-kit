//! `message-role` — the identity marker on a turn.
import { MessageRole } from "../components/message_role";
import { Caption, ExampleSection, Page } from "./ui";

// section: Usage
const usage = <MessageRole role="assistant" name="Assistant" time="09:30" />;
// section: Custom Initials
const initials = <MessageRole role="user" name="Ada Lovelace" />;
// section: Failed State
const failed = <MessageRole role="system" name="Delivery failed" />;
// section: Alignment
const alignment = <MessageRole role="user" name="You" time="09:31" />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>The role marker is the avatar and the name; it does not draw a message surface.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Custom Initials">{initials}</ExampleSection>
      <ExampleSection index={2} title="Failed State">{failed}</ExampleSection>
      <ExampleSection index={3} title="Alignment">{alignment}</ExampleSection>
    </Page>
  );
}
