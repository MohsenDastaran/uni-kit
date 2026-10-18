//! `tabs` — labeled sections with one active panel.
import { signal, Text } from "nsh";
import { Tabs } from "../components/tabs";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const value = signal("account");
  const makeItems = () => [
    { value: "account", label: "Account", content: <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Account settings.</Text> },
    { value: "billing", label: "Billing", content: <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Invoices and payment.</Text> },
    { value: "team", label: "Team", content: <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>People and permissions.</Text> },
  ];
  // section: Basic
  const basic = <Tabs items={makeItems()} value={value()} onChange={(next) => value.set(next)} />;
  // section: Controlled
  const controlled = <Tabs items={makeItems()} value={value()} onChange={(next) => value.set(next)} />;
  // section: Sizes
  const sizes = <Tabs items={makeItems()} size="large" defaultValue="billing" />;
  // section: end
  return (
    <Page>
      <Caption>Tabs switch between panels; the content is owned by the application.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Controlled">{controlled}</ExampleSection>
      <ExampleSection index={2} title="Sizes">{sizes}</ExampleSection>
      <Status>Active: {value()}</Status>
    </Page>
  );
}
