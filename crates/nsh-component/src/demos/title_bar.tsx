//! `title-bar` — a custom window bar, with the desktop window controls where NSH
//! exposes them and a visual preview on web and mobile.
import { Text } from "nsh";
import { TitleBar } from "../components/title_bar";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

// section: Basic Title Bar
const basic = <TitleBar title="uni-kit" showControls />;
// section: Title Bar with Custom Content
const content = <TitleBar title="Project" showControls trailing={<Button label="Share" size="small" onPress={() => {}} />} />;
// section: Title Bar with Menu Bar
const menu = <TitleBar title="Gallery" menu showControls />;
export function Demo() {
  // section: Styled Title Bar
  const styled = <TitleBar title="Styled" showControls style={{ backgroundColor: tokens.muted }} />;
  // section: Title Bar with Status
  const status = <TitleBar title="Editor" showControls trailing={<Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Saved</Text>} />;
  // section: end
  return (
    <Page>
      <Caption>On macOS, Windows and Linux the bar hides the system title bar and lays content around the platform's controls; elsewhere it is a preview.</Caption>
      <ExampleSection index={0} title="Basic Title Bar">{basic}</ExampleSection>
      <ExampleSection index={1} title="Title Bar with Custom Content">{content}</ExampleSection>
      <ExampleSection index={2} title="Title Bar with Menu Bar">{menu}</ExampleSection>
      <ExampleSection index={3} title="Styled Title Bar">{styled}</ExampleSection>
      <ExampleSection index={4} title="Title Bar with Status">{status}</ExampleSection>
    </Page>
  );
}
