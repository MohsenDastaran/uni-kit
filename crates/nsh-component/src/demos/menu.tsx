//! `menu` — a single button menu, a two-item menu bar, a context menu and the
//! raw menu panel states.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { ContextMenu, Menu, MenuBar, MenuButton } from "../components/menu";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const editItems = [
  { label: "New Tab", shortcut: "⌘T", onSelect: () => {} },
  { label: "New Window", shortcut: "⌘N", onSelect: () => {} },
  { label: "New Incognito Window", disabled: true, onSelect: () => {} },
  { separator: true },
  { label: "Print…", shortcut: "⌘P", onSelect: () => {} },
];

const fileItems = [
  { label: "New File", icon: "file" as const, shortcut: "⌘N", onSelect: () => {} },
  { label: "Open…", icon: "folder" as const, shortcut: "⌘O", onSelect: () => {} },
  { label: "Save", icon: "download" as const, shortcut: "⌘S", onSelect: () => {} },
  { separator: true },
  { label: "Close Window", shortcut: "⌘W", tone: "danger" as const, onSelect: () => {} },
];

export function Demo() {
  const chosen = signal("Nothing chosen");
  const wrap = signal(true);
  const mini = signal(false);

  // section: Usage
  const usage = (
    <MenuButton
      label="Actions"
      items={[
        { label: "Edit", icon: "pencil", shortcut: "⌘E", onSelect: () => chosen.set("Edit") },
        { label: "Duplicate", icon: "copy", shortcut: "⌘D", onSelect: () => chosen.set("Duplicate") },
        { separator: true },
        { label: "Delete", icon: "trash", tone: "danger", shortcut: "⌫", onSelect: () => chosen.set("Delete") },
      ]}
      onSelect={() => {}}
    />
  );

  // section: DropdownMenu
  const dropdown = (
    <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
      <MenuButton label="File" items={fileItems} variant="ghost" />
      <MenuButton label="Edit" items={editItems} variant="ghost" />
    </View>
  );

  // section: Menubar
  const menubar = (
    <MenuBar
      menus={[
        { label: "File", items: fileItems },
        { label: "Edit", items: editItems },
        { label: "View", items: [{ label: "Reload", shortcut: "⌘R", icon: "refresh-cw", onSelect: () => {} }, { label: "Toggle Sidebar", icon: "panel-left", onSelect: () => {} }] },
        { label: "Theme", items: [{ label: "Light", checked: false, onSelect: () => {} }, { label: "Dark", checked: true, onSelect: () => {} }, { label: "System", checked: false, onSelect: () => {} }] },
      ]}
    />
  );

  // section: ContextMenu
  const context = (
    <ContextMenu
      items={[
        { label: "Open", icon: "external-link", onSelect: () => chosen.set("Open") },
        { label: "Copy Link", icon: "copy", shortcut: "⌘C", onSelect: () => chosen.set("Copy Link") },
        { separator: true },
        { label: "Delete", icon: "trash", tone: "danger", onSelect: () => chosen.set("Delete") },
      ]}
    >
      <View style={{ width: 210, gap: 4 }}>
        <Text style={{ fontSize: 12, fontWeight: "600", color: tokens.foreground }}>Project card</Text>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Click or long-press for actions.</Text>
      </View>
    </ContextMenu>
  );

  // section: Icons
  const icons = (
    <Menu
      width={230}
      items={[
        { label: "Edit", icon: "pencil", shortcut: "⌘E", onSelect: () => chosen.set("Edit") },
        { label: "Duplicate", icon: "copy", shortcut: "⌘D", onSelect: () => chosen.set("Duplicate") },
        { separator: true },
        { label: "Archive", icon: "folder", onSelect: () => chosen.set("Archive") },
        { label: "Delete", icon: "trash", tone: "danger", onSelect: () => chosen.set("Delete") },
      ]}
    />
  );

  // section: Disabled State
  const disabled = (
    <Menu
      width={220}
      items={[
        { label: "Undo", icon: "refresh-cw", shortcut: "⌘Z", disabled: true },
        { label: "Redo", icon: "refresh-cw", shortcut: "⇧⌘Z", onSelect: () => chosen.set("Redo") },
      ]}
    />
  );

  // section: Check state
  const checked = (
    <Menu
      width={240}
      items={[
        { label: "Show sidebar", checked: true, onSelect: () => chosen.set("Show sidebar") },
        { label: "Show minimap", checked: mini(), onSelect: () => mini.update((value) => !value) },
        { label: "Word wrap", checked: wrap(), onSelect: () => wrap.update((value) => !value) },
      ]}
    />
  );

  // section: Separators
  const separators = (
    <Menu
      width={220}
      items={[
        { label: "Rename", onSelect: () => chosen.set("Rename") },
        { label: "Duplicate", onSelect: () => chosen.set("Duplicate") },
        { separator: true },
        { label: "Delete", tone: "danger", onSelect: () => chosen.set("Delete") },
      ]}
    />
  );

  // section: Labels
  const labels = (
    <Menu
      width={240}
      items={[
        { heading: "Workspace" },
        { label: "Overview", icon: "house", onSelect: () => chosen.set("Overview") },
        { label: "Members", icon: "users", onSelect: () => chosen.set("Members") },
        { heading: "Account" },
        { label: "Settings", icon: "settings", onSelect: () => chosen.set("Settings") },
      ]}
    />
  );

  // section: Keyboard Shortcuts
  const shortcuts = (
    <Menu
      width={250}
      items={[
        { label: "Save", icon: "download", shortcut: "⌘S", onSelect: () => chosen.set("Save") },
        { label: "Search", icon: "search", shortcut: "⌘K", onSelect: () => chosen.set("Search") },
        { label: "Command palette", icon: "terminal", shortcut: "⌘⇧P", onSelect: () => chosen.set("Command palette") },
      ]}
    />
  );

  // section: Submenus
  const submenus = (
    <Menu
      width={240}
      items={[
        { label: "Share", icon: "external-link", submenu: true, onSelect: () => chosen.set("Share") },
        { label: "Export", icon: "download", submenu: true, onSelect: () => chosen.set("Export") },
        { separator: true },
        { label: "Move to", icon: "folder", submenu: true, onSelect: () => chosen.set("Move to") },
      ]}
    />
  );

  // section: File Manager Context Menu
  const fileContext = (
    <ContextMenu
      width={220}
      items={[
        { label: "Open", icon: "external-link", onSelect: () => chosen.set("Open file") },
        { label: "Rename", icon: "pencil", onSelect: () => chosen.set("Rename file") },
        { label: "Duplicate", icon: "copy", onSelect: () => chosen.set("Duplicate file") },
        { separator: true },
        { label: "Move to Trash", icon: "trash", tone: "danger", onSelect: () => chosen.set("Move to Trash") },
      ]}
    >
      <View style={{ width: 260, flexDirection: "row", alignItems: "center", gap: 9 }}>
        <View style={{ width: 32, height: 32, borderRadius: 7, alignItems: "center", justifyContent: "center", backgroundColor: tokens.background }}>
          <Icon name="file-text" size={15} color={tokens.mutedForeground} />
        </View>
        <View style={{ gap: 2 }}>
          <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>Q4 Financial Report.pdf</Text>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>2.4 MB · Modified 2 hours ago</Text>
        </View>
      </View>
    </ContextMenu>
  );

  // section: Action Context
  const action = (
    <View style={{ width: 300, maxWidth: "100%", padding: 10, borderRadius: 9, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, gap: 7 }}>
      <Text style={{ fontSize: 11, fontWeight: "600", color: tokens.foreground }}>Comment actions</Text>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Right-click the comment to open this menu.</Text>
      <ContextMenu
        items={[
          { label: "Copy text", icon: "copy", onSelect: () => chosen.set("Copy text") },
          { label: "Quote reply", icon: "message-circle", onSelect: () => chosen.set("Quote reply") },
          { label: "Report", icon: "circle-alert", tone: "danger", onSelect: () => chosen.set("Report") },
        ]}
      >
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Click or long-press the comment.</Text>
      </ContextMenu>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Menu is the panel. MenuButton anchors it to one button, MenuBar switches
        between File / Edit / View / Theme, and ContextMenu opens over a card.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="DropdownMenu">{dropdown}</ExampleSection>
      <ExampleSection index={2} title="Menubar">{menubar}</ExampleSection>
      <ExampleSection index={3} title="ContextMenu">{context}</ExampleSection>
      <ExampleSection index={4} title="Icons">{icons}</ExampleSection>
      <ExampleSection index={5} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={6} title="Check state">{checked}</ExampleSection>
      <ExampleSection index={7} title="Separators">{separators}</ExampleSection>
      <ExampleSection index={8} title="Labels">{labels}</ExampleSection>
      <ExampleSection index={9} title="Keyboard Shortcuts">{shortcuts}</ExampleSection>
      <ExampleSection index={10} title="Submenus">{submenus}</ExampleSection>
      <ExampleSection index={11} title="File Manager Context Menu">{fileContext}</ExampleSection>
      <ExampleSection index={12} title="Action Context">{action}</ExampleSection>
      <Status>{chosen()}</Status>
    </Page>
  );
}
