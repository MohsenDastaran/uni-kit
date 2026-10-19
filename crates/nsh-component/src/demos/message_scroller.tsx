//! `message-scroller` — conversation viewports with auto-follow, appended
//! streamed turns, prepended history and empty states.
import { signal, Text, View, type JSX } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Message, type MessageProps } from "../components/message";
import { MessageScroller } from "../components/message_scroller";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const initial: MessageProps[] = [
  { role: "assistant", name: "Release bot", time: "09:38", content: "Staging is green on 4.2. Want me to promote it?" },
  { role: "user", name: "You", time: "09:39", content: "Hold it. The billing migration has to go out in the same release." },
  { role: "assistant", name: "Release bot", time: "09:40", content: "Understood. The migration lands after the 2pm review." },
  { role: "user", name: "You", time: "09:41", content: "Then I will cut the tag straight after." },
];

export function Demo() {
  const streamed = signal<MessageProps[]>([
    { role: "assistant", name: "Assistant", content: "Hello! What are we working on?" },
    { role: "user", content: "The NSH port of the catalog." },
    { role: "assistant", content: "The gallery is looking good." },
  ]);
  const earlier = signal<MessageProps[]>([...initial]);
  const follow = signal(true);

  const card = (children: JSX.Element, title: string, subtitle?: string, height = 420) => (
    <View style={{ width: 560, maxWidth: "100%", height, borderRadius: 10, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, overflow: "hidden" }}>
      <View style={{ paddingHorizontal: 14, paddingVertical: 10, flexDirection: "row", alignItems: "center", gap: 8, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <View style={{ flex: 1, gap: 1 }}>
          <Text style={{ fontSize: 12, fontWeight: "600", color: tokens.foreground }}>{title}</Text>
          {subtitle ? <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{subtitle}</Text> : null}
        </View>
        <Icon name="ellipsis" size={13} color={tokens.mutedForeground} />
      </View>
      {children}
    </View>
  );

  // section: Usage
  const usage = (
    <View style={{ height: 280, width: 520, maxWidth: "100%", borderRadius: 10, borderWidth: 1, borderColor: tokens.border, overflow: "hidden" }}>
      <MessageScroller messages={initial} style={{ flex: 1 }} />
    </View>
  );

  // section: Basic thread
  const basic = card(
    <MessageScroller messages={initial} style={{ flex: 1 }} />,
    "Release coordination",
    "4 messages",
    380,
  );

  // section: Append, streaming, and follow-tail behavior
  const append = (
    <View style={{ width: 560, maxWidth: "100%", gap: 8 }}>
      {card(
        <MessageScroller
          messages={streamed()}
          style={{ flex: 1 }}
          footer={
            <View style={{ paddingTop: 4 }}>
              <Button
                label="Append a message"
                size="small"
                variant="secondary"
                onPress={() => streamed.update((list) => [...list, { role: "assistant", content: `Streamed update ${list.length + 1}.` }])}
              />
            </View>
          }
        />,
        "Streaming conversation",
        "Follows the newest row",
        380,
      )}
    </View>
  );

  // section: Prepend earlier history
  const prepend = (
    <View style={{ width: 560, maxWidth: "100%", gap: 8 }}>
      {card(
        <MessageScroller
          messages={earlier()}
          style={{ flex: 1 }}
          footer={
            <View style={{ paddingTop: 4 }}>
              <Button
                label="Load earlier messages"
                size="small"
                variant="secondary"
                onPress={() => earlier.update((list) => [{ role: "user", content: "An earlier message from the support history." }, ...list])}
              />
            </View>
          }
        />,
        "Support escalation",
        "Prepends earlier history",
        400,
      )}
    </View>
  );

  // section: Jump-to-latest behavior
  const jump = (
    <View style={{ width: 560, maxWidth: "100%", gap: 8 }}>
      {card(
        <MessageScroller
          messages={initial}
          autoScroll={follow()}
          style={{ flex: 1 }}
          footer={
            <View style={{ flexDirection: "row", alignItems: "center", gap: 8, paddingTop: 4 }}>
              <Text style={{ flex: 1, fontSize: 9, color: tokens.mutedForeground }}>Auto-follow is {follow() ? "on" : "off"}</Text>
              <Button label={follow() ? "Pause follow" : "Resume follow"} size="xsmall" variant="secondary" onPress={() => follow.set(!follow())} />
            </View>
          }
        />,
        "Follow-tail behavior",
        "A jump to the latest row",
        360,
      )}
    </View>
  );

  // section: Empty state
  const empty = (
    <View style={{ height: 240, width: 520, maxWidth: "100%", borderRadius: 10, borderWidth: 1, borderColor: tokens.border, overflow: "hidden" }}>
      <MessageScroller
        messages={[]}
        empty={
        <View style={{ alignItems: "center", gap: 6, padding: 20 }}>
          <View style={{ width: 34, height: 34, borderRadius: 17, alignItems: "center", justifyContent: "center", backgroundColor: tokens.muted }}>
            <Icon name="message-circle" size={16} color={tokens.mutedForeground} />
          </View>
          <Text style={{ fontSize: 12, fontWeight: "600", color: tokens.foreground }}>No messages yet</Text>
          <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Start a conversation to see it here.</Text>
        </View>
      }
        style={{ flex: 1 }}
      />
    </View>
  );

  // section: Grouped conversation
  const grouped = card(
    <MessageScroller
      messages={[
        { role: "assistant", name: "Alex Johnson", time: "14:04", content: "I pulled the eu-west traces for 14:00 to 14:10." },
        { role: "assistant", showHeader: false, content: "Every slow request goes through the tax service." },
        { role: "assistant", showHeader: false, content: "It is a retry loop, not the database. Two retries at 400ms each." },
        { role: "user", name: "You", time: "14:07", content: "That lines up with the 14:04 deploy. Rolling the tax client back." },
      ]}
      style={{ flex: 1 }}
    />,
    "Alex Johnson",
    "Grouped turns",
    400,
  );
  // section: end

  return (
    <Page>
      <Caption>
        MessageScroller owns the viewport and follows the newest turn. Message owns
        the row; the array and streaming belong to the application.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic thread">{basic}</ExampleSection>
      <ExampleSection index={2} title="Append, streaming, and follow-tail behavior">{append}</ExampleSection>
      <ExampleSection index={3} title="Prepend earlier history">{prepend}</ExampleSection>
      <ExampleSection index={4} title="Jump-to-latest behavior">{jump}</ExampleSection>
      <ExampleSection index={5} title="Empty state">{empty}</ExampleSection>
      <ExampleSection index={6} title="Grouped conversation">{grouped}</ExampleSection>
      <Status>{streamed().length} streamed messages</Status>
    </Page>
  );
}
