//! `command` — the ReUI command panel: search, grouped rows, avatars, role
//! badges and shortcut keys. Inline by default; modal is one option.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Command, type CommandItem } from "../components/command";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Status } from "./ui";

const navigation: CommandItem[] = [
  { label: "Home", group: "Navigation", icon: "house", shortcut: "⌘H", onSelect: () => {} },
  { label: "Inbox", group: "Navigation", icon: "inbox", shortcut: "⌘I", onSelect: () => {} },
  { label: "Documents", group: "Navigation", icon: "file-text", shortcut: "⌘D", onSelect: () => {} },
  { label: "Folders", group: "Navigation", icon: "folder", shortcut: "⌘F", onSelect: () => {} },
  { label: "New File", group: "Actions", icon: "plus", shortcut: "⌘N", onSelect: () => {} },
  { label: "New Folder", group: "Actions", icon: "folder", shortcut: "⇧⌘N", onSelect: () => {} },
  { label: "Copy", group: "Actions", icon: "copy", shortcut: "⌘C", onSelect: () => {} },
];

const create: CommandItem[] = [
  { label: "New Project", group: "Create", icon: "plus", shortcut: "⌘N", onSelect: () => {} },
  { label: "New Document", group: "Create", icon: "file-text", shortcut: "⇧⌘N", onSelect: () => {} },
  { label: "Invite Member", group: "Create", icon: "users", shortcut: "⌘I", onSelect: () => {} },
  { label: "Go to Dashboard", group: "Navigate", icon: "house", shortcut: "⌘D", onSelect: () => {} },
  { label: "Go to Inbox", group: "Navigate", icon: "inbox", shortcut: "⇧⌘I", onSelect: () => {} },
  { label: "Toggle Dark Mode", group: "System", icon: "moon", shortcut: "⇧⌘D", onSelect: () => {} },
];

