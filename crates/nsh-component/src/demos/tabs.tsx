//! `tabs` — underline, segmented, pill and enclosed tab rows with icons,
//! badges, scrolling, dynamic tabs and close actions.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { MenuButton } from "../components/menu";
import { Tabs, type TabItem } from "../components/tabs";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const usage = signal("account");
  const basic = signal("account");
  const variants = signal("account");
  const sizes = signal("account");
  const icons = signal("account");
  const prefixes = signal("account");
  const disabled = signal("account");
  const dynamic = signal("account");
  const menu = signal("overview");
  const maximum = signal("account");
  const scroll = signal("account");
  const individual = signal("account");
  const custom = signal("billing");
  const state = signal("account");
  const close = signal("account");
  const name = signal("Sarah Johnson");
  const username = signal("@sarahj");
  const email = signal("sarah.johnson@example.com");
  const cardNumber = signal("4242 4242 4242 4242");
  const dynamicItems = signal<TabItem[]>([
    { value: "account", label: "Account", closeable: true },
    { value: "billing", label: "Billing", closeable: true },
  ]);
  const closeItems = signal<TabItem[]>([
    { value: "overview", label: "Overview", closeable: true },
    { value: "activity", label: "Activity", closeable: true },
    { value: "settings", label: "Settings", closeable: true },
  ]);
  const stage = (children: any, width = 720) => <View style={{ width, maxWidth: "100%" }}>{children}</View>;
  const panel = (children: any) => (
    <View style={{ padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover, gap: 10 }}>
      {children}
    </View>
  );
  const simple = (content: string) => <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{content}</Text>;
  const accountPanel = () => panel(
    <View style={{ gap: 10 }}>
      <View style={{ gap: 2 }}>
        <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Account</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Update your account information.</Text>
      </View>
      <Input label="Name" value={name()} onChangeText={(next) => name.set(next)} placeholder="Sarah Johnson" />
      <Input label="Username" value={username()} onChangeText={(next) => username.set(next)} placeholder="@sarahj" />
      <Input label="Email" value={email()} onChangeText={(next) => email.set(next)} placeholder="sarah.johnson@example.com" />
      <Button label="Save changes" size="small" variant="primary" onPress={() => {}} style={{ alignSelf: "flex-start" }} />
    </View>,
  );
  const accountItems = (active: () => string, set: (next: string) => void): TabItem[] => [
    { value: "account", label: "Account", icon: "user", content: accountPanel() },
    { value: "password", label: "Password", icon: "lock", content: panel(<Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Change your password and enable two-factor authentication.</Text>) },
    { value: "settings", label: "Settings", icon: "settings", content: panel(<Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Workspace preferences and notification settings.</Text>) },
  ];

  // section: Usage
  const usageView = stage(
    <Tabs items={accountItems(usage, (next) => usage.set(next))} value={usage()} onChange={(next) => usage.set(next)} />,
    440,
  );
  // section: Basic Tabs
  const basicView = stage(
    <Tabs
      items={[
        { value: "account", label: "Account", content: panel(simple("Account settings.")) },
        { value: "billing", label: "Billing", content: panel(simple("Invoices, receipts and payment methods.")) },
        { value: "team", label: "Team", content: panel(simple("People and their permissions.")) },
      ]}
      value={basic()}
      onChange={(next) => basic.set(next)}
    />,
  );
  // section: Tab Variants
  const variantsView = stage(
    <View style={{ gap: 16 }}>
      <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Segmented</Text><Tabs items={[ { value: "account", label: "Account", icon: "user" }, { value: "password", label: "Password", icon: "lock" }, { value: "settings", label: "Settings", icon: "settings" } ]} value={variants()} onChange={(next) => variants.set(next)} variant="segmented" /></View>
      <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Underline</Text><Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={variants()} onChange={(next) => variants.set(next)} variant="underline" /></View>
      <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Pills</Text><Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={variants()} onChange={(next) => variants.set(next)} variant="pills" /></View>
      <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Enclosed</Text><Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={variants()} onChange={(next) => variants.set(next)} variant="enclosed" fullWidth /></View>
    </View>,
  );
  // section: Tab Sizes
  const sizesView = stage(
    <View style={{ gap: 14 }}>
      <Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={sizes()} onChange={(next) => sizes.set(next)} size="xsmall" />
      <Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={sizes()} onChange={(next) => sizes.set(next)} size="small" />
      <Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={sizes()} onChange={(next) => sizes.set(next)} />
      <Tabs items={[{ value: "account", label: "Account" }, { value: "password", label: "Password" }, { value: "settings", label: "Settings" }]} value={sizes()} onChange={(next) => sizes.set(next)} size="large" />
    </View>,
  );
  // section: Tabs with Icons
  const iconsView = stage(
    <Tabs
      items={[
        { value: "account", label: "Account", icon: "user", content: accountPanel() },
        { value: "password", label: "Password", icon: "lock", content: panel(simple("Password and sign-in security.")) },
        { value: "settings", label: "Settings", icon: "settings", content: panel(simple("Workspace preferences.")) },
      ]}
      value={icons()}
      onChange={(next) => icons.set(next)}
      variant="segmented"
      fullWidth
    />,
  );
  // section: Tabs with Prefix and Suffix
  const prefixesView = stage(
    <Tabs
      items={[
        { value: "account", label: "Account", prefix: <Icon name="user" size={12} />, suffix: <Icon name="circle-check" size={11} color={tokens.success} /> },
        { value: "billing", label: "Billing", prefix: <Icon name="hash" size={12} />, badge: "3" },
        { value: "settings", label: "Settings", prefix: <Icon name="settings" size={12} /> },
      ]}
      value={prefixes()}
      onChange={(next) => prefixes.set(next)}
      variant="pills"
    />,
  );
  // section: Disabled Tabs
  const disabledView = stage(
    <Tabs
      items={[
        { value: "account", label: "Account", content: panel(simple("Account is available.")) },
        { value: "billing", label: "Billing", disabled: true },
        { value: "team", label: "Team", disabled: true },
      ]}
      value={disabled()}
      onChange={(next) => disabled.set(next)}
    />,
  );
  // section: Dynamic Tabs
  const dynamicView = stage(
    <View style={{ gap: 10 }}>
      <Tabs
        items={dynamicItems()}
        value={dynamic()}
        onChange={(next) => dynamic.set(next)}
        onClose={(next) => {
          dynamicItems.update((items) => items.filter((item) => item.value !== next));
          if (dynamic() === next) dynamic.set(dynamicItems()[0]?.value ?? "");
        }}
      />
      <Row gap={8}>
        <Button
          label="Add tab"
          size="small"
          variant="secondary"
          icon="plus"
          onPress={() => {
            const next = `tab-${dynamicItems().length + 1}`;
            dynamicItems.update((items) => [...items, { value: next, label: `Tab ${items.length + 1}`, closeable: true }]);
            dynamic.set(next);
          }}
        />
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{dynamicItems().length} open tabs</Text>
      </Row>
    </View>,
  );
  // section: Tabs with Menu
  const menuView = stage(
    <Row gap={8} style={{ alignItems: "flex-start" }}>
      <View style={{ flex: 1 }}>
        <Tabs
          items={[
            { value: "overview", label: "Overview", icon: "layout-dashboard" },
            { value: "analytics", label: "Analytics", icon: "chart-column" },
            { value: "settings", label: "Settings", icon: "settings" },
          ]}
          value={menu()}
          onChange={(next) => menu.set(next)}
          variant="segmented"
          fullWidth
        />
      </View>
      <MenuButton label="Tab actions" items={[{ label: "Duplicate tab", icon: "copy" }, { label: "Close others", icon: "x" }, { label: "Reorder", icon: "grip-vertical" }]} trigger={<View style={{ width: 28, height: 28, alignItems: "center", justifyContent: "center", borderRadius: 6, backgroundColor: tokens.muted }}><Icon name="ellipsis" size={13} color={tokens.mutedForeground} /></View>} align="end" />
    </Row>,
  );
  // section: Maximum Tab Width
  const maximumView = stage(
    <Tabs
      items={[
        { value: "account", label: "Account" },
        { value: "billing", label: "Billing and invoices" },
        { value: "team", label: "Team and permissions" },
        { value: "settings", label: "Workspace settings" },
      ]}
      value={maximum()}
      onChange={(next) => maximum.set(next)}
      maxTabWidth={130}
    />,
  );
  // section: Scrollable Tabs
  const scrollView = stage(
    <Tabs
      items={Array.from({ length: 14 }, (_, index) => ({ value: `tab-${index}`, label: `Workspace tab ${index + 1}` }))}
      value={scroll()}
      onChange={(next) => scroll.set(next)}
      scrollable
    />,
  );
  // section: Individual Tab Configuration
  const individualView = stage(
    <Tabs
      items={[
        { value: "account", label: "Account", icon: "user", suffix: <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>12</Text> },
        { value: "billing", label: "Billing", icon: "hash", badge: "New", closeable: true },
        { value: "team", label: "Team", icon: "users", disabled: true },
      ]}
      value={individual()}
      onChange={(next) => individual.set(next)}
      onClose={() => {}}
      variant="enclosed"
      fullWidth
    />,
  );
  // section: Custom Tab Content
  const customView = stage(
    <Tabs
      items={[
        { value: "billing", label: "Billing", icon: "hash", content: panel(<View style={{ gap: 10 }}><Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Payment method</Text><Input label="Card number" value={cardNumber()} onChangeText={(next) => cardNumber.set(next)} prefixIcon="hash" /><Row gap={8}><Button label="Update card" size="small" variant="primary" onPress={() => {}} /><Button label="Cancel" size="small" variant="ghost" onPress={() => {}} /></Row></View>) },
        { value: "invoices", label: "Invoices", icon: "file-text", content: panel(simple("No invoices yet.")) },
      ]}
      value={custom()}
      onChange={(next) => custom.set(next)}
      variant="segmented"
      fullWidth
    />,
  );
  // section: Tabs with State Management
  const stateView = stage(
    <View style={{ gap: 10 }}>
      <Tabs
        items={[
          { value: "account", label: "Account", content: panel(simple("Account settings are controlled by the parent.")) },
          { value: "billing", label: "Billing", content: panel(simple("Billing settings are controlled by the parent.")) },
        ]}
        value={state()}
        onChange={(next) => state.set(next)}
      />
      <Row gap={8}><Button label="Account" size="small" variant={state() === "account" ? "primary" : "secondary"} onPress={() => state.set("account")} /><Button label="Billing" size="small" variant={state() === "billing" ? "primary" : "secondary"} onPress={() => state.set("billing")} /><Text style={{ fontSize: 10, color: tokens.mutedForeground, alignSelf: "center" }}>active: {state()}</Text></Row>
    </View>,
  );
  // section: Tabs with Close Buttons
  const closeView = stage(
    <View style={{ gap: 10 }}>
      <Tabs
        items={closeItems()}
        value={close()}
        onChange={(next) => close.set(next)}
        onClose={(next) => {
          closeItems.update((items) => items.filter((item) => item.value !== next));
          if (close() === next) close.set(closeItems()[0]?.value ?? "");
        }}
        variant="enclosed"
        fullWidth
      />
      <Button label="Reset tabs" size="small" variant="ghost" onPress={() => closeItems.set([{ value: "overview", label: "Overview", closeable: true }, { value: "activity", label: "Activity", closeable: true }, { value: "settings", label: "Settings", closeable: true }])} />
    </View>,
  );
  // section: end
  return (
    <Page>
      <Caption>Tabs switch between sibling panels; the application owns the panel content and the active value.</Caption>
      <ExampleSection index={0} title="Usage">{usageView}</ExampleSection>
      <ExampleSection index={1} title="Basic Tabs">{basicView}</ExampleSection>
      <ExampleSection index={2} title="Tab Variants">{variantsView}</ExampleSection>
      <ExampleSection index={3} title="Tab Sizes">{sizesView}</ExampleSection>
      <ExampleSection index={4} title="Tabs with Icons">{iconsView}</ExampleSection>
      <ExampleSection index={5} title="Tabs with Prefix and Suffix">{prefixesView}</ExampleSection>
      <ExampleSection index={6} title="Disabled Tabs">{disabledView}</ExampleSection>
      <ExampleSection index={7} title="Dynamic Tabs">{dynamicView}</ExampleSection>
      <ExampleSection index={8} title="Tabs with Menu">{menuView}</ExampleSection>
      <ExampleSection index={9} title="Maximum Tab Width">{maximumView}</ExampleSection>
      <ExampleSection index={10} title="Scrollable Tabs">{scrollView}</ExampleSection>
      <ExampleSection index={11} title="Individual Tab Configuration">{individualView}</ExampleSection>
      <ExampleSection index={12} title="Custom Tab Content">{customView}</ExampleSection>
      <ExampleSection index={13} title="Tabs with State Management">{stateView}</ExampleSection>
      <ExampleSection index={14} title="Tabs with Close Buttons">{closeView}</ExampleSection>
    </Page>
  );
}
