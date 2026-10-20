//! `virtual-list` — a recycling list over thousands of rows, with the same
//! premium row language as List.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Empty } from "../components/empty";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { ListHeader, ListRow } from "../components/list";
import { tokens } from "../components/theme";
import { VirtualList } from "../components/virtual_list";
import { Caption, ExampleSection, Page, Status } from "./ui";

const rows = Array.from({ length: 5000 }, (_, index) => ({ id: String(index), name: `Row ${index + 1}`, email: `person${index + 1}@example.com` }));
const contacts = Array.from({ length: 1000 }, (_, index) => ({
  id: String(index),
  name: ["Sarah Chen", "Marcus Johnson", "Emily Park", "David Kim", "Sofia Davis"][index % 5] + (index > 4 ? ` ${Math.floor(index / 5) + 1}` : ""),
  email: ["sarah", "marcus", "emily", "david", "sofia"][index % 5] + `${index}@example.com`,
  status: index % 7 === 0 ? "Offline" : index % 5 === 0 ? "Away" : "Active",
}));
const files = Array.from({ length: 2000 }, (_, index) => ({
  id: String(index),
  name: ["Q4 Financial Report", "Product Roadmap", "Brand Guidelines", "App Screenshots", "Design System"][index % 5] + ` ${index + 1}`,
  kind: index % 4 === 0 ? "folder" : "file",
  size: index % 4 === 0 ? `${8 + (index % 30)} items` : `${(0.2 + (index % 40) / 10).toFixed(1)} MB`,
}));

