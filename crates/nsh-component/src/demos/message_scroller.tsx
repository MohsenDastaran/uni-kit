//! `message-scroller` — a conversation list that follows the newest message.
import { MessageScroller } from "../components/message_scroller";
import { Caption, ExampleSection, Page } from "./ui";

const messages = [
  { role: "assistant" as const, name: "Assistant", content: "Hello! What are we working on?" },
  { role: "user" as const, name: "You", content: "The NSH port of the catalog." },
  { role: "assistant" as const, name: "Assistant", content: "The gallery is looking good." },
  { role: "user" as const, name: "You", content: "Ninety components next." },
  { role: "assistant" as const, name: "Assistant", content: "Let's keep each one on its own page." },
];

// section: Virtualization, accessibility, and application boundaries
const basic = <MessageScroller messages={messages} style={{ height: 320 }} />;
// section: Append, streaming, and follow-tail behavior
const append = <MessageScroller messages={[...messages, { role: "assistant" as const, content: "A new message at the end." }]} style={{ height: 320 }} />;
// section: Prepend earlier history
const prepend = <MessageScroller messages={[{ role: "user" as const, content: "An earlier message." }, ...messages]} style={{ height: 320 }} />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>MessageScroller is the scroll owner; the array and its streaming are the application's.</Caption>
      <ExampleSection index={0} title="Virtualization, accessibility, and application boundaries">{basic}</ExampleSection>
      <ExampleSection index={1} title="Append, streaming, and follow-tail behavior">{append}</ExampleSection>
      <ExampleSection index={2} title="Prepend earlier history">{prepend}</ExampleSection>
    </Page>
  );
}
