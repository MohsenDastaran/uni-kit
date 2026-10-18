//! `tree` — a hierarchical, expandable list.
import { signal, Text } from "nsh";
import { Tree, type TreeNode } from "../components/tree";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const nodes: TreeNode[] = [
  { id: "src", label: "src", icon: "folder", children: [
    { id: "components", label: "components", icon: "folder", children: [
      { id: "button", label: "button.tsx", icon: "file" },
      { id: "card", label: "card.tsx", icon: "file" },
    ] },
    { id: "app", label: "App.tsx", icon: "file" },
  ] },
  { id: "readme", label: "README.md", icon: "file-text" },
];

export function Demo() {
  const selected = signal("none");
  // section: Basic
  const basic = <Tree nodes={nodes} defaultExpanded={["src"]} onSelect={(next) => selected.set(next)} />;
  // section: Expanded by default
  const expanded = <Tree nodes={nodes} defaultExpanded={["src", "components"]} />;
  // section: end
  return (
    <Page>
      <Caption>Tree keeps the expansion in the component; the selection is the application's.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Expanded by default">{expanded}</ExampleSection>
      <Status>Selected: {selected()}</Status>
    </Page>
  );
}
