//! `settings` — a preferences screen with a section rail, grouped rows and
//! ordinary controls supplied by the application.
import { signal, View } from "nsh";
import { Button } from "../components/button";
import { Select } from "../components/select";
import { Settings, SettingsPage, SettingsRow } from "../components/settings";
import { Input } from "../components/input";
import { Switch } from "../components/switch";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const active = signal("account");
  const sounds = signal(true);
  const notifications = signal(true);
  const updates = signal(false);
  const theme = signal("system");
  const language = signal("english");
  const density = signal("comfortable");
  const displayName = signal("Ada Lovelace");

  const page = (sections: any[], activeId: string, onActive: (next: string) => void, title = "Settings", description = "Manage how your workspace looks, behaves and keeps you informed.") => (
    <Settings
      sections={sections}
      active={activeId}
      onChange={onActive}
      title={title}
      description={description}
      style={{ width: 780, maxWidth: "100%" }}
    />
  );
  const sections = [
    {
      id: "account",
      title: "Account",
      icon: "user",
      description: "Your profile, sign-in address and how teammates reach you.",
      items: [
        { id: "name", label: "Name", description: "Ada Lovelace", icon: "circle-user", onPress: () => {} },
        { id: "email", label: "Email", description: "ada@analytical.engine", icon: "mail", onPress: () => {} },
        { id: "role", label: "Role", description: "Owner", icon: "users", meta: "Owner" },
      ],
    },
    {
      id: "appearance",
      title: "Appearance",
      icon: "palette",
      description: "Theme, density and the language the interface uses.",
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
        { id: "mentions", label: "Mentions", description: "When someone mentions you", icon: "message-circle", control: <Switch checked={notifications()} onChange={(next) => notifications.set(next)} /> },
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

  // section: Usage
  const usage = page(sections.slice(0, 1), active(), (next) => active.set(next), "Settings", "A settings page is a section rail, a page heading and grouped rows.");
  // section: Basic Settings
  const basic = page(sections.slice(0, 3), active(), (next) => active.set(next));
  // section: With Multiple Pages
  const multiple = page(sections, active(), (next) => active.set(next), "Workspace settings", "Six sections, each with its own group of preferences.");
  // section: Group Variants
  const groups = (
    <View style={{ width: 560, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[
          { id: "account", title: "Account", items: [{ id: "name", label: "Name", description: "Ada Lovelace", icon: "circle-user", onPress: () => {} }, { id: "email", label: "Email", description: "ada@analytical.engine", icon: "mail", onPress: () => {} }] },
          { id: "appearance", title: "Appearance", items: [{ id: "theme", label: "Theme", description: "System", icon: "moon", onPress: () => {} }] },
        ]}
        sidebarWidth={130}
        title="Account"
        description="A compact group for a dialog or a narrow pane."
        style={{ width: 520 }}
      />
    </View>
  );
  // section: Complete Settings Example
  const complete = (
    <View style={{ width: 820, maxWidth: "100%", padding: 18, borderWidth: 1, borderColor: tokens.border, borderRadius: 14, backgroundColor: tokens.background }}>
      {page(sections, active(), (next) => active.set(next), "Settings", "Manage your account, appearance, notifications and billing in one place.")}
    </View>
  );
  // section: Setting Page
  const settingPage = (
    <View style={{ width: 500, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover, gap: 12 }}>
      <SettingsPage title="Account" description="Profile and sign-in details.">
        <Settings sections={[sections[0]]} sidebarWidth={0} style={{ width: 460 }} />
      </SettingsPage>
    </View>
  );
  // section: Basic Page
  const basicPage = (
    <View style={{ width: 420, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings sections={[sections[0]]} sidebarWidth={120} title="Account" style={{ width: 380 }} />
    </View>
  );
  // section: Multiple Groups
  const multipleGroups = page(sections.slice(0, 4), active(), (next) => active.set(next), "Preferences", "Account, appearance, notifications and security.");
  // section: Icon
  const icons = page(sections.map((section) => ({ ...section, icon: section.icon ?? "settings" })), active(), (next) => active.set(next), "Workspace settings", "Every section carries an icon in the rail.");
  // section: Title Suffix
  const suffix = (
    <View style={{ width: 500, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[{ id: "account", title: "Account", description: "Profile and sign-in details.", items: [{ id: "name", label: "Name", description: "Ada Lovelace", icon: "circle-user", onPress: () => {} }, { id: "email", label: "Email", description: "ada@analytical.engine", icon: "mail", meta: "Verified", onPress: () => {} }] }]}
        title="Account · Personal"
        description="The title can name the workspace and the section together."
        style={{ width: 460 }}
      />
    </View>
  );
  // section: Single Item
  const singleItem = (
    <View style={{ width: 560, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[{ id: "sounds", title: "Sounds", items: [{ id: "sounds", label: "Play sounds", description: "For new activity in this workspace", icon: "bell", control: <Switch checked={sounds()} onChange={(next) => sounds.set(next)} /> }] }]}
        sidebarWidth={0}
        style={{ width: 408 }}
      />
    </View>
  );
  // section: Disabled
  const disabled = (
    <View style={{ width: 560, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[{ id: "managed", title: "Managed by your organization", items: [{ id: "sso", label: "Single sign-on", description: "Required by your administrator", icon: "shield", meta: "Enforced", disabled: true }, { id: "api", label: "API access", description: "Disabled for this plan", icon: "code", disabled: true }] }]}
        sidebarWidth={0}
        style={{ width: 408 }}
      />
    </View>
  );
  // section: Search Keywords
  const keywords = (
    <View style={{ width: 460, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[{ id: "account", title: "Account", items: [{ id: "email", label: "Email", description: "ada@analytical.engine", icon: "mail", meta: "Searchable", onPress: () => {} }, { id: "username", label: "Username", description: "@ada", icon: "hash", meta: "Searchable", onPress: () => {} }, { id: "name", label: "Display name", description: "Ada Lovelace", icon: "circle-user", onPress: () => {} }] }]}
        title="Account fields"
        description="Each row can carry a short meta label for filtering."
        sidebarWidth={0}
        style={{ width: 428 }}
      />
    </View>
  );
  // section: Switch
  const switchField = (
    <View style={{ width: 420, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover, gap: 10 }}>
      <SettingsRow label="Desktop notifications" description="Show a toast when a release finishes.">
        <Switch checked={notifications()} onChange={(next) => notifications.set(next)} />
      </SettingsRow>
      <SettingsRow label="Sounds" description="Play a sound for direct mentions.">
        <Switch checked={sounds()} onChange={(next) => sounds.set(next)} />
      </SettingsRow>
    </View>
  );
  // section: Input
  const inputField = (
    <View style={{ width: 420, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[{ id: "account", title: "Account", items: [{ id: "name", label: "Display name", description: "Shown across the workspace", icon: "circle-user", control: <Input value={displayName()} onChangeText={(next) => displayName.set(next)} style={{ width: 160 }} /> }] }]}
        sidebarWidth={0}
        style={{ width: 388 }}
      />
    </View>
  );
  // section: Dropdown
  const dropdown = (
    <View style={{ width: 560, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      <Settings
        sections={[{ id: "appearance", title: "Appearance", items: [{ id: "theme", label: "Theme", description: "Follow the system or pick one", icon: "moon", control: <Select options={[{ value: "light", label: "Light" }, { value: "dark", label: "Dark" }, { value: "system", label: "System" }]} value={theme()} onChange={(next) => theme.set(next)} style={{ width: 120 }} /> }, { id: "language", label: "Language", description: "Menu and message language", icon: "globe", control: <Select options={[{ value: "english", label: "English" }, { value: "german", label: "German" }]} value={language()} onChange={(next) => language.set(next)} style={{ width: 120 }} /> }] }]}
        sidebarWidth={0}
        style={{ width: 408 }}
      />
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>Settings is a layout over grouped rows; the controls in each row are ordinary catalog controls.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Settings">{basic}</ExampleSection>
      <ExampleSection index={2} title="With Multiple Pages">{multiple}</ExampleSection>
      <ExampleSection index={3} title="Group Variants">{groups}</ExampleSection>
      <ExampleSection index={4} title="Complete Settings Example">{complete}</ExampleSection>
      <ExampleSection index={5} title="Setting Page">{settingPage}</ExampleSection>
      <ExampleSection index={6} title="Basic Page">{basicPage}</ExampleSection>
      <ExampleSection index={7} title="Multiple Groups">{multipleGroups}</ExampleSection>
      <ExampleSection index={8} title="Icon">{icons}</ExampleSection>
      <ExampleSection index={9} title="Title Suffix">{suffix}</ExampleSection>
      <ExampleSection index={10} title="Single Item">{singleItem}</ExampleSection>
      <ExampleSection index={11} title="Disabled">{disabled}</ExampleSection>
      <ExampleSection index={12} title="Search Keywords">{keywords}</ExampleSection>
      <ExampleSection index={13} title="Switch">{switchField}</ExampleSection>
      <ExampleSection index={14} title="Input">{inputField}</ExampleSection>
      <ExampleSection index={15} title="Dropdown">{dropdown}</ExampleSection>
      <Status>Active: {active()} · sounds {sounds() ? "on" : "off"} · theme {theme()}</Status>
    </Page>
  );
}
