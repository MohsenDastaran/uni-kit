//! `list` — a recycling ListView dressed with premium rows, sections, avatars,
//! icon tiles, search and empty states.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Empty } from "../components/empty";
import { Input } from "../components/input";
import { List, ListHeader, ListRow, ListSectionHeader } from "../components/list";
import { Switch } from "../components/switch";
import { Tag } from "../components/tag";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

type Contact = { name: string; email: string; role: string; status: "Active" | "Away" | "Offline" };
type Row =
  | { kind: "section"; label: string; count?: number }
  | { kind: "person"; name: string; email: string; status: string }
  | { kind: "file"; name: string; kindLabel: string; size: string; icon: "file-text" | "folder" | "image" };

const contacts: Contact[] = [
  { name: "Sarah Chen", email: "sarah@example.com", role: "Design", status: "Active" },
  { name: "Marcus Johnson", email: "marcus@example.com", role: "Engineering", status: "Active" },
  { name: "Emily Park", email: "emily@example.com", role: "Product", status: "Away" },
  { name: "David Kim", email: "david@example.com", role: "Engineering", status: "Offline" },
  { name: "Sofia Davis", email: "sofia@example.com", role: "Marketing", status: "Active" },
  { name: "James Wilson", email: "james@example.com", role: "Sales", status: "Away" },
];

const files: Row[] = [
  { kind: "section", label: "Recent" },
  { kind: "file", name: "Q4 Financial Report.pdf", kindLabel: "PDF", size: "2.4 MB", icon: "file-text" },
  { kind: "file", name: "Product Roadmap.xlsx", kindLabel: "Sheet", size: "856 KB", icon: "file-text" },
  { kind: "section", label: "Folders" },
  { kind: "file", name: "Design System", kindLabel: "Folder", size: "24 items", icon: "folder" },
  { kind: "file", name: "App Screenshots", kindLabel: "Folder", size: "48 items", icon: "folder" },
  { kind: "file", name: "Brand Assets", kindLabel: "Folder", size: "12 items", icon: "folder" },
];

