//! `collapsible` — a header that reveals content.
import { signal, Text } from "nsh";
import { Collapsible } from "../components/collapsible";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const open = signal(true);
  // section: Basic
  const basic = <Collapsible title="Show details" open={open()} onOpenChange={(next) => open.set(next)}><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>The content is yours; the header only toggles it.</Text></Collapsible>;
  // section: Default open
  const defaultOpen = <Collapsible title="Started open" defaultOpen><Text style={{ color: tokens.foreground }}>Visible on the first frame.</Text></Collapsible>;
  // section: Disabled
  const disabled = <Collapsible title="Disabled" disabled><Text style={{ color: tokens.foreground }}>Not reachable.</Text></Collapsible>;
  // section: end
  return (
    <Page>
      <Caption>Collapsible is one section; Accordion stacks several and coordinates them.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Default open">{defaultOpen}</ExampleSection>
      <ExampleSection index={2} title="Disabled">{disabled}</ExampleSection>
      <Status>{open() ? "open" : "closed"}</Status>
    </Page>
  );
}
