//! `avatar` — images, initials, fallbacks, badges, rings, loading states and
//! overlapping groups, plus the compact compositions built from them.
import { signal, Text, View } from "nsh";
import cover from "../../examples/assets/cover.png";
import { Avatar, AvatarGroup } from "../components/avatar";
import { Button } from "../components/button";
import { Empty } from "../components/empty";
import { Icon } from "../components/icon";
import { Menu, type MenuItem } from "../components/menu";
import { Popover } from "../components/popover";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

const people = [
  { name: "Ada Lovelace" },
  { name: "Grace Hopper" },
  { name: "Alan Turing" },
  { name: "Katherine Johnson" },
];

const collaborators = [...people, { name: "Margaret Hamilton" }, { name: "Barbara Liskov" }];

export function Demo() {
  const action = signal("No interaction yet");
  // section: Usage
  const usage = (
    <Row gap={12}>
      <Avatar name="Ada Lovelace" size="lg" />
      <Avatar src={cover} name="Grace Hopper" size="lg" />
      <Avatar name="Alan Turing" size="lg" badges={[{ tone: "success" }]} />
    </Row>
  );
  // section: Image
  const image = (
    <Row gap={16} style={{ alignItems: "center" }}>
      <Avatar src={cover} name="Ada Lovelace" size="lg" />
      <Avatar src={cover} name="Ada Lovelace" size="xl" shape="rounded" />
      <Avatar src={cover} name="Ada Lovelace" size="xl" ring="plain" />
    </Row>
  );
  // section: Fallback
  const fallback = (
    <Row gap={12}>
      <Avatar name="Ada Lovelace" />
      <Avatar name="Grace Hopper" />
      <Avatar fallback="?" />
      <Avatar icon="bot" />
    </Row>
  );
  // section: Avatar Sizes
  const sizes = (
    <Row gap={12} style={{ alignItems: "center" }}>
      <Avatar name="AL" size="xs" />
      <Avatar name="AL" size="sm" />
      <Avatar name="AL" size="md" />
      <Avatar name="AL" size="lg" />
      <Avatar name="AL" size="xl" />
    </Row>
  );
  // section: Group with Limit
  const limit = (
    <View style={{ gap: 10 }}>
      <AvatarGroup items={people} max={3} size="lg" />
      <Caption>Three faces are visible and the +1 chip holds the rest.</Caption>
    </View>
  );
  // section: Group with add button
  const add = (
    <AvatarGroup
      items={collaborators}
      max={3}
      size="lg"
      addon={
        <Button
          icon="plus"
          size="large"
          rounded={20}
          accessibilityLabel="Add collaborator"
          onPress={() => action.set("Add collaborator")}
        />
      }
    />
  );
  // section: Pilled small avatar button
  const pilled = (
    <Row gap={8}>
      <Button
        size="small"
        rounded={12}
        accessibilityLabel="Open Ada Lovelace's profile"
        onPress={() => action.set("Opened Ada Lovelace")}
        style={{ paddingHorizontal: 6, gap: 6 }}
      >
        <>
          <Avatar size="xs" name="Ada Lovelace" />
          <Text style={{ fontSize: 12, fontWeight: "500", color: tokens.foreground }}>Ada</Text>
        </>
      </Button>
      <Button
        size="small"
        rounded={12}
        accessibilityLabel="Open Grace Hopper's profile"
        onPress={() => action.set("Opened Grace Hopper")}
        style={{ paddingHorizontal: 6, gap: 6 }}
      >
        <>
          <Avatar size="xs" src={cover} name="Grace Hopper" />
          <Text style={{ fontSize: 12, fontWeight: "500", color: tokens.foreground }}>Grace</Text>
        </>
      </Button>
    </Row>
  );
  // section: Compact avatar dropdown
  const dropdownOpen = signal(false);
  const choose = (next: string) => {
    action.set(next);
    dropdownOpen.set(false);
  };
  const menuItems: MenuItem[] = [
    { label: "View profile", icon: "user", shortcut: "⌘P", onSelect: () => choose("View profile") },
    { label: "Message", icon: "message-circle", onSelect: () => choose("Message Ada") },
    { separator: true },
    { label: "Sign out", icon: "log-out", onSelect: () => choose("Sign out") },
  ];
  const dropdown = (
    <Popover
      open={dropdownOpen()}
      onOpenChange={(next) => dropdownOpen.set(next)}
      trigger={
        <View
          style={{
            flexDirection: "row",
            alignItems: "center",
            gap: 6,
            height: 32,
            paddingStart: 4,
            paddingEnd: 8,
            borderRadius: 999,
            borderWidth: 1,
            borderColor: tokens.border,
            backgroundColor: tokens.background,
          }}
        >
          <Avatar size="sm" src={cover} name="Ada Lovelace" />
          <Text style={{ fontSize: 12, fontWeight: "500", color: tokens.foreground }}>Ada Lovelace</Text>
          <Icon name="chevron-down" size={14} color={tokens.mutedForeground} />
        </View>
      }
    >
      <Menu style={{ minWidth: 220, borderWidth: 0, paddingVertical: 0 }} items={menuItems} />
    </Popover>
  );
  // section: Avatar with custom badge
  const customBadge = (
    <Row gap={16} style={{ alignItems: "center" }}>
      <Avatar
        name="Ada Lovelace"
        size="xl"
        badges={[{ icon: "check", tone: "info", placement: "bottom-end", accessibilityLabel: "Verified" }]}
      />
      <Avatar
        src={cover}
        name="Grace Hopper"
        size="xl"
        badges={[{ icon: "star", tone: "warning", placement: "bottom-end", accessibilityLabel: "Favorite" }]}
      />
      <Avatar
        name="Alan Turing"
        size="xl"
        badges={[{ icon: "shield", tone: "success", placement: "bottom-end", accessibilityLabel: "Admin" }]}
      />
    </Row>
  );
  // section: Avatar with multiple badges
  const multipleBadges = (
    <Row gap={16} style={{ alignItems: "center" }}>
      <Avatar name="Ada Lovelace" size="xl" badges={[{ tone: "danger" }, { tone: "success" }, { tone: "warning" }]} />
      <Avatar
        src={cover}
        name="Grace Hopper"
        size="xl"
        badges={[{ icon: "bell", tone: "danger" }, { tone: "success" }]}
      />
      <Avatar
        name="Alan Turing"
        size="lg"
        badges={[
          { tone: "success", placement: "bottom-end" },
          { tone: "danger", placement: "top-start" },
        ]}
      />
    </Row>
  );
  // section: Loading state
  const loading = (
    <Row gap={16} style={{ alignItems: "center" }}>
      <Avatar size="sm" loading />
      <Avatar name="Ada Lovelace" size="lg" loading />
      <Avatar src={cover} name="Grace Hopper" size="xl" loading />
    </Row>
  );
  // section: Ring and gradient ring
  const rings = (
    <Row gap={16} style={{ alignItems: "center" }}>
      <Avatar name="Ada Lovelace" size="xl" ring="plain" />
      <Avatar src={cover} name="Grace Hopper" size="xl" ring="gradient" />
      <Avatar name="Alan Turing" size="lg" shape="rounded" ring="gradient" />
      <Avatar name="Katherine Johnson" size="xl" ring="plain" ringWidth={3} />
    </Row>
  );
  // section: Avatar inside an empty state
  const empty = (
    <Empty
      variant="outline"
      media={<AvatarGroup items={people} max={2} size="lg" />}
      title="No active collaborators"
      description="Invite someone to this workspace and their presence will show up here."
      action={<Button label="Invite people" icon="users" onPress={() => action.set("Invite people")} />}
      style={{ maxWidth: 460, width: "100%" }}
    />
  );
  // section: end
  return (
    <Page>
      <Caption>An avatar shows an image, initials, a fallback or an icon, with optional badges and a ring; a group overlaps several and keeps the rest behind an overflow count.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Image">{image}</ExampleSection>
      <ExampleSection index={2} title="Fallback">{fallback}</ExampleSection>
      <ExampleSection index={3} title="Avatar Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Group with Limit">{limit}</ExampleSection>
      <ExampleSection index={5} title="Group with add button">{add}</ExampleSection>
      <ExampleSection index={6} title="Pilled small avatar button">{pilled}</ExampleSection>
      <ExampleSection index={7} title="Compact avatar dropdown">{dropdown}</ExampleSection>
      <ExampleSection index={8} title="Avatar with custom badge">{customBadge}</ExampleSection>
      <ExampleSection index={9} title="Avatar with multiple badges">{multipleBadges}</ExampleSection>
      <ExampleSection index={10} title="Loading state">{loading}</ExampleSection>
      <ExampleSection index={11} title="Ring and gradient ring">{rings}</ExampleSection>
      <ExampleSection index={12} title="Avatar inside an empty state">{empty}</ExampleSection>
      <Status>{action()}</Status>
    </Page>
  );
}
