//! `table` — the static table and the ReUI table compositions: status badges,
//! actions, team avatars, activity, inventory, issues and vertical profiles.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { Table, type TableColumn } from "../components/table";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const money = (value: string) => `$${Number(value.replace(/[^0-9.]/g, "") || 0).toFixed(2)}`;

export function Demo() {
  const quantity = signal("1");
  const pressed = signal("No row pressed");

  const statusBadge = (value: string) => {
    const tone =
      /paid|active|completed|in stock|open|admin/i.test(value)
        ? "success"
        : /pending|progress|review|low stock|trial|editor/i.test(value)
          ? "warning"
          : /unpaid|expired|cancelled|out of stock|offline|failed/i.test(value)
            ? "danger"
            : "secondary";
    return <Badge label={value} variant={tone as never} />;
  };

  const member = (name: string, role: string, status: string) => (
    <View style={{ flexDirection: "row", alignItems: "center", gap: 7 }}>
      <Avatar name={name} size="xs" />
      <View style={{ gap: 1 }}>
        <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{name}</Text>
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{role}</Text>
      </View>
      <View style={{ flex: 1 }} />
      {statusBadge(status)}
    </View>
  );

  const invoiceColumns: TableColumn[] = [
    { key: "invoice", label: "Invoice" },
    { key: "status", label: "Status" },
    { key: "method", label: "Method" },
    { key: "amount", label: "Amount", align: "end", render: (value) => <Text style={{ fontSize: 11, textAlign: "right", color: tokens.foreground }}>{money(value)}</Text> },
  ];
  const invoices = [
    { invoice: "INV001", status: "Paid", method: "Credit Card", amount: "250" },
    { invoice: "INV002", status: "Pending", method: "PayPal", amount: "150" },
    { invoice: "INV003", status: "Unpaid", method: "Bank Transfer", amount: "350" },
  ];

  // section: Usage
  const usage = (
    <Table
      columns={invoiceColumns}
      rows={invoices}
      card
      caption="A list of your recent invoices."
      style={{ maxWidth: 560 }}
    />
  );

  // section: Basic table
  const basic = (
    <Table columns={invoiceColumns} rows={invoices} card style={{ maxWidth: 560 }} />
  );

  // section: Table with footer
  const footer = (
    <Table
      columns={invoiceColumns}
      rows={invoices}
      footer={{ invoice: "Total", status: "", method: "", amount: "$750.00" }}
      card
      style={{ maxWidth: 560 }}
    />
  );

  // section: Table in card with status badges
  const taskColumns: TableColumn[] = [
    { key: "task", label: "Task" },
    { key: "status", label: "Status", align: "end", render: statusBadge },
  ];
  const tasks = [
    { task: "Design homepage", status: "Completed" },
    { task: "Implement API", status: "In Progress" },
    { task: "Write tests", status: "Pending" },
  ];
  const badges = (
    <Table columns={taskColumns} rows={tasks} card style={{ maxWidth: 380 }} />
  );

  // section: Table card with actions column
  const actionColumns: TableColumn[] = [
    { key: "product", label: "Product" },
    { key: "price", label: "Price", align: "end" },
    {
      key: "actions",
      label: "Actions",
      align: "end",
      render: () => (
        <View style={{ alignItems: "flex-end" }}>
          <Button icon="ellipsis" size="xsmall" variant="ghost" accessibilityLabel="Row actions" onPress={() => pressed.set("Actions opened")} />
        </View>
      ),
    },
  ];
  const actions = (
    <Table
      columns={actionColumns}
      rows={[
        { product: "Wireless Mouse", price: "$29.99" },
        { product: "Mechanical Keyboard", price: "$129.99" },
        { product: "USB-C Hub", price: "$49.99" },
      ]}
      card
      style={{ maxWidth: 420 }}
    />
  );

  // section: Table with interactive elements
  const interactiveColumns: TableColumn[] = [
    { key: "item", label: "Item" },
    {
      key: "quantity",
      label: "Quantity",
      render: () => <Input value={quantity()} onChangeText={(next) => quantity.set(next)} size="small" style={{ width: 52 }} />,
    },
    { key: "price", label: "Price", align: "end" },
  ];
  const interactive = (
    <ColumnTable
      columns={interactiveColumns}
      rows={[
        { item: "Item Alpha", quantity: "1", price: "$10.00" },
        { item: "Item Beta", quantity: "2", price: "$20.00" },
        { item: "Item Gamma", quantity: "1", price: "$30.00" },
      ]}
    />
  );

  // section: Team members table
  const teamColumns: TableColumn[] = [
    { key: "member", label: "Member", render: (_, row) => member(row.name, row.email, row.status) },
    { key: "role", label: "Role", render: (value) => <Badge label={value} variant="secondary" /> },
  ];
  const team = (
    <Table
      columns={teamColumns}
      rows={[
        { name: "Sarah Chen", email: "sarah@example.com", role: "Admin", status: "Active" },
        { name: "Marcus Johnson", email: "marcus@example.com", role: "Developer", status: "Active" },
        { name: "Emily Park", email: "emily@example.com", role: "Designer", status: "Away" },
        { name: "David Kim", email: "david@example.com", role: "Viewer", status: "Offline" },
      ]}
      card
      style={{ maxWidth: 440 }}
    />
  );

  // section: Projects table
  const projectColumns: TableColumn[] = [
    {
      key: "project",
      label: "Project",
      render: (_, row) => (
        <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}>
          <Icon name="folder" size={13} color={tokens.mutedForeground} />
          <View style={{ gap: 1 }}>
            <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.project}</Text>
            <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.description}</Text>
          </View>
        </View>
      ),
    },
    { key: "status", label: "Status", render: (value) => <Badge label={value} variant={value === "Completed" ? "success" : value === "Review" ? "warning" : "info"} /> },
    {
      key: "team",
      label: "Team",
      render: (_, row) => (
        <View style={{ flexDirection: "row", alignItems: "center", gap: -4 }}>
          {row.members.split(",").map((name) => (
            <View style={{ marginEnd: -5 }}>
              <Avatar name={name} size="xs" />
            </View>
          ))}
          <Text style={{ marginStart: 8, fontSize: 9, color: tokens.mutedForeground }}>{row.extra}</Text>
        </View>
      ),
    },
    { key: "due", label: "Due", align: "end" },
  ];
  const projects = (
    <Table
      columns={projectColumns}
      rows={[
        { project: "Website Redesign", description: "Landing page and marketing site", status: "In Progress", members: "Sarah Chen,Marcus Johnson", extra: "+2", due: "Mar 15" },
        { project: "Mobile App v2", description: "iOS and Android release", status: "Review", members: "Emily Park,David Kim", extra: "", due: "Apr 1" },
        { project: "API Integration", description: "Third-party payment gateway", status: "Completed", members: "Sarah Chen", extra: "", due: "Feb 28" },
      ]}
      card
      style={{ maxWidth: 560 }}
    />
  );

  // section: Activity log table
  const activityColumns: TableColumn[] = [
    { key: "user", label: "User", render: (_, row) => member(row.user, "Member", row.status) },
    { key: "action", label: "Action", render: (value) => <Badge label={value} variant="outline" /> },
    { key: "target", label: "Target", render: (value) => <Text style={{ fontSize: 10, fontFamily: "monospace", color: tokens.mutedForeground }}>{value}</Text> },
    { key: "time", label: "Time", align: "end" },
  ];
  const activity = (
    <Table
      columns={activityColumns}
      rows={[
        { user: "Sarah Chen", status: "Active", action: "Deployed", target: "production", time: "2 minutes ago" },
        { user: "Marcus Johnson", status: "Active", action: "Merged", target: "feat/auth-flow", time: "15 minutes ago" },
        { user: "Emily Park", status: "Away", action: "Commented", target: "PR #284", time: "1 hour ago" },
        { user: "David Kim", status: "Offline", action: "Reverted", target: "hotfix/api-crash", time: "3 hours ago" },
      ]}
      card
      style={{ maxWidth: 600 }}
    />
  );

  // section: Subscription table
  const serviceColumns: TableColumn[] = [
    { key: "service", label: "Service", render: (_, row) => <View style={{ gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.service}</Text><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.note}</Text></View> },
    { key: "plan", label: "Plan", render: (value) => <Badge label={value} variant="secondary" /> },
    { key: "billing", label: "Billing" },
    { key: "status", label: "Status", render: statusBadge },
    { key: "manage", label: "Actions", align: "end", render: () => <Button icon="settings" label="Manage" size="xsmall" variant="ghost" onPress={() => pressed.set("Manage subscription")} /> },
  ];
  const services = (
    <Table
      columns={serviceColumns}
      rows={[
        { service: "Vercel Pro", note: "Next: Mar 1, 2025", plan: "Pro", billing: "$20/mo", status: "Active" },
        { service: "GitHub Enterprise", note: "Next: Mar 10, 2025", plan: "Enterprise", billing: "$21/user/mo", status: "Active" },
        { service: "Figma Organization", note: "Next: Apr 5, 2025", plan: "Organization", billing: "$45/editor/mo", status: "Active" },
        { service: "Slack Business+", note: "Next: —", plan: "Business+", billing: "$12.50/user/mo", status: "Cancelled" },
      ]}
      card
      style={{ maxWidth: 620 }}
    />
  );

  // section: Inventory table
  const inventoryColumns: TableColumn[] = [
    { key: "product", label: "Product", render: (_, row) => <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}><View style={{ width: 22, height: 22, borderRadius: 6, backgroundColor: tokens.muted, alignItems: "center", justifyContent: "center" }}><Icon name="package" size={12} color={tokens.mutedForeground} /></View><View style={{ gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.product}</Text><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.sku}</Text></View></View> },
    { key: "category", label: "Category" },
    { key: "stock", label: "Stock", render: (_, row) => <View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}>{statusBadge(row.stock)}{row.count ? <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>({row.count})</Text> : null}</View> },
    { key: "price", label: "Price", align: "end" },
    { key: "actions", label: "", align: "end", render: () => <Button icon="ellipsis" size="xsmall" variant="ghost" accessibilityLabel="Actions" onPress={() => {}} /> },
  ];
  const inventory = (
    <Table
      columns={inventoryColumns}
      rows={[
        { product: "Ergonomic Desk Chair", sku: "FRN-001", category: "Furniture", stock: "In Stock", count: "124", price: "$549.00" },
        { product: "Wireless Headphones", sku: "AUD-012", category: "Audio", stock: "Low Stock", count: "8", price: "$349.99" },
        { product: "4K Ultra HD Monitor", sku: "DSP-019", category: "Displays", stock: "Out of Stock", count: "0", price: "$799.00" },
        { product: "Mechanical Keyboard RGB", sku: "INP-087", category: "Input", stock: "In Stock", count: "56", price: "$179.99" },
      ]}
      card
      style={{ maxWidth: 620 }}
    />
  );

  // section: API keys table
  const keyColumns: TableColumn[] = [
    { key: "name", label: "Name", render: (_, row) => <View style={{ gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.name}</Text><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.used}</Text></View> },
    { key: "key", label: "Key", render: (value) => <Text style={{ fontSize: 10, fontFamily: "monospace", color: tokens.mutedForeground }}>{value}</Text> },
    { key: "permissions", label: "Permissions", render: (_, row) => <View style={{ flexDirection: "row", gap: 3 }}>{row.permissions.split(",").map((item) => <Badge label={item} variant="outline" />)}</View> },
    { key: "status", label: "Status", render: statusBadge },
    { key: "actions", label: "", align: "end", render: () => <View style={{ flexDirection: "row", justifyContent: "flex-end", gap: 2 }}><Button icon="copy" size="xsmall" variant="ghost" accessibilityLabel="Copy key" onPress={() => {}} /><Button icon="refresh-cw" size="xsmall" variant="ghost" accessibilityLabel="Regenerate key" onPress={() => {}} /></View> },
  ];
  const keys = (
    <Table
      columns={keyColumns}
      rows={[
        { name: "Production API", used: "Last used: 2 minutes ago", key: "sk_live_••••••••4f3a", permissions: "read,write", status: "Active" },
        { name: "Staging API", used: "Last used: 1 hour ago", key: "sk_test_••••••••8b2c", permissions: "read,write,admin", status: "Active" },
        { name: "CI/CD Pipeline", used: "Last used: 3 days ago", key: "sk_ci_••••••••1d9e", permissions: "read", status: "Active" },
        { name: "Legacy Integration", used: "Last used: 30 days ago", key: "sk_old_••••••••7a5f", permissions: "read,write", status: "Expired" },
      ]}
      card
      style={{ maxWidth: 620 }}
    />
  );

  // section: Files table
  const fileColumns: TableColumn[] = [
    { key: "name", label: "Name", render: (_, row) => <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}><Icon name={row.kind === "folder" ? "folder" : "file-text"} size={13} color={tokens.mutedForeground} /><Text style={{ fontSize: 11, color: tokens.foreground }}>{row.name}</Text></View> },
    { key: "owner", label: "Owner", render: (value) => <View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}><Avatar name={value} size="xs" /><Text style={{ fontSize: 10, color: tokens.foreground }}>{value}</Text></View> },
    { key: "sharing", label: "Sharing", render: (value) => <Badge label={value} variant="secondary" /> },
    { key: "size", label: "Size", align: "end" },
    { key: "modified", label: "Modified", align: "end" },
  ];
  const files = (
    <Table
      columns={fileColumns}
      rows={[
        { kind: "file", name: "Q4 Financial Report.pdf", owner: "Sarah Chen", sharing: "Team", size: "2.4 MB", modified: "2 hours ago" },
        { kind: "folder", name: "Brand Guidelines", owner: "Emily Park", sharing: "Public", size: "—", modified: "1 day ago" },
        { kind: "file", name: "Product Roadmap 2025.xlsx", owner: "Marcus Johnson", sharing: "Private", size: "856 KB", modified: "3 days ago" },
        { kind: "folder", name: "App Screenshots", owner: "David Kim", sharing: "Team", size: "48.2 MB", modified: "1 week ago" },
      ]}
      card
      style={{ maxWidth: 620 }}
    />
  );

  // section: Issues table
  const issueColumns: TableColumn[] = [
    { key: "id", label: "ID", render: (value) => <Text style={{ fontSize: 9, fontFamily: "monospace", color: tokens.mutedForeground }}>{value}</Text> },
    { key: "issue", label: "Issue", render: (_, row) => <View style={{ gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.issue}</Text><Badge label={row.label} variant="outline" /></View> },
    { key: "assignee", label: "Assignee", render: (value) => <View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}><Avatar name={value} size="xs" /><Text style={{ fontSize: 10, color: tokens.foreground }}>{value}</Text></View> },
    { key: "status", label: "Status", render: statusBadge },
  ];
  const issues = (
    <Table
      columns={issueColumns}
      rows={[
        { id: "ISS-421", issue: "Login page returns 500 on mobile", label: "Bug", assignee: "Sarah Chen", status: "In Progress" },
        { id: "ISS-420", issue: "Add dark mode support", label: "Feature", assignee: "Marcus Johnson", status: "In Review" },
        { id: "ISS-419", issue: "Update user onboarding flow", label: "Improvement", assignee: "Emily Park", status: "Open" },
        { id: "ISS-418", issue: "Refactor API rate limiter module", label: "Tech Debt", assignee: "David Kim", status: "Completed" },
      ]}
      card
      style={{ maxWidth: 600 }}
    />
  );

  // section: Leaderboard table
  const leaderColumns: TableColumn[] = [
    { key: "rank", label: "#", width: 24, render: (value) => <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{value}</Text> },
    { key: "player", label: "Player", render: (_, row) => <View style={{ flexDirection: "row", alignItems: "center", gap: 5 }}><Avatar name={row.player} size="xs" /><View style={{ gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{row.player}</Text><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{row.handle}</Text></View></View> },
    { key: "level", label: "Level", render: (value) => <Badge label={value} variant="secondary" /> },
    { key: "score", label: "Score", align: "end", render: (value) => <Text style={{ fontSize: 11, textAlign: "right", color: tokens.foreground }}>{value}</Text> },
    { key: "change", label: "Change", align: "end", render: (value) => <Text style={{ fontSize: 10, textAlign: "right", color: value.startsWith("+") ? tokens.success : tokens.danger }}>{value}</Text> },
  ];
  const leaderboard = (
    <Table
      columns={leaderColumns}
      rows={[
        { rank: "1", player: "Sarah Chen", handle: "@sarahchen", level: "Diamond", score: "12,840", change: "+320" },
        { rank: "2", player: "Marcus Johnson", handle: "@marcus", level: "Platinum", score: "11,250", change: "+180" },
        { rank: "3", player: "Emily Park", handle: "@emilyp", level: "Platinum", score: "10,890", change: "-45" },
        { rank: "4", player: "David Kim", handle: "@davidk", level: "Gold", score: "9,420", change: "+520" },
        { rank: "5", player: "Sofia Davis", handle: "@sofia", level: "Gold", score: "8,750", change: "+90" },
      ]}
      card
      style={{ maxWidth: 600 }}
    />
  );

  // section: Vertical profile table
  const profile = (
    <View style={{ maxWidth: 440 }}>
      <Table
        columns={[
          { key: "field", label: "", width: 110 },
          {
            key: "value",
            label: "",
            render: (value, row) =>
              row.field === "Employee" ? (
                <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}><Avatar name={value} size="sm" /><View style={{ gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{value}</Text><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Lead Product Designer</Text></View></View>
              ) : row.field === "Status" ? (
                statusBadge(value)
              ) : (
                <Text style={{ fontSize: 11, color: tokens.foreground }}>{value}</Text>
              ),
          },
        ]}
        rows={[
          { field: "Employee", value: "Sarah Chen" },
          { field: "Department", value: "Design & User Experience" },
          { field: "Email", value: "sarah.chen@acmecorp.com" },
          { field: "Location", value: "San Francisco, CA" },
          { field: "Status", value: "Active" },
          { field: "Start Date", value: "March 15, 2022" },
          { field: "Reports To", value: "James Rodriguez, VP of Design" },
        ]}
        card
      />
    </View>
  );

  // section: Server status table
  const server = (
    <View style={{ maxWidth: 440 }}>
      <Table
        columns={[
          { key: "field", label: "Droplet Status", width: 120, render: (value) => <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{value}</Text> },
          {
            key: "value",
            label: "Primary production server",
            render: (value, row) =>
              row.field === "Status" ? (
                <View style={{ flexDirection: "row", alignItems: "center", gap: 4 }}><View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: tokens.success }} /><Text style={{ fontSize: 11, color: tokens.success }}>{value}</Text></View>
              ) : /(\d+)%/.test(value) ? (
                <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}><View style={{ width: 60, height: 4, borderRadius: 2, backgroundColor: tokens.muted }}><View style={{ width: Number(value.replace("%", "")) || 0, height: "100%", borderRadius: 2, backgroundColor: tokens.warning }} /></View><Text style={{ fontSize: 10, color: tokens.foreground }}>{value}</Text></View>
              ) : (
                <Text style={{ fontSize: 11, color: tokens.foreground }}>{value}</Text>
              ),
          },
        ]}
        rows={[
          { field: "Status", value: "Operational" },
          { field: "Instance Type", value: "c6g.2xlarge" },
          { field: "Region", value: "US East (N. Virginia)" },
          { field: "CPU Usage", value: "42%" },
          { field: "Memory", value: "10.7 / 16 GB" },
          { field: "Uptime", value: "47 days, 12 hours" },
          { field: "OS", value: "Amazon Linux 2023" },
          { field: "Tags", value: "production · api · critical" },
        ]}
        card
      />
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>Table is the static grid. Rich rows, badges and actions are cell renderers; DataTable adds sorting and filters.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic table">{basic}</ExampleSection>
      <ExampleSection index={2} title="Table with footer">{footer}</ExampleSection>
      <ExampleSection index={3} title="Table in card with status badges">{badges}</ExampleSection>
      <ExampleSection index={4} title="Table card with actions column">{actions}</ExampleSection>
      <ExampleSection index={5} title="Table with interactive elements">{interactive}</ExampleSection>
      <ExampleSection index={6} title="Team members table">{team}</ExampleSection>
      <ExampleSection index={7} title="Projects table">{projects}</ExampleSection>
      <ExampleSection index={8} title="Activity log table">{activity}</ExampleSection>
      <ExampleSection index={9} title="Subscription table">{services}</ExampleSection>
      <ExampleSection index={10} title="Inventory table">{inventory}</ExampleSection>
      <ExampleSection index={11} title="API keys table">{keys}</ExampleSection>
      <ExampleSection index={12} title="Files table">{files}</ExampleSection>
      <ExampleSection index={13} title="Issues table">{issues}</ExampleSection>
      <ExampleSection index={14} title="Leaderboard table">{leaderboard}</ExampleSection>
      <ExampleSection index={15} title="Vertical profile table">{profile}</ExampleSection>
      <ExampleSection index={16} title="Server status table">{server}</ExampleSection>
      <Status>{pressed()}</Status>
    </Page>
  );
}

function ColumnTable(props: { columns: TableColumn[]; rows: Record<string, string>[] }) {
  return <Table columns={props.columns} rows={props.rows} card style={{ maxWidth: 420 }} />;
}
