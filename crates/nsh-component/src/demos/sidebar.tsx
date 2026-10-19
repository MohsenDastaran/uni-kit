//! `sidebar` — grouped navigation, nested rows, counts, a collapse rail and
//! pinned account/header slots, shown inside workspace frames.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { MenuButton, type MenuItem } from "../components/menu";
import { Sidebar, type SidebarItem, type SidebarSection } from "../components/sidebar";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const accountMenu: MenuItem[] = [
  { label: "Account settings", icon: "user" },
  { label: "Sign out", icon: "log-out" },
];

const accountFooter = () => (
  <View style={{ flexDirection: "row", alignItems: "center", gap: 8, padding: 11, borderTopWidth: 1, borderTopColor: tokens.border }}>
    <Avatar name="Nadia Rahman" size="sm" />
    <View style={{ flex: 1, gap: 1 }}>
      <Text style={{ fontSize: 10, fontWeight: "500", color: tokens.foreground }}>Nadia Rahman</Text>
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>nadia@reui.io</Text>
    </View>
    <MenuButton label="Account menu" items={accountMenu} trigger={<View style={{ width: 20, height: 20, alignItems: "center", justifyContent: "center" }}><Icon name="ellipsis" size={13} color={tokens.mutedForeground} /></View>} align="end" />
  </View>
);

const brand = (label = "ReUI Labs", sub = "Pro plan") => (
  <View style={{ paddingHorizontal: 12, paddingVertical: 14, flexDirection: "row", alignItems: "center", gap: 8 }}>
    <View style={{ width: 24, height: 24, borderRadius: 6, alignItems: "center", justifyContent: "center", backgroundColor: tokens.primary }}>
      <Text style={{ fontSize: 11, fontWeight: "700", color: tokens.primaryForeground }}>U</Text>
    </View>
    <View style={{ flex: 1, gap: 1 }}>
      <Text style={{ fontSize: 11, fontWeight: "600", color: tokens.foreground }}>{label}</Text>
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{sub}</Text>
    </View>
    <Icon name="chevrons-up-down" size={12} color={tokens.mutedForeground} />
  </View>
);

const workspace: SidebarSection[] = [
  {
    id: "workspace",
    title: "Workspace",
    items: [
      { id: "overview", label: "Overview", icon: "layout-dashboard" },
      { id: "inbox", label: "Inbox", icon: "inbox", badge: "12" },
      { id: "deployments", label: "Deployments", icon: "send" },
      { id: "analytics", label: "Analytics", icon: "chart-column" },
      { id: "logs", label: "Logs", icon: "file-text" },
      { id: "storage", label: "Storage", icon: "database" },
    ],
  },
  {
    id: "projects",
    title: "Projects",
    action: <View style={{ width: 16, height: 16, alignItems: "center", justifyContent: "center" }}><Icon name="plus" size={11} color={tokens.mutedForeground} /></View>,
    items: [
      { id: "checkout", label: "Checkout API", icon: "square" },
      { id: "design", label: "Design system", icon: "palette" },
      { id: "marketing", label: "Marketing site", icon: "globe" },
      { id: "edge", label: "Edge cache", icon: "layers" },
    ],
  },
];

const platform: SidebarSection[] = [
  {
    id: "platform",
    title: "Platform",
    items: [
      { id: "overview", label: "Overview", icon: "layout-dashboard" },
      { id: "inbox", label: "Inbox", icon: "inbox", badge: "12" },
      { id: "deployments", label: "Deployments", icon: "send" },
      { id: "monitoring", label: "Monitoring", icon: "activity" },
      { id: "analytics", label: "Analytics", icon: "chart-column" },
      { id: "audit", label: "Audit log", icon: "shield" },
      { id: "settings", label: "Settings", icon: "settings" },
    ],
  },
];

