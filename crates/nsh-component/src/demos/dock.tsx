//! `dock` — a tabbed panel group and a resizable workspace, composed from the
//! same primitives. The full-workspace example mirrors a desktop editor.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Dock, type DockPanel, DockSplit } from "../components/dock";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

const fileRows = [
  { name: "src", kind: "folder" as const, depth: 0 },
  { name: "components", kind: "folder" as const, depth: 1 },
  { name: "dock.tsx", kind: "file" as const, depth: 2 },
  { name: "button.tsx", kind: "file" as const, depth: 2 },
  { name: "main.tsx", kind: "file" as const, depth: 1 },
  { name: "package.json", kind: "file" as const, depth: 0 },
  { name: "README.md", kind: "file" as const, depth: 0 },
];

const terminalLines = [
  "$ nsh run examples/dock.tsx --target linux",
  "compiled examples/dock.tsx in 184ms",
  "ready  localhost:8790",
  "",
  "watching src/ for changes…",
];

const placeholder = (text: string) => (
  <View style={{ flex: 1, padding: 16, backgroundColor: tokens.background }}>
    <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{text}</Text>
  </View>
);

export function Demo() {
  const query = signal("");

  const fileTree = () => (
    <View style={{ flex: 1, backgroundColor: tokens.background }}>
      <View style={{ padding: 7, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <Input value={query()} onChangeText={(next) => query.set(next)} size="small" prefixIcon="search" placeholder="Search" cleanable />
      </View>
      <View style={{ paddingVertical: 4 }}>
        {fileRows.map((row) => (
          <View
            class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.05]"
            style={{ flexDirection: "row", alignItems: "center", gap: 5, paddingVertical: 3, paddingLeft: 8 + row.depth * 12, paddingRight: 8 }}
          >
            <Icon name={row.kind === "folder" ? "folder" : "file-text"} size={11} color={tokens.mutedForeground} />
            <Text style={{ fontSize: 11, color: row.name === "dock.tsx" ? tokens.foreground : tokens.mutedForeground }}>{row.name}</Text>
          </View>
        ))}
      </View>
    </View>
  );

  const panels = (prefix: string): DockPanel[] => [
    { id: `${prefix}-explorer`, label: "Explorer", icon: "folder", content: fileTree() },
    { id: `${prefix}-search`, label: "Search", icon: "search", content: placeholder("Search across the workspace.") },
    { id: `${prefix}-outline`, label: "Outline", icon: "file-text", content: placeholder("Outline") },
  ];

  const editorPanels: DockPanel[] = [
    { id: "editor", label: "Editor", icon: "file-text", content: placeholder("Drop a tab near an edge to split this group.") },
    { id: "preview", label: "Preview", icon: "eye", content: placeholder("Preview") },
  ];

  const bottomPanels: DockPanel[] = [
    {
      id: "terminal",
      label: "Terminal",
      icon: "terminal",
      content: (
        <View style={{ flex: 1, backgroundColor: tokens.background, padding: 8, gap: 1 }}>
          {terminalLines.map((line) => (
            <Text style={{ fontSize: 10, lineHeight: 15, fontFamily: "monospace", color: line.startsWith("$") ? tokens.foreground : tokens.mutedForeground }}>
              {line || " "}
            </Text>
          ))}
        </View>
      ),
    },
    { id: "problems", label: "Problems", icon: "circle-alert", badge: "2", content: placeholder("No problems in this workspace.") },
    { id: "output", label: "Output", icon: "file-text", content: placeholder("Build output") },
  ];

  // section: Usage
  const usage = (
    <Dock
      panels={[
        { id: "explorer", label: "Explorer", content: placeholder("Explorer") },
        { id: "search", label: "Search", content: placeholder("Search") },
        { id: "outline", label: "Outline", content: placeholder("Outline") },
      ]}
      actions={false}
      style={{ height: 180, width: 480, maxWidth: "100%" }}
    />
  );

  // section: Create a dock area
  const dockArea = (
    <Dock
      panels={panels("area")}
      style={{ height: 240, width: 620, maxWidth: "100%" }}
    />
  );

  // section: Define a panel
  const definePanel = (
    <Dock
      panels={[
        { id: "explorer", label: "Explorer", icon: "folder", content: fileTree() },
        { id: "problems", label: "Problems", icon: "circle-alert", badge: "2", content: placeholder("Two warnings") },
      ]}
      actions={<View style={{ flexDirection: "row", gap: 2, paddingRight: 4 }}><View style={{ width: 20, height: 20, alignItems: "center", justifyContent: "center" }}><Icon name="plus" size={11} color={tokens.mutedForeground} /></View><View style={{ width: 20, height: 20, alignItems: "center", justifyContent: "center" }}><Icon name="ellipsis" size={12} color={tokens.mutedForeground} /></View></View>}
      style={{ height: 220, width: 500, maxWidth: "100%" }}
    />
  );

  // section: Describe the initial layout
  const initialLayout = (
    <DockSplit
      initial={30}
      left={<Dock panels={panels("left")} style={{ height: 300 }} />}
      right={<Dock panels={editorPanels} style={{ height: 300 }} />}
      style={{ height: 300 }}
    />
  );

  // section: Bottom panel
  const bottom = (
    <DockSplit
      vertical
      initial={63}
      left={<DockSplit initial={28} left={<Dock panels={panels("bottom-left")} />} right={<Dock panels={editorPanels} />} />}
      right={<Dock panels={bottomPanels} />}
      style={{ height: 420 }}
    />
  );

  // section: Style the workspace
  const styled = (
    <Dock
      panels={[
        { id: "files", label: "Files", icon: "folder", content: placeholder("Files") },
        { id: "settings", label: "Settings", icon: "settings", content: placeholder("Settings") },
      ]}
      actions={<Button icon="settings" size="xsmall" variant="ghost" accessibilityLabel="Panel settings" onPress={() => {}} />}
      style={{ height: 220, width: 560, maxWidth: "100%", backgroundColor: tokens.popover }}
    />
  );

  // section: Empty group
  const empty = (
    <Dock panels={[]} emptyText="No panels in this group. Drop a tab here to add one." actions={false} style={{ height: 140, width: 500, maxWidth: "100%" }} />
  );

  // section: Runnable example
  const workspace = (
    <View style={{ height: 660, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, overflow: "hidden", backgroundColor: tokens.background }}>
      <View style={{ height: 32, flexDirection: "row", alignItems: "center", paddingHorizontal: 10, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <View style={{ flexDirection: "row", gap: 5 }}>
          <View style={{ width: 9, height: 9, borderRadius: 5, backgroundColor: tokens.danger }} />
          <View style={{ width: 9, height: 9, borderRadius: 5, backgroundColor: tokens.warning }} />
          <View style={{ width: 9, height: 9, borderRadius: 5, backgroundColor: tokens.success }} />
        </View>
        <View style={{ flex: 1 }} />
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Dock — NSH workspace</Text>
        <View style={{ flex: 1 }} />
        <Button label="Options" size="xsmall" variant="ghost" onPress={() => {}} />
      </View>
      <DockSplit
        vertical
        initial={74}
        left={
          <DockSplit
            initial={26}
            left={
              <Dock
                panels={panels("workspace")}
                style={{ borderRadius: 0, borderWidth: 0, height: "100%" }}
              />
            }
            right={<Dock panels={editorPanels} style={{ borderRadius: 0, borderWidth: 0, height: "100%" }} />}
            style={{ borderWidth: 0, borderRadius: 0 }}
          />
        }
        right={<Dock panels={bottomPanels} style={{ borderRadius: 0, borderWidth: 0, height: "100%" }} />}
        style={{ borderWidth: 0, borderRadius: 0, flex: 1 }}
      />
      <View style={{ height: 22, paddingHorizontal: 10, flexDirection: "row", alignItems: "center", borderTopWidth: 1, borderTopColor: tokens.border, backgroundColor: tokens.muted, gap: 10 }}>
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Ln 42, Col 8</Text>
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>TypeScript</Text>
        <View style={{ flex: 1 }} />
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>UTF-8</Text>
      </View>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>
        NSH has no dock host; Dock is a tabbed group and DockSplit is a resizable
        seam. The application owns the layout tree, so the workspace below is a
        composition of those two pieces.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Create a dock area">{dockArea}</ExampleSection>
      <ExampleSection index={2} title="Define a panel">{definePanel}</ExampleSection>
      <ExampleSection index={3} title="Describe the initial layout">{initialLayout}</ExampleSection>
      <ExampleSection index={4} title="Bottom panel">{bottom}</ExampleSection>
      <ExampleSection index={5} title="Style the workspace">{styled}</ExampleSection>
      <ExampleSection index={6} title="Empty group">{empty}</ExampleSection>
      <ExampleSection index={7} title="Runnable example">{workspace}</ExampleSection>
    </Page>
  );
}