export function Demo() {
  const query = signal("");
  const selected = signal("");
  const contactSearch = signal("");
  const filter = signal("All");
  const visibleContacts = () => contacts.filter((contact) => !contactSearch() || contact.name.toLowerCase().includes(contactSearch().toLowerCase()));

  // section: Usage
  const usage = (
    <List
      items={contacts.slice(0, 4)}
      variant="inset"
      style={{ height: 300, width: 600, maxWidth: "100%" }}
      header={<ListHeader title="Team" description="People in this workspace." />}
      renderItem={(contact) => (
        <ListRow
          leading={<Avatar name={contact.name} size="xs" />}
          title={contact.name}
          description={contact.email}
          trailing={<Badge label={contact.status} variant={contact.status === "Active" ? "success" : contact.status === "Away" ? "warning" : "secondary"} />}
        />
      )}
    />
  );

  // section: Basic List
  const basic = (
    <List
      items={contacts.slice(0, 5)}
      variant="inset"
      style={{ height: 320, width: 600, maxWidth: "100%" }}
      header={<ListHeader title="Contacts" description="A simple row list with recycling." />}
      renderItem={(contact) => <ListRow title={contact.name} description={contact.role} chevron />}
    />
  );

  // section: List with Sections
  const sections = (
    <List
      items={files}
      style={{ height: 360, width: 640, maxWidth: "100%" }}
      renderItem={(row) =>
        row.kind === "section" ? (
          <ListSectionHeader label={row.label} />
        ) : row.kind === "file" ? (
          <ListRow
            icon={row.icon}
            title={row.name}
            description={row.kindLabel}
            meta={row.size}
            chevron
          />
        ) : null
      }
    />
  );

  // section: List Items with Icons and Actions
  const icons = (
    <List
      items={[
        { icon: "folder" as const, title: "Project files", description: "24 items", badge: "New", tone: "info" as const },
        { icon: "users" as const, title: "Team members", description: "8 collaborators", tone: "success" as const },
        { icon: "settings" as const, title: "Workspace settings", description: "Permissions and billing" },
        { icon: "trash" as const, title: "Archived projects", description: "12 archived items", tone: "danger" as const },
      ]}
      variant="inset"
      style={{ height: 300, width: 600, maxWidth: "100%" }}
      renderItem={(item) => (
        <ListRow
          icon={item.icon}
          iconTone={item.tone}
          title={item.title}
          description={item.description}
          badge={item.badge}
          chevron
          onPress={() => selected.set(item.title)}
        />
      )}
    />
  );

  // section: List with Search
  const search = (
    <View style={{ gap: 8, height: 380, width: 600, maxWidth: "100%" }}>
      <Input value={contactSearch()} onChangeText={(next) => contactSearch.set(next)} prefixIcon="search" placeholder="Search contacts…" cleanable />
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{visibleContacts().length} contacts</Text>
      <List
        items={visibleContacts()}
        style={{ flex: 1, width: 600, maxWidth: "100%" }}
        empty={<Empty icon="search" title="No contacts" description="Try another name." size="small" />}
        renderItem={(contact) => (
          <ListRow
            leading={<Avatar name={contact.name} size="xs" />}
            title={contact.name}
            description={contact.email}
            trailing={<Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{contact.role}</Text>}
            onPress={() => selected.set(contact.name)}
          />
        )}
      />
    </View>
  );

  // section: List Events
  const events = (
    <List
      items={contacts.slice(0, 4)}
      style={{ height: 280, width: 560, maxWidth: "100%" }}
      renderItem={(contact) => (
        <ListRow
          title={contact.name}
          description={`Tap to select · ${contact.role}`}
          selected={selected() === contact.name}
          onPress={() => selected.set(contact.name)}
          trailing={selected() === contact.name ? <Text style={{ fontSize: 10, color: tokens.primary }}>Selected</Text> : null}
        />
      )}
    />
  );

  // section: Custom Empty State
  const empty = (
    <List
      items={[]}
      style={{ height: 260, width: 520, maxWidth: "100%" }}
      empty={<Empty icon="folder" title="No projects yet" description="Create your first project to get started." size="small" />}
    />
  );

  // section: Contact List with Sections
  const contactSections = (
    <List
      items={[
        { kind: "section" as const, label: "Online", count: 3 },
        ...contacts.filter((contact) => contact.status === "Active").map((contact) => ({ kind: "person" as const, contact })),
        { kind: "section" as const, label: "Away", count: 2 },
        ...contacts.filter((contact) => contact.status !== "Active").map((contact) => ({ kind: "person" as const, contact })),
      ]}
      style={{ height: 380, width: 640, maxWidth: "100%" }}
      renderItem={(row) =>
        row.kind === "section" ? (
          <ListSectionHeader label={row.label} count={row.count} />
        ) : (
          <ListRow
            leading={<Avatar name={row.contact.name} size="xs" />}
            title={row.contact.name}
            description={row.contact.email}
            trailing={<Badge label={row.contact.status} variant={row.contact.status === "Active" ? "success" : row.contact.status === "Away" ? "warning" : "secondary"} />}
            onPress={() => selected.set(row.contact.name)}
          />
        )
      }
    />
  );

  // section: Different Item Styles
  const styles = (
    <List
      items={[
        { title: "Sounds", description: "New message sounds", icon: "activity" as const },
        { title: "Notifications", description: "Mentions and replies", icon: "bell" as const },
        { title: "Privacy", description: "Who can see your activity", icon: "lock" as const },
      ]}
      variant="inset"
      dividers={false}
      style={{ height: 280, width: 560, maxWidth: "100%" }}
      renderItem={(item) => (
        <ListRow
          icon={item.icon}
          title={item.title}
          description={item.description}
          trailing={<Switch defaultChecked={item.title === "Sounds"} />}
        />
      )}
    />
  );

  // section: File Browser List
  const browser = (
    <List
      items={files}
      style={{ height: 360, width: 640, maxWidth: "100%" }}
      header={<ListHeader title="Files" description="Recent and shared folders." action={<Tag label="Cloud" variant="info" />} />}
      renderItem={(row) =>
        row.kind === "section" ? (
          <ListSectionHeader label={row.label} />
        ) : row.kind === "file" ? (
          <ListRow
            icon={row.icon}
            title={row.name}
            description={row.kindLabel}
            meta={row.size}
            trailing={<Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Open</Text>}
            onPress={() => selected.set(row.name)}
          />
        ) : null
      }
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        List is the styled row list; VirtualList is the same row language over the
        raw recycling list. Rows carry avatars, icon tiles, badges and actions.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic List">{basic}</ExampleSection>
      <ExampleSection index={2} title="List with Sections">{sections}</ExampleSection>
      <ExampleSection index={3} title="List Items with Icons and Actions">{icons}</ExampleSection>
      <ExampleSection index={4} title="List with Search">{search}</ExampleSection>
      <ExampleSection index={5} title="List Events">{events}</ExampleSection>
      <ExampleSection index={6} title="Custom Empty State">{empty}</ExampleSection>
      <ExampleSection index={7} title="Contact List with Sections">{contactSections}</ExampleSection>
      <ExampleSection index={8} title="Different Item Styles">{styles}</ExampleSection>
      <ExampleSection index={9} title="File Browser List">{browser}</ExampleSection>
      <Status>{selected() ? `Selected: ${selected()}` : "Nothing selected"}</Status>
    </Page>
  );
}
