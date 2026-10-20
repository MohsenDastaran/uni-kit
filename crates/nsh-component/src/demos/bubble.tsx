//! `bubble` — the chat surface: metadata, variants, folding, actions and reactions.
import { signal, Text, View } from "nsh";
import { LayoutAnimation } from "nsh/animated";
import { reduceMotion } from "nsh/accessibility";
import {
  Bubble,
  BubbleGroup,
  BubbleReactions,
  type BubbleReaction,
  type BubbleReactionItem,
} from "../components/bubble";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { MONO, RADIUS_XL, tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  // section: Usage
  const usage = (
    <Bubble
      name="Ada Lovelace"
      timestamp="10:24"
      content="Can you review the Q3 invoice draft before I send it?"
      reactions={["👍 2"]}
    />
  );

  // section: Anatomy and basic usage
  const basic = (
    <Column gap={12}>
      <Bubble content="A short message." />
      <Bubble
        variant="muted"
        name="Ada Lovelace"
        timestamp="10:24"
        content="A message with metadata and reactions."
        reactions={[{ emoji: "👍", count: 2, added: true }, "🎉 1"]}
        actions={<Button label="Reply" size="small" variant="ghost" onPress={() => {}} />}
      />
    </Column>
  );

  // section: Alignment
  const alignment = (
    <Column gap={10}>
      <Bubble
        variant="secondary"
        name="Ada Lovelace"
        timestamp="10:24"
        content="Incoming messages sit at the leading edge."
      />
      <Bubble
        align="end"
        variant="filled"
        name="You"
        timestamp="10:25"
        content="Sent messages mirror the same layout at the trailing edge."
      />
    </Column>
  );

  // section: Variants
  const variants = (
    <Row gap={8} style={{ alignItems: "flex-start" }}>
      <Bubble variant="filled" content="Filled" />
      <Bubble variant="secondary" content="Secondary" />
      <Bubble variant="muted" content="Muted" />
      <Bubble variant="tinted" content="Tinted" />
      <Bubble variant="outline" content="Outline" />
      <Bubble variant="ghost" content="Ghost" />
      <Bubble variant="destructive" content="Destructive" />
    </Row>
  );

  // section: Rich content and long messages
  const folded = signal(false);
  const toggleFolded = (next: boolean) => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    folded.set(next);
  };
  const rich = (
    <Column gap={10}>
      <Bubble
        variant="secondary"
        name="Billing assistant"
        timestamp="10:31"
        foldable
        collapsedLines={3}
        expanded={folded()}
        onExpandedChange={toggleFolded}
        content={
          "The Q3 invoice run finished with two failures. Both are card declines from the same customer, and neither retry was attempted because the account is still inside its grace window.\n\n" +
          "You can retry both invoices now, or open the failed invoice log to inspect the decline codes first. The retry uses the saved payment method; no customer action is needed unless the second attempt also fails.\n\n" +
          "If it fails again, the account is paused for 24 hours and the customer receives a message with a link to update the card."
        }
      />
      <Bubble variant="outline" name="Billing assistant" timestamp="10:33">
        <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
          <View
            style={{
              width: 30,
              height: 30,
              borderRadius: 8,
              backgroundColor: tokens.muted,
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <Icon name="file-text" size={15} color={tokens.mutedForeground} />
          </View>
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 13, lineHeight: 18, fontWeight: "500", color: tokens.foreground }}>
              invoice-run-q3.pdf
            </Text>
            <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>PDF · 248 KB</Text>
          </View>
        </View>
      </Bubble>
    </Column>
  );

  // section: Ghost/formatted content
  const ghost = (
    <Bubble variant="ghost" name="Billing assistant" timestamp="10:35">
      <View style={{ gap: 8 }}>
        <Text style={{ fontSize: 16, lineHeight: 22, fontWeight: "600", color: tokens.foreground }}>
          What changed in this run
        </Text>
        <Text style={{ fontSize: 14, lineHeight: 20, color: tokens.foreground }}>
          Both failures now <Text style={{ fontWeight: "600" }}>retry automatically</Text> before the run
          closes. The limit is read from{" "}
          <Text style={{ fontFamily: MONO, fontSize: 12, color: tokens.primary }}>retry_limit</Text> in the
          billing settings.
        </Text>
        <View style={{ gap: 3 }}>
          <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.foreground }}>
            • Card declines retry once after 30 minutes.
          </Text>
          <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.foreground }}>
            • Every attempt is written to the invoice log.
          </Text>
          <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.foreground }}>
            • A second failure pauses the account for 24 hours.
          </Text>
        </View>
        <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>
          Updated 5 minutes ago.
        </Text>
      </View>
    </Bubble>
  );

  // section: Interactive bubbles/actions
  const pressed = signal("No action yet");
  const interactive = (
    <Column gap={10}>
      <Bubble
        variant="tinted"
        name="Billing assistant"
        timestamp="10:38"
        content="Two invoices failed to charge. Retry both now?"
        accessibilityLabel="Retry both invoices now"
        onPress={() => pressed.set("Retry both invoices now")}
      />
      <Row gap={12}>
        <Bubble
          variant="ghost"
          accessibilityLabel="Open the failed invoice log"
          onPress={() => pressed.set("Open the failed invoice log")}
        >
          <Text style={{ fontSize: 13, lineHeight: 18, fontWeight: "500", color: tokens.link }}>
            Open the failed invoice log
          </Text>
        </Bubble>
        <Bubble
          variant="ghost"
          accessibilityLabel="Dismiss"
          onPress={() => pressed.set("Dismissed")}
        >
          <Text style={{ fontSize: 13, lineHeight: 18, color: tokens.mutedForeground }}>Dismiss</Text>
        </Bubble>
      </Row>
      <Text style={{ fontSize: 12, lineHeight: 16, color: tokens.mutedForeground }}>
        Last action: {pressed()}
      </Text>
    </Column>
  );

  // section: Groups
  const thread = (
    <BubbleGroup gap={14}>
      <BubbleGroup gap={4}>
        <Bubble
          variant="secondary"
          name="Ada Lovelace"
          timestamp="10:24"
          content="Here is the Q3 summary."
        />
        <Bubble
          variant="secondary"
          content="Revenue is up 12%, and refunds are down for the second month."
        />
      </BubbleGroup>
      <BubbleGroup gap={4} align="end">
        <Bubble
          align="end"
          variant="filled"
          name="You"
          timestamp="10:26"
          content="Perfect — can you send the full report?"
        />
        <Bubble align="end" variant="filled" content="Attaching it now." />
      </BubbleGroup>
    </BubbleGroup>
  );

  // section: Reactions and interactive content
  const reactionList = signal<BubbleReaction[]>([
    { emoji: "👍", count: 4, added: true },
    { emoji: "🎉", count: 2 },
    { emoji: "❤️", count: 1 },
    { emoji: "🚀", count: 1 },
  ]);
  const toggleReaction = (reaction: BubbleReactionItem) => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    reactionList.set(
      reactionList().map((item) => {
        const current = typeof item === "string" ? { emoji: item } : item;
        if (current.emoji !== reaction.emoji) return item;
        const added = !current.added;
        return { ...current, added, count: Math.max(0, (current.count ?? 0) + (added ? 1 : -1)) };
      }),
    );
  };
  const reactions = (
    <Column gap={10}>
      <Bubble
        variant="outline"
        name="Ada Lovelace"
        timestamp="10:41"
        content="The report is ready. I added the refund breakdown to the last page."
        reactions={reactionList()}
        maxReactions={3}
        onReactionToggle={toggleReaction}
      />
      <Row gap={8}>
        <BubbleReactions reactions={reactionList()} max={2} onToggle={toggleReaction} />
        <Button label="Add reaction" size="small" variant="ghost" onPress={() => {}} />
      </Row>
    </Column>
  );

  // section: Delivered/status
  const delivered = (
    <BubbleGroup gap={8} align="end">
      <Bubble
        align="end"
        variant="filled"
        content="The report is on its way."
        footer={
          <View style={{ flexDirection: "row", alignItems: "center", gap: 3 }}>
            <Icon name="check" size={12} color={tokens.success} />
            <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>
              Delivered · 10:42
            </Text>
          </View>
        }
      />
      <Bubble
        align="end"
        variant="secondary"
        content="One invoice is still retrying."
        footer={
          <View style={{ flexDirection: "row", alignItems: "center", gap: 3 }}>
            <Icon name="clock" size={12} color={tokens.mutedForeground} />
            <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>
              Sending · 10:43
            </Text>
          </View>
        }
      />
    </BubbleGroup>
  );

  // section: Custom styling and theme tokens
  const custom = (
    <Column gap={10}>
      <Bubble
        variant="outline"
        name="Workspace"
        timestamp="10:44"
        content="A bubble with application-owned surface styling."
        style={{ maxWidth: "88%" }}
        surfaceStyle={{
          borderRadius: RADIUS_XL,
          borderColor: tokens.ring,
          backgroundColor: tokens.groupBox,
          paddingHorizontal: 16,
          paddingVertical: 12,
        }}
      />
      <Bubble
        variant="tinted"
        content="Tinted surfaces use the primary token at a low alpha."
        style={{ maxWidth: "78%" }}
      />
    </Column>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Bubble owns the surface, sender metadata, folding and reactions. The message record and the
        commands behind an action stay with the application.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Anatomy and basic usage">{basic}</ExampleSection>
      <ExampleSection index={2} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={3} title="Variants">{variants}</ExampleSection>
      <ExampleSection index={4} title="Rich content and long messages">{rich}</ExampleSection>
      <ExampleSection index={5} title="Ghost/formatted content">{ghost}</ExampleSection>
      <ExampleSection index={6} title="Interactive bubbles/actions">{interactive}</ExampleSection>
      <ExampleSection index={7} title="Groups">{thread}</ExampleSection>
      <ExampleSection index={8} title="Reactions and interactive content">{reactions}</ExampleSection>
      <ExampleSection index={9} title="Delivered/status">{delivered}</ExampleSection>
      <ExampleSection index={10} title="Custom styling and theme tokens">{custom}</ExampleSection>
    </Page>
  );
}
