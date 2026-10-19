// A complete preferences screen: account pages in the rail, grouped rows, and
// ordinary catalog controls. Shown in the blocks gallery.
import { signal, Scroll, Text, View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Select } from "../components/select";
import { Settings } from "../components/settings";
import { Switch } from "../components/switch";
import { tokens } from "../components/theme";

export function Demo() {
  const active = signal("account");
  const sounds = signal(true);
  const mentions = signal(true);
  const updates = signal(false);
  const theme = signal("system");
  const language = signal("english");
  const density = signal("comfortable");

  const sections = [
    {
      id: "account",
      title: "Account",
      icon: "user",
      description: "Your profile, sign-in address and how teammates reach you.",
      items: [
        { id: "name", label: "Name", description: "Ada Lovelace", icon: "circle-user", onPress: () => {} },
        { id: "email", label: "Email", description: "ada@analytical.engine", icon: "mail", onPress: () => {} },
        { id: "role", label: "Workspace role", description: "Owner", icon: "users", meta: "Owner" },
      ],
    },
    {
      id: "appearance",
      title: "Appearance",
      icon: "palette",
      description: "Theme, density and the language used across the workspace.",
      items: [
        { id: "theme", label: "Theme", description: "Follow the system or pick one", icon: "moon", control: <Select options={[{ value: "light", label: "Light" }, { value: "dark", label: "Dark" }, { value: "system", label: "System" }]} value={theme()} onChange={(next) => theme.set(next)} style={{ width: 120 }} /> },
        { id: "density", label: "Density", description: "Spacing between rows", icon: "layout-dashboard", control: <Select options={[{ value: "compact", label: "Compact" }, { value: "comfortable", label: "Comfortable" }, { value: "spacious", label: "Spacious" }]} value={density()} onChange={(next) => density.set(next)} style={{ width: 140 }} /> },
        { id: "language", label: "Language", description: "Menu and message language", icon: "globe", control: <Select options={[{ value: "english", label: "English" }, { value: "german", label: "German" }, { value: "french", label: "French" }]} value={language()} onChange={(next) => language.set(next)} style={{ width: 120 }} /> },
      ],
    },
    {
      id: "notifications",
      title: "Notifications",
      icon: "bell",
      description: "Choose what reaches you and how often.",
      items: [
        { id: "sounds", label: "Sounds", description: "Play a sound for new activity", icon: "bell", control: <Switch checked={sounds()} onChange={(next) => sounds.set(next)} /> },
        { id: "mentions", label: "Mentions", description: "When someone mentions you", icon: "message-circle", control: <Switch checked={mentions()} onChange={(next) => mentions.set(next)} /> },
        { id: "updates", label: "Product updates", description: "A short monthly email", icon: "mail", control: <Switch checked={updates()} onChange={(next) => updates.set(next)} /> },
      ],
    },
    {
      id: "security",
      title: "Security",
      icon: "shield",
      description: "Protect the account and review where it is signed in.",
      items: [
        { id: "password", label: "Password", description: "Last changed 3 months ago", icon: "lock", onPress: () => {} },
        { id: "two-factor", label: "Two-factor authentication", description: "Authenticator app", icon: "shield", meta: "On" },
        { id: "sessions", label: "Active sessions", description: "2 devices", icon: "cpu", onPress: () => {} },
      ],
    },
    {
      id: "billing",
      title: "Billing",
      icon: "table",
      description: "Your plan, invoices and payment method.",
      items: [
        { id: "plan", label: "Plan", description: "Pro, billed yearly", icon: "sparkles", meta: "$19/mo" },
        { id: "payment", label: "Payment method", description: "Visa ending in 4242", icon: "hash", onPress: () => {} },
        { id: "invoices", label: "Invoices", description: "Receipts and past payments", icon: "file-text", onPress: () => {} },
      ],
    },
    {
      id: "advanced",
      title: "Advanced",
      icon: "settings",
      description: "Developer options and account removal.",
      items: [
        { id: "api", label: "API keys", description: "Create and revoke keys", icon: "code", onPress: () => {} },
        { id: "export", label: "Export data", description: "Download a copy of your workspace", icon: "download", onPress: () => {} },
        { id: "delete", label: "Delete account", description: "This cannot be undone", icon: "trash", danger: true, onPress: () => {} },
      ],
    },
  ];

  return (
    <View style={{ flex: 1, backgroundColor: tokens.background }}>
      <View style={{ height: 56, paddingHorizontal: 24, flexDirection: "row", alignItems: "center", gap: 10, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
        <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Workspace</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>/ Settings</Text>
        <View style={{ flex: 1 }} />
        <Button label="Cancel" size="small" variant="ghost" onPress={() => {}} />
        <Button label="Save changes" size="small" variant="primary" onPress={() => {}} />
      </View>
      <Scroll style={{ flex: 1 }}>
        <View style={{ padding: 28, paddingBottom: 56, alignItems: "center" }}>
          <View style={{ width: 860, maxWidth: "100%" }}>
            <Settings sections={sections} active={active()} onChange={(next) => active.set(next)} title="Settings" description="Manage your account, appearance, notifications, security and billing." />
          </View>
        </View>
      </Scroll>
    </View>
  );
}
