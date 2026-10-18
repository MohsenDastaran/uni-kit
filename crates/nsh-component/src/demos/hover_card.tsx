//! `hover-card` — a richer card over a trigger.
import { signal } from "nsh";
import { HoverCard, HoverCardDescription, HoverCardTitle } from "../components/hover_card";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Text } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const open = signal(false);
  // section: Basic
  const basic = (
    <HoverCard
      open={open()}
      onOpenChange={(next) => open.set(next)}
      trigger={<Button label="Ada Lovelace" variant="ghost" onPress={() => open.set(!open())} />}
    >
      <HoverCardTitle>Ada Lovelace</HoverCardTitle>
      <HoverCardDescription>Mathematician and writer.</HoverCardDescription>
    </HoverCard>
  );
  // section: With avatar
  const avatar = (
    <HoverCard trigger={<Button label="Profile" variant="ghost" />}>
      <Row gap={8}><Avatar name="Grace Hopper" /><HoverCardTitle>Grace Hopper</HoverCardTitle></Row>
      <HoverCardDescription>Computer scientist.</HoverCardDescription>
    </HoverCard>
  );
  // section: Placements
  const placements = (
    <Row gap={12}>
      <HoverCard placement="bottom" trigger={<Button label="Below" />}><HoverCardDescription>Below the trigger.</HoverCardDescription></HoverCard>
      <HoverCard placement="top" trigger={<Button label="Above" />}><HoverCardDescription>Above the trigger.</HoverCardDescription></HoverCard>
    </Row>
  );
  // section: Delays
  const delays = <HoverCard delay={700} trigger={<Text style={{ fontSize: 13, color: tokens.foreground }}>A slower card</Text>}><HoverCardDescription>It waits 700ms.</HoverCardDescription></HoverCard>;
  // section: end
  return (
    <Page>
      <Caption>HoverCard is a tooltip with a surface; a press/long press opens it on the web, and the controlled form is explicit.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="With avatar">{avatar}</ExampleSection>
      <ExampleSection index={2} title="Placements">{placements}</ExampleSection>
      <ExampleSection index={3} title="Delays">{delays}</ExampleSection>
    </Page>
  );
}
