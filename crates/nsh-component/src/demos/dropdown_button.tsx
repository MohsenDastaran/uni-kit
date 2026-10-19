//! `dropdown-button` — a button with a menu.
import { signal } from "nsh";
import { DropdownButton } from "../components/dropdown_button";
import { Text } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const chosen = signal("Nothing chosen");
  const items = [
    { label: "Edit", icon: "pencil" as const, shortcut: "⌘E", onSelect: () => chosen.set("Edit") },
    { label: "Duplicate", icon: "copy" as const, shortcut: "⌘D", onSelect: () => chosen.set("Duplicate") },
    { label: "Delete", icon: "trash" as const, onSelect: () => chosen.set("Delete") },
  ];
  // section: Usage
  const usage = <DropdownButton label="Actions" items={items} onPress={() => chosen.set("Actions")} />;
  // section: Basic
  const basic = <DropdownButton label="Actions" items={items} onPress={() => chosen.set("Actions")} />;
  // section: Variants
  const variants = <Row gap={8}><DropdownButton label="Primary" variant="primary" items={items} onPress={() => chosen.set("Primary action")} /><DropdownButton label="Secondary" variant="secondary" items={items} onPress={() => chosen.set("Secondary action")} /><DropdownButton label="Ghost" variant="ghost" items={items} onPress={() => chosen.set("Ghost action")} /></Row>;
  // section: With icons
  const icons = <DropdownButton label="With icons" items={items} onPress={() => chosen.set("Icons action")} />;
  // section: Sizes
  const sizes = <Row gap={8}><DropdownButton label="Small" size="small" items={items} onPress={() => chosen.set("Small action")} /><DropdownButton label="Large" size="large" items={items} onPress={() => chosen.set("Large action")} /></Row>;
  // section: Disabled
  const disabled = <DropdownButton label="Disabled" disabled items={items} onPress={() => chosen.set("Disabled action")} />;
  // section: end
  return (
    <Page>
      <Caption>DropdownButton has a split trigger: the label runs its action, and only the arrow opens the menu.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={2} title="Variants">{variants}</ExampleSection>
      <ExampleSection index={3} title="With icons">{icons}</ExampleSection>
      <ExampleSection index={4} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={5} title="Disabled">{disabled}</ExampleSection>
      <Status>{chosen()}</Status>
    </Page>
  );
}
