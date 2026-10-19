//! `card` — a bordered content surface with media, header actions, body and
//! footer. The last sections follow the ReUI card compositions.
import { signal, Text, View } from "nsh";
import cover from "../../examples/assets/cover.png";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Card } from "../components/card";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { Menu, type MenuItem } from "../components/menu";
import { Popover } from "../components/popover";
import { alpha, tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const email = signal("");
  const password = signal("");
  const action = signal("No action yet");
  const menuOpen = signal(false);
  const menuItems: MenuItem[] = [
    { label: "Edit card", icon: "pencil", onSelect: () => { action.set("Edit card"); menuOpen.set(false); } },
    { label: "Duplicate", icon: "copy", onSelect: () => { action.set("Duplicated"); menuOpen.set(false); } },
    { separator: true },
    { label: "Delete", icon: "trash-2", onSelect: () => { action.set("Deleted"); menuOpen.set(false); } },
  ];

  // section: Usage
  const usage = (
    <Card
      title="Project update"
      description="The weekly summary is ready to review."
      badge="New"
      style={{ maxWidth: 360 }}
    >
      <Text style={{ fontSize: 14, color: tokens.foreground }}>
        You walked 42 km this week, eight more than last week.
      </Text>
    </Card>
  );

  // section: Default
  const normal = (
    <Card
      title="Design system"
      description="A shared foundation for every framework."
      badge="Updated"
      footer={
        <Button label="Open" size="small" onPress={() => action.set("Opened design system")} />
      }
      style={{ maxWidth: 360 }}
    >
      <Text style={{ fontSize: 14, lineHeight: 20, color: tokens.foreground }}>
        Tokens, controls and documentation in one place.
      </Text>
    </Card>
  );

  // section: Stacked
  const stacked = (
    <Column gap={12} style={{ maxWidth: 360 }}>
      <Card title="Design system" description="Tokens, controls and documentation.">
        <Text style={{ fontSize: 14, color: tokens.foreground }}>Shared by every framework.</Text>
      </Card>
      <Card title="Native shell" description="The same controls written in TSX." badge="Draft">
        <Text style={{ fontSize: 14, color: tokens.foreground }}>Runs on every NSH platform.</Text>
      </Card>
    </Column>
  );

  // section: Login
  const login = (
    <Card
      title="Welcome back"
      description="Sign in to continue to your workspace."
      footer={
        <>
          <Button label="Cancel" variant="ghost" onPress={() => action.set("Cancelled")} />
          <Button label="Sign in" variant="primary" onPress={() => action.set("Signed in")} />
        </>
      }
      style={{ maxWidth: 380 }}
    >
      <View style={{ gap: 10 }}>
        <Input
          value={email()}
          placeholder="Email"
          prefixIcon="mail"
          onChangeText={(next) => email.set(next)}
        />
        <Input
          value={password()}
          placeholder="Password"
          prefixIcon="lock"
          masked
          maskToggle
          onChangeText={(next) => password.set(next)}
        />
      </View>
    </Card>
  );

  // section: Size
  const size = (
    <Row gap={12}>
      <Card size="small" title="Small card" description="A tighter inset and title." style={{ width: 250 }}>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>Small is 12 pt of padding.</Text>
      </Card>
      <Card size="medium" title="Medium card" description="The default inset." style={{ width: 250 }}>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>Medium is 16 pt of padding.</Text>
      </Card>
    </Row>
  );

  // section: Spacing
  const spacing = (
    <Row gap={12}>
      <Card title="Inset 8" inset={8} style={{ width: 220 }}>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>A compact card.</Text>
      </Card>
      <Card title="Inset 24" inset={24} style={{ width: 240 }}>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>A roomy card.</Text>
      </Card>
    </Row>
  );

  // section: Image
  const image = (
    <Card
      cover={cover}
      title="Mountain light"
      description="A cover image above the header."
      style={{ maxWidth: 360 }}
    >
      <Text style={{ fontSize: 14, color: tokens.foreground }}>
        The cover is clipped by the card's rounded corner.
      </Text>
    </Card>
  );

  // section: Card with link
  const link = (
    <Card
      title="Need a help in Claim?"
      description="Go to this step by step guideline process on how to certify for your weekly benefits:"
      footer={
        <Button
          label="See our guideline"
          suffixIcon="external-link"
          variant="ghost"
          size="small"
          onPress={() => action.set("Opened guideline")}
        />
      }
      footerAlign="start"
      footerBordered
      style={{ maxWidth: 340 }}
    />
  );

  // section: Card with dropdown menu
  const dropdown = (
    <Card
      title="Need a help in Claim?"
      description="Go to this step by step guideline process on how to certify for your weekly benefits:"
      actions={
        <Popover
          open={menuOpen()}
          onOpenChange={(next) => menuOpen.set(next)}
          trigger={
            <View
              accessibilityLabel="Card actions"
              style={{ width: 26, height: 26, alignItems: "center", justifyContent: "center", borderRadius: 6 }}
            >
              <Icon name="ellipsis" size={16} color={tokens.mutedForeground} />
            </View>
          }
        >
          <Menu items={menuItems} style={{ minWidth: 180 }} />
        </Popover>
      }
      footer={
        <Badge
          label="@shadcn"
          variant="secondary"
          leading={<Avatar name="shadcn" size="xs" />}
        />
      }
      footerAlign="start"
      footerBordered
      style={{ maxWidth: 340 }}
    />
  );

  // section: Card with image
  const imageCard = (
    <Card
      cover={cover}
      coverHeight={150}
      badges={
        <View style={{ flexDirection: "row", alignItems: "center", justifyContent: "space-between", width: "100%" }}>
          <Badge label="Trending" icon="trending-up" variant="secondary" />
          <Badge label="Featured" icon="sparkles" variant="outline" />
        </View>
      }
      title="Simplifying your workflow from day one."
      description="Manage your tasks, projects, and team in one place."
      footer={
        <Button
          label="Get Started"
          suffixIcon="arrow-right"
          style={{ width: "100%" }}
          onPress={() => action.set("Get started")}
        />
      }
      style={{ maxWidth: 340 }}
    />
  );

  // section: Image scale hover effect
  const imageHover = (
    <Card
      cover={cover}
      coverHeight={300}
      imageHoverScale
      inset={0}
      coverOverlay={
        <View style={{ gap: 4 }}>
          <Text style={{ fontSize: 22, lineHeight: 28, fontWeight: "700", color: tokens.foreground }}>
            Image Scale Effect
          </Text>
          <Text style={{ fontSize: 13, lineHeight: 18, color: alpha(tokens.foreground, 0.8) }}>
            This card features a smooth image scaling effect and background overlay on hover.
          </Text>
        </View>
      }
      style={{ width: 340 }}
    />
  );

  // section: Profile card
  const profile = (
    <Card style={{ maxWidth: 340 }}>
      <View style={{ flexDirection: "row", alignItems: "center", gap: 10 }}>
        <Avatar name="Lena Ortiz" size="lg" />
        <View style={{ flex: 1, gap: 2 }}>
          <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Lena Ortiz</Text>
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Billing support · online</Text>
        </View>
        <Badge label="Pro" variant="info" />
      </View>
      <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
        Replies in a few minutes. Handles billing, refunds and plan changes.
      </Text>
      <Row gap={8}>
        <Button label="Message" size="small" onPress={() => action.set("Message Lena")} />
        <Button label="View profile" size="small" variant="ghost" onPress={() => action.set("View Lena")} />
      </Row>
    </Card>
  );

  // section: Stats card
  const stats = (
    <Card style={{ maxWidth: 260 }}>
      <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
        <Text style={{ flex: 1, fontSize: 12, fontWeight: "500", color: tokens.mutedForeground }}>
          Revenue this month
        </Text>
        <Badge label="+12.4%" variant="success" icon="trending-up" />
      </View>
      <Text style={{ fontSize: 30, lineHeight: 36, fontWeight: "700", color: tokens.foreground }}>
        $48,290
      </Text>
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Compared with $42,960 last month.</Text>
    </Card>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Card is a presentational surface. The body is yours; the header, media, actions and
        footer are the parts the catalog standardises.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Default">{normal}</ExampleSection>
      <ExampleSection index={2} title="Stacked">{stacked}</ExampleSection>
      <ExampleSection index={3} title="Login">{login}</ExampleSection>
      <ExampleSection index={4} title="Size">{size}</ExampleSection>
      <ExampleSection index={5} title="Spacing">{spacing}</ExampleSection>
      <ExampleSection index={6} title="Image">{image}</ExampleSection>
      <ExampleSection index={7} title="Card with link">{link}</ExampleSection>
      <ExampleSection index={8} title="Card with dropdown menu">{dropdown}</ExampleSection>
      <ExampleSection index={9} title="Card with image">{imageCard}</ExampleSection>
      <ExampleSection index={10} title="Image scale hover effect">{imageHover}</ExampleSection>
      <ExampleSection index={11} title="Profile card">{profile}</ExampleSection>
      <ExampleSection index={12} title="Stats card">{stats}</ExampleSection>
      <Status>{action()}</Status>
    </Page>
  );
}
