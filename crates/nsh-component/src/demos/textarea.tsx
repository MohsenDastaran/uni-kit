//! `textarea` — multi-line fields with labels, descriptions, validation,
//! disabled states, counters and the existing auto-grow behavior.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Textarea } from "../components/textarea";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const value = signal("A message that can grow.");
  const feedback = signal("Tell us what happened and what you expected.");
  const bio = signal("Tell us about yourself…");
  const message = signal("This message is ready to send.");
  const events = signal("No event yet");
  const stage = (children: any, width = 440) => <View style={{ width, maxWidth: "100%" }}>{children}</View>;

  // section: Usage
  const usage = stage(
    <Textarea
      label="Message"
      description="Keep it short and include the steps to reproduce."
      placeholder="Type your message here…"
      rows={4}
      value={feedback()}
      onChangeText={(next) => feedback.set(next)}
      maxLength={280}
      showCount
    />,
  );
  // section: Basic usage
  const basic = stage(
    <Textarea value={value()} onChangeText={(next) => value.set(next)} placeholder="Type your message here…" rows={4} />,
  );
  // section: Auto-grow
  const growing = stage(
    <Textarea
      defaultValue={"Line one\nLine two\nLine three\nLine four"}
      autoGrow
      rows={4}
      label="Auto-growing message"
      description="The field keeps its original minimum height and grows with the content."
    />,
  );
  // section: Value and events
  const values = stage(
    <View style={{ gap: 8 }}>
      <Textarea
        value={value()}
        onChangeText={(next) => {
          value.set(next);
          events.set(`Changed: ${next.length} characters`);
        }}
        onFocus={() => events.set("Focused")}
        onBlur={() => events.set("Blurred")}
        placeholder="Write a message…"
        rows={3}
      />
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{events()}</Text>
    </View>,
  );
  // section: Sizes
  const sizes = stage(
    <View style={{ gap: 10 }}>
      <Textarea label="Small" size="small" rows={2} defaultValue="A small textarea" />
      <Textarea label="Medium" size="medium" rows={3} defaultValue="A medium textarea" />
      <Textarea label="Large" size="large" rows={3} defaultValue="A large textarea" />
    </View>,
  );
  // section: Appearance
  const appearance = stage(
    <View style={{ padding: 10, borderWidth: 1, borderColor: tokens.border, borderRadius: 9, backgroundColor: tokens.popover }}>
      <Textarea defaultValue="Without its own frame, inside a parent frame." appearance={false} rows={3} />
    </View>,
  );
  // section: Invalid textarea
  const invalid = stage(
    <Textarea
      label="Feedback"
      description="Type your message and press enter to send."
      placeholder="Type your message here…"
      rows={3}
      invalid
      error="Please write at least a sentence before submitting."
    />,
  );
  // section: Disabled textarea
  const disabled = stage(
    <Textarea label="Message (Disabled)" placeholder="Type your message here…" rows={4} disabled />,
  );
  // section: Textarea with description
  const description = stage(
    <Textarea
      label="Feedback"
      description="Type your message and press enter to send."
      placeholder="Type your message here…"
      rows={3}
      value={message()}
      onChangeText={(next) => message.set(next)}
    />,
  );
  // section: Bio with counter
  const counted = stage(
    <Textarea
      label="Bio"
      placeholder="Tell us about yourself…"
      rows={4}
      maxLength={280}
      showCount
      value={bio()}
      onChangeText={(next) => bio.set(next)}
      description="This appears on your public profile."
    />,
  );
  // section: Read-only textarea
  const readOnly = stage(
    <Textarea
      label="Release notes"
      rows={3}
      readOnly
      defaultValue={"Fixed the retry window.\nAdded the missing locale fallback.\nUpdated the billing copy."}
      description="The release bot writes this field."
    />,
  );
  // section: Required message form
  const form = stage(
    <View style={{ gap: 10, padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover }}>
      <Textarea
        label="Message"
        required
        badge="New"
        maxLength={400}
        showCount
        rows={4}
        placeholder="Type your message here…"
        value={message()}
        onChangeText={(next) => message.set(next)}
        description="We reply within one business day."
      />
      <Row gap={8} style={{ justifyContent: "flex-end" }}>
        <Button label="Cancel" size="small" variant="ghost" onPress={() => {}} />
        <Button label="Send message" size="small" variant="primary" onPress={() => {}} />
      </Row>
    </View>,
    480,
  );
  // section: end
  return (
    <Page>
      <Caption>A textarea is a multi-line input; keep the label, description and validation with the field.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={2} title="Auto-grow">{growing}</ExampleSection>
      <ExampleSection index={3} title="Value and events">{values}</ExampleSection>
      <ExampleSection index={4} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={5} title="Appearance">{appearance}</ExampleSection>
      <ExampleSection index={6} title="Invalid textarea">{invalid}</ExampleSection>
      <ExampleSection index={7} title="Disabled textarea">{disabled}</ExampleSection>
      <ExampleSection index={8} title="Textarea with description">{description}</ExampleSection>
      <ExampleSection index={9} title="Bio with counter">{counted}</ExampleSection>
      <ExampleSection index={10} title="Read-only textarea">{readOnly}</ExampleSection>
      <ExampleSection index={11} title="Required message form">{form}</ExampleSection>
    </Page>
  );
}
