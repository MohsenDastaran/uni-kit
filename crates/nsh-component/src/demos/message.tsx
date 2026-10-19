//! `message` — chat turns with identity, bubble/plain surfaces, grouping,
//! attachments, reactions and delivery metadata.
import { Text, View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Message, MessageGroup } from "../components/message";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

const surface = { width: 540, maxWidth: "100%" } as const;

export function Demo() {
  // section: Usage
  const usage = (
    <View style={{ width: 540, maxWidth: "100%", gap: 10 }}>
      <Message role="user" name="You" time="09:38" content="Can you promote 4.2 once the migration lands?" />
      <Message role="assistant" name="Release bot" time="09:39" content="Staging is green on 4.2. Want me to promote it?" />
    </View>
  );

  // section: Anatomy and basic usage
  const basic = (
    <Message role="assistant" name="Release bot" time="09:30" content="How can I help today?" style={surface} />
  );

  // section: Alignment
  const alignment = (
    <View style={{ width: 540, maxWidth: "100%", gap: 10 }}>
      <Message role="assistant" name="Release bot" content="From the assistant. The bubble stays on the leading edge." />
      <Message role="user" name="You" content="From the user. The bubble aligns to the trailing edge." />
    </View>
  );

  // section: Avatar
  const avatar = (
    <Message role="user" name="Ada Lovelace" time="09:31" content="A message with an avatar and name." style={surface} />
  );

  // section: Header
  const header = (
    <Message role="assistant" name="Emma Wilson" time="09:38" content="The invoice template is missing the VAT line on EU orders." style={surface} />
  );

  // section: Content
  const content = (
    <Message
      role="assistant"
      name="Release bot"
      content="A longer body that wraps inside a comfortable reading measure instead of stretching across the page."
      style={surface}
    />
  );

  // section: Footer
  const footer = (
    <Message
      role="user"
      name="You"
      time="09:41"
      content="Then I will re-run the January batch once it is out."
      status="Read 09:41"
      edited="Edited"
      style={surface}
    />
  );

  // section: Rich content and actions
  const rich = (
    <Message
      role="assistant"
      name="Release bot"
      content="Rich content stays inside the bubble."
      reactions={["👍 2", "🎉 1"]}
      actions={
        <>
          <Button icon="copy" size="xsmall" variant="ghost" accessibilityLabel="Copy" onPress={() => {}} />
          <Button icon="refresh-cw" size="xsmall" variant="ghost" accessibilityLabel="Regenerate" onPress={() => {}} />
          <Button icon="thumbs-up" size="xsmall" variant="ghost" accessibilityLabel="Like" onPress={() => {}} />
        </>
      }
      style={surface}
    >
      <View style={{ paddingTop: 6 }}>
        <Text style={{ fontSize: 11, lineHeight: 17, color: tokens.mutedForeground }}>
          Include a short summary, the migration note and the rollback link.
        </Text>
      </View>
    </Message>
  );

  // section: Grouping
  const grouped = (
    <MessageGroup style={surface}>
      <Message role="assistant" name="Alex Johnson" time="14:04" content="I pulled the eu-west traces for 14:00 to 14:10." />
      <Message role="assistant" showHeader={false} content="Every slow request goes through the tax service." />
      <Message role="assistant" showHeader={false} content="It is a retry loop, not the database. Two retries at 400ms each." />
      <Message role="user" name="You" time="14:07" content="That lines up with the 14:04 deploy. Rolling the tax client back." />
    </MessageGroup>
  );

  // section: Ghost surfaces and content insets
  const ghost = (
    <Message
      role="assistant"
      name="Release bot"
      time="14:45"
      plain
      content="The tax service began retrying every confirm call at 14:05. Rolling the tax client back to 1.8.3 returned p95 to 260ms within two minutes."
      status="Read 14:46"
      style={surface}
    />
  );

  // section: Reactions
  const reactions = (
    <Message
      role="assistant"
      name="Design review"
      content="The new empty state reads much better at this size."
      reactions={["👍 4", "🔥 2", "👀 1"]}
      style={surface}
    />
  );

  // section: Attachments
  const attachments = (
    <Message role="user" name="You" time="09:42" content="Here is the release banner. Can you send the palette over?" style={surface}>
      <View style={{ marginTop: 8, gap: 6 }}>
        <View style={{ width: 220, height: 100, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, alignItems: "center", justifyContent: "center", gap: 4 }}>
          <Icon name="image" size={18} color={tokens.mutedForeground} />
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>hero-gradient.png · 2.1 MB</Text>
        </View>
        <View style={{ flexDirection: "row", alignItems: "center", gap: 7, width: 240, padding: 7, borderRadius: 7, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.background }}>
          <Icon name="file-text" size={14} color={tokens.mutedForeground} />
          <View style={{ flex: 1, gap: 1 }}>
            <Text style={{ fontSize: 10, color: tokens.foreground }}>palette-tokens.csv</Text>
            <Text style={{ fontSize: 8, color: tokens.mutedForeground }}>CSV · 184 KB</Text>
          </View>
          <Icon name="download" size={12} color={tokens.mutedForeground} />
        </View>
      </View>
    </Message>
  );

  // section: Custom styling
  const custom = (
    <Message
      role="assistant"
      name="Agent run"
      time="15:02"
      variant="secondary"
      content="A secondary surface separates a system answer from the everyday conversation."
      reactions={["✅ 1"]}
      style={{ width: 520, maxWidth: "100%" }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Message composes the identity, the bubble surface and the metadata row;
        MessageScroller owns the conversation viewport. Attachments and actions
        are children.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Anatomy and basic usage">{basic}</ExampleSection>
      <ExampleSection index={2} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={3} title="Avatar">{avatar}</ExampleSection>
      <ExampleSection index={4} title="Header">{header}</ExampleSection>
      <ExampleSection index={5} title="Content">{content}</ExampleSection>
      <ExampleSection index={6} title="Footer">{footer}</ExampleSection>
      <ExampleSection index={7} title="Rich content and actions">{rich}</ExampleSection>
      <ExampleSection index={8} title="Grouping">{grouped}</ExampleSection>
      <ExampleSection index={9} title="Ghost surfaces and content insets">{ghost}</ExampleSection>
      <ExampleSection index={10} title="Reactions">{reactions}</ExampleSection>
      <ExampleSection index={11} title="Attachments">{attachments}</ExampleSection>
      <ExampleSection index={12} title="Custom styling">{custom}</ExampleSection>
    </Page>
  );
}
