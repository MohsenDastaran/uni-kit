//! `tooltip` — a short label over a trigger.
import { signal } from "nsh";
import { Tooltip } from "../components/tooltip";
import { Button } from "../components/button";
import { Text } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const open = signal(false);
  // section: Basic
  const basic = (
    <Tooltip open={open()} onOpenChange={(next) => open.set(next)} content="Copy the snippet">
      <Button label={open() ? "Hide tooltip" : "Show tooltip"} icon="copy" onPress={() => open.set(!open())} />
    </Tooltip>
  );
  // section: Placements
  const placements = (
    <Row gap={12}>
      <Tooltip content="Above" placement="top"><Button label="Above" /></Tooltip>
      <Tooltip content="Below" placement="bottom"><Button label="Below" /></Tooltip>
      <Tooltip content="Start" placement="start"><Button label="Start" /></Tooltip>
    </Row>
  );
  // section: Delay
  const delay = <Tooltip content="Waits 600ms" delay={600}><Button label="Slow tooltip" /></Tooltip>;
  // section: Rich content
  const rich = <Tooltip content="A title and a sentence."><Text style={{ fontSize: 13, color: tokens.foreground }}>Hover or long-press this text</Text></Tooltip>;
  // section: end
  return (
    <Page>
      <Caption>On desktops the pointer shows the tooltip; the web backend does not raise hover, so the demos also expose a controlled trigger.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Placements">{placements}</ExampleSection>
      <ExampleSection index={2} title="Delay">{delay}</ExampleSection>
      <ExampleSection index={3} title="Rich content">{rich}</ExampleSection>
    </Page>
  );
}
