//! `data-table` — a sorted, searched, filtered and paged table over Table.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { DataTable } from "../components/data_table";
import { type TableColumn } from "../components/table";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const badge = (value: string) => (
  <Badge
    label={value}
    variant={
      /paid|active|completed|in stock/i.test(value)
        ? "success"
        : /pending|review|low stock|trial/i.test(value)
          ? "warning"
          : /unpaid|expired|cancelled|out of stock|inactive/i.test(value)
            ? "danger"
            : "secondary"
    }
  />
);

export function Demo() {
  const chosen = signal("No row selected");

  const invoiceColumns: TableColumn[] = [
    { key: "invoice", label: "Invoice" },
    { key: "status", label: "Status", render: badge },
    { key: "method", label: "Method" },
    { key: "amount", label: "Amount", align: "end" },
  ];
  const invoices = Array.from({ length: 14 }, (_, index) => ({
    invoice: `INV${String(index + 1).padStart(3, "0")}`,
    status: index % 4 === 0 ? "Paid" : index % 4 === 1 ? "Pending" : index % 4 === 2 ? "Unpaid" : "Paid",
    method: index % 3 === 0 ? "Credit Card" : index % 3 === 1 ? "PayPal" : "Bank Transfer",
    amount: `$${(150 + index * 37).toFixed(2)}`,
  }));

  // section: Usage
  const usage = (
    <DataTable
      card
      title="Invoices"
      description="Search, filter and sort recent invoices."
      columns={invoiceColumns}
      rows={invoices}
      pageSize={5}
      searchable
      filters={[
        { key: "status", label: "Status", options: ["Paid", "Pending", "Unpaid"] },
        { key: "method", label: "Method", options: ["Credit Card", "PayPal", "Bank Transfer"] },
      ]}
      sortable
      style={{ maxWidth: 660 }}
    />
  );

  // section: Invoices
  const invoiceOnly = (
    <DataTable
      card
      title="Invoices"
      columns={invoiceColumns}
      rows={invoices}
      pageSize={5}
      searchable
      filters={[{ key: "status", label: "Status", options: ["Paid", "Pending", "Unpaid"] }]}
      sortable
      onRowPress={(row) => chosen.set(row.invoice)}
      style={{ maxWidth: 620 }}
    />
  );

  // section: Column Configuration
  const productColumns: TableColumn[] = [
    {
      key: "product",
      label: "Product",
      render: (_, row) => (
        <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}>
          <View style={{ width: 24, height: 24, borderRadius: 6, backgroundColor: tokens.muted, alignItems: "center", justifyContent: "center" }}>
            <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>◻</Text>
          </View>
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.product}</Text>
            <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.sku}</Text>
          </View>
        </View>
      ),
    },
    { key: "category", label: "Category" },
    { key: "stock", label: "Stock", render: (_, row) => <View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}>{badge(row.stock)}<Text style={{ fontSize: 9, color: tokens.mutedForeground }}>({row.count})</Text></View> },
    { key: "price", label: "Price", align: "end" },
  ];
  const products = Array.from({ length: 10 }, (_, index) => ({
    product: ["Ergonomic Desk Chair", "Noise-Cancelling Headphones", "4K Ultra HD Monitor", "Mechanical Keyboard", "USB-C Docking Station"][index % 5],
    sku: ["FRN-001", "AUD-012", "DSP-019", "INP-087", "ACC-033"][index % 5],
    category: ["Furniture", "Audio", "Displays", "Input", "Accessories"][index % 5],
    stock: index % 4 === 0 ? "In Stock" : index % 4 === 1 ? "Low Stock" : index % 4 === 2 ? "Out of Stock" : "In Stock",
    count: String(120 - index * 9),
    price: `$${(99 + index * 47).toFixed(2)}`,
  }));
  const productTable = (
    <DataTable
      card
      title="Product inventory"
      columns={productColumns}
      rows={products}
      pageSize={5}
      searchable
      sortable
      filters={[{ key: "category", label: "Category", options: ["Furniture", "Audio", "Displays", "Input", "Accessories"] }]}
      pageSizeOptions={[5, 10]}
      style={{ maxWidth: 660 }}
    />
  );

  // section: Sorting Implementation
  const sorting = (
    <DataTable
      card
      title="Sortable inventory"
      description="Press a header to sort; arrows show the direction."
      columns={productColumns}
      rows={products}
      pageSize={6}
      sortable
      searchable
      initialSortKey="price"
      initialSortAscending={false}
      style={{ maxWidth: 660 }}
    />
  );

  // section: Cell Rendering
  const userColumns: TableColumn[] = [
    {
      key: "member",
      label: "Member",
      render: (_, row) => (
        <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}>
          <Avatar name={row.name} size="xs" />
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.name}</Text>
            <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.email}</Text>
          </View>
        </View>
      ),
    },
    { key: "role", label: "Role", render: (value) => <Badge label={value} variant="secondary" /> },
    { key: "status", label: "Status", render: badge },
  ];
  const users = Array.from({ length: 18 }, (_, index) => ({
    name: ["Sarah Chen", "Marcus Johnson", "Emily Park", "David Kim", "Sofia Davis"][index % 5],
    email: ["sarah", "marcus", "emily", "david", "sofia"][index % 5] + "@example.com",
    role: ["Admin", "Developer", "Designer", "Viewer"][index % 4],
    status: index % 7 === 0 ? "Offline" : index % 5 === 0 ? "Away" : "Active",
  }));
  const cell = (
    <DataTable
      card
      title="Team"
      columns={userColumns}
      rows={users}
      pageSize={5}
      searchable
      sortable
      filters={[
        { key: "role", label: "Role", options: ["Admin", "Developer", "Designer", "Viewer"] },
        { key: "status", label: "Status", options: ["Active", "Away", "Offline"] },
      ]}
      onRowPress={(row) => chosen.set(row.name)}
      style={{ maxWidth: 600 }}
    />
  );

  // section: Selection Modes
  const selected = signal<string[]>(["INV003"]);
  const selection = (
    <DataTable
      card
      title="Selectable invoices"
      description="Row selection is application state; the table only highlights it."
      columns={invoiceColumns}
      rows={invoices}
      pageSize={6}
      searchable
      sortable
      selectedKeys={selected()}
      rowKey="invoice"
      onRowPress={(row) => {
        const key = row.invoice;
        selected.update((keys) => (keys.includes(key) ? keys.filter((item) => item !== key) : [...keys, key]));
      }}
      style={{ maxWidth: 620 }}
    />
  );

  // section: Infinite Loading / Pagination
  const paging = (
    <DataTable
      card
      title="All invoices"
      columns={invoiceColumns}
      rows={invoices}
      pageSize={4}
      pageSizeOptions={[4, 7, 14]}
      searchable
      sortable
      style={{ maxWidth: 620 }}
    />
  );

  // section: Financial Data Table
  const financialColumns: TableColumn[] = [
    { key: "date", label: "Date" },
    { key: "description", label: "Description" },
    { key: "category", label: "Category", render: (value) => <Badge label={value} variant="outline" /> },
    { key: "amount", label: "Amount", align: "end", render: (value) => <Text style={{ fontSize: 11, textAlign: "right", color: value.startsWith("-") ? tokens.danger : tokens.success }}>{value}</Text> },
  ];
  const financial = Array.from({ length: 12 }, (_, index) => ({
    date: `Oct ${String(index + 1).padStart(2, "0")}`,
    description: ["Stripe payout", "AWS invoice", "Figma subscription", "Payroll", "Office rent"][index % 5],
    category: ["Revenue", "Infrastructure", "Design", "Payroll", "Office"][index % 5],
    amount: index % 3 === 0 ? `-$${(120 + index * 31).toFixed(2)}` : `+$${(840 + index * 57).toFixed(2)}`,
  }));
  const finance = (
    <DataTable
      card
      title="Financial activity"
      columns={financialColumns}
      rows={financial}
      pageSize={6}
      searchable
      sortable
      filters={[{ key: "category", label: "Category", options: ["Revenue", "Infrastructure", "Design", "Payroll", "Office"] }]}
      style={{ maxWidth: 680 }}
    />
  );

  // section: User Management Table
  const managementColumns: TableColumn[] = [
    ...userColumns,
    { key: "actions", label: "", align: "end", render: () => <Button icon="ellipsis" size="xsmall" variant="ghost" accessibilityLabel="Member actions" onPress={() => {}} /> },
  ];
  const management = (
    <DataTable
      card
      title="User management"
      columns={managementColumns}
      rows={users}
      pageSize={6}
      searchable
      sortable
      filters={[{ key: "role", label: "Role", options: ["Admin", "Developer", "Designer", "Viewer"] }, { key: "status", label: "Status", options: ["Active", "Away", "Offline"] }]}
      style={{ maxWidth: 680 }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        DataTable composes Table with inline search, filter chips, sortable headers and
        pagination. The data stays with the application; no modal is involved.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Invoices">{invoiceOnly}</ExampleSection>
      <ExampleSection index={2} title="Column Configuration">{productTable}</ExampleSection>
      <ExampleSection index={3} title="Sorting Implementation">{sorting}</ExampleSection>
      <ExampleSection index={4} title="Cell Rendering">{cell}</ExampleSection>
      <ExampleSection index={5} title="Selection Modes">{selection}</ExampleSection>
      <ExampleSection index={6} title="Infinite Loading / Pagination">{paging}</ExampleSection>
      <ExampleSection index={7} title="Financial Data Table">{finance}</ExampleSection>
      <ExampleSection index={8} title="User Management Table">{management}</ExampleSection>
      <Status>{chosen()} · {selected().length} selected</Status>
    </Page>
  );
}
