//! `menu` — a popup list of actions.
import { signal, Text } from "nsh";
import { Menu } from "../components/menu";
import { Button } from "../components/button";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const chosen = signal("Nothing chosen");
  // section: Usage
  const usage = (
    <Menu
      items={[
        { label: "Edit", icon: "pencil", onSelect: () => chosen.set("Edit") },
        { label: "Duplicate", icon: "copy", onSelect: () => chosen.set("Duplicate") },
      ]}
    />
  );

  // section: Basic
  const basic = <Menu items={[{ label: "Cut", onSelect: () => chosen.set("Cut") }, { label: "Copy", onSelect: () => chosen.set("Copy") }, { label: "Paste", onSelect: () => chosen.set("Paste") }]} />;
  // section: With icons
  const icons = <Menu items={[{ label: "Edit", icon: "pencil", onSelect: () => chosen.set("Edit") }, { label: "Duplicate", icon: "copy", onSelect: () => chosen.set("Duplicate") }]} />;
  // section: Shortcuts
  const shortcuts = <Menu items={[{ label: "Save", icon: "check", shortcut: "⌘S", onSelect: () => chosen.set("Save") }, { label: "Open", shortcut: "⌘O", onSelect: () => chosen.set("Open") }]} />;
  // section: Separators
  const separators = <Menu items={[{ label: "Rename", onSelect: () => chosen.set("Rename") }, { separator: true }, { label: "Delete", onSelect: () => chosen.set("Delete") }]} />;
  // section: Disabled
  const disabled = <Menu items={[{ label: "Available", onSelect: () => chosen.set("Available") }, { label: "Unavailable", disabled: true }]} />;
  // section: Premium menu
  const premium = (
    <Menu
      items={[
        { label: "Open in new tab", icon: "external-link", shortcut: "⌘↵", onSelect: () => chosen.set("Open in new tab") },
        { label: "Copy link", icon: "copy", shortcut: "⌘C", onSelect: () => chosen.set("Copy link") },
        { separator: true },
        { label: "Delete", icon: "trash", shortcut: "⌫", onSelect: () => chosen.set("Delete") },
      ]}
    />
  );
  // section: end
  return (
    <Page>
      <Caption>Menu is the list; DropdownButton anchors one to a button and Command filters one.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={2} title="With icons">{icons}</ExampleSection>
      <ExampleSection index={3} title="Shortcuts">{shortcuts}</ExampleSection>
      <ExampleSection index={4} title="Separators">{separators}</ExampleSection>
      <ExampleSection index={5} title="Disabled">{disabled}</ExampleSection>
      <ExampleSection index={6} title="Premium menu">{premium}</ExampleSection>
      <Status>{chosen()}</Status>
    </Page>
  );
}
