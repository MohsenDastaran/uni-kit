// A workspace shell: grouped navigation beside the content it controls.
import { signal, Scroll, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Card } from "../components/card";
import { Icon } from "../components/icon";
import { MenuButton } from "../components/menu";
import { Progress } from "../components/progress";
import { Sidebar } from "../components/sidebar";
import { tokens } from "../components/theme";

const stats = [
  ["Open issues", "128", "+12 this week"],
  ["Deploys today", "24", "All green"],
  ["Response time", "184ms", "p95, last hour"],
];

const activity = [
  ["Release 4.2 promoted", "Sofia · 2 minutes ago", "circle-check"],
  ["Invoices synced", "Billing worker · 18 minutes ago", "refresh-cw"],
  ["Search index rebuilt", "Search worker · 42 minutes ago", "search"],
  ["New member invited", "Ada · 1 hour ago", "user"],
];

export function Demo() {
  const active = signal("home");
  const title = () => ({ home: "Overview", inbox: "Inbox", projects: "Projects", settings: "Settings" })[active()] ?? "Overview";
  const navigate = (next: string) => active.set(next);
  return (
    <View style={{ flex: 1, flexDirection: "row", backgroundColor: tokens.background }}>
      <Sidebar
        sections={[
          {
            id: "workspace",
            title: "Workspace",
            items: [
              { id: "home", label: "Overview", icon: "layout-dashboard" },
              { id: "inbox", label: "Inbox", icon: "inbox", badge: "8" },
              { id: "projects", label: "Projects", icon: "folder", children: [{ id: "checkout", label: "Checkout API", icon: "square" }, { id: "design", label: "Design system", icon: "palette" }] },
              { id: "settings", label: "Settings", icon: "settings" },
            ],
          },
          {
            id: "teams",
            title: "Teams",
            items: [{ id: "engineering", label: "Engineering", icon: "users", badge: "12" }, { id: "design-team", label: "Design", icon: "palette" }],
          },
        ]}
        active={active()}
        onSelect={navigate}
        width={224}
        header={
          <View style={{ paddingHorizontal: 14, paddingVertical: 14 }}>
            <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
              <View style={{ width: 26, height: 26, borderRadius: 7, alignItems: "center", justifyContent: "center", backgroundColor: tokens.primary }}>
                <Icon name="layers" size={14} color={tokens.primaryForeground} />
              </View>
              <View style={{ flex: 1, gap: 1 }}>
                <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Northstar</Text>
                <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Acme workspace</Text>
              </View>
              <MenuButton label="Switch workspace" items={[{ label: "Acme workspace", icon: "check" }, { label: "Personal", icon: "user" }, { label: "Create workspace", icon: "plus" }]} trigger={<View style={{ width: 20, height: 20, alignItems: "center", justifyContent: "center" }}><Icon name="chevrons-up-down" size={12} color={tokens.mutedForeground} /></View>} align="end" />
            </View>
          </View>
        }
        footer={
          <View style={{ padding: 12, borderTopWidth: 1, borderTopColor: tokens.border, flexDirection: "row", alignItems: "center", gap: 8 }}>
            <Avatar name="Ada Lovelace" size="sm" />
            <View style={{ flex: 1, gap: 1 }}>
              <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>Ada Lovelace</Text>
              <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Owner</Text>
            </View>
            <Icon name="chevrons-up-down" size={12} color={tokens.mutedForeground} />
          </View>
        }
      />
      <View style={{ flex: 1 }}>
        <View style={{ height: 56, paddingHorizontal: 22, flexDirection: "row", alignItems: "center", gap: 10, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
          <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>{title()}</Text>
          <Badge label="Live" variant="secondary" style={{ alignSelf: "center" }} />
          <View style={{ flex: 1 }} />
          <Button label="New project" size="small" variant="primary" icon="plus" onPress={() => {}} style={{ alignSelf: "center" }} />
        </View>
        <Scroll style={{ flex: 1 }}>
          <View style={{ padding: 22, gap: 18 }}>
            <View style={{ flexDirection: "row", gap: 12, flexWrap: "wrap" }}>
              {stats.map(([label, value, note]) => (
                <Card style={{ flex: 1, minWidth: 170 }} contentStyle={{ gap: 4 }}>
                  <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{label}</Text>
                  <Text style={{ fontSize: 24, fontWeight: "600", color: tokens.foreground }}>{value}</Text>
                  <Text style={{ fontSize: 9, color: tokens.success }}>{note}</Text>
                </Card>
              ))}
            </View>
            <View style={{ flexDirection: "row", gap: 14, flexWrap: "wrap" }}>
              <Card title="Recent activity" style={{ flex: 2, minWidth: 320 }}>
                <View style={{ gap: 0 }}>
                  {activity.map(([label, meta, icon]) => (
                    <View style={{ flexDirection: "row", alignItems: "center", gap: 10, paddingVertical: 8, borderTopWidth: 1, borderTopColor: tokens.border }}>
                      <View style={{ width: 26, height: 26, borderRadius: 7, alignItems: "center", justifyContent: "center", backgroundColor: tokens.muted }}>
                        <Icon name={icon} size={13} color={tokens.mutedForeground} />
                      </View>
                      <View style={{ flex: 1, gap: 1 }}>
                        <Text style={{ fontSize: 11, color: tokens.foreground }}>{label}</Text>
                        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{meta}</Text>
                      </View>
                      <Icon name="chevron-right" size={12} color={tokens.mutedForeground} />
                    </View>
                  ))}
                </View>
              </Card>
              <Card title="Team" style={{ flex: 1, minWidth: 220 }}>
                <View style={{ gap: 8 }}>
                  {["Ada Lovelace", "Sofia Reyes", "Michael Rodriguez"].map((name) => (
                    <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
                      <Avatar name={name} size="sm" />
                      <Text style={{ flex: 1, fontSize: 10, color: tokens.foreground }}>{name}</Text>
                      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Active</Text>
                    </View>
                  ))}
                </View>
              </Card>
            </View>
            <Card title="Indexing" description="Search is rebuilding in the background.">
              <View style={{ gap: 8 }}>
                <Progress value={68} label="Search index" size="small" status="info" />
                <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
                  <Icon name="loader-circle" size={12} color={tokens.info} />
                  <Text style={{ flex: 1, fontSize: 9, color: tokens.mutedForeground }}>About 3 minutes remaining</Text>
                  <Button label="View job" size="xsmall" variant="ghost" onPress={() => {}} />
                </View>
              </View>
            </Card>
            <Card title="Getting started" description="Three steps to finish the workspace setup." footer={<Button label="Continue setup" size="small" variant="primary" onPress={() => {}} />} footerAlign="start" footerBordered>
              <View style={{ gap: 7 }}>
                {["Invite the rest of the team", "Connect the billing account", "Create the first project"].map((step, index) => (
                  <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
                    <View style={{ width: 18, height: 18, borderRadius: 9, alignItems: "center", justifyContent: "center", backgroundColor: index === 0 ? tokens.success : tokens.muted }}>
                      <Text style={{ fontSize: 8, color: index === 0 ? tokens.primaryForeground : tokens.mutedForeground }}>{index + 1}</Text>
                    </View>
                    <Text style={{ fontSize: 10, color: index === 0 ? tokens.foreground : tokens.mutedForeground }}>{step}</Text>
                  </View>
                ))}
              </View>
            </Card>
          </View>
        </Scroll>
      </View>
    </View>
  );
}
