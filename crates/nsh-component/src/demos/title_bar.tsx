//! `title-bar` — a custom window bar with aligned menu items, preview controls
//! and trailing actions.
import { Text, View } from "nsh";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Icon } from "../components/icon";
import { TitleBar } from "../components/title_bar";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const frame = (bar: any, width = 780) => (
    <View style={{ width, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 8, overflow: "hidden", backgroundColor: tokens.popover }}>
      {bar}
    </View>
  );
  // section: Usage
  const usage = frame(<TitleBar title="Project" showControls styled={false} />);
  // section: Basic Title Bar
  const basic = frame(<TitleBar title="uni-kit" showControls styled={false} />);
  // section: Title Bar with Custom Content
  const content = frame(
    <TitleBar
      title="Project"
      showControls
      styled={false}
      trailing={<Button label="Share" size="small" variant="secondary" onPress={() => {}} />}
    />,
  );
  // section: Title Bar with Menu Bar
  const menu = frame(<TitleBar title="Gallery" menu showControls styled={false} />);
  // section: Application Title Bar
  const application = frame(
    <TitleBar
      title="Project"
      menu
      showControls
      styled={false}
      menuItems={[{ label: "File" }, { label: "Edit" }, { label: "View" }]}
      trailing={<Button label="Share" size="small" variant="secondary" onPress={() => {}} />}
    />,
  );
  // section: Title Bar with Breadcrumbs
  const breadcrumbs = frame(
    <TitleBar
      title="Settings"
      showControls
      styled={false}
      leading={<Row gap={4} style={{ alignItems: "center" }}><Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Workspace</Text><Icon name="chevron-right" size={10} color={tokens.mutedForeground} /></Row>}
      trailing={<Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Edited 2m ago</Text>}
    />,
  );
  // section: Title Bar with Search
  const search = frame(
    <TitleBar
      title="Library"
      showControls
      styled={false}
      trailing={<Input placeholder="Search files…" prefixIcon="search" size="small" style={{ width: 220 }} />}
    />,
  );
  // section: Styled Title Bar
  const styled = frame(<TitleBar title="Styled" showControls styled={false} style={{ backgroundColor: tokens.muted }} />);
  // section: Title Bar with Status
  const status = frame(<TitleBar title="Editor" showControls styled={false} trailing={<Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Saved</Text>} />);
  // section: Minimal Title Bar
  const minimal = frame(<TitleBar title="Minimal" showControls styled={false} style={{ height: 34 }} />);
  // section: end
  return (
    <Page>
      <Caption>On macOS, Windows and Linux the bar hides the system title bar and lays content around the platform's controls; elsewhere it is a preview.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Title Bar">{basic}</ExampleSection>
      <ExampleSection index={2} title="Title Bar with Custom Content">{content}</ExampleSection>
      <ExampleSection index={3} title="Title Bar with Menu Bar">{menu}</ExampleSection>
      <ExampleSection index={4} title="Application Title Bar">{application}</ExampleSection>
      <ExampleSection index={5} title="Title Bar with Breadcrumbs">{breadcrumbs}</ExampleSection>
      <ExampleSection index={6} title="Title Bar with Search">{search}</ExampleSection>
      <ExampleSection index={7} title="Styled Title Bar">{styled}</ExampleSection>
      <ExampleSection index={8} title="Title Bar with Status">{status}</ExampleSection>
      <ExampleSection index={9} title="Minimal Title Bar">{minimal}</ExampleSection>
    </Page>
  );
}
