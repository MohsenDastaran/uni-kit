//! `status-bar` — left, centre and right regions, from a single label to a full
//! editor or application status line.
import { signal, Text, View } from "nsh";
import { Spinner } from "../components/spinner";
import { StatusBar, StatusBarItem, StatusBarSeparator } from "../components/status_bar";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const stack = (children: any) => (
  <View style={{ width: 760, maxWidth: "100%", gap: 8 }}>{children}</View>
);

export function Demo() {
  const selected = signal("source");
  // section: Usage
  const usage = stack(
    <StatusBar
      left={<StatusBarItem iconName="check" tone="success">Ready</StatusBarItem>}
      right={<StatusBarItem monospace>UTF-8</StatusBarItem>}
    />,
  );
  // section: Labels
  const labels = stack(
    <StatusBar
      left={<StatusBarItem iconName="circle-check" tone="success">Connected</StatusBarItem>}
      center={<StatusBarItem>Build passed</StatusBarItem>}
      right={<><StatusBarItem>Ln 1, Col 1</StatusBarItem><StatusBarSeparator /><StatusBarItem>UTF-8</StatusBarItem></>}
    />,
  );
  // section: Buttons
  const buttons = stack(
    <StatusBar
      left={<StatusBarItem iconName="check" tone="success">Saved</StatusBarItem>}
      right={<><StatusBarItem>Ln 12, Col 34</StatusBarItem><StatusBarSeparator /><StatusBarItem monospace>UTF-8</StatusBarItem><StatusBarSeparator /><StatusBarItem>Rust</StatusBarItem></>}
    />,
  );
  // section: Separators and custom elements
  const separators = stack(
    <StatusBar
      left={<><StatusBarItem iconName="github">main</StatusBarItem><StatusBarSeparator /><StatusBarItem iconName="check" tone="success">0</StatusBarItem><StatusBarItem iconName="circle-alert" tone="warning">2</StatusBarItem></>}
      center={<StatusBarItem iconName="sparkles">Synced</StatusBarItem>}
      right={<><StatusBarItem>Ln 12, Col 34</StatusBarItem><StatusBarSeparator /><StatusBarItem monospace>UTF-8</StatusBarItem></>}
    />,
  );
  // section: Custom styling
  const styled = stack(
    <View style={{ gap: 8 }}>
      <StatusBar
        variant="inset"
        left={<StatusBarItem iconName="circle-check" tone="success">Connected</StatusBarItem>}
        center={<StatusBarItem iconName="refresh-cw">Syncing changes…</StatusBarItem>}
        right={<StatusBarItem iconName="bell">All changes saved</StatusBarItem>}
      />
      <StatusBar
        variant="floating"
        left={<StatusBarItem iconName="message-circle">2 conversations</StatusBarItem>}
        right={<StatusBarItem iconName="settings">Preferences</StatusBarItem>}
      />
    </View>,
  );
  // section: Application
  const application = stack(
    <StatusBar
      left={<><StatusBarItem iconName="circle-check" tone="success">Connected</StatusBarItem><StatusBarSeparator /><StatusBarItem icon={<Spinner size="xsmall" />}>Syncing…</StatusBarItem></>}
      right={<><StatusBarItem>All changes saved</StatusBarItem><StatusBarSeparator /><StatusBarItem iconName="bell" badge="3">Notifications</StatusBarItem></>}
    />,
  );
  // section: Editor status
  const editor = stack(
    <StatusBar
      size="large"
      left={<><StatusBarItem iconName="github">main</StatusBarItem><StatusBarSeparator size="large" /><StatusBarItem iconName="check" tone="success">0</StatusBarItem><StatusBarItem iconName="circle-alert" tone="warning">2</StatusBarItem></>}
      right={<><StatusBarItem>Ln 12, Col 34</StatusBarItem><StatusBarSeparator size="large" /><StatusBarItem monospace>UTF-8</StatusBarItem><StatusBarSeparator size="large" /><StatusBarItem>Rust</StatusBarItem></>}
    />,
  );
  // section: Alignment
  const alignment = (
    <View style={{ width: 700, maxWidth: "100%", gap: 8 }}>
      <StatusBar left={<StatusBarItem>Left</StatusBarItem>} center={<StatusBarItem>Center → end</StatusBarItem>} />
      <StatusBar center={<StatusBarItem>Center → start</StatusBarItem>} right={<StatusBarItem>Right</StatusBarItem>} />
      <StatusBar left={<StatusBarItem>Left</StatusBarItem>} center={<StatusBarItem>Center → centered</StatusBarItem>} right={<StatusBarItem>Right</StatusBarItem>} />
      <StatusBar center={<StatusBarItem>Center only → start-aligned</StatusBarItem>} />
      <StatusBar left={<StatusBarItem>Left</StatusBarItem>} right={<StatusBarItem>Right</StatusBarItem>} />
    </View>
  );
  // section: Status tones
  const tones = stack(
    <StatusBar
      left={<><StatusBarItem iconName="check" tone="success">Successful</StatusBarItem><StatusBarItem iconName="circle-alert" tone="warning">Warning</StatusBarItem><StatusBarItem iconName="circle-x" tone="danger">Failed</StatusBarItem><StatusBarItem iconName="info" tone="info">Information</StatusBarItem></>}
      right={<StatusBarItem>4 messages</StatusBarItem>}
    />,
  );
  // section: Clickable actions
  const actions = stack(
    <StatusBar
      left={<StatusBarItem iconName="folder" onPress={() => selected.set("files")} active={selected() === "files"}>Files</StatusBarItem>}
      center={<StatusBarItem iconName="code" onPress={() => selected.set("source")} active={selected() === "source"}>Source</StatusBarItem>}
      right={<StatusBarItem iconName="github" onPress={() => selected.set("branch")} active={selected() === "branch"}>{selected() === "branch" ? "main" : "branch"}</StatusBarItem>}
    />,
  );
  // section: Size variants
  const sizes = (
    <View style={{ width: 700, maxWidth: "100%", gap: 8 }}>
      <StatusBar size="small" left={<StatusBarItem iconName="check" tone="success">Small</StatusBarItem>} right={<StatusBarItem>26pt</StatusBarItem>} />
      <StatusBar left={<StatusBarItem iconName="check" tone="success">Medium</StatusBarItem>} right={<StatusBarItem>32pt</StatusBarItem>} />
      <StatusBar size="large" left={<StatusBarItem iconName="check" tone="success">Large</StatusBarItem>} right={<StatusBarItem>38pt</StatusBarItem>} />
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>StatusBar holds small persistent context at the edge of a window; keep actions secondary and the labels short.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Labels">{labels}</ExampleSection>
      <ExampleSection index={2} title="Buttons">{buttons}</ExampleSection>
      <ExampleSection index={3} title="Separators and custom elements">{separators}</ExampleSection>
      <ExampleSection index={4} title="Custom styling">{styled}</ExampleSection>
      <ExampleSection index={5} title="Application">{application}</ExampleSection>
      <ExampleSection index={6} title="Editor status">{editor}</ExampleSection>
      <ExampleSection index={7} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={8} title="Status tones">{tones}</ExampleSection>
      <ExampleSection index={9} title="Clickable actions">{actions}</ExampleSection>
      <ExampleSection index={10} title="Size variants">{sizes}</ExampleSection>
    </Page>
  );
}
