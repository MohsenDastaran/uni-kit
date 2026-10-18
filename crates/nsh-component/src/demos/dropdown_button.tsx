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
  // section: Basic
  const basic = <DropdownButton label="Actions" items={items} />;
  // section: Variants
  const variants = <Row gap={8}><DropdownButton label="Primary" variant="primary" items={items} /><DropdownButton label="Secondary" variant="secondary" items={items} /><DropdownButton label="Ghost" variant="ghost" items={items} /></Row>;
  // section: With icons
  const icons = <DropdownButton label="With icons" items={items} />;
  // section: Sizes
  const sizes = <Row gap={8}><DropdownButton label="Small" size="small" items={items} /><DropdownButton label="Large" size="large" items={items} /></Row>;
  // section: Disabled
  const disabled = <DropdownButton label="Disabled" disabled items={items} />;
  // section: end
  return (
    <Page>
      <Caption>DropdownButton is a Button and a Menu; it owns the open state and nothing else.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Variants">{variants}</ExampleSection>
      <ExampleSection index={2} title="With icons">{icons}</ExampleSection>
      <ExampleSection index={3} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Disabled">{disabled}</ExampleSection>
      <Status>{chosen()}</Status>
    </Page>
  );
}
