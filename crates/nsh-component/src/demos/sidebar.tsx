//! `sidebar` — navigation rows with an active state.
import { signal, Text } from "nsh";
import { Sidebar } from "../components/sidebar";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const active = signal("home");
  const items = [
    { id: "home", label: "Home", icon: "house" as const },
    { id: "inbox", label: "Inbox", icon: "inbox" as const, badge: "3" },
    { id: "projects", label: "Projects", icon: "folder" as const },
    { id: "settings", label: "Settings", icon: "settings" as const },
  ];
  // section: Basic
  const basic = <Sidebar items={items} active={active()} onSelect={(next) => active.set(next)} style={{ height: 280 }} />;
  // section: Header and footer
  const decorated = <Sidebar items={items} active={active()} onSelect={(next) => active.set(next)} header={<Text style={{ padding: 12, fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Workspace</Text>} footer={<Text style={{ padding: 12, fontSize: 11, color: tokens.mutedForeground }}>v1.0</Text>} style={{ height: 320 }} />;
  // section: end
  return (
    <Page>
      <Caption>Sidebar is the navigation list; the app owns the active route and the content beside it.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Header and footer">{decorated}</ExampleSection>
      <Status>Active: {active()}</Status>
    </Page>
  );
}