export function Demo() {
  const query = signal("");
  const tableQuery = signal("");
  const selected = signal("");
  const filteredContacts = () => contacts.filter((contact) => !query() || contact.name.toLowerCase().includes(query().toLowerCase()));
  const filteredRows = () => rows.filter((row) => !tableQuery() || row.name.toLowerCase().includes(tableQuery().toLowerCase()));

  // section: Usage
  const usage = (
    <VirtualList
      items={rows.slice(0, 1000)}
      itemHeight={44}
      contentPadding={14}
      style={{ height: 280, width: 700, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10 }}
      header={<ListHeader title="VirtualList" description="Only the visible rows exist." />}
      renderItem={(row, index) => (
        <View style={{ height: 44, justifyContent: "center" }}>
          <ListRow title={row.name} description={row.email} meta={`#${index + 1}`} />
        </View>
      )}
    />
  );

  // section: Basic Vertical Virtual List
  const basic = (
    <VirtualList
      items={rows}
      itemHeight={42}
      contentPadding={12}
      style={{ height: 360, width: 700, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10 }}
      header={<ListHeader title="5,000 rows" description="Scroll to recycle row cells." />}
      renderItem={(row, index) => (
        <View style={{ height: 42, flexDirection: "row", alignItems: "center", gap: 8 }}>
          <Text style={{ width: 30, fontSize: 9, color: tokens.mutedForeground }}>{index + 1}</Text>
          <Text style={{ flex: 1, fontSize: 11, color: tokens.foreground }}>{row.name}</Text>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.email}</Text>
        </View>
      )}
    />
  );

  // section: Contact list
  const contactList = (
    <View style={{ height: 420, width: 650, maxWidth: "100%", gap: 8 }}>
      <Input value={query()} onChangeText={(next) => query.set(next)} prefixIcon="search" placeholder="Search 1,000 contacts…" cleanable />
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{filteredContacts().length} contacts</Text>
      <VirtualList
        items={filteredContacts()}
        itemHeight={56}
        contentPadding={0}
        style={{ flex: 1, borderWidth: 1, borderColor: tokens.border, borderRadius: 10 }}
        empty={<Empty icon="search" title="No contacts" description="Try another name." size="small" />}
        renderItem={(contact) => (
          <View style={{ height: 56, justifyContent: "center" }}>
            <ListRow
              leading={<Avatar name={contact.name} size="xs" />}
              title={contact.name}
              description={contact.email}
              trailing={<Badge label={contact.status} variant={contact.status === "Active" ? "success" : contact.status === "Away" ? "warning" : "secondary"} />}
              onPress={() => selected.set(contact.name)}
            />
          </View>
        )}
      />
    </View>
  );

  // section: Table-like Layout with Multiple Columns
  const table = (
    <View style={{ height: 400, width: 720, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10, overflow: "hidden" }}>
      <View style={{ padding: 10, gap: 8, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <Input value={tableQuery()} onChangeText={(next) => tableQuery.set(next)} prefixIcon="search" placeholder="Filter rows…" cleanable />
        <View style={{ flexDirection: "row", gap: 8 }}>
          <Text style={{ width: 32, fontSize: 9, color: tokens.mutedForeground }}>#</Text>
          <Text style={{ flex: 1, fontSize: 9, color: tokens.mutedForeground }}>Name</Text>
          <Text style={{ width: 150, fontSize: 9, color: tokens.mutedForeground }}>Email</Text>
          <Text style={{ width: 60, fontSize: 9, color: tokens.mutedForeground }}>Status</Text>
        </View>
      </View>
      <VirtualList
        items={filteredRows()}
        itemHeight={42}
        contentPadding={10}
        style={{ flex: 1 }}
        renderItem={(row, index) => (
          <View style={{ height: 42, flexDirection: "row", alignItems: "center", gap: 8 }}>
            <Text style={{ width: 32, fontSize: 9, color: tokens.mutedForeground }}>{index + 1}</Text>
            <Text style={{ flex: 1, fontSize: 11, color: tokens.foreground }}>{row.name}</Text>
            <Text style={{ width: 150, fontSize: 9, color: tokens.mutedForeground }}>{row.email}</Text>
            <View style={{ width: 60 }}><Badge label={index % 3 === 0 ? "Active" : "Away"} variant={index % 3 === 0 ? "success" : "warning"} /></View>
          </View>
        )}
      />
    </View>
  );

  // section: File Explorer with Virtual Scrolling
  const browser = (
    <VirtualList
      items={files}
      itemHeight={54}
      contentPadding={10}
      style={{ height: 380, width: 720, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10 }}
      header={<ListHeader title="Project files" description="2,000 files and folders." action={<Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Virtualized</Text>} />}
      renderItem={(file) => (
        <View style={{ height: 54, justifyContent: "center" }}>
          <ListRow
            icon={file.kind === "folder" ? "folder" : "file-text"}
            title={file.name}
            description={file.kind === "folder" ? "Folder" : "Document"}
            meta={file.size}
            chevron
          />
        </View>
      )}
    />
  );

  // section: Icon and metadata rows
  const metadata = (
    <VirtualList
      items={contacts.slice(0, 800)}
      itemHeight={60}
      contentPadding={12}
      style={{ height: 360, width: 700, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10 }}
      header={<ListHeader title="Members" description="Avatar, email and status per row." />}
      renderItem={(contact) => (
        <View style={{ height: 60, justifyContent: "center" }}>
          <ListRow
            leading={<Avatar name={contact.name} size="xs" />}
            title={contact.name}
            description={contact.email}
            trailing={<View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}><Icon name="circle" size={7} color={contact.status === "Active" ? tokens.success : tokens.mutedForeground} /><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{contact.status}</Text></View>}
            onPress={() => selected.set(contact.name)}
          />
        </View>
      )}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Only the rows near the viewport exist; scrolling recycles their cells.
        The same premium row language as List keeps large data readable.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Vertical Virtual List">{basic}</ExampleSection>
      <ExampleSection index={2} title="Contact list">{contactList}</ExampleSection>
      <ExampleSection index={3} title="Table-like Layout with Multiple Columns">{table}</ExampleSection>
      <ExampleSection index={4} title="File Explorer with Virtual Scrolling">{browser}</ExampleSection>
      <ExampleSection index={5} title="Icon and metadata rows">{metadata}</ExampleSection>
      <Status>{selected() ? `Selected: ${selected()}` : "5,000 rows · 1,000 contacts · 2,000 files"}</Status>
    </Page>
  );
}
