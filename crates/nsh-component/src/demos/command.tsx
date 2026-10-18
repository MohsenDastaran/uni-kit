//! `command` — a filterable command palette.
import { signal } from "nsh";
import { Command } from "../components/command";
import { Button } from "../components/button";
import { Text } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const open = signal(false);
  const chosen = signal("Nothing chosen");
  const items = [
    { label: "New file", group: "File", icon: "file" as const, shortcut: "⌘N", onSelect: () => chosen.set("New file") },
    { label: "Open file", group: "File", icon: "folder-open" as const, shortcut: "⌘O", onSelect: () => chosen.set("Open file") },
    { label: "Toggle sidebar", group: "View", icon: "panel-left" as const, shortcut: "⌘B", onSelect: () => chosen.set("Toggle sidebar") },
    { label: "Search", group: "Go", icon: "search" as const, shortcut: "⌘K", onSelect: () => chosen.set("Search") },
  ];
  // section: Basic
  const basic = <><Button label="Open command palette" variant="primary" onPress={() => open.set(true)} /><Command open={open()} items={items} onOpenChange={(next) => open.set(next)} /></>;
  // section: Groups
  const groups = <Command open={false} items={items} />;
  // section: Keyboard
  const keyboard = <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Press ⌘K to open the palette.</Text>;
  // section: Recent
  const recent = <Status>{chosen()}</Status>;
  // section: end
  return (
    <Page>
      <Caption>The palette is a modal with a filter field; the key binding is `nsh/keymap`'s `mod+k`.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Groups">{groups}</ExampleSection>
      <ExampleSection index={2} title="Keyboard">{keyboard}</ExampleSection>
      <ExampleSection index={3} title="Recent">{recent}</ExampleSection>
    </Page>
  );
}
