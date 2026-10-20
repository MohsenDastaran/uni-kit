//! `empty` — composed empty states with illustrations, titles, copy and actions.
import { Text, View } from "nsh";
import { Avatar, AvatarGroup } from "../components/avatar";
import { Button } from "../components/button";
import { Empty } from "../components/empty";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const productArt = (
    <View style={{ position: "relative", width: 170, height: 86, alignItems: "center", justifyContent: "flex-end" }}>
      <View style={{ position: "absolute", top: 0, width: 126, height: 48, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, opacity: 0.45 }} />
      <View style={{ position: "absolute", top: 10, width: 140, height: 58, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, opacity: 0.7 }} />
      <View style={{ width: 154, height: 66, padding: 8, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.background, flexDirection: "row", gap: 7 }}>
        <View style={{ width: 24, height: 24, borderRadius: 5, backgroundColor: tokens.muted }} />
        <View style={{ flex: 1, gap: 5, justifyContent: "center" }}>
          <View style={{ height: 5, width: "78%", borderRadius: 3, backgroundColor: tokens.muted }} />
          <View style={{ height: 4, width: "55%", borderRadius: 2, backgroundColor: tokens.muted }} />
        </View>
      </View>
    </View>
  );

  const fileArt = (
    <View style={{ position: "relative", width: 170, height: 80, alignItems: "center", justifyContent: "flex-end" }}>
      <View style={{ position: "absolute", top: 0, width: 118, height: 42, borderRadius: 7, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, opacity: 0.45 }} />
      <View style={{ width: 154, height: 54, padding: 8, borderRadius: 7, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.background, flexDirection: "row", alignItems: "center", gap: 8 }}>
        <View style={{ width: 22, height: 22, borderRadius: 5, backgroundColor: tokens.muted }} />
        <View style={{ flex: 1, gap: 5 }}>
          <View style={{ height: 5, width: "72%", borderRadius: 3, backgroundColor: tokens.muted }} />
          <View style={{ height: 4, width: "44%", borderRadius: 2, backgroundColor: tokens.muted }} />
        </View>
        <Icon name="search" size={12} color={tokens.mutedForeground} />
      </View>
    </View>
  );

  const chatArt = (
    <View style={{ position: "relative", width: 160, height: 82 }}>
      <View style={{ position: "absolute", top: 0, left: 12, width: 76, height: 42, borderRadius: 9, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, padding: 7, gap: 4 }}>
        <View style={{ width: "65%", height: 4, borderRadius: 2, backgroundColor: tokens.mutedForeground, opacity: 0.5 }} />
        <View style={{ width: "45%", height: 4, borderRadius: 2, backgroundColor: tokens.mutedForeground, opacity: 0.35 }} />
      </View>
      <View style={{ position: "absolute", top: 38, right: 12, width: 86, height: 42, borderRadius: 9, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.background, padding: 7, gap: 4 }}>
        <View style={{ width: "70%", height: 4, borderRadius: 2, backgroundColor: tokens.mutedForeground, opacity: 0.5 }} />
        <View style={{ width: "50%", height: 4, borderRadius: 2, backgroundColor: tokens.mutedForeground, opacity: 0.35 }} />
      </View>
    </View>
  );

  const paymentArt = (
    <View style={{ width: 170, height: 82, alignItems: "center", justifyContent: "center" }}>
      <View style={{ width: 148, height: 74, borderRadius: 10, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, padding: 9, gap: 7 }}>
        <View style={{ flexDirection: "row", alignItems: "center", gap: 5 }}>
          <View style={{ width: 14, height: 10, borderRadius: 2, backgroundColor: tokens.mutedForeground, opacity: 0.5 }} />
          <View style={{ flex: 1 }} />
          <Icon name="activity" size={11} color={tokens.mutedForeground} />
        </View>
        <View style={{ flexDirection: "row", gap: 3 }}>
          {[0, 1, 2, 3].map(() => <View style={{ width: 8, height: 4, borderRadius: 2, backgroundColor: tokens.mutedForeground, opacity: 0.45 }} />)}
        </View>
        <View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}>
          <View style={{ width: 20, height: 6, borderRadius: 3, backgroundColor: tokens.mutedForeground, opacity: 0.3 }} />
          <View style={{ flex: 1 }} />
          <View style={{ width: 18, height: 18, borderRadius: 9, borderWidth: 1, borderColor: tokens.mutedForeground, opacity: 0.4 }} />
          <View style={{ width: 18, height: 18, borderRadius: 9, borderWidth: 1, borderColor: tokens.mutedForeground, opacity: 0.4 }} />
        </View>
      </View>
    </View>
  );

  // section: Usage
  const usage = (
    <Empty
      icon="folder"
      title="No projects yet"
      description="Create your first project to get started."
      action={<Button label="Create project" variant="primary" onPress={() => {}} />}
    />
  );

  // section: Basic usage
  const basic = (
    <Empty
      icon="folder"
      title="No projects yet"
      description="You haven't created any projects yet. Get started by creating your first project."
      action={
        <Row gap={6}>
          <Button label="Create project" variant="primary" onPress={() => {}} />
          <Button label="Import project" variant="secondary" onPress={() => {}} />
        </Row>
      }
      style={{ maxWidth: 420 }}
    >
      <Text style={{ fontSize: 11, color: tokens.primary }}>Learn more ↗</Text>
    </Empty>
  );

  // section: No products
  const noProducts = (
    <Empty
      media={productArt}
      title="No products"
      description="No data here yet. We will notify you when there's an update."
      style={{ maxWidth: 360 }}
    />
  );

  // section: Search empty
  const searchEmpty = (
    <Empty
      media={fileArt}
      title="Looking for something?"
      description="Type a keyword and we'll search through files, folders, hashes, and transcripts."
      style={{ maxWidth: 380 }}
    />
  );

  // section: No messages
  const noMessages = (
    <Empty
      media={chatArt}
      title="No messages yet"
      description="Start a conversation with your team. Messages will appear here."
      action={<Button label="Send a message" variant="secondary" onPress={() => {}} />}
      style={{ maxWidth: 340 }}
    />
  );

  // section: No payment methods
  const noPayments = (
    <Empty
      media={paymentArt}
      title="No payment methods"
      description="Add a payment method to start making transactions securely."
      action={<Button label="Add payment method" variant="secondary" onPress={() => {}} />}
      style={{ maxWidth: 350 }}
    />
  );

  // section: Outline
  const outline = (
    <Empty
      variant="outline"
      media={<View style={{ width: 34, height: 34, borderRadius: 17, alignItems: "center", justifyContent: "center", backgroundColor: tokens.muted }}><Icon name="search" size={16} color={tokens.mutedForeground} /></View>}
      title="No results found"
      description="No results found for your search. Try adjusting your search terms."
      action={<Button label="Try again" variant="secondary" size="small" onPress={() => {}} />}
      style={{ maxWidth: 360 }}
    />
  );

  // section: Background
  const background = (
    <Empty
      variant="background"
      icon="file"
      title="Nothing to see here"
      description="No posts have been created yet. Get started by creating your first post."
      action={<Button label="New Post" variant="secondary" size="small" onPress={() => {}} />}
      style={{ maxWidth: 360 }}
    />
  );

  // section: Avatar
  const avatar = <Empty media={<Avatar name="Ada Lovelace" size="xl" />} title="No profile" description="Choose a profile to continue." />;

  // section: Avatar group
  const avatarGroup = <Empty media={<AvatarGroup items={[{ name: "Ada" }, { name: "Grace" }, { name: "Alan" }]} max={2} />} title="No team yet" description="Invite people to collaborate." />;

  // section: Inputs and custom content
  const custom = (
    <Empty variant="outline" dashed title="404 — Not Found" description="The page you're looking for doesn't exist. Try searching for what you need." style={{ maxWidth: 420 }}>
      <Input placeholder="Try searching for pages…" suffixIcon="search" onChangeText={() => {}} style={{ width: 300 }} />
      <Row gap={3}><Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Need help?</Text><Text style={{ fontSize: 11, color: tokens.primary }}>Contact support</Text></Row>
    </Empty>
  );

  // section: Constrained layouts
  const constrained = (
    <Empty
      variant="outline"
      dashed
      media={<View style={{ width: 30, height: 30, borderRadius: 15, alignItems: "center", justifyContent: "center", backgroundColor: tokens.muted }}><Icon name="folder" size={14} color={tokens.mutedForeground} /></View>}
      title="Nothing to see here"
      description="No posts have been created yet. Get started by creating your first post."
      action={<Button label="New Post" variant="secondary" size="small" onPress={() => {}} />}
      style={{ maxWidth: 360 }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>An empty state explains what is missing and offers the next action.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={2} title="No products">{noProducts}</ExampleSection>
      <ExampleSection index={3} title="Search empty">{searchEmpty}</ExampleSection>
      <ExampleSection index={4} title="No messages">{noMessages}</ExampleSection>
      <ExampleSection index={5} title="No payment methods">{noPayments}</ExampleSection>
      <ExampleSection index={6} title="Outline">{outline}</ExampleSection>
      <ExampleSection index={7} title="Background">{background}</ExampleSection>
      <ExampleSection index={8} title="Avatar">{avatar}</ExampleSection>
      <ExampleSection index={9} title="Avatar group">{avatarGroup}</ExampleSection>
      <ExampleSection index={10} title="Inputs and custom content">{custom}</ExampleSection>
      <ExampleSection index={11} title="Constrained layouts">{constrained}</ExampleSection>
    </Page>
  );
}
