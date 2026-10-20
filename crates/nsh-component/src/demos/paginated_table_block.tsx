// A result set one page at a time: a directory table with a count and a pager
// that replaces the rows.
import { signal, Scroll, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Input } from "../components/input";
import { Pagination } from "../components/pagination";
import { Table } from "../components/table";
import { tokens } from "../components/theme";

const people = Array.from({ length: 47 }, (_, index) => ({
  id: `person-${index + 1}`,
  name: ["Ada Lovelace", "Sofia Reyes", "Michael Rodriguez", "Emma Wilson", "David Kim", "Maria Garcia", "Nick Johnson"][index % 7],
  email: ["ada", "sofia", "michael", "emma", "david", "maria", "nick"][index % 7] + "@northstar.dev",
  role: ["Owner", "Admin", "Editor", "Viewer"][index % 4],
  status: index % 5 === 0 ? "Away" : "Active",
}));

export function Demo() {
  const query = signal("");
  const page = signal(1);
  const filtered = () => people.filter((person) => `${person.name} ${person.email}`.toLowerCase().includes(query().toLowerCase()));
  const totalPages = () => Math.max(1, Math.ceil(filtered().length / 7));
  const rows = () => filtered().slice((page() - 1) * 7, page() * 7);
  return (
    <View style={{ flex: 1, backgroundColor: tokens.background }}>
      <View style={{ height: 56, paddingHorizontal: 24, flexDirection: "row", alignItems: "center", gap: 10, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>People</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>/ Directory</Text>
        <View style={{ flex: 1 }} />
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{filtered().length} members</Text>
      </View>
      <Scroll style={{ flex: 1 }}>
        <View style={{ padding: 24, alignItems: "center" }}>
          <View style={{ width: 860, maxWidth: "100%", gap: 14 }}>
            <View style={{ gap: 3 }}>
              <Text style={{ fontSize: 19, fontWeight: "600", color: tokens.foreground }}>Team directory</Text>
              <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Everyone with access to this workspace, seven per page.</Text>
            </View>
            <Input value={query()} placeholder="Search by name or email…" prefixIcon="search" cleanable onChangeText={(next) => { query.set(next); page.set(1); }} style={{ width: 360 }} />
            <Table
              card
              columns={[
                {
                  key: "name",
                  label: "Member",
                  render: (value, row) => (
                    <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
                      <Avatar name={value} size="xs" />
                      <View style={{ gap: 1 }}>
                        <Text style={{ fontSize: 11, color: tokens.foreground }}>{value}</Text>
                        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.email}</Text>
                      </View>
                    </View>
                  ),
                },
                { key: "role", label: "Role", width: 120 },
                {
                  key: "status",
                  label: "Status",
                  width: 110,
                  render: (value) => <Badge label={value} variant={value === "Active" ? "secondary" : "warning"} />,
                },
              ]}
              rows={rows()}
              rowKey="id"
              emptyText="No members match this search."
            />
            <View style={{ gap: 6 }}>
              <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
                Showing {(page() - 1) * 7 + 1}–{Math.min(page() * 7, filtered().length)} of {filtered().length}
              </Text>
              <View style={{ alignItems: "flex-end" }}>
                <Pagination totalPages={totalPages()} page={page()} onPageChange={(next) => page.set(next)} showPageInfo={false} size="small" />
              </View>
            </View>
          </View>
        </View>
      </Scroll>
    </View>
  );
}
