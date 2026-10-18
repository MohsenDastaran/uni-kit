//! `kbd` — keyboard shortcut chips spelled for the current platform.
import { Kbd, KbdGroup } from "../components/kbd";
import { Caption, ExampleSection, Page, Row } from "./ui";

// section: Shortcuts
const shortcuts = (
  <Row gap={10}>
    <Kbd keys={["mod+k"]} />
    <Kbd keys={["mod+shift+p"]} />
    <Kbd keys={["escape"]} />
  </Row>
);
// section: Multiple Modifiers
const modifiers = (
  <Row gap={10}>
    <Kbd keys={["mod", "shift", "k"]} />
    <Kbd keys={["ctrl", "alt", "delete"]} />
    <Kbd keys={["mod+enter"]} />
  </Row>
);
// section: Arrow Keys and Function Keys
const arrows = (
  <Row gap={10}>
    <Kbd keys={["up"]} />
    <Kbd keys={["down"]} />
    <Kbd keys={["left"]} />
    <Kbd keys={["right"]} />
    <Kbd keys={["f5"]} />
  </Row>
);
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>`keymap.label` writes ⌘K on Apple platforms and Ctrl+K elsewhere.</Caption>
      <ExampleSection index={0} title="Shortcuts">{shortcuts}</ExampleSection>
      <ExampleSection index={1} title="Multiple Modifiers">{modifiers}</ExampleSection>
      <ExampleSection index={2} title="Arrow Keys and Function Keys">{arrows}</ExampleSection>
    </Page>
  );
}
