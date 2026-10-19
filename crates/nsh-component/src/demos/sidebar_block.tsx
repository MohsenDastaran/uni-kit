// A workspace shell: grouped navigation beside the content it controls.
import { signal, Scroll, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Card } from "../components/card";
import { Icon } from "../components/icon";
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
        items={[
          { id: "home", label: "Overview", icon: "layout-dashboard" },
          { id: "inbox", label: "Inbox", icon: "inbox", badge: "8" },
          { id: "projects", label: "Projects", icon: "folder" },
          { id: "settings", label: "Settings", icon: "settings" },
        ]}
        active={active()}
        onSelect={navigate}
        width={224}
        header={
          <View style={{ paddingHorizontal: 14, paddingVertical: 16, gap: 2 }}>
            <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
              <View style={{ width: 26, height: 26, borderRadius: 7, alignItems: "center", justifyContent: "center", backgroundColor: tokens.primary }}>
                <Icon name="layers" size={14} color={tokens.primaryForeground} />
              </View>
              <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Northstar</Text>
            </View>
            <Text style={{ fontSize: 10, color: tokens.mutedForeground, paddingStart: 34 }}>Acme workspace</Text>
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
          <Badge label="Live" variant="secondary" />
          <View style={{ flex: 1 }} />
          <Button label="New project" size="small" variant="primary" icon="plus" onPress={() => {}} />
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