export function Demo() {
  const basicActive = signal("overview");
  const groupActive = signal("deployments");
  const nestedActive = signal("starred");
  const badgeActive = signal("inbox");
  const collapseActive = signal("overview");
  const collapsed = signal(false);
  const widthActive = signal("checkout");
  const contextActive = signal("deployments");
  const fileActive = signal("src");
  const adminActive = signal("overview");
  const settingsActive = signal("general");
  const headerActive = signal("overview");
  const rightActive = signal("overview");

  const frame = (sidebar: any, contentTitle: string, width = 720, reverse = false) => (
    <View style={{ width, maxWidth: "100%", height: 430, flexDirection: "row", borderWidth: 1, borderColor: tokens.border, borderRadius: 12, overflow: "hidden", backgroundColor: tokens.background }}>
      {!reverse ? sidebar : null}
      <View style={{ flex: 1, padding: 18, gap: 10 }}>
        <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}>
          <Icon name="panel-left" size={13} color={tokens.mutedForeground} />
          <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>{contentTitle}</Text>
        </View>
        <Text style={{ fontSize: 11, lineHeight: 16, color: tokens.mutedForeground }}>
          The sidebar owns navigation; the page beside it owns the work. Header and footer stay pinned while the middle scrolls.
        </Text>
        {[0, 1, 2].map((index) => (
          <View style={{ height: 48, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, flexDirection: "row", alignItems: "center", gap: 9, paddingHorizontal: 11 }}>
            {index === 0 ? <Icon name="circle-check" size={13} color={tokens.success} /> : <Icon name="clock" size={13} color={tokens.mutedForeground} />}
            <View style={{ flex: 1, gap: 3 }}>
              <View style={{ width: index === 0 ? "42%" : index === 1 ? "58%" : "35%", height: 7, borderRadius: 4, backgroundColor: tokens.muted }} />
              <View style={{ width: "70%", height: 6, borderRadius: 3, backgroundColor: tokens.muted }} />
            </View>
            <Text style={{ fontSize: 8, color: tokens.mutedForeground }}>{["12m", "1h", "3h"][index]}</Text>
          </View>
        ))}
      </View>
      {reverse ? sidebar : null}
    </View>
  );

  // section: Usage
  const usage = (
    <View style={{ width: 240, height: 380, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, overflow: "hidden" }}>
      <Sidebar
        sections={[{ id: "usage", title: "Workspace", items: [{ id: "overview", label: "Overview", icon: "layout-dashboard" }, { id: "inbox", label: "Inbox", icon: "inbox", badge: "12" }, { id: "deployments", label: "Deployments", icon: "send" }, { id: "settings", label: "Settings", icon: "settings" }] }]}
        active={basicActive()}
        onSelect={(next) => basicActive.set(next)}
        header={brand()}
        footer={accountFooter()}
      />
    </View>
  );
  // section: Basic Sidebar
  const basic = (
    <View style={{ width: 240, height: 400, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, overflow: "hidden" }}>
      <Sidebar sections={[{ id: "main", items: [{ id: "overview", label: "Overview", icon: "layout-dashboard" }, { id: "deployments", label: "Deployments", icon: "send" }, { id: "analytics", label: "Analytics", icon: "chart-column" }, { id: "settings", label: "Settings", icon: "settings" }] }]} active={basicActive()} onSelect={(next) => basicActive.set(next)} header={brand()} footer={accountFooter()} />
    </View>
  );
  // section: Collapsible Sidebar
  const collapsible = frame(
    <Sidebar
      sections={platform}
      active={collapseActive()}
      onSelect={(next) => collapseActive.set(next)}
      collapsed={collapsed()}
      header={
        <View style={{ padding: 10, flexDirection: "row", alignItems: "center", justifyContent: collapsed() ? "center" : "space-between", gap: 8 }}>
          {!collapsed() ? <Text style={{ fontSize: 11, fontWeight: "600", color: tokens.foreground }}>Platform</Text> : null}
          <View accessibilityRole="button" accessibilityLabel="Toggle sidebar" onPress={() => collapsed.set(!collapsed())} style={{ width: 22, height: 22, alignItems: "center", justifyContent: "center", borderRadius: 5, backgroundColor: tokens.muted }}>
            <Icon name="panel-left" size={12} color={tokens.mutedForeground} />
          </View>
        </View>
      }
      footer={!collapsed() ? accountFooter : <View style={{ padding: 10, alignItems: "center" }}><Avatar name="Nadia Rahman" size="sm" /></View>}
    />,
    "Deployments",
  );
  // section: Nested Menu Items
  const nested = frame(
    <Sidebar
      sections={[{ id: "docs", title: "Platform", items: [{ id: "playground", label: "Playground", icon: "box", children: [{ id: "history", label: "History", icon: "clock" }, { id: "starred", label: "Starred", icon: "star" }, { id: "settings", label: "Settings", icon: "settings" }] }, { id: "models", label: "Models", icon: "layers", suffixIcon: "chevron-right" }, { id: "docs", label: "Documentation", icon: "book-open" }] }]}
      active={nestedActive()}
      onSelect={(next) => nestedActive.set(next)}
      width={220}
      footer={accountFooter()}
    />,
    "Playground",
  );
  // section: Multiple Groups
  const groups = frame(<Sidebar sections={workspace} active={groupActive()} onSelect={(next) => groupActive.set(next)} width={224} footer={accountFooter()} />, "Deployments");
  // section: With Badges and Suffixes
  const badges = frame(
    <Sidebar
      sections={[{ id: "workspace", title: "Workspace", items: [
        { id: "inbox", label: "Inbox", icon: "inbox", badge: "12" },
        { id: "prs", label: "Pull requests", icon: "code", badge: "3" },
        { id: "logs", label: "Logs", icon: "file-text", suffixIcon: "external-link" },
        { id: "storage", label: "Storage", icon: "database", badge: "82%" },
      ] }]}
      active={badgeActive()}
      onSelect={(next) => badgeActive.set(next)}
      width={224}
    />,
    "Notifications",
  );
  // section: Right-Side Placement
  const right = frame(<Sidebar sections={workspace} active={rightActive()} onSelect={(next) => rightActive.set(next)} style={{ borderRightWidth: 0, borderLeftWidth: 1, borderLeftColor: tokens.border }} footer={accountFooter()} />, "Overview", 720, true);
  // section: Context Menus
  const context = frame(
    <Sidebar
      sections={[{ id: "deployments", title: "Deployments", items: [
        { id: "production", label: "Production", icon: "globe", trailing: <MenuButton label="Production actions" items={[{ label: "View logs", icon: "file-text" }, { label: "Redeploy", icon: "refresh-cw" }, { label: "Archive", icon: "trash", tone: "danger" }]} trigger={<View style={{ width: 18, height: 18, alignItems: "center", justifyContent: "center" }}><Icon name="ellipsis" size={12} color={tokens.mutedForeground} /></View>} align="end" /> },
        { id: "staging", label: "Staging", icon: "layers", trailing: <MenuButton label="Staging actions" items={[{ label: "View logs", icon: "file-text" }, { label: "Redeploy", icon: "refresh-cw" }]} trigger={<View style={{ width: 18, height: 18, alignItems: "center", justifyContent: "center" }}><Icon name="ellipsis" size={12} color={tokens.mutedForeground} /></View>} align="end" /> },
        { id: "preview", label: "Preview", icon: "eye" },
      ] }]}
      active={contextActive()}
      onSelect={(next) => contextActive.set(next)}
      width={224}
    />,
    "Environment actions",
  );
  // section: Custom Width and Styling
  const custom = (
    <View style={{ width: 260, height: 380, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, overflow: "hidden", backgroundColor: tokens.popover }}>
      <Sidebar
        sections={[{ id: "projects", title: "Projects", items: [{ id: "checkout", label: "Checkout API", icon: "square" }, { id: "design", label: "Design system", icon: "palette" }, { id: "edge", label: "Edge cache", icon: "layers" }] }]}
        active={widthActive()}
        onSelect={(next) => widthActive.set(next)}
        width={260}
        style={{ width: 260, backgroundColor: tokens.muted, borderRightWidth: 0 }}
        header={brand("ReUI Labs", "8 projects")}
      />
    </View>
  );
  // section: Interactive Header with Popup Menu
  const interactiveHeader = frame(
    <Sidebar
      sections={workspace}
      active={headerActive()}
      onSelect={(next) => headerActive.set(next)}
      width={224}
      header={
        <View style={{ padding: 10, flexDirection: "row", alignItems: "center", gap: 7 }}>
          <View style={{ width: 24, height: 24, borderRadius: 6, alignItems: "center", justifyContent: "center", backgroundColor: tokens.primary }}><Text style={{ fontSize: 11, fontWeight: "700", color: tokens.primaryForeground }}>U</Text></View>
          <View style={{ flex: 1, gap: 1 }}><Text style={{ fontSize: 11, fontWeight: "600", color: tokens.foreground }}>ReUI Labs</Text><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Production</Text></View>
          <MenuButton label="Switch workspace" items={[{ label: "ReUI Labs", icon: "check" }, { label: "Design system", icon: "palette" }, { label: "Personal", icon: "user" }]} trigger={<View style={{ width: 20, height: 20, alignItems: "center", justifyContent: "center" }}><Icon name="chevrons-up-down" size={12} color={tokens.mutedForeground} /></View>} align="end" />
        </View>
      }
      footer={accountFooter()}
    />,
    "Workspace",
  );
  // section: Footer with User Information
  const footer = frame(<Sidebar sections={platform.slice(0, 1)} active={settingsActive()} onSelect={(next) => settingsActive.set(next)} width={224} footer={accountFooter()} />, "Account", 660);
  // section: Responsive Sidebar
  const responsive = frame(<Sidebar sections={platform} active={collapseActive()} onSelect={(next) => collapseActive.set(next)} collapsed width={224} footer={<View style={{ padding: 10, alignItems: "center" }}><Avatar name="Nadia Rahman" size="sm" /></View>} />, "Collapsed rail", 660);
  // section: File Explorer Sidebar
  const fileExplorer = frame(
    <Sidebar
      sections={[{ id: "files", title: "Explorer", items: [
        { id: "src", label: "src", icon: "folder", children: [{ id: "components", label: "components", icon: "folder" }, { id: "app", label: "app.tsx", icon: "code" }, { id: "styles", label: "styles.css", icon: "file-text" }] },
        { id: "public", label: "public", icon: "folder" },
        { id: "readme", label: "README.md", icon: "file-text" },
        { id: "package", label: "package.json", icon: "code-xml" },
      ] }]}
      active={fileActive()}
      onSelect={(next) => fileActive.set(next)}
      width={240}
    />,
    "src",
  );
  // section: Admin Dashboard Sidebar
  const admin = frame(<Sidebar sections={platform.map((section) => ({ ...section, title: "Administration" }))} active={adminActive()} onSelect={(next) => adminActive.set(next)} width={224} header={brand("Admin", "Operations")} footer={accountFooter()} />, "Operations", 720);
  // section: Settings Sidebar
  const settings = frame(
    <Sidebar
      sections={[{ id: "settings", title: "Settings", items: [{ id: "general", label: "General", icon: "settings" }, { id: "appearance", label: "Appearance", icon: "palette" }, { id: "notifications", label: "Notifications", icon: "bell", badge: "3" }, { id: "security", label: "Security", icon: "shield" }] }]}
      active={settingsActive()}
      onSelect={(next) => settingsActive.set(next)}
      width={224}
      header={brand("Settings", "Preferences")}
    />,
    "General",
  );
  // section: end
  return (
    <Page>
      <Caption>Sidebar is the navigation rail; the app owns the active route and the content beside it. Header and footer stay pinned.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Sidebar">{basic}</ExampleSection>
      <ExampleSection index={2} title="Collapsible Sidebar">{collapsible}</ExampleSection>
      <ExampleSection index={3} title="Nested Menu Items">{nested}</ExampleSection>
      <ExampleSection index={4} title="Multiple Groups">{groups}</ExampleSection>
      <ExampleSection index={5} title="With Badges and Suffixes">{badges}</ExampleSection>
      <ExampleSection index={6} title="Right-Side Placement">{right}</ExampleSection>
      <ExampleSection index={7} title="Context Menus">{context}</ExampleSection>
      <ExampleSection index={8} title="Custom Width and Styling">{custom}</ExampleSection>
      <ExampleSection index={9} title="Interactive Header with Popup Menu">{interactiveHeader}</ExampleSection>
      <ExampleSection index={10} title="Footer with User Information">{footer}</ExampleSection>
      <ExampleSection index={11} title="Responsive Sidebar">{responsive}</ExampleSection>
      <ExampleSection index={12} title="File Explorer Sidebar">{fileExplorer}</ExampleSection>
      <ExampleSection index={13} title="Admin Dashboard Sidebar">{admin}</ExampleSection>
      <ExampleSection index={14} title="Settings Sidebar">{settings}</ExampleSection>
      <Status>Active: {basicActive()} · {groupActive()} · {settingsActive()}</Status>
    </Page>
  );
}
