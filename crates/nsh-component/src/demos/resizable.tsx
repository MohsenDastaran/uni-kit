//! `resizable` — two panes with a draggable divider, from a plain split to a
//! small IDE workspace.
import { signal, Text, View } from "nsh";
import { Resizable } from "../components/resizable";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const split = signal(30);
  const editorSplit = signal(30);
  const verticalSplit = signal(50);

  const pane = (label: string, background = tokens.background) => (
    <View style={{ flex: 1, alignItems: "center", justifyContent: "center", backgroundColor: background }}>
      <Text style={{ fontSize: 13, color: tokens.foreground }}>{label}</Text>
    </View>
  );

  const ide = (
    <Resizable
      initial={30}
      onResize={(next) => split.set(next)}
      left={
        <View style={{ flex: 1, alignItems: "center", justifyContent: "center", backgroundColor: tokens.background }}>
          <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Nav</Text>
        </View>
      }
      right={
        <View style={{ flex: 1, backgroundColor: tokens.background }}>
          <View style={{ height: 34, alignItems: "center", justifyContent: "center", borderBottomWidth: 1, borderBottomColor: tokens.border }}>
            <Text style={{ fontSize: 12, fontWeight: "600", color: tokens.foreground }}>Toolbar</Text>
          </View>
          <View style={{ flex: 1, alignItems: "center", justifyContent: "center" }}>
            <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Editor</Text>
          </View>
        </View>
      }
      style={{ height: 330, width: 680, maxWidth: "100%" }}
    />
  );

  // section: Usage
  const usage = (
    <Resizable
      left={pane("Left", tokens.muted)}
      right={pane("Right")}
      style={{ height: 190, width: 620, maxWidth: "100%" }}
    />
  );

  // section: IDE Layout
  const ideLayout = (
    <View style={{ width: 680, maxWidth: "100%", gap: 8 }}>
      {ide}
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Navigation split: {Math.round(split())}%</Text>
    </View>
  );

  // section: Basic
  const basic = (
    <Resizable
      initial={40}
      onResize={(next) => editorSplit.set(next)}
      left={pane("Left", tokens.muted)}
      right={pane("Right")}
      style={{ height: 220, width: 620, maxWidth: "100%" }}
    />
  );

  // section: Vertical
  const vertical = (
    <Resizable
      vertical
      initial={55}
      onResize={(next) => verticalSplit.set(next)}
      left={pane("Top", tokens.muted)}
      right={pane("Bottom")}
      style={{ height: 280, width: 620, maxWidth: "100%" }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Drag the divider to resize. The handle reports the movement delta, so the
        panes follow the pointer without jumping or flickering.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="IDE Layout">{ideLayout}</ExampleSection>
      <ExampleSection index={2} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={3} title="Vertical">{vertical}</ExampleSection>
      <Status>
        Sidebar {Math.round(split())}% · editor {Math.round(editorSplit())}% · rows {Math.round(verticalSplit())}%
      </Status>
    </Page>
  );
}
