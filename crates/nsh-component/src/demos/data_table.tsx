//! `data-table` — a sortable, paginated table.
import { DataTable } from "../components/data_table";
import { type TableColumn } from "../components/table";
import { Caption, ExampleSection, Page } from "./ui";

const columns: TableColumn[] = [
  { key: "name", label: "Name" },
  { key: "role", label: "Role" },
  { key: "status", label: "Status" },
];
const rows = Array.from({ length: 24 }, (_, index) => ({
  name: `Person ${index + 1}`,
  role: index % 3 === 0 ? "Engineer" : index % 3 === 1 ? "Designer" : "Writer",
  status: index % 4 === 0 ? "Away" : "Active",
}));
// section: Basic
const basic = <DataTable columns={columns} rows={rows} pageSize={8} />;
// section: Sorting
const sorting = <DataTable columns={columns} rows={rows} sortable pageSize={5} />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>DataTable composes Table with sorting and Pagination; the rows are the application's data.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Sorting">{sorting}</ExampleSection>
    </Page>
  );
}
