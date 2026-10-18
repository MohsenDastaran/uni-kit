//! `table` — a basic tabular surface.
import { Text } from "nsh";
import { Table, type TableColumn } from "../components/table";
import { Caption, ExampleSection, Page } from "./ui";

const columns: TableColumn[] = [
  { key: "name", label: "Name" },
  { key: "role", label: "Role" },
  { key: "status", label: "Status" },
];
const rows = [
  { name: "Ada", role: "Engineer", status: "Active" },
  { name: "Grace", role: "Admiral", status: "Active" },
  { name: "Alan", role: "Mathematician", status: "Away" },
];

// section: Basic
const basic = <Table columns={columns} rows={rows} bordered />;
// section: Striped
const striped = <Table columns={columns} rows={rows} striped />;
// section: Compact
const compact = <Table columns={columns} rows={rows} size="small" />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>Table is the static grid; DataTable adds sorting, paging and a scroll body.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Striped">{striped}</ExampleSection>
      <ExampleSection index={2} title="Compact">{compact}</ExampleSection>
    </Page>
  );
}