export function Demo() {
  const modalOpen = signal(false);
  const chosen = signal("Nothing chosen yet");
  const query = signal("");
  const dynamic = signal<CommandItem[]>([
    { label: "Deploy", group: "Actions", icon: "send", shortcut: "⌘D" },
    { label: "Rollback", group: "Actions", icon: "refresh-cw", shortcut: "⌘R" },
  ]);
  const team: CommandItem[] = [
    { label: "Alex Johnson", group: "Team Members", description: "alex@example.com", badge: "Admin", badgeTone: "default", avatarName: "Alex Johnson", keywords: "alex johnson alex@example.com" },
    { label: "Sarah Chen", group: "Team Members", description: "sarah@example.com", badge: "Editor", badgeTone: "info", avatarName: "Sarah Chen", keywords: "sarah chen sarah@example.com" },
    { label: "David Kim", group: "Team Members", description: "david@example.com", badge: "Viewer", badgeTone: "success", avatarName: "David Kim", keywords: "david kim david@example.com" },
    { label: "Emma Wilson", group: "Team Members", description: "emma@example.com", badge: "Admin", badgeTone: "default", avatarName: "Emma Wilson", keywords: "emma wilson emma@example.com" },
    { label: "Michael Rodriguez", group: "Team Members", description: "michael@example.com", badge: "Editor", badgeTone: "info", avatarName: "Michael Rodriguez", keywords: "michael rodriguez michael@example.com" },
  ];

  const withSelect = (items: CommandItem[]) =>
    items.map((item) => ({ ...item, onSelect: () => chosen.set(item.label) }));

  // section: Usage
  const usage = (
    <Command
      inline
      items={withSelect(team)}
      placeholder="Search by name or email…"
      width={360}
      maxHeight={300}
    />
  );

  // section: Inline
  const inline = (
    <Command
      inline
      items={withSelect(navigation)}
      placeholder="Type a command or search…"
      width={360}
      maxHeight={300}
    />
  );

  // section: Quick Actions Without Search
  const quick = (
    <Command
      inline
      showSearch={false}
      items={withSelect([
        { label: "New File", group: "Actions", icon: "file-text", shortcut: "⌘N" },
        { label: "New Folder", group: "Actions", icon: "folder", shortcut: "⇧⌘N" },
        { label: "Duplicate", group: "Actions", icon: "copy", shortcut: "⌘D" },
        { label: "Archive", group: "Actions", icon: "inbox", shortcut: "⌘E" },
      ])}
      width={320}
    />
  );

  // section: In a Dialog
  const dialog = (
    <>
      <Button label="Open command palette" variant="primary" onPress={() => modalOpen.set(true)} />
      <Command
        open={modalOpen()}
        hotkey
        items={withSelect(navigation)}
        onOpenChange={(next) => modalOpen.set(next)}
        width={420}
      />
    </>
  );

  // section: Callbacks and Actions
  const callbacks = (
    <Column gap={8} style={{ width: 360 }}>
      <Command inline items={withSelect(create.slice(0, 3))} width={360} maxHeight={200} />
      <Status>{chosen()}</Status>
    </Column>
  );

  // section: Dynamic Entries
  const dynamicEntries = (
    <Column gap={8} style={{ width: 360 }}>
      <Command inline items={dynamic()} width={360} maxHeight={200} />
      <Button
        label="Add command"
        size="small"
        variant="ghost"
        onPress={() =>
          dynamic.update((items) => [
            ...items,
            { label: `Command ${items.length + 1}`, group: "Actions", icon: "terminal", shortcut: `⌘${items.length + 1}` },
          ])
        }
      />
    </Column>
  );

  // section: Scrollable
  const scrollable = (
    <Command
      inline
      items={Array.from({ length: 16 }, (_, index) => ({
        label: `Command ${index + 1}`,
        group: index < 8 ? "First group" : "Second group",
        icon: "terminal",
        shortcut: index % 3 === 0 ? `⌘${index + 1}` : undefined,
        onSelect: () => chosen.set(`Command ${index + 1}`),
      }))}
      width={340}
      maxHeight={220}
    />
  );

  // section: Variable-height rows
  const variable = (
    <Command
      inline
      items={[
        { label: "Invite teammate", description: "Send an invitation to the workspace", group: "Team", icon: "users", shortcut: "⌘I" },
        { label: "Create project", description: "Start a project from a template", group: "Projects", icon: "folder", shortcut: "⌘P" },
        { label: "Export report", description: "Download the current view as CSV", group: "Reports", icon: "download", shortcut: "⌘E" },
      ]}
      width={360}
    />
  );

  // section: Search panel
  const search = (
    <Column gap={8} style={{ width: 360 }}>
      <Command
        inline
        query={query()}
        onQueryChange={(next) => query.set(next)}
        items={withSelect(team)}
        placeholder="Search by name or email…"
        width={360}
        maxHeight={260}
      />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
        Query: {query() || "(empty)"}
      </Text>
    </Column>
  );

  // section: Last confirmed
  const lastConfirmed = (
    <Column gap={8} style={{ width: 360 }}>
      <Command inline items={withSelect(create.slice(0, 2))} width={360} maxHeight={180} />
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Last confirmed: {chosen()}</Text>
    </Column>
  );

  // section: Team members
  const teamPanel = (
    <Command
      inline
      items={team.map((item) => ({ ...item, onSelect: () => chosen.set(item.label) }))}
      placeholder="Search by name or email…"
      width={360}
      maxHeight={300}
      footer={
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>
          {team.length} members · click a row to select
        </Text>
      }
    />
  );

  // section: Navigation commands
  const navigationPanel = (
    <Command
      inline
      items={navigation.slice(0, 4).map((item) => ({ ...item, onSelect: () => chosen.set(item.label) }))}
      placeholder="Search by name or email…"
      width={360}
      maxHeight={260}
    />
  );

  // section: Create commands
  const createPanel = (
    <Command
      inline
      items={create.map((item) => ({ ...item, onSelect: () => chosen.set(item.label) }))}
      placeholder="What do you need?"
      width={360}
      maxHeight={300}
    />
  );

  // section: Grouped commands
  const grouped = (
    <Command
      inline
      items={withSelect([...navigation, ...create.slice(0, 3)])}
      placeholder="Search commands…"
      width={380}
      maxHeight={300}
      footer={<Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Use ↑ ↓ to move, Enter to run.</Text>}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        The command panel filters as you type and groups its rows. It is inline by default;
        `inline={false}` presents the same panel as a modal.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Inline">{inline}</ExampleSection>
      <ExampleSection index={2} title="Quick Actions Without Search">{quick}</ExampleSection>
      <ExampleSection index={3} title="In a Dialog">{dialog}</ExampleSection>
      <ExampleSection index={4} title="Callbacks and Actions">{callbacks}</ExampleSection>
      <ExampleSection index={5} title="Dynamic Entries">{dynamicEntries}</ExampleSection>
      <ExampleSection index={6} title="Scrollable">{scrollable}</ExampleSection>
      <ExampleSection index={7} title="Variable-height rows">{variable}</ExampleSection>
      <ExampleSection index={8} title="Search panel">{search}</ExampleSection>
      <ExampleSection index={9} title="Last confirmed">{lastConfirmed}</ExampleSection>
      <ExampleSection index={10} title="Team members">{teamPanel}</ExampleSection>
      <ExampleSection index={11} title="Navigation commands">{navigationPanel}</ExampleSection>
      <ExampleSection index={12} title="Create commands">{createPanel}</ExampleSection>
      <ExampleSection index={13} title="Grouped commands">{grouped}</ExampleSection>
    </Page>
  );
}
