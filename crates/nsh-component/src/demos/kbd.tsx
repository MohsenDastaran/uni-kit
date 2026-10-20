//! `kbd` — keyboard shortcut chips, groups, hints and shortcut help.
import { Text, View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { InputGroup, InputGroupInput } from "../components/input_group";
import { Kbd, KbdGroup, KbdHint } from "../components/kbd";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

const shortcutRow = (label: string, keys: string[]) => (
  <View style={{ flexDirection: "row", alignItems: "center", paddingVertical: 7, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
    <Text style={{ flex: 1, fontSize: 12, color: tokens.foreground }}>{label}</Text>
    <Kbd keys={keys} />
  </View>
);

export function Demo() {
  // section: Usage
  const usage = (
    <Row gap={6} style={{ alignItems: "center" }}>
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Open search</Text>
      <Kbd keys={["mod", "k"]} />
    </Row>
  );

  // section: Shortcuts
  const shortcuts = (
    <Row gap={8}>
      <Kbd keys={["mod", "k"]} />
      <Kbd keys={["mod", "shift", "p"]} />
      <Kbd keys={["escape"]} />
    </Row>
  );

  // section: Basic Keyboard Shortcut
  const basic = (
    <Row gap={8}>
      <Kbd keys={["ctrl", "shift", "p"]} />
      <Kbd keys={["mod", "b"]} />
      <Kbd keys={["enter"]} />
    </Row>
  );

  // section: Multiple Modifiers
  const modifiers = (
    <Row gap={8}>
      <Kbd keys={["mod", "shift", "k"]} />
      <Kbd keys={["ctrl", "alt", "delete"]} />
      <Kbd keys={["mod", "enter"]} />
      <Kbd keys={["alt", "up"]} />
    </Row>
  );

  // section: Arrow Keys and Function Keys
  const arrows = (
    <Row gap={8}>
      <Kbd keys={["up"]} />
      <Kbd keys={["down"]} />
      <Kbd keys={["left"]} />
      <Kbd keys={["right"]} />
      <Kbd keys={["f5"]} />
    </Row>
  );

  // section: Without Visual Styling
  const unstyled = (
    <Row gap={8} style={{ alignItems: "center" }}>
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Press</Text>
      <Kbd keys={["mod", "k"]} styled={false} />
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>to search</Text>
    </Row>
  );

  // section: Keyboard keys grouped together
  const grouped = (
    <KbdGroup style={{ gap: 3 }}>
      <Kbd label="Ctrl" />
      <Kbd keys={["mod+k"]} />
      <Kbd label="Ctrl + B" />
    </KbdGroup>
  );

  // section: Keyboard key within an input
  const input = (
    <InputGroup style={{ width: 340 }}>
      <Kbd label="Space" size="xsmall" />
      <InputGroupInput placeholder="Search…" />
    </InputGroup>
  );

  // section: Keyboard Shortcut Help
  const help = (
    <View style={{ width: 380, maxWidth: "100%", gap: 8 }}>
      <KbdHint label="Open command palette" keys={["mod", "k"]} />
      <KbdHint label="Create a new file" keys={["mod", "n"]} />
      <KbdHint label="Save the document" keys={["mod", "s"]} />
    </View>
  );

  // section: Menu Item with Shortcut
  const menu = (
    <View style={{ width: 320, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 8, padding: 4, gap: 1 }}>
      {[
        { label: "Search", keys: ["mod", "k"], icon: "search" as const },
        { label: "New file", keys: ["mod", "n"], icon: "file" as const },
        { label: "Save", keys: ["mod", "s"], icon: "download" as const },
      ].map((item) => (
        <View style={{ flexDirection: "row", alignItems: "center", gap: 7, paddingHorizontal: 7, paddingVertical: 5, borderRadius: 5 }}>
          <Icon name={item.icon} size={12} color={tokens.mutedForeground} />
          <Text style={{ flex: 1, fontSize: 11, color: tokens.foreground }}>{item.label}</Text>
          <Kbd keys={item.keys} size="xsmall" />
        </View>
      ))}
    </View>
  );

  // section: Inline Documentation
  const inline = (
    <Row gap={4} style={{ alignItems: "center", flexWrap: "wrap" }}>
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Press</Text>
      <Kbd keys={["mod", "k"]} size="xsmall" />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>to focus search, then</Text>
      <Kbd keys={["enter"]} size="xsmall" />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>to open the result.</Text>
    </Row>
  );

  // section: Custom Styling
  const custom = (
    <Row gap={8}>
      <Kbd keys={["mod", "k"]} style={{ borderColor: tokens.primary }} />
      <Kbd keys={["escape"]} style={{ borderRadius: 10, borderColor: tokens.primary }} />
      <Kbd keys={["shift", "enter"]} size="large" />
    </Row>
  );

  // section: Keyboard Shortcuts
  const list = (
    <View style={{ width: 380, maxWidth: "100%" }}>
      <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground, paddingBottom: 4 }}>Keyboard Shortcuts</Text>
      {shortcutRow("Search", ["mod", "k"])}
      {shortcutRow("New File", ["mod", "n"])}
      {shortcutRow("Save", ["mod", "s"])}
      {shortcutRow("Undo", ["mod", "z"])}
      {shortcutRow("Redo", ["mod", "shift", "z"])}
    </View>
  );

  // section: Text-Only Format
  const text = (
    <Row gap={8}>
      <Kbd label="Ctrl + K" />
      <Kbd label="⌘ K" />
      <Kbd label="Alt + F4" />
    </Row>
  );

  // section: Icon with label
  const icon = (
    <Row gap={8}>
      <View style={{ flexDirection: "row", alignItems: "center", gap: 4, paddingHorizontal: 7, paddingVertical: 3, borderRadius: 5, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted }}>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>←</Text><Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Left</Text>
      </View>
      <View style={{ flexDirection: "row", alignItems: "center", gap: 4, paddingHorizontal: 7, paddingVertical: 3, borderRadius: 5, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted }}>
        <Icon name="mic" size={10} color={tokens.mutedForeground} /><Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Voice Enabled</Text>
      </View>
    </Row>
  );
  // section: end

  return (
    <Page>
      <Caption>`keymap.label` writes ⌘K on Apple platforms and Ctrl+K elsewhere.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Shortcuts">{shortcuts}</ExampleSection>
      <ExampleSection index={2} title="Basic Keyboard Shortcut">{basic}</ExampleSection>
      <ExampleSection index={3} title="Multiple Modifiers">{modifiers}</ExampleSection>
      <ExampleSection index={4} title="Arrow Keys and Function Keys">{arrows}</ExampleSection>
      <ExampleSection index={5} title="Without Visual Styling">{unstyled}</ExampleSection>
      <ExampleSection index={6} title="Keyboard keys grouped together">{grouped}</ExampleSection>
      <ExampleSection index={7} title="Keyboard key within an input">{input}</ExampleSection>
      <ExampleSection index={8} title="Keyboard Shortcut Help">{help}</ExampleSection>
      <ExampleSection index={9} title="Menu Item with Shortcut">{menu}</ExampleSection>
      <ExampleSection index={10} title="Inline Documentation">{inline}</ExampleSection>
      <ExampleSection index={11} title="Custom Styling">{custom}</ExampleSection>
      <ExampleSection index={12} title="Keyboard Shortcuts">{list}</ExampleSection>
      <ExampleSection index={13} title="Text-Only Format">{text}</ExampleSection>
      <ExampleSection index={14} title="Icon with label">{icon}</ExampleSection>
      <Status>Shortcut chips follow the platform label.</Status>
    </Page>
  );
}
