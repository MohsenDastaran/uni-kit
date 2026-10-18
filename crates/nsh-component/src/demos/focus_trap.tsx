//! `focus-trap` — keep focus inside a region with the native modal layer.
import { signal } from "nsh";
import { FocusTrap } from "../components/focus_trap";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Text } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const active = signal(false);
  // section: Basic
  const basic = <><Button label="Trap focus" variant="primary" onPress={() => active.set(true)} /><FocusTrap active={active()} onEscape={() => active.set(false)}><Button label="Close" onPress={() => active.set(false)} /></FocusTrap></>;
  // section: With dialog
  const dialog = <FocusTrap active={false}><Input placeholder="Only this field" onChangeText={() => {}} /></FocusTrap>;
  // section: Escape
  const escape = <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>The native modal answers the Escape key and reports it through onEscape.</Text>;
  // section: end
  return (
    <Page>
      <Caption>NSH has no tab-focus API; the modal layer is the honest way to keep the rest of the app inert.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="With dialog">{dialog}</ExampleSection>
      <ExampleSection index={2} title="Escape">{escape}</ExampleSection>
    </Page>
  );
}
