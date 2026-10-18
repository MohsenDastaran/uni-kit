//! `status-bar` — a bar with left, centre and right regions.
import { Text } from "nsh";
import { StatusBar, StatusBarItem, StatusBarSeparator } from "../components/status_bar";
import { Icon } from "../components/icon";
import { Caption, ExampleSection, Page } from "./ui";

// section: Labels
const labels = <StatusBar left={<StatusBarItem>Ready</StatusBarItem>} right={<StatusBarItem>UTF-8</StatusBarItem>} />;
// section: Buttons
const buttons = <StatusBar left={<StatusBarItem icon={<Icon name="check" size={12} />}>Saved</StatusBarItem>} right={<StatusBarItem>Ln 1, Col 1</StatusBarItem>} />;
// section: Separators and custom elements
const separators = <StatusBar left={<StatusBarItem>main</StatusBarItem>} center={<StatusBarItem>build passed</StatusBarItem>} right={<><StatusBarItem>85%</StatusBarItem><StatusBarSeparator /><StatusBarItem>TypeScript</StatusBarItem></>} />;
// section: Custom styling
const styled = <StatusBar left={<StatusBarItem>Custom</StatusBarItem>} style={{ backgroundColor: "#171717" }} />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>StatusBar is the bottom region; the content is any small text or control.</Caption>
      <ExampleSection index={0} title="Labels">{labels}</ExampleSection>
      <ExampleSection index={1} title="Buttons">{buttons}</ExampleSection>
      <ExampleSection index={2} title="Separators and custom elements">{separators}</ExampleSection>
      <ExampleSection index={3} title="Custom styling">{styled}</ExampleSection>
    </Page>
  );
}
