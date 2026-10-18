//! `avatar` — images, initials, fallbacks and overlapping groups.
import { Text, View } from "nsh";
import cover from "../../examples/assets/cover.png";
import { Avatar, AvatarGroup } from "../components/avatar";
import { Icon } from "../components/icon";
import { Button } from "../components/button";
import { alpha, RADIUS_LG, tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const people = [
  { name: "Ada Lovelace" },
  { name: "Grace Hopper" },
  { name: "Alan Turing" },
  { name: "Katherine Johnson" },
];

export function Demo() {
  // section: Usage
  const usage = <Avatar name="Ada Lovelace" size="lg" />;
  // section: Image
const image = <Avatar src={cover} name="Ada Lovelace" size="lg" />;
  // section: Fallback
const fallback = (
  <Row gap={12}>
    <Avatar name="Ada Lovelace" />
    <Avatar name="Grace Hopper" />
    <Avatar fallback="?" />
    <Avatar icon="bot" />
  </Row>
);
  // section: Group
const group = <AvatarGroup items={people} max={3} />;
  // section: Custom shape
const shape = (
  <Row gap={12}>
    <Avatar name="Ada Lovelace" shape="circle" />
    <Avatar name="Ada Lovelace" shape="rounded" />
    <Avatar name="Ada Lovelace" shape="square" />
  </Row>
);
  // section: Custom style
  const style = (
    <Row gap={12}>
      <Avatar name="Ada Lovelace" color={tokens.info} />
      <Avatar name="Ada Lovelace" color={tokens.success} />
      <Avatar name="Ada Lovelace" color={tokens.warning} />
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
  // section: Team Display
  const team = (
    <View
      style={{
        padding: 16,
        gap: 12,
        borderRadius: RADIUS_LG,
        borderWidth: 1,
        borderColor: tokens.border,
        backgroundColor: tokens.background,
      }}
    >
      <View style={{ flexDirection: "row", alignItems: "center", justifyContent: "space-between" }}>
        <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Design team</Text>
        <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>12 members</Text>
      </View>
      <AvatarGroup items={[...people, { name: "Margaret Hamilton" }]} max={5} size="lg" />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
        Ada, Grace, Alan and 9 others are in this workspace.
      </Text>
    </View>
  );

  // section: User Profile Header
  const profile = (
    <View
      style={{
        padding: 20,
        gap: 16,
        borderRadius: RADIUS_LG,
        borderWidth: 1,
        borderColor: alpha(tokens.primary, 0.12),
        backgroundImage: `linear-gradient(135deg, ${alpha(tokens.primary, 0.06)}, ${alpha(tokens.success, 0.10)})`,
      }}
    >
      <Row gap={12} style={{ alignItems: "center" }}>
        <Avatar name="Ada Lovelace" size="xl" />
        <View style={{ flex: 1, gap: 2 }}>
          <Text style={{ fontSize: 16, fontWeight: "600", color: tokens.foreground }}>Ada Lovelace</Text>
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Mathematician · London</Text>
        </View>
        <View style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: tokens.success }} />
      </Row>
      <Row gap={8}>
        <Button label="Follow" variant="primary" icon="users" onPress={() => {}} />
        <Button label="Message" variant="secondary" icon="message-circle" onPress={() => {}} />
      </Row>
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>An avatar shows an image, initials, a fallback or an icon; a group overlaps several with an overflow count.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Image">{image}</ExampleSection>
      <ExampleSection index={2} title="Fallback">{fallback}</ExampleSection>
      <ExampleSection index={3} title="Group">{group}</ExampleSection>
      <ExampleSection index={4} title="Custom shape">{shape}</ExampleSection>
      <ExampleSection index={5} title="Custom style">{style}</ExampleSection>
      <ExampleSection index={6} title="Avatar Sizes">{sizes}</ExampleSection>
      <ExampleSection index={7} title="Team Display">{team}</ExampleSection>
      <ExampleSection index={8} title="User Profile Header">{profile}</ExampleSection>
    </Page>
  );
}
