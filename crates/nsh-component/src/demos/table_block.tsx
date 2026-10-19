// An invoice list: the toolbar that shapes the query, the table it returns and
// the control that replaces the rows.
import { signal, Scroll, Text, View } from "nsh";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Pagination } from "../components/pagination";
import { Select } from "../components/select";
import { Table } from "../components/table";
import { tokens } from "../components/theme";

const invoices = Array.from({ length: 34 }, (_, index) => {
  const status = ["Paid", "Open", "Void"][index % 3];
  return {
    id: `INV-${String(1042 + index)}`,
    customer: ["Acme Inc.", "Northstar Labs", "Sofia Reyes", "Michael Rodriguez", "Ada Lovelace"][index % 5],
    amount: `$${(120 + index * 37).toLocaleString()}.00`,
    status,
    date: ["Oct 12", "Oct 11", "Oct 09", "Oct 04"][index % 4],
  };
});

export function Demo() {
  const query = signal("");
  const status = signal("all");
  const page = signal(1);
  const rows = () =>
    invoices
      .filter((row) => status() === "all" || row.status.toLowerCase() === status())
      .filter((row) => `${row.id} ${row.customer}`.toLowerCase().includes(query().toLowerCase()))
      .slice((page() - 1) * 6, page() * 6);
  const totalPages = () => Math.max(1, Math.ceil(invoices.filter((row) => status() === "all" || row.status.toLowerCase() === status()).length / 6));
  return (
    <View style={{ flex: 1, backgroundColor: tokens.background }}>
      <View style={{ height: 56, paddingHorizontal: 24, flexDirection: "row", alignItems: "center", gap: 10, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Billing</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>/ Invoices</Text>
        <View style={{ flex: 1 }} />
        <Button label="Export" size="small" variant="secondary" icon="download" onPress={() => {}} />
        <Button label="New invoice" size="small" variant="primary" icon="plus" onPress={() => {}} />
      </View>
      <Scroll style={{ flex: 1 }}>
        <View style={{ padding: 24, gap: 16, alignItems: "center" }}>
          <View style={{ width: 920, maxWidth: "100%", gap: 14 }}>
            <View style={{ gap: 3 }}>
              <Text style={{ fontSize: 19, fontWeight: "600", color: tokens.foreground }}>Invoices</Text>
              <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Search, filter and review every invoice in the workspace.</Text>
            </View>
            <View style={{ flexDirection: "row", gap: 8, alignItems: "center", flexWrap: "wrap" }}>
              <Input value={query()} placeholder="Search invoices…" prefixIcon="search" cleanable onChangeText={(next) => { query.set(next); page.set(1); }} style={{ width: 280 }} />
              <Select options={[{ value: "all", label: "All status" }, { value: "paid", label: "Paid" }, { value: "open", label: "Open" }, { value: "void", label: "Void" }]} value={status()} onChange={(next) => { status.set(next); page.set(1); }} style={{ width: 140 }} />
              <View style={{ flex: 1 }} />
              <Button label="Columns" size="small" variant="ghost" icon="layout-dashboard" onPress={() => {}} />
            </View>
            <Table
              card
              columns={[
                { key: "id", label: "Invoice", width: 120 },
                { key: "customer", label: "Customer" },
                {
                  key: "status",
                  label: "Status",
                  width: 100,
                  render: (value) => (
                    <Badge
                      label={value}
                      variant={value === "Paid" ? "success" : value === "Open" ? "warning" : "secondary"}
                    />
                  ),
                },
                { key: "date", label: "Issued", width: 100 },
                { key: "amount", label: "Amount", width: 100, align: "end" },
              ]}
              rows={rows()}
              rowKey="id"
              emptyText="No invoices match this search."
              caption="All amounts in USD."
            />
            <View style={{ gap: 6 }}>
              <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Page {page()} of {totalPages()} · 34 invoices</Text>
              <View style={{ alignItems: "flex-end" }}>
                <Pagination totalPages={totalPages()} page={page()} onPageChange={(next) => page.set(next)} showPageInfo={false} />
              </View>
            </View>
          </View>
        </View>
      </Scroll>
    </View>
  );
}
