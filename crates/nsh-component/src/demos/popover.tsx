//! `popover` — a floating panel anchored to a trigger.
import { signal } from "nsh";
import { Popover } from "../components/popover";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Text } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const basicOpen = signal(false);
  const controlled = signal(false);
  // section: Basic
  const basic = (
    <Popover
      open={basicOpen()}
      onOpenChange={(next) => basicOpen.set(next)}
      trigger={<Button label="Open popover" onPress={() => basicOpen.set(!basicOpen())} />}
    >
      <Text style={{ fontSize: 13, color: tokens.foreground }}>A small floating panel.</Text>
    </Popover>
  );
  // section: Placement
  const placement = (
    <Row gap={12}>
      <Popover placement="bottom" trigger={<Button label="Below" />}>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>Below the trigger.</Text>
      </Popover>
      <Popover placement="top" trigger={<Button label="Above" />}>
        <Text style={{ fontSize: 13, color: tokens.foreground }}>Above the trigger.</Text>
      </Popover>
    </Row>
  );
  // section: With form
  const form = (
    <Popover trigger={<Button label="Rename" variant="secondary" />}>
      <Input placeholder="New name" onChangeText={() => {}} />
      <Button label="Save" variant="primary" onPress={() => {}} />
    </Popover>
  );
  // section: Controlled
  const control = (
    <Popover
      open={controlled()}
      onOpenChange={(next) => controlled.set(next)}
      trigger={<Button label="Controlled" onPress={() => controlled.set(!controlled())} />}
    >
      <Text style={{ fontSize: 13, color: tokens.foreground }}>Controlled by the app.</Text>
    </Popover>
  );
  // section: end
  return (
    <Page>
      <Caption>Popover is the anchor and the layer; its content is any JSX.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Placement">{placement}</ExampleSection>
      <ExampleSection index={2} title="With form">{form}</ExampleSection>
      <ExampleSection index={3} title="Controlled">{control}</ExampleSection>
    </Page>
  );
}
