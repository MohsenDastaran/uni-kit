//! `collapsible` — a header that reveals content.
import { signal, Text, View } from "nsh";
import { Canvas } from "nsh/canvas";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Card } from "../components/card";
import { Collapsible } from "../components/collapsible";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { Switch } from "../components/switch";
import { Tag } from "../components/tag";
import { alpha, SPACING_SM, tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const usageOpen = signal(true);
  const email = signal("james@alignui.com");

  // section: Usage
  const qrPattern = Array.from({ length: 64 }, (_, index) => {
    const row = Math.floor(index / 8);
    const col = index % 8;
    const finder = (row < 3 && col < 3) || (row < 3 && col > 4) || (row > 4 && col < 3);
    return finder || (index * 13 + row * 7 + col * 3) % 11 < 5;
  });
  const qr = (
    <Canvas
      style={{ width: 76, height: 76, borderRadius: 8, flexShrink: 0 }}
      accessibilityRole="image"
      accessibilityLabel="Point of sale QR code"
      draw={(ctx) => {
        ctx.fillStyle = tokens.foreground;
        ctx.fillRect(0, 0, 76, 76);
        ctx.fillStyle = tokens.background;
        for (let row = 0; row < 8; row += 1) {
          for (let col = 0; col < 8; col += 1) {
            if (qrPattern[row * 8 + col]) ctx.fillRect(6 + col * 8, 6 + row * 8, 8, 8);
          }
        }
      }}
    />
  );
  const usage = (
    <Collapsible
      bordered
      open={usageOpen()}
      onOpenChange={(next) => usageOpen.set(next)}
      title="Add products"
      description="Set up the catalogue and the tools around it."
      leading={<Icon name="circle-check" size={20} color={tokens.success} />}
      badge="Ready"
      badgeTone="success"
    >
      <Column gap={8}>
        <Collapsible
          bordered
          size="small"
          defaultOpen
          title="Get the point of sale application"
          description="Scan the QR code or send yourself the link."
          leading={<Icon name="circle-dot" size={18} color={tokens.warning} />}
        >
          <Row gap={14} style={{ alignItems: "flex-start" }}>
            <View style={{ flex: 1, gap: 6 }}>
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                The mobile app is where you’ll manage orders, track inventory, and view
                analytics on the go.
              </Text>
              <Row gap={6}>
                <Input
                  value={email()}
                  placeholder="Email address"
                  onChangeText={(next) => email.set(next)}
                  style={{ flex: 1 }}
                  size="small"
                />
                <Button label="Send link" size="small" variant="primary" onPress={() => {}} />
              </Row>
            </View>
            {qr}
          </Row>
        </Collapsible>
        <Collapsible
          bordered
          size="small"
          title="Product price & stock"
          description="Prices, variants and the low-stock threshold."
          leading={<Icon name="refresh-cw" size={18} color={tokens.mutedForeground} />}
        >
          <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
            Set the base price once and override it per variant. Stock counts update from
            every sale.
          </Text>
        </Collapsible>
      </Column>
    </Collapsible>
  );

  // section: Details
  const people = [
    {
      name: "Alex Johnson",
      role: "Admin",
      tone: "success" as const,
      note: "Alex has full administrative access to the platform, including billing management, user provisioning, and security configurations.",
    },
    {
      name: "Sarah Chen",
      role: "Viewer",
      tone: "neutral" as const,
      note: "Sarah can view projects and comment, but cannot change workspace settings.",
    },
    {
      name: "Michael Rodriguez",
      role: "Editor",
      tone: "info" as const,
      note: "Michael can create and edit project content, and invite collaborators.",
    },
  ];
  const details = (
    <Column gap={8}>
      {people.map((person, index) => (
        <Collapsible
          bordered
          defaultOpen={index === 0}
          leading={<Avatar name={person.name} size="md" />}
          title={person.name}
          badge={person.role}
          badgeTone={person.tone}
        >
          <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
            {person.note}
          </Text>
        </Collapsible>
      ))}
    </Column>
  );

  // section: Basic
  const basicOpen = signal(false);
  const basic = (
    <Collapsible
      bordered
      open={basicOpen()}
      onOpenChange={(next) => basicOpen.set(next)}
      trigger={
        <Row style={{ justifyContent: "space-between" }}>
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>
              Order #4189
            </Text>
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Arrives Thursday</Text>
          </View>
          <Tag label="Shipped" variant="success" size="xsmall" />
        </Row>
      }
    >
      <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
        A trigger keeps the summary visible while the details stay one press away. The
        chevron still belongs to the component.
      </Text>
    </Collapsible>
  );

  // section: Row trigger
  const rowTrigger = (
    <Collapsible
      bordered
      trigger={
        <Row style={{ justifyContent: "space-between" }}>
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>
              How do I reset my password?
            </Text>
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
              Account access · 2 min read
            </Text>
          </View>
          <Tag label="Help" variant="info" size="xsmall" />
        </Row>
      }
    >
      <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
        Open the sign-in page and choose “Forgot password”. The reset link is valid for
        thirty minutes and can be used once.
      </Text>
    </Collapsible>
  );

  // section: Settings
  const settingsOpen = signal(true);
  const push = signal(true);
  const weekly = signal(false);
  const settings = (
    <Collapsible
      bordered
      open={settingsOpen()}
      onOpenChange={(next) => settingsOpen.set(next)}
      icon="settings"
      title="Notifications"
      description="Choose what reaches you."
    >
      <Row style={{ justifyContent: "space-between" }}>
        <Text style={{ flex: 1, fontSize: 13, color: tokens.foreground }}>Push notifications</Text>
        <Switch checked={push()} onChange={(next) => push.set(next)} />
      </Row>
      <Row style={{ justifyContent: "space-between" }}>
        <Text style={{ flex: 1, fontSize: 13, color: tokens.foreground }}>Weekly digest</Text>
        <Switch checked={weekly()} onChange={(next) => weekly.set(next)} />
      </Row>
    </Collapsible>
  );

  // section: Row actions
  const rowActions = (
    <Collapsible
      bordered
      icon="lock"
      title="API keys"
      trigger={
        <Row style={{ justifyContent: "space-between" }}>
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>
              API keys
            </Text>
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Two active keys</Text>
          </View>
          <Button label="Add key" icon="plus" size="xsmall" variant="secondary" onPress={() => {}} />
        </Row>
      }
    >
      <Column gap={8}>
        <Row style={{ justifyContent: "space-between" }}>
          <Text style={{ fontSize: 13, fontFamily: "monospace", color: tokens.foreground }}>
            sk_live_••••8f2a
          </Text>
          <Tag label="Active" variant="success" size="xsmall" />
        </Row>
        <Row style={{ justifyContent: "space-between" }}>
          <Text style={{ fontSize: 13, fontFamily: "monospace", color: tokens.foreground }}>
            sk_test_••••1c9d
          </Text>
          <Tag label="Test" variant="warning" size="xsmall" />
        </Row>
      </Column>
    </Collapsible>
  );

  // section: Nested
  const foldersOpen = signal(true);
  const uiOpen = signal(false);
  const nested = (
    <Collapsible
      bordered
      open={foldersOpen()}
      onOpenChange={(next) => foldersOpen.set(next)}
      title="components"
      description="A folder reveals the folders inside it."
      icon={foldersOpen() ? "folder-open" : "folder"}
    >
      <Collapsible
        size="small"
        open={uiOpen()}
        onOpenChange={(next) => uiOpen.set(next)}
        title="ui"
        icon={uiOpen() ? "folder-open" : "folder"}
      >
        <Column gap={6}>
          <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>button.tsx</Text>
          <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>card.tsx</Text>
          <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>dialog.tsx</Text>
        </Column>
      </Collapsible>
      <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
        Each panel is the same component; the chevron turns with the panel it belongs to.
      </Text>
    </Collapsible>
  );

  // section: Profile
  const profileOpen = signal(true);
  const profile = (
    <Collapsible
      bordered
      open={profileOpen()}
      onOpenChange={(next) => profileOpen.set(next)}
      leading={<Avatar name="Jason Lee" size="md" />}
      title="Jason Lee"
      description="@huacnlee"
    >
      <Row style={{ justifyContent: "space-between" }}>
        <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Role</Text>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>Maintainer</Text>
      </Row>
      <Row style={{ justifyContent: "space-between" }}>
        <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Joined</Text>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>March 2023</Text>
      </Row>
    </Collapsible>
  );

  // section: Controlled
  const controlledOpen = signal(false);
  const controlled = (
    <Column gap={SPACING_SM}>
      <Collapsible
        bordered
        icon="settings"
        title="Advanced options"
        description="Only needed for a custom build."
        open={controlledOpen()}
        onOpenChange={(next) => controlledOpen.set(next)}
      >
        <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
          The caller holds the open state and receives every change, so a toolbar can drive
          the same panel.
        </Text>
      </Collapsible>
      <Row>
        <Button label="Open" size="small" onPress={() => controlledOpen.set(true)} />
        <Button label="Toggle" size="small" outline onPress={() => controlledOpen.set(!controlledOpen())} />
        <Button label="Close" size="small" variant="ghost" onPress={() => controlledOpen.set(false)} />
      </Row>
    </Column>
  );

  // section: Card surface
  const cardSurface = (
    <Card title="Billing" description="The plan, the invoice and the payment method.">
      <Collapsible
        bordered
        size="small"
        title="Payment method"
        description="Visa ending 4242"
        icon="lock"
        defaultOpen
      >
        <Row style={{ justifyContent: "space-between" }}>
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Next charge</Text>
          <Tag label="Active" variant="success" size="xsmall" />
        </Row>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
          1 September, 12.00 USD
        </Text>
      </Collapsible>
      <Collapsible bordered size="small" title="Invoices" description="Three receipts this year">
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
          A card holds the disclosure, so the surface and the panel read as one section.
        </Text>
      </Collapsible>
    </Card>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Collapsible is one quiet card: a plain chevron, a small status pill and content that
        appears immediately. Accordion stacks several of them.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Details">{details}</ExampleSection>
      <ExampleSection index={2} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={3} title="Row trigger">{rowTrigger}</ExampleSection>
      <ExampleSection index={4} title="Settings">{settings}</ExampleSection>
      <ExampleSection index={5} title="Row actions">{rowActions}</ExampleSection>
      <ExampleSection index={6} title="Nested">{nested}</ExampleSection>
      <ExampleSection index={7} title="Profile">{profile}</ExampleSection>
      <ExampleSection index={8} title="Controlled">{controlled}</ExampleSection>
      <ExampleSection index={9} title="Card surface">{cardSurface}</ExampleSection>
      <Status>{usageOpen() ? "open" : "closed"}</Status>
    </Page>
  );
}
