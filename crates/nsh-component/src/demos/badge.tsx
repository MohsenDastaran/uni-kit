//! `badge` — a count, a dot or an icon anchored to an avatar or an icon.
import { Text, View } from "nsh";
import { Badge } from "../components/badge";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const avatar = (
    <View
      style={{
        width: 40,
        height: 40,
        borderRadius: 20,
        backgroundColor: tokens.muted,
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>AL</Text>
    </View>
  );
  // section: Icon
  const icon = (
    <Row gap={20}>
      <Badge count={5}><Icon name="bell" size={26} color={tokens.foreground} /></Badge>
      <Badge icon="check"><Icon name="mail" size={26} color={tokens.foreground} /></Badge>
      <Badge count={12}>{avatar}</Badge>
    </Row>
  );
  // section: Count
  const count = (
    <Row gap={20}>
      <Badge count={3}>{avatar}</Badge>
      <Badge count={42}>{avatar}</Badge>
      <Badge count={120} max={99}>{avatar}</Badge>
    </Row>
  );
  // section: Badge icon
  const badgeIcon = (
    <Row gap={20}>
      <Badge icon="check" color={tokens.success}>{avatar}</Badge>
      <Badge icon="star" color={tokens.warning}>{avatar}</Badge>
      <Badge icon="circle-alert" color={tokens.danger}>{avatar}</Badge>
    </Row>
  );
  // section: Dot
  const dot = (
    <Row gap={20}>
      <Badge dot>{avatar}</Badge>
      <Badge dot color={tokens.success}>{avatar}</Badge>
      <Badge dot color={tokens.warning}>{avatar}</Badge>
    </Row>
  );
  // section: Color
  const color = (
    <Row gap={20}>
      <Badge count={2} color={tokens.info}>{avatar}</Badge>
      <Badge count={3} color={tokens.success}>{avatar}</Badge>
      <Badge count={4} color={tokens.warning}>{avatar}</Badge>
      <Badge count={5} color={tokens.danger}>{avatar}</Badge>
    </Row>
  );
  // section: Sizes
  const sizes = (
    <Row gap={20}>
      <Badge count={1} size="xsmall">{avatar}</Badge>
      <Badge count={2} size="small">{avatar}</Badge>
      <Badge count={3} size="medium">{avatar}</Badge>
      <Badge count={4} size="large">{avatar}</Badge>
    </Row>
  );
  // section: Complex Nested Badges
  const nested = (
    <Row gap={20}>
      <Badge count={9}>
        <Badge dot color={tokens.success}><Icon name="bell" size={30} color={tokens.foreground} /></Badge>
      </Badge>
      <Badge icon="check" color={tokens.success}><Badge count={4}>{avatar}</Badge></Badge>
    </Row>
  );
  // section: Notification Indicators
  const notifications = (
    <Row gap={24}>
      <Badge count={3}><Icon name="bell" size={28} color={tokens.foreground} /></Badge>
      <Badge count={8}><Icon name="message-circle" size={28} color={tokens.foreground} /></Badge>
      <Badge icon="info" color={tokens.info}><Icon name="inbox" size={28} color={tokens.foreground} /></Badge>
    </Row>
  );
  // section: Status Indicators
  const status = (
    <Row gap={16}>
      <Badge dot color={tokens.success}>{avatar}</Badge>
      <Badge dot color={tokens.warning}>{avatar}</Badge>
      <Badge dot color={tokens.danger}>{avatar}</Badge>
      <Badge dot color={tokens.mutedForeground}>{avatar}</Badge>
    </Row>
  );
  // section: Different Badge Positions
  const positions = (
    <Row gap={24}>
      <Badge count={1}>{avatar}</Badge>
      <Badge icon="check">{avatar}</Badge>
      <Badge dot>{avatar}</Badge>
    </Row>
  );
  // section: Count Formatting
  const formatting = (
    <Row gap={20}>
      <Badge count={0}>{avatar}</Badge>
      <Badge count={9} max={9}>{avatar}</Badge>
      <Badge count={99} max={99}>{avatar}</Badge>
      <Badge count={1000} max={99}>{avatar}</Badge>
    </Row>
  );
  // section: end
  return (
    <Page>
      <Caption>
        A badge is an overlay: it hangs past the top-right corner of the avatar or icon it
        marks, and leaves that child's layout alone.
      </Caption>
      <ExampleSection index={0} title="Icon">{icon}</ExampleSection>
      <ExampleSection index={1} title="Count">{count}</ExampleSection>
      <ExampleSection index={2} title="Badge icon">{badgeIcon}</ExampleSection>
      <ExampleSection index={3} title="Dot">{dot}</ExampleSection>
      <ExampleSection index={4} title="Color">{color}</ExampleSection>
      <ExampleSection index={5} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={6} title="Complex Nested Badges">{nested}</ExampleSection>
      <ExampleSection index={7} title="Notification Indicators">{notifications}</ExampleSection>
      <ExampleSection index={8} title="Status Indicators">{status}</ExampleSection>
      <ExampleSection index={9} title="Different Badge Positions">{positions}</ExampleSection>
      <ExampleSection index={10} title="Count Formatting">{formatting}</ExampleSection>
    </Page>
  );
}
