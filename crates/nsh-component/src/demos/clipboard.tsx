//! `clipboard` — copy text with user feedback.
import { signal } from "nsh";
import { ClipboardButton } from "../components/clipboard";
import { Input } from "../components/input";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const text = signal("npm install @dastaran/uni-kit");
  // section: Basic Clipboard
  const basic = <ClipboardButton text={text()} />;
  // section: Using Dynamic Values
  const dynamic = <ClipboardButton text={`add nsh ${text()}`} label="Copy command" />;
  // section: With Custom Content
  const custom = <ClipboardButton text={text()} label="Copy the snippet" variant="primary" />;
  // section: In Input Fields
  const field = <Input value={text()} readOnly onChangeText={() => {}} suffixIcon="copy" />;
  // section: With User Feedback
  const feedback = <Status>Copying puts the text on the system clipboard.</Status>;
  // section: end
  return (
    <Page>
      <Caption>On the web the clipboard needs a secure context; the browser may ask the user.</Caption>
      <ExampleSection index={0} title="Basic Clipboard">{basic}</ExampleSection>
      <ExampleSection index={1} title="Using Dynamic Values">{dynamic}</ExampleSection>
      <ExampleSection index={2} title="With Custom Content">{custom}</ExampleSection>
      <ExampleSection index={3} title="In Input Fields">{field}</ExampleSection>
      <ExampleSection index={4} title="With User Feedback">{feedback}</ExampleSection>
    </Page>
  );
}
